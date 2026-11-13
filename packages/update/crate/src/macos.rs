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

/// Whether macOS runs this bundle from a translocated copy: a quarantined app opened where it was
/// downloaded runs from a random read-only path, and replacing that path changes nothing.
pub(crate) fn is_translocated(bundle: &std::path::Path) -> bool {
    bundle
        .components()
        .any(|part| part.as_os_str() == "AppTranslocation")
}

/// How `codesign` verifies an app whose seal is checked before an update. Not `--strict`: that
/// flag also rejects Finder metadata (`com.apple.FinderInfo`) on the files, and every file that
/// has been on an HFS+ disk image or copied by the Finder carries it, so a perfectly good app
/// failed ("resource fork, Finder information, or similar detritus not allowed"). Without it
/// `codesign` still checks the signature, that every sealed file is unchanged, and nested code.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) const VERIFY_ARGS: [&str; 3] = ["--verify", "--deep", "--verbose=2"];

/// Reads the signature of an `.app` with `codesign`, after verifying it.
#[cfg(target_os = "macos")]
pub(crate) fn read_identity(bundle: &std::path::Path) -> Result<CodeIdentity, String> {
    use std::process::Command;
    let verify = Command::new("/usr/bin/codesign")
        .args(VERIFY_ARGS)
        .arg(bundle)
        .output()
        .map_err(|e| e.to_string())?;
    if !verify.status.success() {
        // codesign says which file and why; keep it, so the message points at the cause.
        let why = String::from_utf8_lossy(&verify.stderr);
        let why = why
            .lines()
            .rev()
            .find(|l| !l.trim().is_empty())
            .unwrap_or("")
            .trim();
        return Err(format!(
            "The macOS app at {} has an invalid code signature ({why}).",
            bundle.display()
        ));
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

/// The `NODALIA_UPDATE_*` variables below exist so tests can steer the helper (a stand-in for
/// `open`, no waiting, no signature). A shipped build must not honour them: whoever can set the
/// environment of the app could otherwise switch the signature check off.
fn without_test_hooks() -> &'static str {
    if cfg!(debug_assertions) {
        ""
    } else {
        "unset NODALIA_UPDATE_OPEN NODALIA_UPDATE_WAIT_STEPS NODALIA_UPDATE_GRACE_STEPS NODALIA_UPDATE_SKIP_SIGNATURE"
    }
}

/// Swaps the installed bundle for the one inside the downloaded disk image once the running app
/// has exited. Arguments: `<dmg> <mount-dir> <bundle> <parent-pid> <log-file> <result-file>
/// <sha256>`. The disk image is copied into a private folder and the copy is hashed against
/// `<sha256>` (the digest the download verified) before it is mounted, so what is installed is
/// what was verified, even if the file in the cache changed in between.
///
/// It never fails silently: every step is logged, the outcome goes to `<result-file>` (`ok`, or a
/// short code [`take_last_failure`](crate::take_last_failure) reads at the next start), and on
/// failure the previous app is put back and reopened (or, if even that fails, left beside the
/// bundle and named in the log). The signature of the copy it installs is checked here too. It gives up if the app has not exited after
/// about a minute. The new app must stay running for about five seconds before the old copy is deleted; otherwise the old one is restored. `NODALIA_UPDATE_OPEN`, `NODALIA_UPDATE_WAIT_STEPS`, `NODALIA_UPDATE_GRACE_STEPS` and `NODALIA_UPDATE_SKIP_SIGNATURE` exist for tests, and only debug builds honour them.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn helper_script(bundle_name: &str, app_name: &str) -> String {
    let hooks = without_test_hooks();
    format!(
        r#"#!/bin/bash
DMG=$1
MOUNT=$2
BUNDLE=$3
PARENT=$4
LOG=$5
RESULT=$6
EXPECTED=$7
STAGE=
exec >>"$LOG" 2>&1
{hooks}
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
  [ -n "$STAGE" ] && /bin/rm -rf "$STAGE"
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
# A private copy (mktemp makes it owner-only), hashed and then mounted: there is no gap between
# the check and the use for another process to put a different image into.
STAGE=$(/usr/bin/mktemp -d "${{TMPDIR:-/tmp}}/nodalia-update.XXXXXX") || fail copy
/bin/cp "$DMG" "$STAGE/update.dmg" || fail copy
ACTUAL=$(/usr/bin/shasum -a 256 "$STAGE/update.dmg" | /usr/bin/awk '{{print $1}}')
if [ -z "$EXPECTED" ] || [ "$ACTUAL" != "$EXPECTED" ]; then
  echo "the disk image does not match the digest that was verified"
  fail contents
fi
/usr/bin/hdiutil attach "$STAGE/update.dmg" -nobrowse -readonly -mountpoint "$MOUNT" || fail mount
SOURCE="$MOUNT/{bundle_name}"
[ -d "$SOURCE" ] || fail contents

[ -e "$NEW" ] && /bin/rm -rf "$NEW"
[ -e "$OLD" ] && /bin/rm -rf "$OLD"
/usr/bin/ditto "$SOURCE" "$NEW" || fail copy
/usr/bin/hdiutil detach "$MOUNT" -force >/dev/null 2>&1
/bin/rm -rf "$STAGE"
STAGE=

# The copy that is about to be installed is the one that is checked, so nothing that changed the
# disk image after the app looked at it can get through: valid signature, same publisher.
if [ -z "$NODALIA_UPDATE_SKIP_SIGNATURE" ]; then
  # Not the strict mode: it refuses the Finder metadata every file from a disk image carries, which
  # `ditto` keeps even with --noextattr, and which is not part of the signature.
  /usr/bin/codesign --verify --deep "$NEW" || fail signature
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
    fn a_translocated_copy_is_recognised() {
        assert!(is_translocated(std::path::Path::new(
            "/private/var/folders/x/T/AppTranslocation/1234/d/Lienzo.app"
        )));
        assert!(!is_translocated(std::path::Path::new(
            "/Applications/Lienzo.app"
        )));
    }

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

    /// A disk image's files all carry `com.apple.FinderInfo`, which `--strict` refuses: the
    /// seal is checked without it (see `VERIFY_ARGS`), for the downloaded app, the installed one and the helper's copy alike.
    #[test]
    fn the_seal_check_is_not_strict() {
        assert!(!VERIFY_ARGS.contains(&"--strict"));
        assert!(VERIFY_ARGS.contains(&"--deep") && VERIFY_ARGS.contains(&"--verify"));
    }

    #[cfg(unix)]
    #[test]
    fn the_helper_names_the_bundle_and_is_valid_bash() {
        for name in ["Vinilo.app", "Vinilo Native.app"] {
            assert_valid_bash(&helper_script(name, "Vinilo"));
        }
        let script = helper_script("Vinilo Native.app", "Vinilo");
        assert_eq!(
            script.contains("unset NODALIA_UPDATE_OPEN"),
            !cfg!(debug_assertions),
            "a shipped helper ignores the test hooks (notably the one that skips the signature)"
        );
        assert!(script.contains("/*/\"Vinilo Native.app\") ;;"));
        assert!(script.contains("SOURCE=\"$MOUNT/Vinilo Native.app\""));
        assert!(
            !script.contains("--strict"),
            "the helper's seal check must not be strict: a disk image's Finder metadata fails it"
        );
        let hash = script
            .find("shasum -a 256")
            .expect("the helper hashes the image it is about to mount");
        let attach = script.find("hdiutil attach").expect("it mounts the image");
        assert!(
            hash < attach && script.contains("attach \"$STAGE/update.dmg\""),
            "the digest is checked on the private copy that is then mounted, not on the cache file"
        );
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
        // A link rather than a copy: running a file that was just written can fail with
        // "text file busy" while other tests fork. The command line shows the link's path.
        std::os::unix::fs::symlink("/bin/sleep", &bin).unwrap();
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

/// The whole macOS update on a real machine: an ad-hoc signed app (as every Nodalia DMG ships),
/// on an HFS+ disk image whose files carry Finder metadata, checked the way the app checks it and
/// then installed by the helper itself, signature check included. Runs on the macOS CI runner.
#[cfg(all(test, target_os = "macos"))]
mod end_to_end {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn run(program: &str, args: &[&str]) {
        let status = Command::new(program).args(args).status().unwrap();
        assert!(status.success(), "{program} {args:?} failed");
    }

    /// `Probe.app` whose program is a copy of `sleep`, so it shows in `ps` under its own path,
    /// with `version.txt` saying which build it is. Signed ad hoc, then given Finder metadata the
    /// way a file copied off a disk image has it.
    fn make_app(at: &Path, version: &str) -> PathBuf {
        let app = at.join("Probe.app");
        let contents = app.join("Contents");
        fs::create_dir_all(contents.join("MacOS")).unwrap();
        fs::create_dir_all(contents.join("Resources")).unwrap();
        fs::copy("/bin/sleep", contents.join("MacOS/probe")).unwrap();
        fs::write(contents.join("Resources/version.txt"), version).unwrap();
        fs::write(
            contents.join("Info.plist"),
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleIdentifier</key><string>dev.nodalia.update-probe</string>
<key>CFBundleExecutable</key><string>probe</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>{version}</string>
</dict></plist>"#
            ),
        )
        .unwrap();
        run(
            "/usr/bin/codesign",
            &["--force", "--deep", "--sign", "-", app.to_str().unwrap()],
        );
        let finder_info = "0000000000000000040000000000000000000000000000000000000000000000";
        run(
            "/usr/bin/xattr",
            &[
                "-wx",
                "com.apple.FinderInfo",
                finder_info,
                contents.join("Info.plist").to_str().unwrap(),
            ],
        );
        app
    }

    fn make_dmg(dir: &Path, version: &str) -> PathBuf {
        let stage = dir.join(format!("stage-{version}"));
        fs::create_dir_all(&stage).unwrap();
        make_app(&stage, version);
        let dmg = dir.join(format!("probe-{version}-macos-arm64.dmg"));
        run(
            "/usr/bin/hdiutil",
            &[
                "create",
                "-quiet",
                "-fs",
                "HFS+",
                "-volname",
                "Probe",
                "-srcfolder",
                stage.to_str().unwrap(),
                "-format",
                "UDZO",
                dmg.to_str().unwrap(),
            ],
        );
        dmg
    }

    fn sha256(path: &Path) -> String {
        use sha2::{Digest, Sha256};
        let bytes = fs::read(path).unwrap();
        Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nodalia-e2e-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir.canonicalize().unwrap()
    }

    const APP: crate::App = crate::App {
        name: "Probe",
        repo: "o/r",
        bundle: "Probe.app",
        version: "1.0.0",
        edition: None,
    };

    #[test]
    fn an_adhoc_app_from_a_disk_image_passes_the_seal_check() {
        let dir = scratch("seal");
        let installed = make_app(&dir.join("Applications"), "1");
        let dmg = make_dmg(&dir, "2");
        crate::apply::verify_macos_package(&APP, &installed, &dmg, &dir.join("mount"))
            .unwrap_or_else(|e| panic!("the app's pre-install check refused it: {e}"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_helper_installs_the_new_app_and_starts_it() {
        let dir = scratch("helper");
        let installed = make_app(&dir.join("Applications"), "1");
        let dmg = make_dmg(&dir, "2");
        let script = dir.join("helper.sh");
        fs::write(&script, super::helper_script("Probe.app", "Probe")).unwrap();
        // A stand-in for `open` that starts the app's program in the background.
        let open = dir.join("open.sh");
        fs::write(
            &open,
            "#!/bin/bash\n\"$1/Contents/MacOS/probe\" 20 >/dev/null 2>&1 &\nexit 0\n",
        )
        .unwrap();
        run("/bin/chmod", &["+x", open.to_str().unwrap()]);
        // The parent has already exited, as the app has once it hands over.
        let mut parent = Command::new("/usr/bin/true").spawn().unwrap();
        let pid = parent.id().to_string();
        parent.wait().unwrap();
        let (log, result) = (dir.join("update.log"), dir.join("result.txt"));
        let status = Command::new("/bin/bash")
            .arg(&script)
            .arg(&dmg)
            .arg(dir.join("dmg-mount"))
            .arg(&installed)
            .arg(&pid)
            .arg(&log)
            .arg(&result)
            .arg(sha256(&dmg))
            .env("NODALIA_UPDATE_OPEN", &open)
            .env("NODALIA_UPDATE_WAIT_STEPS", "4")
            .env("NODALIA_UPDATE_GRACE_STEPS", "2")
            .status()
            .unwrap();
        let log_text = fs::read_to_string(&log).unwrap_or_default();
        let _ = Command::new("/usr/bin/pkill")
            .args([
                "-f",
                installed.join("Contents/MacOS/probe").to_str().unwrap(),
            ])
            .status();
        assert!(status.success(), "the helper failed:\n{log_text}");
        assert_eq!(
            fs::read_to_string(&result).unwrap().trim(),
            "ok",
            "{log_text}"
        );
        assert_eq!(
            fs::read_to_string(installed.join("Contents/Resources/version.txt")).unwrap(),
            "2",
            "the new version is installed"
        );
        assert!(
            !dir.join("Applications/Probe.app.nodalia-backup").exists(),
            "the backup is removed once the new app runs"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_helper_refuses_a_disk_image_that_changed_after_the_download() {
        let dir = scratch("digest");
        let installed = make_app(&dir.join("Applications"), "1");
        let dmg = make_dmg(&dir, "2");
        let script = dir.join("helper.sh");
        fs::write(&script, super::helper_script("Probe.app", "Probe")).unwrap();
        let mut parent = Command::new("/usr/bin/true").spawn().unwrap();
        let pid = parent.id().to_string();
        parent.wait().unwrap();
        let result = dir.join("result.txt");
        let status = Command::new("/bin/bash")
            .arg(&script)
            .arg(&dmg)
            .arg(dir.join("dmg-mount"))
            .arg(&installed)
            .arg(&pid)
            .arg(dir.join("update.log"))
            .arg(&result)
            .arg("0".repeat(64))
            .env("NODALIA_UPDATE_OPEN", "/usr/bin/true")
            .env("NODALIA_UPDATE_WAIT_STEPS", "4")
            .status()
            .unwrap();
        assert!(!status.success());
        assert_eq!(fs::read_to_string(&result).unwrap().trim(), "contents");
        assert_eq!(
            fs::read_to_string(installed.join("Contents/Resources/version.txt")).unwrap(),
            "1",
            "nothing was replaced"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
