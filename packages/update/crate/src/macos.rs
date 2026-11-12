//! macOS: which bundle is running, who signed an app, and the helper that swaps the bundle.

use std::path::PathBuf;

use crate::App;

/// Identity read from `codesign -dv --verbose=4` (it prints on stderr).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeIdentity {
    pub identifier: String,
    pub team: String,
    /// Ad-hoc signed, or no team: there is no publisher to compare beyond the identifier.
    pub adhoc: bool,
}

pub fn parse_codesign_display(stderr: &str) -> CodeIdentity {
    let field = |key: &str| {
        stderr
            .lines()
            .find_map(|line| line.strip_prefix(key))
            .map(|value| value.trim().to_owned())
            .unwrap_or_default()
    };
    let team = field("TeamIdentifier=");
    let signature_adhoc = field("Signature=").eq_ignore_ascii_case("adhoc");
    CodeIdentity {
        identifier: field("Identifier="),
        adhoc: signature_adhoc || team.is_empty() || team == "not set",
        team,
    }
}

/// An update may replace an install only if the same publisher signed both: ad-hoc builds with
/// the same identifier may replace one another, and Developer ID builds need the same team.
pub fn same_publisher(installed: &CodeIdentity, downloaded: &CodeIdentity) -> bool {
    if installed.identifier.is_empty() || installed.identifier != downloaded.identifier {
        return false;
    }
    if installed.adhoc && downloaded.adhoc {
        return true;
    }
    installed.adhoc == downloaded.adhoc && installed.team == downloaded.team
}

/// `…/Vinilo.app/Contents/MacOS/vinilo` → `…/Vinilo.app`.
pub(crate) fn running_bundle(app: &App) -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let bundle = exe.ancestors().nth(3)?;
    (bundle.file_name()?.to_str()? == app.bundle).then(|| bundle.to_path_buf())
}

/// Reads the signature of an `.app` with `codesign`, after verifying it.
#[cfg(target_os = "macos")]
pub(crate) fn read_identity(bundle: &std::path::Path) -> Result<CodeIdentity, String> {
    use std::process::Command;
    let verify = Command::new("/usr/bin/codesign")
        .args(["--verify", "--deep", "--strict"])
        .arg(bundle)
        .status()
        .map_err(|e| e.to_string())?;
    if !verify.success() {
        return Err("The downloaded macOS app has an invalid code signature.".into());
    }
    let shown = Command::new("/usr/bin/codesign")
        .args(["-dv", "--verbose=4"])
        .arg(bundle)
        .output()
        .map_err(|e| e.to_string())?;
    let identity = parse_codesign_display(&String::from_utf8_lossy(&shown.stderr));
    if identity.identifier.is_empty() {
        return Err("Could not identify the macOS app publisher.".into());
    }
    Ok(identity)
}

/// Swaps the installed bundle for the one inside the downloaded disk image once the running app
/// has exited. Arguments: `<dmg> <mount-dir> <bundle> <parent-pid> <log-file> <result-file>`.
///
/// It never fails silently: every step is logged, the outcome goes to `<result-file>` (`ok`, or a
/// short code [`take_last_failure`](crate::take_last_failure) reads at the next start), and on
/// failure the previous app is put back and reopened. It gives up if the app has not exited after
/// about a minute. `NODALIA_UPDATE_OPEN` and `NODALIA_UPDATE_WAIT_STEPS` exist for tests.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn helper_script(bundle_name: &str, app_name: &str) -> String {
    format!(
        r#"#!/bin/bash
DMG=$1
MOUNT=$2
BUNDLE=$3
PARENT=$4
LOG=$5
RESULT=$6
exec >>"$LOG" 2>&1
echo "== $(date '+%Y-%m-%d %H:%M:%S') update helper: $DMG -> $BUNDLE"
case "$BUNDLE" in
  /*/{bundle_name}) ;;
  *) echo "refusing unexpected bundle path"; printf 'internal\n' >"$RESULT"; exit 2 ;;
esac
NEW="$BUNDLE.nodalia-new"
OLD="$BUNDLE.nodalia-backup"
OPEN="${{NODALIA_UPDATE_OPEN:-/usr/bin/open}}"

fail() {{
  echo "FAILED at: $1"
  printf '%s\n' "$1" >"$RESULT"
  /usr/bin/hdiutil detach "$MOUNT" -force >/dev/null 2>&1
  if [ ! -d "$BUNDLE" ] && [ -d "$OLD" ]; then /bin/mv "$OLD" "$BUNDLE"; fi
  [ -d "$NEW" ] && /bin/rm -rf "$NEW"
  "$OPEN" "$BUNDLE"
  exit 1
}}

# The application must be gone before its files are replaced.
steps="${{NODALIA_UPDATE_WAIT_STEPS:-120}}"
while [ "$steps" -gt 0 ] && kill -0 "$PARENT" 2>/dev/null; do
  sleep 0.5
  steps=$((steps - 1))
done
if kill -0 "$PARENT" 2>/dev/null; then
  echo "{app_name} (pid $PARENT) is still running: nothing was changed"
  printf 'still-running\n' >"$RESULT"
  exit 1
fi

/usr/bin/hdiutil detach "$MOUNT" -force >/dev/null 2>&1
/bin/mkdir -p "$MOUNT" || fail mount
/usr/bin/hdiutil attach "$DMG" -nobrowse -readonly -mountpoint "$MOUNT" || fail mount
SOURCE="$MOUNT/{bundle_name}"
[ -d "$SOURCE" ] || fail contents

[ -e "$NEW" ] && /bin/rm -rf "$NEW"
[ -e "$OLD" ] && /bin/rm -rf "$OLD"
/usr/bin/ditto "$SOURCE" "$NEW" || fail copy
/bin/mv "$BUNDLE" "$OLD" || fail replace
if ! /bin/mv "$NEW" "$BUNDLE"; then
  /bin/mv "$OLD" "$BUNDLE"
  fail replace
fi
/usr/bin/hdiutil detach "$MOUNT" -force >/dev/null 2>&1
/bin/rm -rf "$OLD"
/bin/rm -f "$DMG"
printf 'ok\n' >"$RESULT"
echo "updated; starting $BUNDLE"
"$OPEN" "$BUNDLE"
/bin/rm -f "$0"
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADHOC: &str = "Executable=/A/Vinilo.app/Contents/MacOS/vinilo\nIdentifier=dev.danielmiguelt.Vinilo\nSignature=adhoc\nTeamIdentifier=not set\n";
    const TEAM: &str =
        "Identifier=dev.danielmiguelt.Vinilo\nSignature size=9000\nTeamIdentifier=ABCDE12345\n";

    #[test]
    fn reads_adhoc_and_team_signatures() {
        let adhoc = parse_codesign_display(ADHOC);
        assert_eq!(adhoc.identifier, "dev.danielmiguelt.Vinilo");
        assert!(adhoc.adhoc);
        let team = parse_codesign_display(TEAM);
        assert_eq!(team.team, "ABCDE12345");
        assert!(!team.adhoc);
    }

    #[test]
    fn publishers_must_match() {
        let adhoc = parse_codesign_display(ADHOC);
        let team = parse_codesign_display(TEAM);
        assert!(same_publisher(&adhoc, &adhoc));
        assert!(same_publisher(&team, &team));
        assert!(
            !same_publisher(&adhoc, &team),
            "ad-hoc must not stand in for a team"
        );
        let mut other = team.clone();
        other.team = "ZZZZZ99999".into();
        assert!(!same_publisher(&team, &other));
        let mut renamed = adhoc.clone();
        renamed.identifier = "dev.other.App".into();
        assert!(!same_publisher(&adhoc, &renamed));
        assert!(!same_publisher(
            &CodeIdentity::default(),
            &CodeIdentity::default()
        ));
    }

    #[test]
    fn the_helper_names_the_bundle_and_is_valid_bash() {
        let script = helper_script("Vinilo.app", "Vinilo");
        assert!(script.contains("/*/Vinilo.app) ;;"));
        assert!(script.contains("SOURCE=\"$MOUNT/Vinilo.app\""));
        let path = std::env::temp_dir().join(format!("nodalia-helper-{}.sh", std::process::id()));
        std::fs::write(&path, script).unwrap();
        let ok = std::process::Command::new("bash")
            .arg("-n")
            .arg(&path)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        let _ = std::fs::remove_file(path);
        assert!(ok, "bash -n rejected the helper");
    }
}
