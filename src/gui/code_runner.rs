use crate::assembler::code::Code;
use crate::gui::code_editor::CodeEditor;
use crate::gui::syntax_highlighting_runner::{highlight, CodeTheme};
use crate::interpreter::interpreter;
use crate::interpreter::state::{RunningState, State};
use egui::Galley;
use log::{log, Level};
use serde_diff::{Apply, Diff};
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeRunner {
    pub state: State,
    pub history: VecDeque<String>,
    pub running: bool,
    pub code: Code,
    pub data_flow: bool,
    pub name: String,
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
            data_flow: false,
            name: uuid::Uuid::new_v4().to_string(),
        }
    }
}

impl CodeRunner {
    pub fn gui(&mut self, ui: &mut egui::Ui, code: String) {
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

                h_ui.add(egui::Checkbox::new(&mut self.data_flow, "Data Flow"));

                if reset.clicked() {
                    self.reset(code);
                }
            });
            egui::ScrollArea::new([true, true])
                .max_height(v_ui.available_height() - 30.0)
                .show(v_ui, |ui| {
                    ui.horizontal(|h_ui| {
                        let line = self
                            .code
                            .memory_to_code
                            .get(&(self.state.pc.get_ui() as usize))
                            .unwrap_or(&0);

                        CodeEditor::make_line_counter(
                            &self.code.get_code(),
                            h_ui,
                            Some((
                                &mut |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                                    CodeRunner::layouter(&ui, string, _wrap_width)
                                },
                                line,
                            )),
                        );
                        CodeEditor::make_editor(&mut self.code.get_code(), h_ui, false);
                    });
                });
            self.make_errors(v_ui);
        });
    }

    pub fn layouter(ui: &egui::Ui, string: &str, _wrap_width: f32) -> Arc<Galley> {
        let layout_job = highlight(ui.ctx(), &mut CodeTheme::default(), string);
        // layout_job.wrap.max_width = wrap_width; // no wrapping
        ui.fonts(|font| font.layout_job(layout_job))
    }

    fn make_errors(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            for error in &self.code.errors {
                ui.label(format!("Error on line {:?}", error.line));
                ui.label(&error.message);
                ui.label(&error.resolution);
                ui.separator();
            }
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

    pub fn reset(&mut self, code: String) {
        self.code = Code::new(code);
        self.state = State::new(&self.code);
        self.history = VecDeque::new();
        self.state.verbose = true;
        self.running = false;
    }

    fn step(&mut self) {
        let base = self.state.clone();
        self.state.reset_altered();

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

        if self.state.state != RunningState::Step {
            self.state.state = RunningState::Step;
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
