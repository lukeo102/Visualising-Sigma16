//use crate::gui::app::VisualisingSigma16;
//use crate::gui::code_editor::CodeEditor;
//use crate::interpreter::state::State;
//
//struct CodeRunner {
//    state: State,
//
//
//}
//
//impl Default for CodeRunner {
//    fn default() -> Self {
//        Self {
//            state: State::new(&[0_u16]),
//        }
//    }
//}
//
//impl CodeRunner {
//    pub fn gui(ui: &mut egui::Ui, app: &mut VisualisingSigma16) -> egui::Frame {
//        let root = egui::Frame::default();
//
//        root.show(ui, |ui| {
//            let step_button = egui::Button::new("Step");
//
//
//            let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
//                let layout_job = highlight(ui.ctx(), &theme, string, language);
//                // layout_job.wrap.max_width = wrap_width; // no wrapping
//                ui.fonts(|font| { font.layout_job(layout_job) })
//            };
//
//            egui::TextEdit::multiline(&mut app.code_editor.code)
//                .font(egui::TextStyle::Monospace) // for cursor height
//                .code_editor()
//                .desired_rows(10)
//                .lock_focus(true)
//                .desired_width(f32::INFINITY)
//                .layouter(&mut layouter)
//
//
//
//        });
//
//        root
//
//    }
//}
