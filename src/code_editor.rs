use log::{log, Level};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeEditor {
    pub code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            code: "".to_string(),
        }
    }
}

impl CodeEditor {
    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { code } = self;
        egui::ScrollArea::vertical().show(ui, |ui| {
            let editor = ui.add(
                egui::TextEdit::multiline(code)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY),
            );
            if editor.changed() {
                log!(Level::Info, "Changed to: {}", &code)
            }
        });
    }
}
