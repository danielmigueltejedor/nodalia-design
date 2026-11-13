//! A GitHub release, and which of its assets is this machine's package.

use serde_json::Value;

use crate::{Error, Release, ReleaseAsset};

/// Names a package for this platform, best first. Matching is case-insensitive and on the end
/// of the asset's name, so `Vinilo-2.0.1-macos-arm64.dmg` answers `macos-arm64.dmg` and a
/// `.zsync` or `.sha256` beside the package never does. A package for another CPU is never
/// listed: an AppImage built for x86-64 would replace an aarch64 one and not start.
pub fn preferred_assets() -> &'static [&'static str] {
    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            &["macos-arm64.dmg", "macos-universal.dmg", "macos.dmg"]
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            &[
                "macos-x86_64.dmg",
                "macos-intel.dmg",
                "macos-universal.dmg",
                "macos.dmg",
            ]
        }
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        &[
            "linux-x86_64.appimage",
            "linux-amd64.appimage",
            "x86_64.appimage",
        ]
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        &[
            "linux-aarch64.appimage",
            "linux-arm64.appimage",
            "aarch64.appimage",
            "arm64.appimage",
        ]
    }
    #[cfg(all(
        target_os = "linux",
        not(any(target_arch = "x86_64", target_arch = "aarch64"))
    ))]
    {
        &[]
    }
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        &["windows-x64.msi", "windows-amd64.msi", "windows-x86_64.msi"]
    }
    #[cfg(all(windows, target_arch = "aarch64"))]
    {
        &["windows-arm64.msi", "windows-aarch64.msi"]
    }
    #[cfg(all(windows, not(any(target_arch = "x86_64", target_arch = "aarch64"))))]
    {
        &[]
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        &[]
    }
}

/// `sha256:<hex>` as GitHub reports an asset's digest, lower-cased; `None` for anything else.
pub fn asset_digest(asset: &Value) -> Option<String> {
    let digest = asset.get("digest")?.as_str()?.strip_prefix("sha256:")?;
    is_sha256(digest).then(|| digest.to_ascii_lowercase())
}

pub(crate) fn is_sha256(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Like [`preferred_assets`], for an app whose release carries several builds per platform:
/// on macOS every name is searched as `{version}-<edition>-<name>`, which is how the disk images
/// are called (`Vinilo-2.0.1-native-macos-arm64.dmg`). `{version}` is the release's own version,
/// filled in by [`parse`]: it is what stops `native` from also matching `non-native`.
/// Elsewhere, and without an edition, it is [`preferred_assets`].
pub fn preferred_assets_for(edition: Option<&str>) -> Vec<String> {
    let base = preferred_assets();
    match edition {
        Some(edition) if cfg!(target_os = "macos") => {
            let edition = edition.to_ascii_lowercase();
            base.iter()
                .map(|name| format!("{{version}}-{edition}-{name}"))
                .collect()
        }
        _ => base.iter().map(|name| (*name).to_owned()).collect(),
    }
}

/// What marks a package for this operating system, whatever its CPU or format: a release
/// without any of these is not for this machine (a macOS-only bug-fix release, say).
pub fn platform_markers() -> &'static [&'static str] {
    #[cfg(target_os = "macos")]
    {
        &[".dmg"]
    }
    #[cfg(target_os = "linux")]
    {
        &[".appimage", ".deb", ".rpm", ".flatpak", ".pkg.tar.zst"]
    }
    #[cfg(windows)]
    {
        &[".msi", "-portable.zip", ".exe"]
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        &[]
    }
}

/// Whether the body of `releases` lists no published release at all (a repository that has not
/// released yet). Drafts and pre-releases do not count.
pub(crate) fn is_empty_list(json: &[u8]) -> bool {
    serde_json::from_slice::<Value>(json)
        .ok()
        .and_then(|list| {
            list.as_array().map(|releases| {
                !releases.iter().any(|r| {
                    !r.get("draft").and_then(Value::as_bool).unwrap_or(false)
                        && !r
                            .get("prerelease")
                            .and_then(Value::as_bool)
                            .unwrap_or(false)
                })
            })
        })
        .unwrap_or(false)
}

/// Reads the body of `releases` (newest first) and returns the newest published release that
/// carries a package for this operating system, read as [`parse`] reads one. Releases of a
/// single platform (a z version for macOS only) are then never offered elsewhere; a list where
/// no release names a package for this system falls back to the newest one, as `latest` did.
pub fn parse_list<S: AsRef<str>>(json: &[u8], preferred: &[S]) -> Result<Release, Error> {
    let list: Value = serde_json::from_slice(json).map_err(|_| Error::BadResponse)?;
    let releases: Vec<&Value> = list
        .as_array()
        .ok_or(Error::BadResponse)?
        .iter()
        .filter(|r| {
            !r.get("draft").and_then(Value::as_bool).unwrap_or(false)
                && !r
                    .get("prerelease")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
        })
        .collect();
    let for_this_system = |release: &Value| {
        release
            .get("assets")
            .and_then(Value::as_array)
            .is_some_and(|assets| {
                assets.iter().any(|asset| {
                    let name = asset
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_ascii_lowercase();
                    platform_markers()
                        .iter()
                        .any(|marker| name.ends_with(marker))
                })
            })
    };
    let chosen = releases
        .iter()
        .copied()
        .find(|release| for_this_system(release))
        .or_else(|| releases.first().copied())
        .ok_or(Error::NoVersion)?;
    let body = serde_json::to_vec(chosen).map_err(|_| Error::BadResponse)?;
    parse(&body, preferred)
}

/// Reads the body of `releases/latest`.
pub fn parse<S: AsRef<str>>(json: &[u8], preferred: &[S]) -> Result<Release, Error> {
    let root: Value = serde_json::from_slice(json).map_err(|_| Error::BadResponse)?;
    let tag = root
        .get("tag_name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let version = crate::version::strip(&tag).to_owned();
    if version.is_empty() {
        return Err(Error::NoVersion);
    }
    let preferred: Vec<String> = preferred
        .iter()
        .map(|needle| needle.as_ref().replace("{version}", &version))
        .collect();
    let text = |key: &str| {
        root.get(key)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    let assets = root
        .get("assets")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    Ok(Release {
        tag,
        version,
        notes: text("body"),
        page: text("html_url"),
        asset: select_asset(assets, &preferred),
    })
}

fn select_asset<S: AsRef<str>>(assets: &[Value], preferred: &[S]) -> Option<ReleaseAsset> {
    for needle in preferred {
        let needle = needle.as_ref();
        for asset in assets {
            let name = asset
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            // The end of the name, not any part of it: `….AppImage.zsync` and `….AppImage.sha256`
            // contain the package's name and are not the package.
            if !name
                .to_ascii_lowercase()
                .ends_with(&needle.to_ascii_lowercase())
            {
                continue;
            }
            let url = asset
                .get("browser_download_url")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if url.is_empty() {
                continue;
            }
            return Some(ReleaseAsset {
                name: name.to_owned(),
                url: url.to_owned(),
                sha256: asset_digest(asset),
                size: asset.get("size").and_then(Value::as_u64),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789ABCDEF";

    fn body() -> Vec<u8> {
        format!(
            r#"{{"tag_name":"v2.0.1","body":"notes","html_url":"https://example.org/r",
            "assets":[
              {{"name":"Vinilo-2.0.1-x86_64.AppImage.sha256","browser_download_url":"https://e/x.sha","size":1}},
              {{"name":"Vinilo-2.0.1-x86_64.AppImage","browser_download_url":"https://e/a","size":9,"digest":"sha256:{HASH}"}},
              {{"name":"Vinilo-2.0.1-macos-arm64.dmg","browser_download_url":"https://e/d","size":7}}
            ]}}"#
        )
        .into_bytes()
    }

    #[test]
    fn an_edition_only_ever_picks_its_own_disk_image() {
        let json = br#"{"tag_name":"v2.0.1","body":"","html_url":"https://e/r","assets":[
            {"name":"Vinilo-2.0.1-native-macos-arm64.dmg","browser_download_url":"https://e/n","size":1},
            {"name":"Vinilo-2.0.1-nodalia-macos-arm64.dmg","browser_download_url":"https://e/d","size":1}]}"#;
        for (edition, expected) in [
            ("nodalia", "Vinilo-2.0.1-nodalia-macos-arm64.dmg"),
            ("native", "Vinilo-2.0.1-native-macos-arm64.dmg"),
        ] {
            let needles = [format!("{{version}}-{edition}-macos-arm64.dmg")];
            let release = parse(json, &needles).unwrap();
            assert_eq!(release.asset.unwrap().name, expected);
        }
    }

    #[test]
    fn an_edition_name_is_not_found_inside_a_longer_one() {
        let json = br#"{"tag_name":"v2.0.1","body":"","html_url":"https://e/r","assets":[
            {"name":"Vinilo-2.0.1-non-native-macos-arm64.dmg","browser_download_url":"https://e/x","size":1},
            {"name":"Vinilo-2.0.1-native-macos-arm64.dmg","browser_download_url":"https://e/n","size":1}]}"#;
        let release = parse(json, &["{version}-native-macos-arm64.dmg"]).unwrap();
        assert_eq!(
            release.asset.unwrap().name,
            "Vinilo-2.0.1-native-macos-arm64.dmg"
        );
        let only_the_other = br#"{"tag_name":"v2.0.1","body":"","html_url":"https://e/r","assets":[
            {"name":"Vinilo-2.0.1-non-native-macos-arm64.dmg","browser_download_url":"https://e/x","size":1}]}"#;
        assert!(parse(only_the_other, &["{version}-native-macos-arm64.dmg"])
            .unwrap()
            .asset
            .is_none());
    }

    #[test]
    fn a_zsync_beside_the_appimage_is_not_the_appimage() {
        let json = br#"{"tag_name":"v2.0.1","body":"","html_url":"https://e/r","assets":[
            {"name":"Vinilo-2.0.1-x86_64.AppImage.zsync","browser_download_url":"https://e/z","size":1},
            {"name":"Vinilo-2.0.1-x86_64.AppImage","browser_download_url":"https://e/a","size":9}]}"#;
        let release = parse(json, &["x86_64.appimage"]).unwrap();
        assert_eq!(release.asset.unwrap().name, "Vinilo-2.0.1-x86_64.AppImage");
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    #[test]
    fn an_arm_machine_is_never_offered_the_x86_package() {
        assert!(preferred_assets().iter().all(|n| !n.contains("x86")));
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[test]
    fn an_x86_machine_is_never_offered_the_arm_package() {
        assert!(preferred_assets()
            .iter()
            .all(|n| !n.contains("arm") && !n.contains("aarch")));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_edition_prefixes_every_macos_name() {
        assert!(preferred_assets_for(Some("Native"))
            .iter()
            .all(|n| n.starts_with("{version}-native-")));
        assert_eq!(preferred_assets_for(None).len(), preferred_assets().len());
    }

    #[test]
    fn picks_the_package_not_its_checksum() {
        let release = parse(&body(), &["x86_64.appimage"]).unwrap();
        assert_eq!(release.version, "2.0.1");
        let asset = release.asset.unwrap();
        assert_eq!(asset.name, "Vinilo-2.0.1-x86_64.AppImage");
        assert_eq!(asset.size, Some(9));
        assert_eq!(
            asset.sha256.as_deref(),
            Some(HASH.to_ascii_lowercase().as_str())
        );
    }

    #[test]
    fn macos_asset_without_digest_has_none() {
        let release = parse(&body(), &["macos-arm64.dmg"]).unwrap();
        assert!(release.asset.unwrap().sha256.is_none());
    }

    #[test]
    fn no_match_is_none_and_bad_json_is_an_error() {
        assert!(parse(&body(), &["windows.exe"]).unwrap().asset.is_none());
        assert!(matches!(
            parse(b"[]", &[] as &[&str]),
            Err(Error::NoVersion)
        ));
        assert!(matches!(
            parse(b"nope", &[] as &[&str]),
            Err(Error::BadResponse)
        ));
    }

    #[test]
    fn only_a_full_sha256_is_a_digest() {
        let short: Value = serde_json::from_str(r#"{"digest":"sha256:abc"}"#).unwrap();
        assert!(asset_digest(&short).is_none());
        let other: Value = serde_json::from_str(&format!(r#"{{"digest":"sha1:{HASH}"}}"#)).unwrap();
        assert!(asset_digest(&other).is_none());
    }

    #[test]
    fn a_release_for_another_system_is_passed_over() {
        let other = if cfg!(target_os = "macos") {
            "Vinilo-0.1.3-x86_64.AppImage"
        } else {
            "Vinilo-0.1.3-nodalia-macos-arm64.dmg"
        };
        let mine = if cfg!(target_os = "macos") {
            "Vinilo-0.1.1-nodalia-macos-arm64.dmg"
        } else {
            "Vinilo-0.1.1-x86_64.AppImage"
        };
        let list = format!(
            r#"[
              {{"tag_name": "v0.1.4", "draft": true, "assets": [{{"name": "{mine}"}}]}},
              {{"tag_name": "v0.1.3", "assets": [{{"name": "{other}"}}]}},
              {{"tag_name": "v0.1.2", "prerelease": true, "assets": [{{"name": "{mine}"}}]}},
              {{"tag_name": "v0.1.1", "assets": [{{"name": "{mine}"}}]}}
            ]"#
        );
        let release = super::parse_list(list.as_bytes(), &["nothing-matches"]).unwrap();
        if cfg!(any(target_os = "macos", target_os = "linux")) {
            assert_eq!(release.version, "0.1.1");
        }
    }

    #[test]
    fn a_repository_without_releases_is_an_empty_list() {
        assert!(is_empty_list(b"[]"));
        assert!(is_empty_list(br#"[{"tag_name":"v1.0.0","draft":true}]"#));
        assert!(!is_empty_list(br#"[{"tag_name":"v1.0.0"}]"#));
        assert!(
            !is_empty_list(b"{\"message\":\"Not Found\"}"),
            "an error is not an empty list"
        );
    }

    #[test]
    fn windows_picks_the_msi_not_the_portable_zip_or_symbols() {
        let json = br#"{"tag_name":"v0.3.2","body":"","html_url":"https://e/r","assets":[
            {"name":"lienzo-0.3.2-windows-x64-portable.zip","browser_download_url":"https://e/z","size":1},
            {"name":"lienzo-0.3.2-windows-x64-symbols.zip","browser_download_url":"https://e/s","size":1},
            {"name":"lienzo-0.3.2-windows-arm64.msi","browser_download_url":"https://e/a","size":1},
            {"name":"lienzo-0.3.2-windows-x64.msi","browser_download_url":"https://e/m","size":1}]}"#;
        let release = parse(json, &["windows-x64.msi"]).unwrap();
        assert_eq!(release.asset.unwrap().name, "lienzo-0.3.2-windows-x64.msi");
        let arm = parse(json, &["windows-arm64.msi"]).unwrap();
        assert_eq!(arm.asset.unwrap().name, "lienzo-0.3.2-windows-arm64.msi");
    }

    #[cfg(windows)]
    #[test]
    fn windows_has_packages_and_markers() {
        assert!(!preferred_assets().is_empty());
        assert!(preferred_assets().iter().all(|n| n.ends_with(".msi")));
        assert!(platform_markers().contains(&".msi"));
    }

    #[test]
    fn without_any_package_for_this_system_the_newest_is_kept() {
        let list =
            br#"[{"tag_name": "v2.0.0", "assets": []}, {"tag_name": "v1.0.0", "assets": []}]"#;
        let release = super::parse_list(list, &["x"]).unwrap();
        assert_eq!(release.version, "2.0.0");
    }
}
