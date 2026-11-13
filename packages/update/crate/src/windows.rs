//! Windows: which MSI install is running, and the helper that installs the new MSI.
//!
//! An app installed from its MSI records its folder as `Software\<App>\InstallDir` (HKCU for a
//! per-user install, HKLM for a per-machine one). When that folder is the running executable's,
//! the update is the release's MSI, installed by `msiexec` once the app has exited, into the same
//! folder and the same scope, and the app is started again. A copy with `portable.txt` beside the
//! executable is a portable ZIP: it is pointed at the release page.

use std::path::{Path, PathBuf};

/// The line `reg query` prints for a string value: `    InstallDir    REG_SZ    C:\Apps\Lienzo\`.
pub fn parse_reg_install_dir(output: &str) -> Option<PathBuf> {
    output.lines().find_map(|line| {
        let line = line.trim();
        let rest = line.strip_prefix("InstallDir")?;
        let value = rest.trim_start().strip_prefix("REG_SZ")?.trim();
        (!value.is_empty()).then(|| PathBuf::from(value))
    })
}

/// Whether two Windows folders are the same, the way Windows compares them: without case, and
/// with or without a trailing separator.
pub fn same_folder(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        p.to_string_lossy()
            .replace('/', "\\")
            .trim_end_matches('\\')
            .to_lowercase()
    };
    let (a, b) = (norm(a), norm(b));
    !a.is_empty() && a == b
}

/// A folder as `msiexec` reads it on its command line: no trailing backslash, which would
/// otherwise escape the closing quote.
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) fn msi_folder(dir: &Path) -> String {
    dir.to_string_lossy().trim_end_matches('\\').to_owned()
}

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(windows)]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;

/// How this executable was installed on Windows, or `None` when it is neither an MSI install nor
/// a portable copy (a dev build, a copied binary).
#[cfg(windows)]
pub(crate) fn detect() -> Option<crate::InstallKind> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?.to_path_buf();
    if dir.join("portable.txt").is_file() {
        return Some(crate::InstallKind::Portable(dir));
    }
    // The registry key is the product's name and the executable is its lower-case form
    // (`Software\Lienzo`, `lienzo.exe`); registry keys ignore case.
    let stem = exe.file_stem()?.to_str()?.to_owned();
    for (hive, per_machine) in [("HKCU", false), ("HKLM", true)] {
        let Some(installed) = query_install_dir(hive, &stem) else {
            continue;
        };
        if same_folder(&installed, &dir) {
            return Some(crate::InstallKind::WindowsMsi { dir, per_machine });
        }
    }
    None
}

#[cfg(windows)]
fn query_install_dir(hive: &str, key: &str) -> Option<PathBuf> {
    use std::os::windows::process::CommandExt;
    let output = std::process::Command::new(system32("reg.exe"))
        .args([
            "query",
            &format!("{hive}\\Software\\{key}"),
            "/v",
            "InstallDir",
            "/reg:64",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_reg_install_dir(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(windows)]
fn system32(program: &str) -> PathBuf {
    let root = std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into());
    PathBuf::from(root).join("System32").join(program)
}

/// What `msiexec` is: the system one, or a stand-in that a debug build's tests name.
#[cfg(windows)]
fn msiexec() -> PathBuf {
    if cfg!(debug_assertions) {
        if let Some(stand_in) = std::env::var_os("NODALIA_UPDATE_MSIEXEC").filter(|v| !v.is_empty())
        {
            return PathBuf::from(stand_in);
        }
    }
    system32("msiexec.exe")
}

/// Writes the helper and starts it. It waits for this process to exit, so the caller must exit
/// right after (`Action::Exit` in the egui flow).
#[cfg(windows)]
pub(crate) fn start_helper(
    app: &crate::App,
    dir: &Path,
    per_machine: bool,
    package: &Path,
) -> Result<(), crate::Error> {
    use crate::Error;
    use std::os::windows::process::CommandExt;
    let digest = std::fs::read_to_string(crate::verified_digest_path(package))
        .map(|d| d.trim().to_ascii_lowercase())
        .ok()
        .filter(|d| crate::release::is_sha256(d))
        .ok_or(Error::NoDigest)?;
    let exe = std::env::current_exe()
        .map_err(|e| Error::Io(format!("Could not find the running app: {e}")))?;
    let update = crate::update_dir(app);
    let script = update.join("apply-windows-update.ps1");
    let result = update.join("last-update-result.txt");
    let _ = std::fs::remove_file(&result);
    std::fs::write(&script, helper_script())
        .map_err(|e| Error::Io(format!("Could not write the Windows update helper: {e}")))?;
    std::process::Command::new(system32("WindowsPowerShell\\v1.0\\powershell.exe"))
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-WindowStyle",
            "Hidden",
            "-File",
        ])
        .arg(&script)
        .arg("-Msi")
        .arg(package)
        .arg("-ParentPid")
        .arg(std::process::id().to_string())
        .arg("-Exe")
        .arg(&exe)
        .arg("-Dir")
        .arg(msi_folder(dir))
        .arg("-Scope")
        .arg(if per_machine { "machine" } else { "user" })
        .arg("-Msiexec")
        .arg(msiexec())
        .arg("-Log")
        .arg(update.join("update.log"))
        .arg("-Result")
        .arg(&result)
        .arg("-Expected")
        .arg(&digest)
        .creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP)
        .spawn()
        .map_err(|e| Error::Io(format!("Could not start the Windows update helper: {e}")))?;
    Ok(())
}

/// Installs a verified MSI once the app has exited. Parameters: `-Msi -ParentPid -Exe -Dir -Scope
/// (user|machine) -Msiexec -Log -Result -Expected`. The package is copied into a private folder
/// and that copy is hashed against `-Expected` before `msiexec` gets it, so what is installed is
/// what was verified. `msiexec` runs with a progress bar and no questions; a per-machine install
/// asks for administrator rights. The outcome goes to `-Result` (`ok`, or a code
/// [`take_last_failure`](crate::take_last_failure) reads at the next start), every step to `-Log`,
/// and the app is started again whatever happened: a failed MSI rolls back to the installed version.
pub fn helper_script() -> &'static str {
    r#"param(
  [string]$Msi, [int]$ParentPid, [string]$Exe, [string]$Dir, [string]$Scope,
  [string]$Msiexec, [string]$Log, [string]$Result, [string]$Expected
)
$ErrorActionPreference = 'Stop'
function Note([string]$text) {
  Add-Content -LiteralPath $Log -Value ("{0} {1}" -f (Get-Date -Format 's'), $text)
}
function Finish([string]$code) {
  Set-Content -LiteralPath $Result -Value $code -NoNewline
  if ($code -eq 'ok') { Note 'updated' } else { Note "FAILED at: $code" }
  try { Start-Process -FilePath $Exe } catch { Note "could not start ${Exe}: $_" }
  if ($code -eq 'ok') { exit 0 } else { exit 1 }
}
$stage = $null
try {
  Note "== update helper: $Msi -> $Dir ($Scope)"
  $parent = Get-Process -Id $ParentPid -ErrorAction SilentlyContinue
  if ($parent -and -not $parent.WaitForExit(60000)) {
    Note "the app (pid $ParentPid) is still running: nothing was changed"
    Set-Content -LiteralPath $Result -Value 'still-running' -NoNewline
    exit 1
  }
  $stage = Join-Path ([IO.Path]::GetTempPath()) ('nodalia-update-' + [guid]::NewGuid().ToString('N'))
  New-Item -ItemType Directory -Path $stage | Out-Null
  $copy = Join-Path $stage 'update.msi'
  Copy-Item -LiteralPath $Msi -Destination $copy
  $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $copy).Hash.ToLowerInvariant()
  if ([string]::IsNullOrEmpty($Expected) -or $actual -ne $Expected) {
    Note 'the package does not match the digest that was verified'
    Finish 'contents'
  }
  $msiLog = Join-Path (Split-Path -Parent $Log) 'msiexec.log'
  $arguments = @('/i', ('"{0}"' -f $copy), '/passive', '/norestart', '/l*v', ('"{0}"' -f $msiLog), ('INSTALLFOLDER="{0}"' -f $Dir))
  if ($Scope -eq 'machine') { $arguments += 'ALLUSERS=1' } else { $arguments += @('ALLUSERS=2', 'MSIINSTALLPERUSER=1') }
  $start = @{ FilePath = $Msiexec; ArgumentList = $arguments; Wait = $true; PassThru = $true }
  if ($Scope -eq 'machine') { $start.Verb = 'RunAs' }
  $process = Start-Process @start
  $code = $process.ExitCode
  Note "msiexec exited with $code"
  # 3010: installed, a restart of Windows is pending; 1602: the person cancelled.
  if ($code -eq 0 -or $code -eq 3010) {
    Remove-Item -LiteralPath $Msi -Force -ErrorAction SilentlyContinue
    Finish 'ok'
  } elseif ($code -eq 1602 -or $code -eq 1223) {
    Finish 'cancelled'
  } else {
    Finish 'install'
  }
} catch {
  Note "error: $_"
  # The UAC prompt refused: Start-Process throws instead of returning 1223.
  if ("$_" -match 'canceled|cancelled') { Finish 'cancelled' } else { Finish 'install' }
} finally {
  if ($stage) { Remove-Item -LiteralPath $stage -Recurse -Force -ErrorAction SilentlyContinue }
}
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_install_dir_reg_query_prints() {
        let out = "\r\nHKEY_CURRENT_USER\\Software\\Lienzo\r\n    InstallDir    REG_SZ    C:\\Users\\Ana\\AppData\\Local\\Programs\\ArtCraft\\Lienzo\\\r\n\r\n";
        assert_eq!(
            parse_reg_install_dir(out),
            Some(PathBuf::from(
                "C:\\Users\\Ana\\AppData\\Local\\Programs\\ArtCraft\\Lienzo\\"
            ))
        );
        assert_eq!(parse_reg_install_dir("ERROR: not found"), None);
        assert_eq!(
            parse_reg_install_dir("    InstallDir    REG_SZ    \r\n"),
            None
        );
        assert_eq!(
            parse_reg_install_dir("    InstallDirX    REG_SZ    C:\\x\r\n"),
            None,
            "only the InstallDir value"
        );
    }

    #[test]
    fn folders_compare_like_windows() {
        assert!(same_folder(
            Path::new("C:\\Program Files\\Lienzo\\"),
            Path::new("c:\\program files\\lienzo")
        ));
        assert!(!same_folder(
            Path::new("C:\\Program Files\\Lienzo"),
            Path::new("C:\\Program Files\\Lienzo2")
        ));
        assert!(!same_folder(Path::new(""), Path::new("")));
    }

    #[test]
    fn msiexec_gets_no_trailing_backslash() {
        assert_eq!(
            msi_folder(Path::new("C:\\Apps\\Lienzo\\")),
            "C:\\Apps\\Lienzo"
        );
    }

    #[test]
    fn the_helper_hashes_the_copy_it_installs_and_keeps_the_scope() {
        let script = helper_script();
        let hash = script.find("Get-FileHash").unwrap();
        let install = script.find("Start-Process @start").unwrap();
        assert!(
            hash < install,
            "the copy is verified before msiexec sees it"
        );
        assert!(
            script.contains("'/i', ('\"{0}\"' -f $copy)"),
            "msiexec installs the verified copy"
        );
        assert!(script.contains("ALLUSERS=1") && script.contains("MSIINSTALLPERUSER=1"));
        assert!(script.contains("$start.Verb = 'RunAs'"));
    }
}

/// The helper on a real Windows machine, with a stand-in for `msiexec` that records what it was
/// asked and a stand-in app that records that it was started again. Runs on the Windows CI runner.
#[cfg(all(test, windows))]
mod end_to_end {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nodalia-e2e-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn sha256(path: &Path) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(fs::read(path).unwrap())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    /// Runs the helper as the app does, and returns (exit success, result, msiexec's arguments,
    /// whether the app was started again).
    fn helper(
        dir: &Path,
        scope: &str,
        msi_exit: u32,
        expected: Option<&str>,
    ) -> (bool, String, String, bool) {
        let msi = dir.join("probe-2.0.0-windows-x64.msi");
        fs::write(&msi, b"not really an msi").unwrap();
        let args = dir.join("msiexec-args.txt");
        let msiexec = dir.join("msiexec.cmd");
        fs::write(
            &msiexec,
            format!(
                "@echo off\r\necho %* > \"{}\"\r\nexit /b {msi_exit}\r\n",
                args.display()
            ),
        )
        .unwrap();
        let started = dir.join("started.txt");
        let exe = dir.join("probe.cmd");
        fs::write(
            &exe,
            format!("@echo off\r\necho yes > \"{}\"\r\n", started.display()),
        )
        .unwrap();
        let script = dir.join("helper.ps1");
        fs::write(&script, super::helper_script()).unwrap();
        let result = dir.join("result.txt");
        let digest = expected.map(str::to_owned).unwrap_or_else(|| sha256(&msi));
        let status = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
            ])
            .arg(&script)
            .arg("-Msi")
            .arg(&msi)
            .args(["-ParentPid", "4194303"])
            .arg("-Exe")
            .arg(&exe)
            .args(["-Dir", "C:\\Apps\\Probe", "-Scope", scope])
            .arg("-Msiexec")
            .arg(&msiexec)
            .arg("-Log")
            .arg(dir.join("update.log"))
            .arg("-Result")
            .arg(&result)
            .arg("-Expected")
            .arg(&digest)
            .status()
            .unwrap();
        // The app is started without waiting for it.
        for _ in 0..50 {
            if started.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        (
            status.success(),
            fs::read_to_string(&result)
                .unwrap_or_default()
                .trim()
                .to_owned(),
            fs::read_to_string(&args).unwrap_or_default(),
            started.exists(),
        )
    }

    #[test]
    fn a_per_user_install_runs_msiexec_on_the_verified_copy_and_restarts() {
        let dir = scratch("msi-user");
        let (ok, result, args, started) = helper(&dir, "user", 0, None);
        let log = fs::read_to_string(dir.join("update.log")).unwrap_or_default();
        assert!(ok, "{log}");
        assert_eq!(result, "ok", "{log}");
        assert!(args.contains("/i") && args.contains("update.msi"), "{args}");
        assert!(args.contains("/passive"), "{args}");
        assert!(args.contains("INSTALLFOLDER=\"C:\\Apps\\Probe\""), "{args}");
        assert!(
            args.contains("MSIINSTALLPERUSER=1") && !args.contains("ALLUSERS=1"),
            "{args}"
        );
        assert!(started, "the app is started again");
        assert!(
            !dir.join("probe-2.0.0-windows-x64.msi").exists(),
            "the package is removed"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_failed_msi_is_reported_and_the_app_still_starts() {
        let dir = scratch("msi-fail");
        let (ok, result, _, started) = helper(&dir, "user", 1603, None);
        assert!(!ok);
        assert_eq!(result, "install");
        assert!(started);
        assert_eq!(
            crate::apply::take_last_failure_in(&dir.join("result.txt")),
            Some(crate::Failure::Install)
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_package_that_changed_is_never_installed() {
        let dir = scratch("msi-digest");
        let (ok, result, args, started) = helper(&dir, "user", 0, Some(&"0".repeat(64)));
        assert!(!ok);
        assert_eq!(result, "contents");
        assert!(args.is_empty(), "msiexec never ran: {args}");
        assert!(started);
        let _ = fs::remove_dir_all(&dir);
    }
}
