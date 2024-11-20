use crate::assembler::code::Code;
use crate::gui::app::VisualisingSigma16;
use crate::gui::code_editor::CodeEditor;
use crate::interpreter::state::{RunningState, State};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeRunner {
    state: State,
}

impl Default for CodeRunner {
    fn default() -> Self {
        Self {
            state: State::new(&[0_u16]),
        }
    }
}

impl CodeRunner {
    pub fn gui(&mut self, ui: &mut egui::Ui, code_editor: &mut CodeEditor) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let reset = ui.add(egui::Button::new("Reset"));
                let step = ui.add(egui::Button::new("Step"));
                ui.add(egui::Label::new("Log to console:"));
                ui.add(egui::Checkbox::new(&mut self.state.verbose, ""));

                if reset.clicked() {
                    self.reset(code_editor);
                }

                if step.clicked() {
                    self.step();
                }
            });
            ui.horizontal(|ui| {
                code_editor.make_line_counter(ui, None);
                code_editor.make_editor(ui, false);
            })
        });
    }

    fn reset(&mut self, code_editor: &CodeEditor) {
        self.state = State::new(&Code::new(code_editor.code.clone()).memory);
        self.state.state = RunningState::Step;
    }

    fn step(&mut self) {
        self.state.run();
    }
}
