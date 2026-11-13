//! Release notes: the reader's language only, as blocks a toolkit can draw.
//!
//! Nodalia releases are written in both languages at once: a summary line in Spanish with an
//! italic English line under it, then `## Español` and `## English` sections, then a bilingual
//! `## Descargas · Downloads` table. An update window wants only the reader's section, so
//! [`notes_blocks`] picks it and turns its Markdown into [`Block`]s; [`notes_markup`] and
//! [`notes_in`] render those as Pango markup and as plain text. Older notes used `**Español**`
//! and `**English**` lines instead of headings, and are read the same way. Notes with neither come
//! back whole.

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

    fn names(self) -> &'static [&'static str] {
        match self {
            Language::Spanish => &["español", "espanol"],
            Language::English => &["english", "inglés", "ingles"],
        }
    }

    fn other(self) -> Self {
        match self {
            Language::Spanish => Language::English,
            Language::English => Language::Spanish,
        }
    }
}

/// A run of text with one style.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
}

/// One paragraph-level piece of the notes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    /// A `###` heading inside the section: "Correcciones", "Notas".
    Heading(Vec<Span>),
    /// A `-`, `*` or `+` list item.
    Bullet(Vec<Span>),
    /// Anything else; consecutive lines are joined into one.
    Paragraph(Vec<Span>),
}

/// Notes longer than this are cut: a release body is someone else's text, and a window that tries
/// to lay out megabytes of it is a window that hangs.
const LIMIT: usize = 8_000;

/// The notes in `language`, as blocks.
pub fn notes_blocks(notes: &str, language: Language) -> Vec<Block> {
    let mut notes = notes;
    let capped;
    if notes.len() > LIMIT {
        let mut end = LIMIT;
        while !notes.is_char_boundary(end) {
            end -= 1;
        }
        capped = notes.get(..end).unwrap_or_default();
        notes = capped;
    }
    let lines: Vec<&str> = notes.lines().collect();
    let picked = section(&lines, language).unwrap_or(&lines);
    parse(picked)
}

/// The notes in `language` as Pango markup, for a GTK label with `use-markup`.
pub fn notes_markup(notes: &str, language: Language) -> String {
    let mut out = String::new();
    for (i, block) in notes_blocks(notes, language).iter().enumerate() {
        if i > 0 {
            out.push('\n');
            if matches!(block, Block::Heading(_)) {
                out.push('\n');
            }
        }
        match block {
            Block::Heading(spans) => {
                out.push_str("<b>");
                push_markup(&mut out, spans);
                out.push_str("</b>");
            }
            Block::Bullet(spans) => {
                out.push_str("•  ");
                push_markup(&mut out, spans);
            }
            Block::Paragraph(spans) => push_markup(&mut out, spans),
        }
    }
    out
}

/// The notes in `language` as plain text.
pub fn notes_in(notes: &str, language: Language) -> String {
    notes_blocks(notes, language)
        .iter()
        .map(|block| match block {
            Block::Heading(spans) | Block::Paragraph(spans) => plain(spans),
            Block::Bullet(spans) => format!("• {}", plain(spans)),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The plain text of some spans.
pub fn plain(spans: &[Span]) -> String {
    spans.iter().map(|s| s.text.as_str()).collect()
}

/// `(level, text)` for a `#` heading; level 0 for a line that is only `**bold**`.
fn heading(line: &str) -> Option<(usize, &str)> {
    let line = line.trim();
    let level = line.chars().take_while(|c| *c == '#').count();
    if level > 0 {
        let rest = line.get(level..)?;
        if rest.is_empty() || rest.starts_with(' ') {
            return Some((level, rest.trim()));
        }
        return None;
    }
    let inner = line.strip_prefix("**")?.strip_suffix("**")?;
    (!inner.is_empty() && !inner.contains("**")).then_some((0, inner.trim()))
}

fn names_language(text: &str, language: Language) -> bool {
    let text = text.trim().trim_matches('*').trim().to_lowercase();
    language.names().iter().any(|name| text == *name)
}

/// The lines of the section in `language`, without its heading.
fn section<'a>(lines: &'a [&'a str], language: Language) -> Option<&'a [&'a str]> {
    let (start, level) = lines.iter().enumerate().find_map(|(i, line)| {
        let (level, text) = heading(line)?;
        names_language(text, language).then_some((i, level))
    })?;
    let body = lines.get(start + 1..)?;
    let end = body
        .iter()
        .position(|line| match heading(line) {
            Some((_, text)) if names_language(text, language.other()) => true,
            // A `## Español` section ends at the next heading as big; a `**Español**` one at the
            // next `#` or `##` heading, which is where the old notes put their downloads.
            Some((l, _)) if level > 0 => l > 0 && l <= level,
            Some((l, _)) => l > 0 && l <= 2,
            None => false,
        })
        .unwrap_or(body.len());
    body.get(..end)
}

fn parse(lines: &[&str]) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut paragraph = String::new();
    let flush = |paragraph: &mut String, blocks: &mut Vec<Block>| {
        if !paragraph.is_empty() {
            blocks.push(Block::Paragraph(inline(paragraph)));
            paragraph.clear();
        }
    };
    for raw in lines {
        let line = raw.trim();
        if line.is_empty() || is_rule(line) {
            flush(&mut paragraph, &mut blocks);
            continue;
        }
        if let Some((level, text)) = heading(line) {
            if level > 0 {
                flush(&mut paragraph, &mut blocks);
                if !text.is_empty() {
                    blocks.push(Block::Heading(inline(text)));
                }
                continue;
            }
        }
        if let Some(item) = ["- ", "* ", "+ "].iter().find_map(|m| line.strip_prefix(m)) {
            flush(&mut paragraph, &mut blocks);
            blocks.push(Block::Bullet(inline(item.trim())));
            continue;
        }
        if !paragraph.is_empty() {
            paragraph.push(' ');
        }
        paragraph.push_str(line);
    }
    flush(&mut paragraph, &mut blocks);
    blocks
}

/// `---`, `***`, or a table's `|---|---|` separator.
fn is_rule(line: &str) -> bool {
    line.len() >= 3
        && line
            .chars()
            .all(|c| matches!(c, '-' | '*' | '_' | '|' | ':' | ' '))
}

/// Inline Markdown: `**bold**`, `*italic*`, `_italic_`, `` `code` `` and `[text](url)`.
fn inline(text: &str) -> Vec<Span> {
    let chars: Vec<char> = text.chars().collect();
    let mut spans: Vec<Span> = Vec::new();
    let mut current = Span::default();
    let (mut bold, mut italic) = (false, false);
    let mut i = 0;
    let push = |spans: &mut Vec<Span>, current: &mut Span| {
        if !current.text.is_empty() {
            spans.push(std::mem::take(current));
        }
    };
    while let Some(&c) = chars.get(i) {
        let next = chars.get(i + 1).copied();
        if c == '`' {
            if let Some(len) = chars
                .get(i + 1..)
                .and_then(|r| r.iter().position(|&d| d == '`'))
            {
                push(&mut spans, &mut current);
                spans.push(Span {
                    text: chars
                        .get(i + 1..i + 1 + len)
                        .unwrap_or_default()
                        .iter()
                        .collect(),
                    bold,
                    italic,
                    code: true,
                });
                current = Span {
                    bold,
                    italic,
                    ..Span::default()
                };
                i += len + 2;
                continue;
            }
        }
        if c == '*' && next == Some('*') {
            push(&mut spans, &mut current);
            bold = !bold;
            current = Span {
                bold,
                italic,
                ..Span::default()
            };
            i += 2;
            continue;
        }
        // An `_` inside a word (`snake_case`) is a letter, not emphasis.
        let word_before = i > 0 && chars.get(i - 1).is_some_and(|p| p.is_alphanumeric());
        let word_after = next.is_some_and(char::is_alphanumeric);
        let emphasis = c == '*' || (c == '_' && !(word_before && word_after));
        if emphasis && (italic || next.is_some_and(|n| !n.is_whitespace())) {
            push(&mut spans, &mut current);
            italic = !italic;
            current = Span {
                bold,
                italic,
                ..Span::default()
            };
            i += 1;
            continue;
        }
        if c == '[' {
            let rest = chars.get(i + 1..).unwrap_or_default();
            if let Some(close) = rest.iter().position(|&d| d == ']') {
                let after = rest.get(close + 1..).unwrap_or_default();
                if after.first() == Some(&'(') {
                    if let Some(end) = after.iter().position(|&d| d == ')') {
                        current.text.extend(rest.get(..close).unwrap_or_default());
                        i += 1 + close + 1 + end + 1;
                        continue;
                    }
                }
            }
        }
        current.text.push(c);
        i += 1;
    }
    push(&mut spans, &mut current);
    spans
}

fn push_markup(out: &mut String, spans: &[Span]) {
    for span in spans {
        let mut open = String::new();
        let mut close = String::new();
        for (on, tag) in [(span.bold, "b"), (span.italic, "i"), (span.code, "tt")] {
            if on {
                open.push_str(&format!("<{tag}>"));
                close.insert_str(0, &format!("</{tag}>"));
            }
        }
        out.push_str(&open);
        for c in span.text.chars() {
            match c {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '"' => out.push_str("&quot;"),
                '\'' => out.push_str("&#39;"),
                c => out.push(c),
            }
        }
        out.push_str(&close);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OLD: &str = "## App 2.0.1\n\n**Español**\n\n- Arreglo uno.\n- Arreglo `dos`.\n\n**English**\n\n- Fix one.\n- Fix **two**.\n";

    /// The house format, as Vinilo 0.2.3 shipped it.
    const HOUSE: &str = "Vinilo Native: actualizar desde la app ya cierra Vinilo.\n*Vinilo Native: updating from inside the app now quits Vinilo.*\n\n## Español\n\n### Correcciones en Vinilo Native\n- Al pulsar **Instalar y reiniciar**, Vinilo se quedaba en «Instalando…».\n\n### Notas\n- Las ediciones de macOS aún no están firmadas.\n\n## English\n\n### Fixes in Vinilo Native\n- After **Install and Restart**, Vinilo stayed on \"Installing…\".\n\n### Notes\n- The macOS editions are not yet signed.\n\n## Descargas · Downloads\n\n| Sistema · System | Archivo · File |\n|---|---|\n| Linux | `Vinilo-0.2.3-x86_64.AppImage` |\n";

    #[test]
    fn the_house_format_shows_only_the_readers_section() {
        let es = notes_in(HOUSE, Language::Spanish);
        assert_eq!(
            es,
            "Correcciones en Vinilo Native\n• Al pulsar Instalar y reiniciar, Vinilo se quedaba en «Instalando…».\nNotas\n• Las ediciones de macOS aún no están firmadas."
        );
        let en = notes_in(HOUSE, Language::English);
        assert!(en.starts_with("Fixes in Vinilo Native\n• After Install and Restart"));
        assert!(!en.contains("Correcciones"));
        for text in [&es, &en] {
            assert!(
                !text.contains("AppImage"),
                "the downloads table stays out: {text}"
            );
            assert!(!text.contains('*'), "no raw Markdown: {text}");
        }
    }

    #[test]
    fn the_house_format_becomes_blocks() {
        let blocks = notes_blocks(HOUSE, Language::English);
        assert_eq!(blocks.len(), 4);
        assert!(matches!(&blocks[0], Block::Heading(s) if plain(s) == "Fixes in Vinilo Native"));
        let Block::Bullet(spans) = &blocks[1] else {
            panic!("{blocks:?}")
        };
        assert!(spans
            .iter()
            .any(|s| s.bold && s.text == "Install and Restart"));
    }

    #[test]
    fn markup_is_escaped_and_styled() {
        let markup = notes_markup(
            "## English\n\n- Use `a<b>` & **this**.\n",
            Language::English,
        );
        assert_eq!(markup, "•  Use <tt>a&lt;b&gt;</tt> &amp; <b>this</b>.");
        let markup = notes_markup(HOUSE, Language::Spanish);
        assert!(markup.starts_with(
            "<b>Correcciones en Vinilo Native</b>\n•  Al pulsar <b>Instalar y reiniciar</b>"
        ));
        assert!(markup.contains("\n\n<b>Notas</b>\n"));
    }

    #[test]
    fn the_old_format_follows_the_language() {
        assert_eq!(notes_in(OLD, Language::English), "• Fix one.\n• Fix two.");
        assert_eq!(
            notes_in(OLD, Language::Spanish),
            "• Arreglo uno.\n• Arreglo dos."
        );
    }

    #[test]
    fn notes_without_language_headings_come_back_whole() {
        assert_eq!(
            notes_in(
                "## Title\n\n**Bold** text\nwith a [link](https://example.org)",
                Language::English
            ),
            "Title\nBold text with a link"
        );
    }

    #[test]
    fn underscores_inside_words_stay() {
        assert_eq!(
            notes_in("Set `x` in snake_case _now_", Language::English),
            "Set x in snake_case now"
        );
    }

    #[test]
    fn hostile_notes_do_not_panic_and_are_capped() {
        for notes in [
            "**",
            "`",
            "[",
            "[a](",
            "*",
            "_",
            "#",
            "## ",
            "**Español**",
            "ñ**ñ`ñ[ñ](ñ",
        ] {
            let _ = notes_markup(notes, Language::Spanish);
            let _ = notes_in(notes, Language::English);
        }
        let long = "ñ".repeat(20_000);
        assert!(notes_in(&long, Language::Spanish).len() <= LIMIT);
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
