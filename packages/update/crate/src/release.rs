//! A GitHub release, and which of its assets is this machine's package.

use serde_json::Value;

use crate::{Error, Release, ReleaseAsset};

/// Names a package for this platform, best first. Matching is case-insensitive and on a
/// substring, so `Vinilo-2.0.1-macos-arm64.dmg` answers `macos-arm64.dmg`.
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
    #[cfg(target_os = "linux")]
    {
        &[
            "linux-x86_64.appimage",
            "linux-amd64.appimage",
            "x86_64.appimage",
            ".appimage",
        ]
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
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
/// on macOS every name is searched as `<edition>-<name>`, which is how the disk images are
/// called (`Vinilo-2.0.1-native-macos-arm64.dmg`). Elsewhere, and without an edition, it is
/// [`preferred_assets`].
pub fn preferred_assets_for(edition: Option<&str>) -> Vec<String> {
    let base = preferred_assets();
    match edition {
        Some(edition) if cfg!(target_os = "macos") => {
            let edition = edition.to_ascii_lowercase();
            base.iter()
                .map(|name| format!("{edition}-{name}"))
                .collect()
        }
        _ => base.iter().map(|name| (*name).to_owned()).collect(),
    }
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
        asset: select_asset(assets, preferred),
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
            if !name
                .to_ascii_lowercase()
                .contains(&needle.to_ascii_lowercase())
            {
                continue;
            }
            // Checksum sidecars (`….AppImage.sha256`) contain the package's name but are not it.
            if name.ends_with(".sha256") || name.ends_with(".sig") {
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
            let needles: Vec<String> = ["macos-arm64.dmg"]
                .iter()
                .map(|n| format!("{edition}-{n}"))
                .collect();
            let release = parse(json, &needles).unwrap();
            assert_eq!(release.asset.unwrap().name, expected);
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_edition_prefixes_every_macos_name() {
        assert!(preferred_assets_for(Some("Native"))
            .iter()
            .all(|n| n.starts_with("native-")));
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
}
