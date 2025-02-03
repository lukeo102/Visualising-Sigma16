use crate::assembler::code::Code;
use crate::gui::code_editor::CodeEditor;
use crate::interpreter::interpreter;
use crate::interpreter::state::{RunningState, State};
use log::{log, Level};
use serde_diff::{Apply, Diff};
use std::collections::VecDeque;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeRunner {
    pub state: State,
    pub history: VecDeque<String>,
    pub running: bool,
    code: Code,
}

impl Default for CodeRunner {
    fn default() -> Self {
        let state = State::new(&Code::new("".to_string()));
        let history = VecDeque::new();

        Self {
            state,
            history,
            running: false,
            code: Code::new("".to_string()),
        }
    }
}

impl CodeRunner {
    pub fn gui(&mut self, ui: &mut egui::Ui, code_editor: &mut CodeEditor) {
        ui.vertical(|v_ui| {
            v_ui.horizontal(|h_ui| {
                let mut selected = self.state.state.clone();
                selected = self.make_state(h_ui, selected);

                // Allow reset at any time
                let reset = h_ui.add(egui::Button::new("Reset"));

                if self.state.state == RunningState::Error {
                } else {
                    // If we are haulted, we should not be able to step
                    if selected == RunningState::Step {
                        let step = h_ui.add_enabled(!self.running, egui::Button::new("Step"));

                        if step.clicked() {
                            self.step()
                        }
                    }
                    if selected == RunningState::Running {
                        let run = h_ui.add_enabled(!self.running, egui::Button::new("Run"));

                        if run.clicked() {
                            self.running = true;
                        }

                        if self.running {
                            self.step();
                        }
                    }

                    // Step Back
                    if selected != RunningState::Running {
                        let step_back = h_ui.add(egui::Button::new("Step Back"));
                        if step_back.clicked() {
                            self.step_back();
                        }
                    }
                }
                if reset.clicked() {
                    self.reset(code_editor);
                }
            });
            v_ui.horizontal(|h_ui| {
                code_editor.make_line_counter(h_ui, None);
                code_editor.make_editor(h_ui, false);
            })
        });
    }

    fn make_state(&mut self, ui: &mut egui::Ui, mut selected: RunningState) -> RunningState {
        ui.add(egui::Label::new("State: "));

        match selected {
            RunningState::Haulted => {
                ui.add(egui::Label::new("Haulted"));
            }
            RunningState::Error => {
                ui.add(egui::Label::new("Error"));
            }
            _ => {
                egui::ComboBox::from_id_salt("Run Type")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, RunningState::Running, "Run");
                        ui.selectable_value(&mut selected, RunningState::Step, "Step");
                    });
                self.state.state = selected.clone();
            }
        }
        selected
    }

    fn reset(&mut self, code_editor: &CodeEditor) {
        self.code = Code::new(code_editor.code.clone());
        self.state = State::new(&self.code);
        self.history = VecDeque::new();
        self.state.verbose = true;
        self.running = false;
    }

    fn step(&mut self) {
        let base = self.state.clone();

        match self.state.state {
            RunningState::Running | RunningState::Step => interpreter::step(&mut self.state),
            _ => {
                log!(Level::Warn, "Unknown Sigma16 interpreter state");
            }
        }

        self.diff(base);

        if self.state.state == RunningState::Haulted {
            self.running = false;
        }

        self.state.print_verbose();
    }

    fn diff(&mut self, old: State) {
        let diff = serde_json::to_string(&Diff::serializable(&self.state, &old));
        match diff {
            Ok(diff) => {
                self.history.push_back(diff);
            }
            Err(error) => {
                log!(
                    Level::Error,
                    "Couldnt generate diff for state history.\n{:?}",
                    error,
                );
            }
        }
    }

    fn step_back(&mut self) {
        if self.history.len() < 1 {
            log!(Level::Warn, "No more history to step back into.");
            return;
        }

        let diff = self.history.pop_back().unwrap();
        let mut deserializer = serde_json::Deserializer::from_str(&diff);

        match Apply::apply(&mut deserializer, &mut self.state) {
            Ok(_) => {}
            Err(e) => {
                log!(Level::Error, "Could not step back.\n{:?}", e)
            }
        }
        self.state.print_verbose();
    }
}
