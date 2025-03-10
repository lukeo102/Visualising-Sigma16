use crate::gui::app::VisualisingSigma16;
use crate::gui::code_runner::CodeRunner;
use crate::gui::syntax_highlighting::{highlight, CodeTheme};
use egui::{Galley, Response};
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeEditor {
    pub code: String,
    pub opened: bool,
    pub windowed: bool,
    pub runner: Option<CodeRunner>,
    pub name: String,
    pub renaming: bool,
    pub deleting: bool,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            opened: true,
            windowed: false,
            runner: None,
            name: "untitled".to_string(),
            renaming: false,
            deleting: false,
        }
    }
}

impl CodeEditor {
    pub fn new_windowed() -> Self {
        let mut new_editor = CodeEditor::default();
        new_editor.windowed = true;
        new_editor
    }

    pub fn gui(
        &mut self,
        ui: &mut egui::Ui,
        editable: bool,
        line_number_layouter: Option<
            &mut dyn for<'a, 'b> FnMut(&'a egui::Ui, &'b str, f32) -> Arc<Galley>,
        >,
    ) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                match self.runner {
                    Some(_) => {
                        let button = ui.add(egui::Button::new("Close Runner"));

                        if button.clicked() {
                            self.close_runner();
                        }
                    }
                    None => {
                        let button = ui.add(egui::Button::new("Open Runner"));

                        if button.clicked() {
                            self.open_runner();
                        }
                    }
                }

                if ui.add(egui::Button::new("Rename")).clicked() {
                    self.renaming = !self.renaming;
                }

                if ui.add(egui::Button::new("Close")).clicked() {
                    self.opened = false;
                }

                ui.checkbox(&mut self.windowed, "Windowed");

                if ui.add(egui::Button::new("Delete")).clicked() {
                    self.deleting = true;
                }
            });
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    CodeEditor::make_line_counter(&self.code, ui, None);
                    CodeEditor::make_editor(&mut self.code, ui, editable);
                });
            });
        });
    }

    fn open_runner(&mut self) {
        let mut runner = CodeRunner::default();
        runner.reset(self.code.clone());
        self.runner = Some(runner);
    }

    fn close_runner(&mut self) {
        self.runner = None;
    }

    pub fn make_editor(code: &mut String, ui: &mut egui::Ui, editable: bool) -> Response {
        ui.add(
            egui::TextEdit::multiline(code)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY)
                .layouter(&mut |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                    CodeEditor::layouter(&ui, string, _wrap_width)
                })
                .interactive(editable),
        )
    }

    pub fn make_line_counter(
        code: &String,
        ui: &mut egui::Ui,
        line_number_layouter: Option<(
            &mut dyn for<'a, 'b> FnMut(&'a egui::Ui, &'b str, f32) -> Arc<Galley>,
            &usize,
        )>,
    ) -> Response {
        let line_count = code.as_str().lines().count();
        let mut line_numbers_builder: Vec<String> = Vec::with_capacity(line_count);
        let indent = line_count.to_string().len() + 1;
        if let Some((_, line)) = line_number_layouter {
            line_numbers_builder.push(format!("{line},"));
        }
        for i in 1..line_count + 1 {
            line_numbers_builder.push(format!("{:>indent$}\n", i,));
        }
        let _line_numbers = line_numbers_builder.concat();
        let mut line_numbers = _line_numbers.as_str();

        let mut lines = egui::TextEdit::multiline(&mut line_numbers)
            .font(egui::TextStyle::Monospace)
            .code_editor()
            .desired_rows(10)
            .desired_width(6.9 * indent as f32)
            .lock_focus(false);
        if let Some((line_number_layouter, _)) = line_number_layouter {
            lines = lines.layouter(line_number_layouter);
        };
        ui.add(lines)
    }

    pub fn layouter(ui: &egui::Ui, string: &str, _wrap_width: f32) -> Arc<Galley> {
        let layout_job = highlight(ui.ctx(), &CodeTheme::default(), string, "Sigma16");
        // layout_job.wrap.max_width = wrap_width; // no wrapping
        ui.fonts(|font| font.layout_job(layout_job))
    }
}
