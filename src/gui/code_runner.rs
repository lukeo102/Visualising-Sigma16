use crate::assembler::code::Code;
use crate::gui::app::VisualisingSigma16;
use crate::gui::code_editor::CodeEditor;
use crate::interpreter::state::{RunningState, State};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeRunner {
    pub state: State,
}

impl Default for CodeRunner {
    fn default() -> Self {
        Self {
            state: State::new(&Code::new("".to_string())),
        }
    }
}

impl CodeRunner {
    pub fn gui(&mut self, ui: &mut egui::Ui, code_editor: &mut CodeEditor) {
        ui.vertical(|v_ui| {
            v_ui.horizontal(|h_ui| {
                let reset = h_ui.add(egui::Button::new("Reset"));
                let step = h_ui.add(egui::Button::new("Step"));
                h_ui.add(egui::Label::new("Log to console:"));
                h_ui.add(egui::Checkbox::new(&mut self.state.verbose, ""));

                if reset.clicked() {
                    self.reset(code_editor);
                }

                if step.clicked() {
                    self.step();
                }
            });
            v_ui.horizontal(|h_ui| {
                code_editor.make_line_counter(h_ui, None);
                code_editor.make_editor(h_ui, false);
            })
        });
    }

    fn reset(&mut self, code_editor: &CodeEditor) {
        self.state = State::new(&Code::new(code_editor.code.clone()));
        self.state.state = RunningState::Step;
    }

    fn step(&mut self) {
        self.state.run();
    }
}
