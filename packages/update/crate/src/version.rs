//! Version ordering.

use std::cmp::Ordering;

/// `v1.2.3-beta+5` → `1.2.3-beta`: a leading `v` and any build metadata are dropped. The
/// pre-release stays, because it orders: `2.0.0-beta` comes before `2.0.0`.
pub fn strip(version: &str) -> &str {
    let version = version.trim();
    let version = version
        .strip_prefix('v')
        .or_else(|| version.strip_prefix('V'))
        .unwrap_or(version);
    match version.find('+') {
        Some(cut) if cut > 0 => &version[..cut],
        _ => version,
    }
}

/// The numeric core and the pre-release identifiers of a version.
fn split(version: &str) -> (Vec<u64>, Option<&str>) {
    let version = strip(version);
    let (core, pre) = match version.find('-') {
        Some(cut) if cut > 0 => (&version[..cut], version.get(cut + 1..)),
        _ => (version, None),
    };
    let parts = core
        .split('.')
        .map(|part| part.parse().unwrap_or(0))
        .collect();
    (parts, pre.filter(|p| !p.is_empty()))
}

/// Semver's rule for two pre-release strings: identifier by identifier, numbers before words,
/// and a shorter list first when the rest is equal.
fn compare_pre(left: &str, right: &str) -> Ordering {
    let mut a = left.split('.');
    let mut b = right.split('.');
    loop {
        match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) => {
                let order = match (x.parse::<u64>(), y.parse::<u64>()) {
                    (Ok(x), Ok(y)) => x.cmp(&y),
                    (Ok(_), Err(_)) => Ordering::Less,
                    (Err(_), Ok(_)) => Ordering::Greater,
                    (Err(_), Err(_)) => x.cmp(y),
                };
                if order != Ordering::Equal {
                    return order;
                }
            }
        }
    }
}

/// Compares two dotted versions numerically; a missing component counts as zero, and a version
/// with a pre-release (`2.0.0-beta`) is older than the same one without.
pub fn compare(left: &str, right: &str) -> Ordering {
    let ((a, pre_a), (b, pre_b)) = (split(left), split(right));
    for index in 0..a.len().max(b.len()) {
        let order = a
            .get(index)
            .copied()
            .unwrap_or(0)
            .cmp(&b.get(index).copied().unwrap_or(0));
        if order != Ordering::Equal {
            return order;
        }
    }
    match (pre_a, pre_b) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (Some(x), Some(y)) => compare_pre(x, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_not_textual() {
        assert_eq!(compare("2.0.10", "2.0.9"), Ordering::Greater);
        assert_eq!(compare("v2.0.0", "2.0.0"), Ordering::Equal);
        assert_eq!(compare("2.0", "2.0.0"), Ordering::Equal);
        assert_eq!(compare("1.9.9", "2.0.0"), Ordering::Less);
    }

    #[test]
    fn a_pre_release_is_older_than_its_release() {
        assert_eq!(compare("2.0.0-beta", "2.0.0"), Ordering::Less);
        assert_eq!(compare("2.0.1-dev", "2.0.1"), Ordering::Less);
        assert_eq!(compare("2.0.1", "2.0.1-dev"), Ordering::Greater);
        assert_eq!(compare("2.0.1-dev", "2.0.0"), Ordering::Greater);
    }

    #[test]
    fn pre_releases_order_like_semver() {
        assert_eq!(compare("1.0.0-alpha", "1.0.0-alpha.1"), Ordering::Less);
        assert_eq!(compare("1.0.0-alpha.2", "1.0.0-alpha.10"), Ordering::Less);
        assert_eq!(compare("1.0.0-alpha.1", "1.0.0-alpha.beta"), Ordering::Less);
        assert_eq!(compare("1.0.0-beta", "1.0.0-rc"), Ordering::Less);
        assert_eq!(compare("1.0.0-rc.1", "1.0.0-rc.1"), Ordering::Equal);
    }

    #[test]
    fn build_metadata_is_ignored() {
        assert_eq!(compare("0.2.0+7", "0.2.0"), Ordering::Equal);
        assert_eq!(compare("0.2.0-beta+7", "0.2.0-beta"), Ordering::Equal);
        assert_eq!(strip("V3.1"), "3.1");
        assert_eq!(strip("v2.0.1-beta+9"), "2.0.1-beta");
    }

    #[test]
    fn junk_is_zero() {
        assert_eq!(compare("", "0.0.0"), Ordering::Equal);
        assert_eq!(compare("x.y", "0"), Ordering::Equal);
    }
}
