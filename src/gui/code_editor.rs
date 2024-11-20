use crate::assembler::code::Code;
use crate::gui::app::VisualisingSigma16;
use crate::gui::syntax_highlighting::{highlight, CodeTheme};
use crate::gui::util::format_code;
use egui::{Galley, Layout, Response, TextBuffer};
use std::sync::Arc;
use web_sys::js_sys::Function;

pub fn code_editor_frame(ui: &mut egui::Ui, app: &mut VisualisingSigma16, ctx: &egui::Context) {
    //egui::panel::SidePanel::left("line_numbers").show_inside(ui, |ui| {
    //    CodeEditor::make_line_counter(&mut app.code_editor, ui);
    //});
    //egui::panel::SidePanel::right("code").show_inside(ui, |ui| {
    //    CodeEditor::make_editor(&mut app.code_editor, ui);
    //});
    ui.horizontal(|ui| {
        CodeEditor::make_line_counter(&mut app.code_editor, ui);
        CodeEditor::make_editor(&mut app.code_editor, ui);
    });
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeEditor {
    pub code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            code: "".to_string(),
        }
    }
}

impl CodeEditor {
    pub fn make_editor(&mut self, ui: &mut egui::Ui) -> Response {
        let Self { code } = self;
        ui.add(
            egui::TextEdit::multiline(code)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY)
                .layouter(&mut |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                    CodeEditor::layouter(ui, string, _wrap_width)
                }),
        )
    }

    pub fn make_line_counter(&mut self, ui: &mut egui::Ui) -> Response {
        let Self { code } = self;

        let line_count = code.as_str().lines().count();
        let mut line_numbers_builder: Vec<String> = Vec::with_capacity(line_count);
        for i in 1..line_count + 1 {
            line_numbers_builder.push(format!(
                "{:>indent$}\n",
                i,
                indent = (line_count / 100) + 2.0 as usize
            ));
        }
        let line_numbers = line_numbers_builder.concat();

        ui.add(
            egui::TextEdit::multiline(&mut line_numbers.as_str())
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_rows(10)
                .desired_width(15.0)
                .lock_focus(false),
        )
    }

    fn layouter(ui: &egui::Ui, string: &str, _wrap_width: f32) -> Arc<Galley> {
        let layout_job = highlight(ui.ctx(), &CodeTheme::default(), string, "Sigma16");
        // layout_job.wrap.max_width = wrap_width; // no wrapping
        ui.fonts(|font| font.layout_job(layout_job))
    }
}
