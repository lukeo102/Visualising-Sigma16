use egui::text::LayoutJob;


/// Memoized Code highlighting
pub fn highlight(ctx: &egui::Context, theme: &CodeTheme, code: &str, language: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<(&CodeTheme, &str, &str), LayoutJob> for Highlighter {
        fn compute(&mut self, (theme, code, lang): (&CodeTheme, &str, &str)) -> LayoutJob {
            self.highlight(theme, code, lang)
        }
    }

    type HighlightCache<'a> = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    ctx.memory(|memory| {
        memory
            .clone()
            .caches
            .cache::<HighlightCache<'_>>()
            .get((theme, code, language))
    })
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
// #[derive(serde::Deserialize, serde::Serialize)]
#[derive(enum_map::Enum)]
enum TokenType {
    Comment,
    Keyword,
    Literal,
    HexLiteral,
    Punctuation,
    Whitespace,
}

#[derive(Clone, Hash, PartialEq)]
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)]
pub struct CodeTheme {
    formats: enum_map::EnumMap<TokenType, egui::TextFormat>,
}

impl Default for CodeTheme {
    fn default() -> Self {
        let font_id = egui::FontId::monospace(12.0);
        use egui::{Color32, TextFormat};
        Self {
            formats: enum_map::enum_map![
                TokenType::Comment => TextFormat::simple(font_id.clone(), Color32::GRAY),
                TokenType::Keyword => TextFormat::simple(font_id.clone(), Color32::from_rgb(235, 0, 0)),
                TokenType::Literal => TextFormat::simple(font_id.clone(), Color32::from_rgb(153, 134, 255)),
                TokenType::HexLiteral => TextFormat::simple(font_id.clone(), Color32::from_rgb(37, 203, 105)),
                TokenType::Punctuation => TextFormat::simple(font_id.clone(), Color32::DARK_GRAY),
                TokenType::Whitespace => TextFormat::simple(font_id.clone(), Color32::TRANSPARENT),
            ],
        }    
    }
}


#[derive(Default)]
struct Highlighter {}

impl Highlighter {
    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn highlight(&self, theme: &CodeTheme, mut text: &str, _language: &str) -> LayoutJob {
        // Extremely simple syntax highlighter for when we compile without syntect

        let mut job = LayoutJob::default();

        while !text.is_empty() {
            // =======
            // Comment
            // =======
            if text.starts_with("--") {
                let end = text.find('\n').unwrap_or(text.len());
                job.append(&text[..end], 0.0, theme.formats[TokenType::Comment].clone());
                text = &text[end..];
            } else if text.starts_with('$') {
                let end = text[6..]
                    .rfind(r"![0-9a-fA-F]")
                    .map(|i| i + 2)
                    .or_else(|| text.find('['))
                    .unwrap_or(text.len());
                job.append(
                    &text[..end],
                    0.0,
                    theme.formats[TokenType::HexLiteral].clone(),
                );
                text = &text[end..];
            } else if text.starts_with(|c: char| c.is_ascii_alphanumeric()) {
                let end = text[1..]
                    .find(|c: char| !c.is_ascii_alphanumeric())
                    .map_or_else(|| text.len(), |i| i + 1);
                let word = &text[..end];
                let tt = if is_keyword(word) {
                    TokenType::Keyword
                } else {
                    TokenType::Literal
                };
                job.append(word, 0.0, theme.formats[tt].clone());
                text = &text[end..];
            } else if text.starts_with(|c: char| c.is_ascii_whitespace()) {
                let end = text[1..]
                    .find(|c: char| !c.is_ascii_whitespace())
                    .map_or_else(|| text.len(), |i| i + 1);
                job.append(
                    &text[..end],
                    0.0,
                    theme.formats[TokenType::Whitespace].clone(),
                );
                text = &text[end..];
            } else {
                let mut it = text.char_indices();
                it.next();
                let end = it.next().map_or(text.len(), |(idx, _chr)| idx);
                job.append(
                    &text[..end],
                    0.0,
                    theme.formats[TokenType::Punctuation].clone(),
                );
                text = &text[end..];
            }
        }

        job
    }
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        // RRR Instructions
          "add"
        | "sub"
        | "mul"
        | "muln"
        | "divn"
        | "rrr1"
        | "rrr2"
        | "rrr3"
        | "trap"
        // RR Instructions
        | "cmp"
        // RX Instructions
        | "lea" 
        | "load"
        | "store"
        | "jal"
        | "jumpnz"
        | "jumpz"
        | "jumplt"
        | "jumple"
        | "jumpeq"
        | "jumpne"
        | "jumpge"
        | "jumpgt"
        | "testset"
        // Non-Instructions
        | "data"
    )
}