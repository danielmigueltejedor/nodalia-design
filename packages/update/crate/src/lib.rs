//! In-app updates for Nodalia apps.
//!
//! One flow for every app, whatever its toolkit: ask GitHub Releases for the latest release,
//! compare it with the running version, download the package for this machine and check its
//! SHA-256 against the digest GitHub publishes, then replace the running AppImage (Linux) or
//! `.app` (macOS) and restart, or install the release's MSI (Windows). Installs that something
//! else manages (Flatpak, `.deb`, `.rpm`, the AUR, a dev build), portable copies and copies in a
//! folder this user cannot write are not touched: [`install_kind`] says so and the app points to
//! the release page instead.
//!
//! The crate draws nothing. It returns data and typed errors, and the app words them in its own
//! language and shows them in its own toolkit. Every call blocks: run it off the UI thread.

mod apply;
mod download;
mod macos;
mod notes;
mod release;
mod version;
mod windows;

/// The update windows for egui apps.
#[cfg(feature = "egui")]
pub mod egui;

use std::cmp::Ordering;
use std::path::PathBuf;

pub use apply::{apply, take_last_failure, Failure};
pub use download::download;
pub use macos::{parse_codesign_display, same_publisher, CodeIdentity};
pub use notes::{notes_in, Language};
pub use release::{
    asset_digest, parse as parse_release, parse_list as parse_release_list, platform_markers,
    preferred_assets, preferred_assets_for, systems_for_package, PACKAGES,
};
pub use version::{compare as compare_versions, strip as strip_version};
pub use windows::{helper_script as windows_helper_script, parse_reg_install_dir, same_folder};

/// The app being updated.
#[derive(Clone, Debug)]
pub struct App {
    /// Shown to people and used for cache folders and backup file names: `Vinilo`.
    pub name: &'static str,
    /// `owner/repo` on GitHub.
    pub repo: &'static str,
    /// The macOS bundle's folder name: `Vinilo.app`.
    pub bundle: &'static str,
    /// The running version, normally `env!("CARGO_PKG_VERSION")`.
    pub version: &'static str,
    /// Set when one release carries several builds of the app for the same platform (the two
    /// macOS editions of Vinilo, say): the package is then the one named `<edition>-macos-…`,
    /// so an edition never offers to install the other.
    pub edition: Option<&'static str>,
}

/// A published release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub tag: String,
    /// The tag without its `v` and suffix: `2.0.1`.
    pub version: String,
    /// The release notes, Markdown.
    pub notes: String,
    pub page: String,
    /// The package for this machine, if the release has one. A managed install may see `None`.
    pub asset: Option<ReleaseAsset>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub url: String,
    /// Lower-case hex, from GitHub's `digest` field. Without it nothing is installed.
    pub sha256: Option<String>,
    pub size: Option<u64>,
}

/// The outcome of asking for the latest release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Check {
    UpToDate,
    Available(Release),
}

#[derive(Debug)]
pub enum Error {
    /// GitHub could not be reached, or answered with an error.
    Network(String),
    /// The answer was not a release.
    BadResponse,
    /// The latest release has no version tag.
    NoVersion,
    /// A newer release exists, but has no package for this platform, and this install replaces
    /// itself.
    NoPackage(String),
    /// The release publishes no SHA-256 for the package: nothing is installed unverified.
    NoDigest,
    /// The download does not match its digest.
    Checksum,
    /// This install cannot be replaced in place.
    NotUpdatable,
    /// The app's owner refused to restart (unsaved work).
    Refused(String),
    /// A file or process step failed.
    Io(String),
    /// The downloaded macOS app is unsigned or signed by someone else.
    Signature(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Network(why) => write!(f, "Could not reach GitHub Releases: {why}"),
            Error::BadResponse => f.write_str("Unexpected response from GitHub Releases."),
            Error::NoVersion => f.write_str("The latest release has no version tag."),
            Error::NoPackage(version) => write!(
                f,
                "A new version ({version}) is available, but no package for this platform was found."
            ),
            Error::NoDigest => f.write_str(
                "The release does not provide a trusted SHA-256 digest for this package.",
            ),
            Error::Checksum => f.write_str("The downloaded update failed its SHA-256 check."),
            Error::NotUpdatable => f.write_str("This installation cannot be updated in place."),
            Error::Refused(why) => f.write_str(why),
            Error::Io(why) => f.write_str(why),
            Error::Signature(why) => f.write_str(why),
        }
    }
}

impl std::error::Error for Error {}

/// How this process was installed, which decides whether it can replace itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstallKind {
    /// Running from an AppImage at this path.
    AppImage(PathBuf),
    /// Running from this `.app` bundle.
    MacBundle(PathBuf),
    /// Installed from the app's MSI into `dir`, for this user or for the whole machine.
    WindowsMsi { dir: PathBuf, per_machine: bool },
    /// A portable copy (the Windows ZIP, with `portable.txt` beside the program) in this folder:
    /// the person replaces it, from the release page.
    Portable(PathBuf),
    /// An AppImage or an app this user cannot replace where it is: a folder they cannot write,
    /// the disk image itself, or a copy macOS runs from a translocated, read-only place.
    NotWritable(PathBuf),
    /// Flatpak, a distribution package or a dev build: another tool updates it.
    Managed,
}

impl InstallKind {
    /// Whether the update is installed from inside the app.
    pub fn can_apply_in_place(&self) -> bool {
        matches!(
            self,
            InstallKind::AppImage(_) | InstallKind::MacBundle(_) | InstallKind::WindowsMsi { .. }
        )
    }

    /// Whether the app looks for releases by itself. Not for an install a package manager owns:
    /// that tool says when there is something new. A portable or read-only copy has no such tool,
    /// so it is told, and pointed at the release page.
    pub fn checks_at_start(&self) -> bool {
        !matches!(self, InstallKind::Managed)
    }
}

/// Looks at how the running process was started. Worked out once per process: on Windows it asks
/// the registry, and the windows ask on every frame.
pub fn install_kind(app: &App) -> InstallKind {
    static KINDS: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, InstallKind>>,
    > = std::sync::OnceLock::new();
    let kinds = KINDS.get_or_init(Default::default);
    let key = format!("{}\0{}", app.name, app.bundle);
    if let Some(kind) = kinds
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&key)
    {
        return kind.clone();
    }
    let kind = detect_install_kind(app);
    kinds
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(key, kind.clone());
    kind
}

fn detect_install_kind(app: &App) -> InstallKind {
    if let Some(path) = std::env::var_os("APPIMAGE").filter(|p| !p.is_empty()) {
        if runs_from(std::env::var_os("APPDIR"), std::env::current_exe().ok()) {
            let path = PathBuf::from(path);
            return match path.parent() {
                Some(dir) if writable(dir) => InstallKind::AppImage(path),
                _ => InstallKind::NotWritable(path),
            };
        }
    }
    if cfg!(target_os = "macos") {
        if let Some(bundle) = macos::running_bundle(app) {
            let replaceable =
                !macos::is_translocated(&bundle) && bundle.parent().is_some_and(writable);
            return if replaceable {
                InstallKind::MacBundle(bundle)
            } else {
                InstallKind::NotWritable(bundle)
            };
        }
    }
    #[cfg(windows)]
    if let Some(kind) = windows::detect() {
        return kind;
    }
    let _ = app;
    InstallKind::Managed
}

/// Whether this user can create a file in `dir`, tried rather than read from its permissions:
/// a read-only volume, an ACL or a sandbox all answer the same way.
pub(crate) fn writable(dir: &std::path::Path) -> bool {
    let probe = dir.join(format!(".nodalia-update-probe-{}", std::process::id()));
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

/// Whether this process runs from inside the mounted AppImage `APPDIR` names. `APPIMAGE` alone is
/// not proof: a program started from another app's AppImage inherits its variables, and taking
/// them at their word would replace that other app's file.
fn runs_from(appdir: Option<std::ffi::OsString>, exe: Option<PathBuf>) -> bool {
    match (appdir.filter(|d| !d.is_empty()), exe) {
        (Some(dir), Some(exe)) => exe.starts_with(PathBuf::from(dir)),
        _ => false,
    }
}

/// Asks GitHub for the newest release with a package for this system and compares it with
/// `app.version`. Not `releases/latest`: a z version published for one platform only must not
/// be offered on the others.
pub fn check(app: &App) -> Result<Check, Error> {
    let url = format!(
        "https://api.github.com/repos/{}/releases?per_page=20",
        app.repo
    );
    let body = download::get(&url, "application/vnd.github+json")?;
    // Nothing published yet is nothing newer, not an error to show.
    if release::is_empty_list(&body) {
        return Ok(Check::UpToDate);
    }
    let release = release::parse_list(&body, &preferred_assets_for(app.edition))?;
    if compare_versions(&release.version, app.version) != Ordering::Greater {
        return Ok(Check::UpToDate);
    }
    // A managed install is only ever pointed at the release page, which needs no package; only
    // one that would replace itself has to have one.
    if release.asset.is_none() && install_kind(app).can_apply_in_place() {
        return Err(Error::NoPackage(release.version));
    }
    Ok(Check::Available(release))
}

/// Where downloads and the helper's log live: the app's cache folder, under `update/`.
pub fn update_dir(app: &App) -> PathBuf {
    let base = if cfg!(target_os = "macos") {
        home().join("Library").join("Caches")
    } else if let Some(dir) = std::env::var_os("XDG_CACHE_HOME").filter(|d| !d.is_empty()) {
        PathBuf::from(dir)
    } else {
        home().join(".cache")
    };
    base.join(app.name.to_ascii_lowercase()).join("update")
}

/// Where [`download`] records the SHA-256 it verified for `package`.
pub(crate) fn verified_digest_path(package: &std::path::Path) -> PathBuf {
    let mut name = package.as_os_str().to_owned();
    name.push(".verified-sha256");
    PathBuf::from(name)
}

fn home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_appimage_this_process_runs_from_counts() {
        let exe = Some(PathBuf::from("/tmp/.mount_PlumaX/usr/bin/pluma"));
        assert!(runs_from(Some("/tmp/.mount_PlumaX".into()), exe.clone()));
        assert!(
            !runs_from(Some("/tmp/.mount_OtherY".into()), exe.clone()),
            "another app's variables"
        );
        assert!(!runs_from(None, exe));
        assert!(!runs_from(
            Some("".into()),
            Some(PathBuf::from("/usr/bin/pluma"))
        ));
    }

    #[test]
    fn only_appimages_apps_and_msi_installs_replace_themselves() {
        let p = || PathBuf::from("/x");
        assert!(InstallKind::AppImage(p()).can_apply_in_place());
        assert!(InstallKind::MacBundle(p()).can_apply_in_place());
        assert!(InstallKind::WindowsMsi {
            dir: p(),
            per_machine: true
        }
        .can_apply_in_place());
        for kind in [
            InstallKind::Portable(p()),
            InstallKind::NotWritable(p()),
            InstallKind::Managed,
        ] {
            assert!(!kind.can_apply_in_place(), "{kind:?}");
        }
        assert!(InstallKind::Portable(p()).checks_at_start());
        assert!(InstallKind::NotWritable(p()).checks_at_start());
        assert!(!InstallKind::Managed.checks_at_start());
    }

    #[test]
    fn a_folder_is_writable_only_if_a_file_can_be_made_there() {
        let dir = std::env::temp_dir().join(format!("nodalia-update-w-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(writable(&dir));
        assert_eq!(
            std::fs::read_dir(&dir).unwrap().count(),
            0,
            "the probe is removed"
        );
        assert!(!writable(&dir.join("missing")));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
