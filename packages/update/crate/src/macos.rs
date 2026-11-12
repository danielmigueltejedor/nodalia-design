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

/// Whether a process runs a program whose command line starts with the path given. The path is
/// compared as plain text: a bundle in a folder with parentheses or dots in its name is not a
/// pattern, as it would be for `pgrep -f`.
const RUNNING_FN: &str = r#"running() {
  /bin/ps -axo command= | /usr/bin/awk -v p="$1" 'index($0, p) == 1 { found = 1 } END { exit !found }'
}"#;

/// Swaps the installed bundle for the one inside the downloaded disk image once the running app
/// has exited. Arguments: `<dmg> <mount-dir> <bundle> <parent-pid> <log-file> <result-file>`.
///
/// It never fails silently: every step is logged, the outcome goes to `<result-file>` (`ok`, or a
/// short code [`take_last_failure`](crate::take_last_failure) reads at the next start), and on
/// failure the previous app is put back and reopened (or, if even that fails, left beside the
/// bundle and named in the log). The signature of the copy it installs is checked here too. It gives up if the app has not exited after
/// about a minute. The new app must stay running for about five seconds before the old copy is deleted; otherwise the old one is restored. `NODALIA_UPDATE_OPEN`, `NODALIA_UPDATE_WAIT_STEPS`, `NODALIA_UPDATE_GRACE_STEPS` and `NODALIA_UPDATE_SKIP_SIGNATURE` exist for tests.
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
  /*/"{bundle_name}") ;;
  *) echo "refusing unexpected bundle path"; printf 'internal\n' >"$RESULT"; exit 2 ;;
esac
NEW="$BUNDLE.nodalia-new"
OLD="$BUNDLE.nodalia-backup"
OPEN="${{NODALIA_UPDATE_OPEN:-/usr/bin/open}}"

fail() {{
  echo "FAILED at: $1"
  printf '%s\n' "$1" >"$RESULT"
  /usr/bin/hdiutil detach "$MOUNT" -force >/dev/null 2>&1
  [ -d "$NEW" ] && /bin/rm -rf "$NEW"
  if [ ! -d "$BUNDLE" ] && [ -d "$OLD" ]; then
    if ! /bin/mv "$OLD" "$BUNDLE"; then
      # Reopening a bundle that is not there would show nothing and hide why.
      echo "COULD NOT PUT THE PREVIOUS VERSION BACK. It is at: $OLD"
      echo "To recover, move it to $BUNDLE."
      exit 1
    fi
  fi
  "$OPEN" "$BUNDLE"
  exit 1
}}

{RUNNING_FN}

# Who signed an app, as codesign prints it: the identifier and the team, nothing else.
identity() {{
  /usr/bin/codesign -dv --verbose=4 "$1" 2>&1 | /usr/bin/grep -E '^(Identifier|TeamIdentifier)=' | /usr/bin/sort
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
/usr/bin/hdiutil detach "$MOUNT" -force >/dev/null 2>&1

# The copy that is about to be installed is the one that is checked, so nothing that changed the
# disk image after the app looked at it can get through: valid signature, same publisher.
if [ -z "$NODALIA_UPDATE_SKIP_SIGNATURE" ]; then
  /usr/bin/codesign --verify --deep --strict "$NEW" || fail signature
  NEW_ID=$(identity "$NEW")
  OLD_ID=$(identity "$BUNDLE")
  if [ -z "$NEW_ID" ] || [ "$NEW_ID" != "$OLD_ID" ]; then
    echo "the new app is signed by a different publisher"
    fail signature
  fi
fi

/bin/mv "$BUNDLE" "$OLD" || fail replace
/bin/mv "$NEW" "$BUNDLE" || fail replace
if ! "$OPEN" "$BUNDLE"; then
  # Installed, but it would not start: the previous version stays beside it for recovery.
  echo "updated, but $BUNDLE could not be started. The previous version is at: $OLD"
  printf 'launch\n' >"$RESULT"
  exit 1
fi

# `open` returning only says the launch was handed over. The backup is kept until the new app has
# stayed up for a few seconds; if it is gone by then it crashed on start, and the old one goes back.
steps="${{NODALIA_UPDATE_GRACE_STEPS:-10}}"
while [ "$steps" -gt 0 ]; do
  sleep 0.5
  steps=$((steps - 1))
done
if running "$BUNDLE/Contents/MacOS/"; then
  /bin/rm -rf "$OLD"
  /bin/rm -f "$DMG"
  printf 'ok\n' >"$RESULT"
  echo "updated; started $BUNDLE"
  /bin/rm -f "$0"
else
  echo "the new $BUNDLE did not stay running; putting the previous version back"
  /bin/rm -rf "$BUNDLE"
  # Only put the backup back into an empty place: `mv` into a directory that is still there would
  # tuck it inside and report success.
  if [ ! -e "$BUNDLE" ] && /bin/mv "$OLD" "$BUNDLE"; then
    printf 'launch\n' >"$RESULT"
    "$OPEN" "$BUNDLE"
  else
    echo "COULD NOT PUT THE PREVIOUS VERSION BACK. It is at: $OLD"
    echo "To recover, remove what is left of $BUNDLE and move it there."
    printf 'replace\n' >"$RESULT"
  fi
  exit 1
fi
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

    #[cfg(unix)]
    #[test]
    fn the_helper_names_the_bundle_and_is_valid_bash() {
        for name in ["Vinilo.app", "Vinilo Native.app"] {
            assert_valid_bash(&helper_script(name, "Vinilo"));
        }
        let script = helper_script("Vinilo Native.app", "Vinilo");
        assert!(script.contains("/*/\"Vinilo Native.app\") ;;"));
        assert!(script.contains("SOURCE=\"$MOUNT/Vinilo Native.app\""));
        let grace = script
            .find("if running \"$BUNDLE")
            .expect("it waits to see the app stay up");
        let delete = script
            .find("rm -rf \"$OLD\"\n  /bin/rm -f \"$DMG\"")
            .expect("it deletes the backup");
        assert!(
            grace < delete,
            "the backup goes only after the new app is seen running"
        );
        let check = script
            .find("codesign --verify")
            .expect("the helper checks the signature");
        let swap = script
            .find("/bin/mv \"$BUNDLE\" \"$OLD\"")
            .expect("it swaps the bundle");
        assert!(
            check < swap,
            "the copy is checked before it replaces anything"
        );
    }

    /// A bundle in a folder named like a regular expression is still found by its plain path, and
    /// a different path is not mistaken for it.
    #[cfg(target_os = "linux")]
    #[test]
    fn running_matches_a_path_literally() {
        let dir = std::env::temp_dir().join(format!("nodalia (run) {}.x", std::process::id()));
        let bin_dir = dir.join("Contents/MacOS");
        std::fs::create_dir_all(&bin_dir).unwrap();
        let bin = bin_dir.join("sleep");
        std::fs::copy("/bin/sleep", &bin).unwrap();
        let mut child = std::process::Command::new(&bin).arg("30").spawn().unwrap();
        let check = |path: &std::path::Path| {
            std::process::Command::new("bash")
                .arg("-c")
                .arg(format!("{RUNNING_FN}\nrunning \"$1\""))
                .arg("bash")
                .arg(path)
                .status()
                .unwrap()
                .success()
        };
        let found = check(&bin_dir);
        let other = check(&dir.join("Elsewhere/MacOS"));
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(found, "the process under the bundle is seen");
        assert!(!other, "another path is not matched");
    }

    #[cfg(unix)]
    fn assert_valid_bash(script: &str) {
        static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("nodalia-helper-{}-{n}.sh", std::process::id()));
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
