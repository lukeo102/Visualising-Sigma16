use super::app::VisualisingSigma16;

const RED_TEXT: egui::Color32 = egui::Color32::from_rgb(255, 25, 25);
const GREEN_TEXT: egui::Color32 = egui::Color32::from_rgb(50, 255, 50);

pub fn make(ui: &mut egui::Ui, app: &mut VisualisingSigma16) {
    ui.horizontal(|ui| {
        make_registers(ui, app);
        ui.add(egui::Separator::default());
        make_memory(ui, app);
    });
}

fn make_registers(ui: &mut egui::Ui, app: &mut VisualisingSigma16) {
    ui.vertical(|ui| {
        ui.heading("Registers");
        for reg in &app.code_runner.code.used_registers {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(format!("R{:?}: ", reg)));
                if app.code_runner.state.r[*reg].get_altered() {
                    ui.label(
                        egui::RichText::new(format!(
                            "{:?}",
                            app.code_runner.state.r[*reg].get_ui()
                        ))
                        .color(GREEN_TEXT),
                    );
                } else if app.code_runner.state.r[*reg].get_accessed() {
                    ui.label(
                        egui::RichText::new(format!(
                            "{:?}",
                            app.code_runner.state.r[*reg].get_ui()
                        ))
                        .color(egui::Color32::from_rgb(255, 25, 25)),
                    );
                } else {
                    ui.label(egui::RichText::new(format!(
                        "{:?}",
                        app.code_runner.state.r[*reg].get_ui()
                    )));
                }
            });
        }
    });
}

fn make_memory(ui: &mut egui::Ui, app: &mut VisualisingSigma16) {
    ui.vertical(|ui| {
        ui.heading("Memory");
        for mem in 0..app.code_runner.state.memory.get_used().len() {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Line: "));
                if app.code_runner.state.pc.get() as usize == mem {
                    ui.label(
                        egui::RichText::new(format!(
                            "{:?}",
                            app.code_runner.code.memory_to_code.get(&mem).unwrap()
                        ))
                        .color(RED_TEXT),
                    );
                } else {
                    ui.label(egui::RichText::new(format!(
                        "{:?}",
                        app.code_runner.code.memory_to_code.get(&mem).unwrap()
                    )));
                }

                ui.label(egui::RichText::new(format!(" | {:#06X} ", mem)));

                if app.code_runner.state.memory.get_altered_i().contains(&mem) {
                    ui.label(
                        egui::RichText::new(format!("{:#06X}", app.code_runner.state.memory[mem]))
                            .color(GREEN_TEXT),
                    );
                } else {
                    ui.label(egui::RichText::new(format!(
                        "{:#06X}",
                        app.code_runner.state.memory[mem]
                    )));
                }
            });
        }
    });
}
