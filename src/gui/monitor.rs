use crate::state;
use egui;

pub fn make_monitor_toggles(ui: &mut egui::Ui, state: &mut state::State) {
    // Make register toggles

    // Make symbol toggles
    let mut symbols: Vec<egui::Checkbox> = Vec::new();
    for (symbol, monitored) in &mut state.monitored_symbols {
        symbols.push(egui::Checkbox::new(monitored, format!("{}", symbol)));
    }

    // Add elements to UI
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            // Add register toggles
            ui.vertical(|ui: &mut egui::Ui| {
                ui.add(egui::Label::new("Monitor registers"));
                for register in 0..16 {
                    ui.add(egui::Checkbox::new(
                        &mut state.monitored_registers[register],
                        format!("R{}", register),
                    ));
                }
            });

            ui.vertical(|ui| {
                ui.add(egui::Label::new("Monitor symbols"));
                // Add symbol table toggles
                egui::ScrollArea::vertical()
                    .max_height(16.0)
                    .show(ui, |ui: &mut egui::Ui| {
                        for (symbol, monitor) in &mut state.monitored_symbols {
                            ui.add(egui::Checkbox::new(monitor, symbol.as_str()));
                        }
                    });
            });
        });
        // Make memory toggles
    });
}
