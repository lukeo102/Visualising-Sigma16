use crate::assembler::code::Code;
use crate::gui::code_editor::{code_editor_frame, CodeEditor};
use crate::gui::code_runner::CodeRunner;
use crate::gui::monitor::make_monitor_toggles;
use crate::interpreter::state::State;
use eframe::epaint::text::LayoutJob;
use egui::TextBuffer;
use log::{log, Level};

use super::data_flow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VisualisingSigma16 {
    show_code_editor: bool,
    pub code_editor: CodeEditor,
    pub code_runner: CodeRunner,
}

impl Default for VisualisingSigma16 {
    fn default() -> Self {
        Self {
            show_code_editor: true,
            code_editor: CodeEditor::default(),
            code_runner: CodeRunner::default(),
        }
    }
}

impl VisualisingSigma16 {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for VisualisingSigma16 {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.

        if !self.code_editor.code.ends_with("\n") {
            self.code_editor.code.push('\n');
        }
        self.code_editor.code = self.code_editor.code.replace("\t", "    ");

        egui::Window::new("Monitor Toggles").show(ctx, |ui| {
            make_monitor_toggles(ui, &mut self.code_runner.state);
        });

        egui::Window::new("Code Editor")
            .resizable([true, true])
            .show(ctx, |ui| {
                code_editor_frame(ui, self, ctx, true, None);
            });

        egui::Window::new("Code Runner")
            .resizable([true, true])
            .show(ctx, |ui| {
                CodeRunner::gui(&mut self.code_runner, ui, &mut self.code_editor);
            });

        egui::Window::new("Test")
            .resizable([true, true])
            .show(ctx, |ui| {
                data_flow::make(ui, self);
            });

        let code = Code::new(self.code_editor.code.clone());
        let mem_loc_count = code.get_memory_location_count();

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {

                // CodeEditor::editable(&mut self.code_editor, ui);
                // self.code_hex.code = format_code(parse_code(self.code_editor.code.as_str()));
                // CodeEditor::un_editable(&mut self.code_hex, ui);
            });

            ui.separator();
        });
    }
}
