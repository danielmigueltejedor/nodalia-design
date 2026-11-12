//! Version ordering.

use std::cmp::Ordering;

/// `v1.2.3-beta+5` → `1.2.3`: a leading `v` and any pre-release or build suffix are dropped, so
/// `0.2.0-beta` orders as `0.2.0`.
pub fn strip(version: &str) -> &str {
    let version = version.trim();
    let version = version
        .strip_prefix('v')
        .or_else(|| version.strip_prefix('V'))
        .unwrap_or(version);
    match version.find(['+', '-']) {
        Some(cut) if cut > 0 => &version[..cut],
        _ => version,
    }
}

fn parts(version: &str) -> Vec<u64> {
    strip(version)
        .split('.')
        .map(|part| part.parse().unwrap_or(0))
        .collect()
}

/// Compares two dotted versions numerically; a missing component counts as zero.
pub fn compare(left: &str, right: &str) -> Ordering {
    let (a, b) = (parts(left), parts(right));
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
    Ordering::Equal
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
    fn suffixes_are_ignored() {
        assert_eq!(compare("2.0.1-dev", "2.0.1"), Ordering::Equal);
        assert_eq!(compare("0.2.0-beta+7", "0.2.0"), Ordering::Equal);
        assert_eq!(strip("V3.1"), "3.1");
    }

    #[test]
    fn junk_is_zero() {
        assert_eq!(compare("", "0.0.0"), Ordering::Equal);
        assert_eq!(compare("x.y", "0"), Ordering::Equal);
    }
}
