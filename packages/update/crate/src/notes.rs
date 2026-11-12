//! Release notes, cut to what fits in a question.

/// The language an app shows its update windows in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    /// `es`, `es-ES`, `es_MX.UTF-8` → Spanish; anything else → English.
    pub fn from_code(code: &str) -> Self {
        let code = code.trim().to_ascii_lowercase();
        if code == "es" || code.starts_with("es-") || code.starts_with("es_") {
            Language::Spanish
        } else {
            Language::English
        }
    }
}

/// The part of a release's notes in `language`, as plain text and short enough for a question.
///
/// Nodalia releases are written bilingually, under `**Español**` and `**English**` headings; notes
/// without them come back whole. Markdown emphasis, headings and code marks are dropped, empty
/// lines removed, and the text cut at 700 characters.
pub fn notes_in(notes: &str, language: Language) -> String {
    const ES: &str = "**Español**";
    const EN: &str = "**English**";
    let section = |start: &str, other: &str| -> Option<String> {
        let from = notes.find(start)? + start.len();
        let rest = notes.get(from..)?;
        let end = rest.find(other).unwrap_or(rest.len());
        rest.get(..end).map(str::to_owned)
    };
    let picked = match language {
        Language::Spanish => section(ES, EN),
        Language::English => section(EN, ES),
    }
    .unwrap_or_else(|| notes.to_owned());

    let text = picked
        .lines()
        .map(|line| {
            line.trim()
                .trim_start_matches('#')
                .replace("**", "")
                .replace('`', "")
                .trim()
                .to_owned()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    const LIMIT: usize = 700;
    if text.chars().count() <= LIMIT {
        return text;
    }
    let cut: String = text.chars().take(LIMIT).collect();
    format!("{}…", cut.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTES: &str = "## App 2.0.1\n\n**Español**\n\n- Arreglo uno.\n- Arreglo `dos`.\n\n**English**\n\n- Fix one.\n- Fix **two**.\n";

    #[test]
    fn the_notes_follow_the_language() {
        assert_eq!(notes_in(NOTES, Language::English), "- Fix one.\n- Fix two.");
        assert_eq!(
            notes_in(NOTES, Language::Spanish),
            "- Arreglo uno.\n- Arreglo dos."
        );
    }

    #[test]
    fn notes_without_language_headings_pass_through_as_plain_text() {
        assert_eq!(
            notes_in("## Title\n\n**Bold** text", Language::English),
            "Title\nBold text"
        );
    }

    #[test]
    fn long_notes_are_cut() {
        let cut = notes_in(&"palabra ".repeat(400), Language::Spanish);
        assert!(cut.chars().count() <= 701);
        assert!(cut.ends_with('…'));
    }

    #[test]
    fn language_codes() {
        for code in ["es", "ES", "es-ES", "es_MX.UTF-8"] {
            assert_eq!(Language::from_code(code), Language::Spanish, "{code}");
        }
        for code in ["en", "", "est", "fr_FR"] {
            assert_eq!(Language::from_code(code), Language::English, "{code}");
        }
    }
}
