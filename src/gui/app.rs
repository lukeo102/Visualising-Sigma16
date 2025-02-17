use crate::gui::code_editor::CodeEditor;
use crate::gui::code_runner::CodeRunner;
use crate::gui::data_flow;
use crate::gui::monitor::make_monitor_toggles;
use log::{log, Level};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VisualisingSigma16 {
    show_code_editor: bool,
    pub code_editor: Vec<CodeEditor>,
}

impl Default for VisualisingSigma16 {
    fn default() -> Self {
        Self {
            show_code_editor: true,
            code_editor: vec![CodeEditor::default()],
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

    fn build_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Code Editors
        for editor in &mut self.code_editor {
            if editor.opened {
                VisualisingSigma16::code_editor_gui(editor, ui, ctx);

                // Code Runners
                match &mut editor.runner {
                    Some(runner) => {
                        VisualisingSigma16::code_runner_gui(runner, editor.code.clone(), ctx);

                        if runner.data_flow {
                            egui::Window::new("Data Flow").show(ctx, |ui| {
                                data_flow::make(ui, runner);
                            });
                        }
                    }
                    None => {}
                }
            }
        }
    }

    pub fn code_editor_gui(editor: &mut CodeEditor, ui: &mut egui::Ui, ctx: &egui::Context) {
        if editor.windowed {
            egui::Window::new("Code Editor").show(ctx, |ui| editor.gui(ui, true, None));
        } else {
            editor.gui(ui, true, None);
        }
    }

    fn code_runner_gui(runner: &mut CodeRunner, code: String, ctx: &egui::Context) {
        egui::Window::new("Code Runner").show(ctx, |ui| CodeRunner::gui(runner, ui, code));
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

        for editor in &mut self.code_editor {
            if !editor.code.ends_with("\n") {
                editor.code.push('\n');
            }
        }

        //egui::Window::new("Monitor Toggles").show(ctx, |ui| {
        //    make_monitor_toggles(ui, &mut self.code_runner.state);
        //});

        //egui::Window::new("Test")
        //    .resizable([true, true])
        //    .show(ctx, |ui| {
        //        data_flow::make(ui, self);
        //    });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.horizontal(|ui| {
                self.build_ui(ctx, ui);
                // CodeEditor::editable(&mut self.code_editor, ui);
                // self.code_hex.code = format_code(parse_code(self.code_editor.code.as_str()));
                // CodeEditor::un_editable(&mut self.code_hex, ui);
            });

            ui.separator();
        });
    }
}
