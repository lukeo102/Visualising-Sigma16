use egui::text::LayoutJob;
use log::{log, Level};
use regex::Regex;

pub fn highlight(ctx: &egui::Context, theme: &CodeTheme, code: &str, language: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<(&CodeTheme, &str, &str), LayoutJob> for Highlighter {
        fn compute(&mut self, (theme, code, lang): (&CodeTheme, &str, &str)) -> LayoutJob {
            self.highlight(theme, code, lang)
        }
    }

    type HighlightCache<'a> = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    log!(Level::Debug, "FETCHING FROM CACHE");
    ctx.memory(|memory| {
        memory
            .clone()
            .caches
            .cache::<HighlightCache<'_>>()
            .get((theme, code, language))
    })
}

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
    fn highlight(&self, theme: &CodeTheme, code: &str, language: &str) -> LayoutJob {
        match language {
            "sigma16" => self.sigma16_highlight(theme, code, language),
            _ => self.sigma16_highlight(theme, code, language),
        }
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn sigma16_highlight(&self, theme: &CodeTheme, mut text: &str, _language: &str) -> LayoutJob {
        // Extremely simple syntax highlighter for when we compile without syntect

        let mut job = LayoutJob::default();
        let re = Regex::new(r"$[a-fA-F0-9]{0,4}").unwrap();

        while !text.is_empty() {
            // =======
            // Comment
            // =======
            if text.starts_with(";") {
                let end = text.find('\n').unwrap_or(text.len());
                job.append(&text[..end], 0.0, theme.formats[TokenType::Comment].clone());
                text = &text[end..];
            } else if text.starts_with('$') {
                let mut end = 1;
                let l = if text.len() >= 5 { 5 } else { text.len() };

                for i in 1..l {
                    end = if text[i..i + 1]
                        .find(|c| "abcdefABCDEF1234567890".contains(c))
                        .unwrap_or_else(|| 1)
                        != 0
                    {
                        break;
                    } else {
                        i + 1
                    }
                }

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
        | "jump"
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
