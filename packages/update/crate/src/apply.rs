//! Replacing the running install, and reading back how that went.

use std::fs;
use std::path::Path;

use crate::{App, Error, InstallKind};

/// Why the last update was not installed, read at the next start. Words are the app's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        InstallKind::Managed => Err(Error::NotUpdatable),
    }
}

/// Moves the running AppImage aside, puts the new one in its place and starts it. The running
/// file stays open under its new name, so this is safe on Linux; any failure puts it back.
pub(crate) fn apply_appimage(app: &App, current: &Path, package: &Path) -> Result<(), Error> {
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
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(current, fs::Permissions::from_mode(0o755));
    }
    let _ = fs::remove_file(package);
    if let Err(e) = std::process::Command::new(current).spawn() {
        let _ = fs::remove_file(current);
        let _ = fs::rename(&backup, current);
        return Err(io("Could not launch the updated AppImage", e));
    }
    let _ = fs::remove_file(&backup);
    Ok(())
}

#[cfg(target_os = "macos")]
fn apply_macos(app: &App, bundle: &Path, package: &Path) -> Result<(), Error> {
    use std::process::Command;
    let dir = crate::update_dir(app);
    let mount = dir.join("verify-mount");
    verify_macos_package(app, bundle, package, &mount)?;

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
        .spawn()
        .map_err(|e| Error::Io(format!("Could not start the macOS update helper: {e}")))?;
    Ok(())
}

/// Mounts the disk image read-only and checks that the app inside is validly signed by the same
/// publisher as the installed one.
#[cfg(target_os = "macos")]
fn verify_macos_package(app: &App, bundle: &Path, dmg: &Path, mount: &Path) -> Result<(), Error> {
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

#[cfg(not(target_os = "macos"))]
fn apply_macos(_: &App, _: &Path, _: &Path) -> Result<(), Error> {
    Err(Error::NotUpdatable)
}

/// Reads and clears what the macOS helper left behind. `None` after a good update, or when there
/// was no update; the helper's log is [`update_dir`](crate::update_dir)`/update.log`.
pub fn take_last_failure(app: &App) -> Option<Failure> {
    let file = crate::update_dir(app).join("last-update-result.txt");
    let code = fs::read_to_string(&file).ok()?;
    let _ = fs::remove_file(&file);
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
        _ => Some(Failure::Other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const APP: App = App {
        name: "Probe",
        repo: "o/r",
        bundle: "Probe.app",
        version: "1.0.0",
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
}
