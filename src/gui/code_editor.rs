use std::sync::Arc;
use egui::{Galley, Response};
use crate::assembler::assembler::parse_code;
use crate::gui::app::VisualisingSigma16;
use crate::gui::syntax_highlighting::{highlight, CodeTheme};
use crate::gui::util::format_code;


pub fn code_editor_frame(ui: &mut egui::Ui, app: &mut VisualisingSigma16, ctx: &egui::Context) {
    let frame = egui::frame::Frame::default()
        .fill(egui::Color32::BLACK)
        .show(ui, |frame_ui| {
            egui::Frame::default().show(frame_ui, |ui| {
                CodeEditor::editable(&mut app.code_editor, ui);
            });
            egui::Frame::default().show(frame_ui, |ui| {
                app.code_hex.code = format_code(parse_code(app.code_editor.code.as_str()));
                CodeEditor::un_editable(&mut app.code_hex, ui);
            });
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
    const LAYOUTER: fn(&egui::Ui, &str, f32) -> Arc<Galley> = |ui: &egui::Ui, string: &str, wrap_width: f32| {
        let mut layout_job: egui::text::LayoutJob = egui::text::LayoutJob::default();
        layout_job.wrap.max_width = f32::INFINITY;
        ui.fonts(|f| f.layout_job(layout_job))
    };

    pub(crate) fn editable(&mut self, ui: &mut egui::Ui) -> Response {
        let Self { code } = self;
        let language = "Sigma16";
        let theme = CodeTheme::default();

        
        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = highlight(ui.ctx(), &theme, string, language);
            // layout_job.wrap.max_width = wrap_width; // no wrapping
            ui.fonts(|font| { font.layout_job(layout_job) })
        };

        let editor = ui.add(
            egui::TextEdit::multiline(code)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter)

        );
        editor
    }

    pub(crate) fn un_editable(&mut self, ui: &mut egui::Ui) -> Response {
        let Self { code } = self;

        let editor = ui.add(
            egui::TextEdit::multiline(&mut code.as_str())
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(10)
            .lock_focus(true)
            .desired_width(f32::INFINITY)
            // .layouter(&mut Self::LAYOUTER.clone())

        );
        editor
    }


}


