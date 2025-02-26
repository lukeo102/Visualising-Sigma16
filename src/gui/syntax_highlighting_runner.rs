use egui::text::LayoutJob;
use log::{log, Level};
use regex::Regex;

pub fn highlight(ctx: &egui::Context, theme: &CodeTheme, code: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<(&CodeTheme, &str), LayoutJob> for Highlighter {
        fn compute(&mut self, (theme, code): (&CodeTheme, &str)) -> LayoutJob {
            self.highlight(theme, code)
        }
    }

    type HighlightCache<'a> = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    ctx.memory(|memory| {
        memory
            .clone()
            .caches
            .cache::<HighlightCache<'_>>()
            .get((theme, code))
    })
}

#[derive(Clone, Copy, PartialEq)]
// #[derive(serde::Deserialize, serde::Serialize)]
#[derive(enum_map::Enum)]
enum TokenType {
    Line,
    Default,
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
                TokenType::Line => TextFormat::simple(font_id.clone(), Color32::RED),
                TokenType::Default => TextFormat::simple(font_id.clone(), Color32::WHITE),
            ],
        }
    }
}

#[derive(Default)]
struct Highlighter {}

impl Highlighter {
    fn highlight(&self, theme: &CodeTheme, code: &str) -> LayoutJob {
        // Extremely simple syntax highlighter for when we compile without syntect

        let mut text = code;

        let temp = code.find(",").unwrap_or(0);
        log!(Level::Info, "FOUND: {:?}", &text[..temp]);
        let target_line = usize::from_str_radix(&text[..temp], 10).unwrap_or(1) - 1;
        text = &text[temp + 1..];

        let mut job = LayoutJob::default();
        let mut line = 0;

        while !text.is_empty() {
            if line == target_line {
                let end = text.find("\n").unwrap_or(text.len());
                job.append(&text[..end], 0.0, theme.formats[TokenType::Line].clone());
                job.append(&text[end..], 0.0, theme.formats[TokenType::Default].clone());
                text = "";
            } else {
                let end = text.find("\n").unwrap_or(text.len()) + 1;
                job.append(&text[..end], 0.0, theme.formats[TokenType::Default].clone());
                line = line + 1;
                text = &text[end..];
            }
        }

        job
    }
}
