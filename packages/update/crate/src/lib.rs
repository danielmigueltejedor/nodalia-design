//! In-app updates for Nodalia apps.
//!
//! One flow for every app, whatever its toolkit: ask GitHub Releases for the latest release,
//! compare it with the running version, download the package for this machine and check its
//! SHA-256 against the digest GitHub publishes, then replace the running AppImage (Linux) or
//! `.app` (macOS) and restart. Installs that something else manages (Flatpak, `.deb`, `.rpm`,
//! the AUR, a dev build) are not touched: [`install_kind`] says so and the app points to the
//! release page instead.
//!
//! The crate draws nothing. It returns data and typed errors, and the app words them in its own
//! language and shows them in its own toolkit. Every call blocks: run it off the UI thread.

mod apply;
mod download;
mod macos;
mod notes;
mod release;
mod version;

/// The update windows for egui apps.
#[cfg(feature = "egui")]
pub mod egui;

use std::cmp::Ordering;
use std::path::PathBuf;

pub use apply::{apply, take_last_failure, Failure};
pub use download::download;
pub use macos::{parse_codesign_display, same_publisher, CodeIdentity};
pub use notes::{notes_in, Language};
pub use release::{asset_digest, parse as parse_release, preferred_assets, preferred_assets_for};
pub use version::{compare as compare_versions, strip as strip_version};

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
    /// The package for this machine, if the release has one.
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
    /// A newer release exists, but has no package for this platform.
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
    /// Flatpak, a distribution package or a dev build: another tool updates it.
    Managed,
}

impl InstallKind {
    pub fn can_apply_in_place(&self) -> bool {
        !matches!(self, InstallKind::Managed)
    }
}

/// Looks at how the running process was started.
pub fn install_kind(app: &App) -> InstallKind {
    if let Some(path) = std::env::var_os("APPIMAGE").filter(|p| !p.is_empty()) {
        if runs_from(std::env::var_os("APPDIR"), std::env::current_exe().ok()) {
            return InstallKind::AppImage(PathBuf::from(path));
        }
    }
    if cfg!(target_os = "macos") {
        if let Some(bundle) = macos::running_bundle(app) {
            return InstallKind::MacBundle(bundle);
        }
    }
    InstallKind::Managed
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

/// Asks GitHub for the latest release and compares it with `app.version`.
pub fn check(app: &App) -> Result<Check, Error> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", app.repo);
    let body = download::get(&url, "application/vnd.github+json")?;
    let release = release::parse(&body, &preferred_assets_for(app.edition))?;
    if compare_versions(&release.version, app.version) != Ordering::Greater {
        return Ok(Check::UpToDate);
    }
    if release.asset.is_none() {
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
}
