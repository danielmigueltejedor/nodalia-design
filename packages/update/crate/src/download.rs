//! HTTP: the release lookup and the verified download.

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use sha2::{Digest, Sha256};

use crate::{App, Error, Release};

const USER_AGENT: &str = "Nodalia-Updater";

/// Tells two downloads in one process apart, so neither can write into the other's partial file.
static DOWNLOADS: AtomicU64 = AtomicU64::new(0);

/// The operating system's certificate store, so a company or distribution trust anchor works.
fn tls() -> ureq::tls::TlsConfig {
    ureq::tls::TlsConfig::builder()
        .root_certs(ureq::tls::RootCerts::PlatformVerifier)
        .build()
}

pub(crate) fn get(url: &str, accept: &str) -> Result<Vec<u8>, Error> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .tls_config(tls())
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .into();
    let mut response = agent
        .get(url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", accept)
        .call()
        .map_err(|e| Error::Network(e.to_string()))?;
    response
        .body_mut()
        .with_config()
        .limit(8 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| Error::Network(e.to_string()))
}

/// Downloads the release's package into [`update_dir`](crate::update_dir) and returns its path.
///
/// `progress(received, total)` is called as bytes arrive; `total` is `None` when the server does
/// not say. The file is hashed while it is written, and is kept only if the SHA-256 matches the
/// digest GitHub published for it: a release without one is refused.
pub fn download(
    app: &App,
    release: &Release,
    mut progress: impl FnMut(u64, Option<u64>),
) -> Result<PathBuf, Error> {
    let asset = release
        .asset
        .as_ref()
        .ok_or(Error::NoPackage(release.version.clone()))?;
    let expected = asset.sha256.as_deref().ok_or(Error::NoDigest)?;

    let dir = crate::update_dir(app);
    fs::create_dir_all(&dir)
        .map_err(|e| Error::Io(format!("Could not create {}: {e}", dir.display())))?;
    // A release's asset name is the server's text: keep only its last component.
    let name = std::path::Path::new(&asset.name)
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|n| !n.is_empty())
        .ok_or(Error::BadResponse)?;
    let dest = dir.join(name);
    // Unique per call: two downloads of one asset must not truncate each other's file, nor
    // publish bytes that were not the ones they hashed.
    let partial = dir.join(format!(
        "{name}.{}-{}.part",
        std::process::id(),
        DOWNLOADS.fetch_add(1, Ordering::Relaxed)
    ));

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .tls_config(tls())
        // Large packages: no overall deadline, but every wait for the server is bounded, so a
        // stalled connection, a server that never answers or one that stops mid-body gives up.
        .timeout_resolve(Some(Duration::from_secs(15)))
        .timeout_connect(Some(Duration::from_secs(15)))
        .timeout_recv_response(Some(Duration::from_secs(30)))
        .timeout_recv_body(Some(Duration::from_secs(60)))
        .build()
        .into();
    let response = agent
        .get(&asset.url)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| Error::Network(e.to_string()))?;
    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .or(asset.size);

    let result = (|| {
        let mut reader = response.into_body().into_reader();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&partial)
            .map_err(|e| Error::Io(e.to_string()))?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 64 * 1024];
        let mut received = 0u64;
        loop {
            let read = reader
                .read(&mut buffer)
                .map_err(|e| Error::Network(e.to_string()))?;
            let Some(chunk) = buffer.get(..read).filter(|c| !c.is_empty()) else {
                break;
            };
            hasher.update(chunk);
            file.write_all(chunk)
                .map_err(|e| Error::Io(e.to_string()))?;
            received += read as u64;
            progress(received, total);
        }
        file.flush().map_err(|e| Error::Io(e.to_string()))?;
        if !hex(&hasher.finalize()).eq_ignore_ascii_case(expected) {
            return Err(Error::Checksum);
        }
        fs::rename(&partial, &dest).map_err(|e| Error::Io(e.to_string()))?;
        Ok(dest)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&partial);
    }
    result
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_is_lower_case_and_padded() {
        assert_eq!(hex(&[0x00, 0xab, 0x0f]), "00ab0f");
    }
}
