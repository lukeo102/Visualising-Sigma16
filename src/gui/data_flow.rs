use super::app::VisualisingSigma16;

pub fn make(ui: &mut egui::Ui, app: &VisualisingSigma16) {
    ui.horizontal(|ui| {
        make_registers(ui, app);
        make_memory(ui, app);
    });
}

fn make_registers(ui: &mut egui::Ui, app: &VisualisingSigma16) {
    ui.vertical(|ui| {});
}

fn make_memory(ui: &mut egui::Ui, app: &VisualisingSigma16) {}
