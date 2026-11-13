//! Replacing the running install, and reading back how that went.

use std::fs;
use std::path::Path;
use std::process::Child;
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::{App, Error, InstallKind};

/// Why the last update was not installed, read at the next start. Words are the app's.
///
/// More reasons may come: match with a `_` arm that says the update was not installed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Failure {
    /// The app did not close in time: nothing was changed.
    StillRunning,
    /// The disk image could not be opened.
    Mount,
    /// The disk image holds no app of this name.
    Contents,
    /// The new version could not be copied next to the installed one.
    Copy,
    /// macOS did not allow replacing the app (System Settings → Privacy & Security → App
    /// Management).
    Replace,
    /// The installer (Windows: `msiexec`) failed; the installed version stays.
    Install,
    /// The person declined the installer or its administrator prompt; nothing was changed.
    Cancelled,
    Other,
}

/// Installs the package `download` returned and starts the new version. On `Ok` the caller must
/// exit at once (`std::process::exit(0)`, not a window close that something may veto): the new
/// copy is already starting, or the macOS helper is waiting for this process to end.
///
/// `before_restart` runs first and can refuse (unsaved work) by returning the reason, which
/// comes back as [`Error::Refused`]; the downloaded package stays in the cache.
pub fn apply(
    app: &App,
    package: &Path,
    before_restart: impl FnOnce() -> Result<(), String>,
) -> Result<(), Error> {
    let kind = crate::install_kind(app);
    if !kind.can_apply_in_place() {
        return Err(Error::NotUpdatable);
    }
    before_restart().map_err(Error::Refused)?;
    match kind {
        InstallKind::AppImage(current) => apply_appimage(app, &current, package),
        InstallKind::MacBundle(bundle) => apply_macos(app, &bundle, package),
        InstallKind::WindowsMsi { dir, per_machine } => {
            apply_windows(app, &dir, per_machine, package)
        }
        InstallKind::Portable(_) | InstallKind::NotWritable(_) | InstallKind::Managed => {
            Err(Error::NotUpdatable)
        }
    }
}

/// How long a freshly started AppImage has to stay up, or exit cleanly, before the one it
/// replaced is let go. A release that verifies but dies on start must leave the old one usable.
const STARTUP_GRACE: Duration = Duration::from_secs(3);

/// Moves the running AppImage aside, puts the new one in its place and starts it. The running
/// file stays open under its new name, so this is safe on Linux; any failure puts it back, and
/// so does a new version that crashes during its first seconds.
pub(crate) fn apply_appimage(app: &App, current: &Path, package: &Path) -> Result<(), Error> {
    apply_appimage_within(app, current, package, STARTUP_GRACE)
}

fn apply_appimage_within(
    app: &App,
    current: &Path,
    package: &Path,
    grace: Duration,
) -> Result<(), Error> {
    let io = |what: &str, e: std::io::Error| Error::Io(format!("{what}: {e}"));
    let backup = current.with_extension(format!("{}-backup", app.name.to_ascii_lowercase()));
    let _ = fs::remove_file(&backup);
    fs::rename(current, &backup).map_err(|e| io("Could not move the current AppImage aside", e))?;
    if let Err(e) = fs::copy(package, current) {
        let _ = fs::rename(&backup, current);
        return Err(io("Could not install the new AppImage", e));
    }
    #[cfg(unix)]
    {
        // Owner-only: the file is the user's own copy of the app, and nobody else needs to run or write it.
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(current, fs::Permissions::from_mode(0o700));
    }
    let restore = || {
        let _ = fs::remove_file(current);
        let _ = fs::rename(&backup, current);
    };
    let mut child = match std::process::Command::new(current).spawn() {
        Ok(child) => child,
        Err(e) => {
            restore();
            return Err(io("Could not launch the updated AppImage", e));
        }
    };
    if let Err(why) = survives_start(&mut child, grace) {
        restore();
        return Err(Error::Io(format!(
            "The updated AppImage stopped right after starting ({why}); the previous version was put back."
        )));
    }
    let _ = fs::remove_file(package);
    let _ = fs::remove_file(&backup);
    Ok(())
}

/// Waits up to `grace` for the new process: still running, or finished cleanly, is a start;
/// finishing with an error is not.
fn survives_start(child: &mut Child, grace: Duration) -> Result<(), String> {
    let deadline = Instant::now() + grace;
    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => return Ok(()),
            Ok(Some(status)) => return Err(status.to_string()),
            Ok(None) if Instant::now() >= deadline => return Ok(()),
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            // Cannot tell: the process exists, which is all a start needs to be.
            Err(_) => return Ok(()),
        }
    }
}

/// An update is already on its way in this process: a second helper would race the first over
/// the same staged copy and backup.
#[cfg(target_os = "macos")]
static MACOS_UPDATE_STARTED: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "macos")]
fn apply_macos(app: &App, bundle: &Path, package: &Path) -> Result<(), Error> {
    if MACOS_UPDATE_STARTED.swap(true, Ordering::SeqCst) {
        return Err(Error::Io("An update is already being installed.".into()));
    }
    let started = start_macos_helper(app, bundle, package);
    if started.is_err() {
        // Nothing has been changed: a later attempt may run. Once the helper is up it stays set.
        MACOS_UPDATE_STARTED.store(false, Ordering::SeqCst);
    }
    started
}

#[cfg(target_os = "macos")]
fn start_macos_helper(app: &App, bundle: &Path, package: &Path) -> Result<(), Error> {
    use std::process::Command;
    let dir = crate::update_dir(app);
    let mount = dir.join("verify-mount");
    verify_macos_package(app, bundle, package, &mount)?;

    // The digest `download` verified. The helper hashes the bytes it is about to mount against it,
    // so a disk image swapped after the download cannot be installed.
    let digest = fs::read_to_string(crate::verified_digest_path(package))
        .map(|d| d.trim().to_ascii_lowercase())
        .ok()
        .filter(|d| crate::release::is_sha256(d))
        .ok_or(Error::NoDigest)?;

    let script = dir.join("apply-macos-update.sh");
    let result = dir.join("last-update-result.txt");
    let _ = fs::remove_file(&result);
    fs::write(&script, crate::macos::helper_script(app.bundle, app.name))
        .map_err(|e| Error::Io(format!("Could not write the macOS update helper: {e}")))?;
    let log = dir.join("update.log");
    Command::new("/bin/bash")
        .arg(&script)
        .arg(package)
        .arg(dir.join("dmg-mount"))
        .arg(bundle)
        .arg(std::process::id().to_string())
        .arg(&log)
        .arg(&result)
        .arg(&digest)
        .spawn()
        .map_err(|e| Error::Io(format!("Could not start the macOS update helper: {e}")))?;
    Ok(())
}

/// Mounts the disk image read-only and checks that the app inside is validly signed by the same
/// publisher as the installed one.
#[cfg(target_os = "macos")]
pub(crate) fn verify_macos_package(
    app: &App,
    bundle: &Path,
    dmg: &Path,
    mount: &Path,
) -> Result<(), Error> {
    use std::process::Command;
    let sig = |why: String| Error::Signature(why);
    let _ = fs::create_dir_all(mount);
    let attached = Command::new("/usr/bin/hdiutil")
        .args(["attach", "-nobrowse", "-readonly", "-mountpoint"])
        .arg(mount)
        .arg(dmg)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !attached {
        return Err(sig(
            "Could not mount the downloaded macOS update for verification.".into(),
        ));
    }
    let verdict = (|| {
        let inside = mount.join(app.bundle);
        if !inside.is_dir() {
            return Err(format!(
                "The downloaded disk image does not contain {}.",
                app.bundle
            ));
        }
        let downloaded = crate::macos::read_identity(&inside)?;
        let installed = crate::macos::read_identity(bundle)?;
        if crate::macos::same_publisher(&installed, &downloaded) {
            Ok(())
        } else {
            Err("The downloaded macOS app is signed by a different publisher.".to_owned())
        }
    })();
    let detached = Command::new("/usr/bin/hdiutil")
        .arg("detach")
        .arg(mount)
        .arg("-force")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    verdict.map_err(sig)?;
    if !detached {
        return Err(sig(
            "Could not unmount the downloaded macOS update after verification.".into(),
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn apply_windows(app: &App, dir: &Path, per_machine: bool, package: &Path) -> Result<(), Error> {
    crate::windows::start_helper(app, dir, per_machine, package)
}

#[cfg(not(windows))]
fn apply_windows(_: &App, _: &Path, _: bool, _: &Path) -> Result<(), Error> {
    Err(Error::NotUpdatable)
}

#[cfg(not(target_os = "macos"))]
fn apply_macos(_: &App, _: &Path, _: &Path) -> Result<(), Error> {
    Err(Error::NotUpdatable)
}

/// Reads and clears what the macOS helper left behind. `None` after a good update, or when there
/// was no update; the helper's log is [`update_dir`](crate::update_dir)`/update.log`.
pub fn take_last_failure(app: &App) -> Option<Failure> {
    take_last_failure_in(&crate::update_dir(app).join("last-update-result.txt"))
}

pub(crate) fn take_last_failure_in(file: &Path) -> Option<Failure> {
    let code = fs::read_to_string(file).ok()?;
    let _ = fs::remove_file(file);
    failure_from_code(code.trim())
}

fn failure_from_code(code: &str) -> Option<Failure> {
    match code {
        "" | "ok" => None,
        "still-running" => Some(Failure::StillRunning),
        "mount" => Some(Failure::Mount),
        "contents" => Some(Failure::Contents),
        "copy" => Some(Failure::Copy),
        "replace" => Some(Failure::Replace),
        "install" => Some(Failure::Install),
        "cancelled" => Some(Failure::Cancelled),
        _ => Some(Failure::Other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(not(unix), allow(dead_code))]
    const APP: App = App {
        name: "Probe",
        repo: "o/r",
        bundle: "Probe.app",
        version: "1.0.0",
        edition: None,
    };

    #[test]
    fn helper_codes_map_to_failures() {
        assert_eq!(failure_from_code("ok"), None);
        assert_eq!(failure_from_code(""), None);
        assert_eq!(failure_from_code("replace"), Some(Failure::Replace));
        assert_eq!(
            failure_from_code("still-running"),
            Some(Failure::StillRunning)
        );
        assert_eq!(failure_from_code("install"), Some(Failure::Install));
        assert_eq!(failure_from_code("cancelled"), Some(Failure::Cancelled));
        assert_eq!(failure_from_code("???"), Some(Failure::Other));
    }

    #[cfg(unix)]
    #[test]
    fn an_appimage_is_swapped_and_started() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("nodalia-update-swap-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let current = dir.join("Probe.AppImage");
        let package = dir.join("new.AppImage");
        let marker = dir.join("started");
        fs::write(&current, "#!/bin/sh\nexit 0\n").unwrap();
        fs::write(
            &package,
            format!("#!/bin/sh\ntouch '{}'\n", marker.display()),
        )
        .unwrap();
        fs::set_permissions(&package, fs::Permissions::from_mode(0o755)).unwrap();

        apply_appimage(&APP, &current, &package).unwrap();

        assert!(fs::read_to_string(&current).unwrap().contains("touch"));
        assert!(!package.exists(), "the package is consumed");
        assert!(
            !current.with_extension("probe-backup").exists(),
            "no backup is left"
        );
        for _ in 0..50 {
            if marker.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(marker.exists(), "the new AppImage was started");
        let _ = fs::remove_dir_all(dir);
    }

    #[cfg(unix)]
    #[test]
    fn a_failed_copy_puts_the_old_appimage_back() {
        let dir = std::env::temp_dir().join(format!("nodalia-update-undo-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let current = dir.join("Probe.AppImage");
        fs::write(&current, "old").unwrap();
        let missing = dir.join("nope");
        assert!(apply_appimage(&APP, &current, &missing).is_err());
        assert_eq!(fs::read_to_string(&current).unwrap(), "old");
        let _ = fs::remove_dir_all(dir);
    }

    #[cfg(unix)]
    #[test]
    fn a_new_appimage_that_crashes_on_start_leaves_the_old_one_in_place() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("nodalia-update-crash-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let current = dir.join("Probe.AppImage");
        let package = dir.join("new.AppImage");
        fs::write(&current, "#!/bin/sh\nexit 0\n").unwrap();
        fs::write(&package, "#!/bin/sh\nexit 3\n").unwrap();
        fs::set_permissions(&package, fs::Permissions::from_mode(0o755)).unwrap();

        let result = apply_appimage_within(&APP, &current, &package, Duration::from_secs(10));

        assert!(result.is_err(), "a crash on start is not an update");
        assert_eq!(fs::read_to_string(&current).unwrap(), "#!/bin/sh\nexit 0\n");
        assert!(package.exists(), "the download stays for another try");
        assert!(!current.with_extension("probe-backup").exists());
        let _ = fs::remove_dir_all(dir);
    }
}
