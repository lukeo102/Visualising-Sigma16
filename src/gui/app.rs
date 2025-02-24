use crate::gui::code_editor::CodeEditor;
use crate::gui::code_runner::CodeRunner;
use crate::gui::data_flow;
use crate::gui::exercises::EXERCISES;
use log::{log, Level};

use super::code_editor;

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
        ui.horizontal(|ui| {
            for editor in &mut self.code_editor {
                ui.vertical(|ui| {
                    if editor.opened {
                        VisualisingSigma16::code_editor_gui(editor, ui, ctx);

                        if editor.renaming {
                            egui::Window::new("Rename Program").show(ctx, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut editor.name));
                                if ui.add(egui::Button::new("close")).clicked() {
                                    editor.renaming = false;
                                }
                            });
                        }

                        // Code Runners
                        match &mut editor.runner {
                            Some(runner) => {
                                VisualisingSigma16::code_runner_gui(
                                    runner,
                                    editor.code.clone(),
                                    ctx,
                                    &editor.name,
                                );

                                if runner.data_flow {
                                    egui::Window::new(format!("Data Flow: {}", &editor.name)).show(
                                        ctx,
                                        |ui| {
                                            data_flow::make(ui, runner);
                                        },
                                    );
                                }
                            }
                            None => {}
                        }
                    }
                });
            }
        });
    }

    pub fn code_editor_gui(editor: &mut CodeEditor, ui: &mut egui::Ui, ctx: &egui::Context) {
        if editor.windowed {
            egui::Window::new(&editor.name).show(ctx, |ui| editor.gui(ui, true, None));
        } else {
            editor.gui(ui, true, None);
        }
    }

    fn code_runner_gui(runner: &mut CodeRunner, code: String, ctx: &egui::Context, name: &String) {
        egui::Window::new(format!("Runner: {}", name))
            .show(ctx, |ui| CodeRunner::gui(runner, ui, code));
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
            let mut removed = false;
            for i in 0..self.code_editor.len() {
                if self.code_editor[i].deleting {
                    egui::Window::new("Confirm Delete").show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.label(format!(
                                "Are you sure you want to delete {}?",
                                self.code_editor[i].name
                            ));
                            ui.horizontal(|ui| {
                                if ui.add(egui::Button::new("Yes")).clicked() {
                                    self.code_editor.remove(i);
                                    removed = true;
                                }
                                if ui.add(egui::Button::new("No ")).clicked() {
                                    self.code_editor[i].deleting = false;
                                }
                            });
                        });
                    });
                }
                if removed {
                    break;
                }
            }

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let new = ui.add(egui::Button::new("New"));
                    egui::ComboBox::from_label("")
                        .selected_text("Load")
                        .show_ui(ui, |ui| {
                            for editor in &mut self.code_editor {
                                if !editor.opened {
                                    if ui.selectable_label(false, editor.name.clone()).clicked() {
                                        editor.opened = true;
                                    }
                                }
                            }
                        });

                    egui::ComboBox::from_label(" ")
                        .selected_text("Load Exercise")
                        .show_ui(ui, |ui| {
                            for (exercise, code) in EXERCISES {
                                if ui.selectable_label(false, exercise).clicked() {
                                    let mut editor = CodeEditor::new_windowed();
                                    editor.code = code.to_string();
                                    editor.opened = true;
                                    editor.name = exercise.to_string();
                                    self.code_editor.push(editor);
                                }
                            }
                        });

                    if new.clicked() {
                        self.code_editor.push(CodeEditor::new_windowed());
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    self.build_ui(ctx, ui);
                    // CodeEditor::editable(&mut self.code_editor, ui);
                    // self.code_hex.code = format_code(parse_code(self.code_editor.code.as_str()));
                    // CodeEditor::un_editable(&mut self.code_hex, ui);
                });
            });
        });
    }
}
