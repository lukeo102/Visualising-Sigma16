use crate::assembler::code::Code;
use crate::gui::app::VisualisingSigma16;
use crate::gui::code_editor::CodeEditor;
use crate::interpreter::history::History;
use crate::interpreter::state::{RunningState, State};
use log::{log, Level};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeRunner {
    pub state: State,
    history: Option<History>,
}

impl Default for CodeRunner {
    fn default() -> Self {
        let state = State::new(&Code::new("".to_string()));
        let history = None;

        Self { state, history }
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

                // If we are haulted, we should not be able to step
                if selected == RunningState::Running || selected == RunningState::Step {
                    let run_text = if selected == RunningState::Step {
                        "Step"
                    } else {
                        "Run"
                    };
                    let run = h_ui.add(egui::Button::new(run_text));
                    if selected == RunningState::Step {
                        let step_back = h_ui.add(egui::Button::new("Step Back"));
                        if step_back.clicked() {
                            self.step_back();
                        }
                    }

                    if run.clicked() {
                        self.step();
                    }
                }
                h_ui.add(egui::Label::new("Log to console:"));
                h_ui.add(egui::Checkbox::new(&mut self.state.verbose, ""));

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
        self.state = State::new(&Code::new(code_editor.code.clone()));
    }

    fn step(&mut self) {
        self.state.run();
    }

    fn step_back(&mut self) {
        match self.history.clone() {
            None => {}
            Some(history) => {
                self.history = Some(History::apply(history, &mut self.state));
            }
        }
    }
}
