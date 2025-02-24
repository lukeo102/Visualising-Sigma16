use crate::{gui::code_runner::CodeRunner, interpreter::memory::U16_MAX};
use egui_extras::{Column, TableBuilder};
use log::{log, Level};

const RED_TEXT: egui::Color32 = egui::Color32::from_rgb(255, 25, 25);
const GREEN_TEXT: egui::Color32 = egui::Color32::from_rgb(50, 255, 50);

pub fn make(ui: &mut egui::Ui, runner: &mut CodeRunner) {
    ui.horizontal(|ui| {
        make_registers(ui, runner);
        TableBuilder::new(ui)
            .columns(Column::auto(), 2)
            .vscroll(false)
            .body(|mut body| {
                body.row(100.0, |mut row| {
                    row.col(|ui| {
                        make_memory(ui, runner);
                    });
                    row.col(|ui| {
                        make_symbol_table(ui, runner);
                    });
                });
            });
    });
}

fn make_registers(ui: &mut egui::Ui, runner: &mut CodeRunner) {
    ui.vertical(|ui| {
        ui.heading("Registers");
        for reg in &runner.code.used_registers {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(format!("R{:?}: ", reg)));
                let value = format!("{:#06X}", runner.state.r[*reg].get_ui())
                    .split_at(2)
                    .1
                    .to_string();

                if runner.state.r[*reg].get_altered() {
                    ui.label(egui::RichText::new(value).color(GREEN_TEXT));
                } else if runner.state.r[*reg].get_accessed() {
                    ui.label(
                        egui::RichText::new(value).color(egui::Color32::from_rgb(255, 25, 25)),
                    );
                } else {
                    ui.label(egui::RichText::new(value));
                }
            });
        }
    });
}

fn make_memory(ui: &mut egui::Ui, runner: &mut CodeRunner) {
    let mut last_line = U16_MAX as usize + 1;
    let mut scroll_row: usize = 0;
    ui.vertical(|ui| {
        ui.set_min_width(160.0);
        ui.heading("Memory");
        TableBuilder::new(ui)
            .id_salt(format!("memory-{:?}", runner.name))
            //.cell_layout(make_table_layout())
            .striped(true)
            .columns(Column::remainder(), 3)
            .resizable(false)
            .scroll_to_row(runner.state.pc.get() as usize, Some(egui::Align::TOP))
            .header(15.0, |mut header| {
                header.col(|ui| {
                    ui.label("Line");
                });
                header.col(|ui| {
                    ui.label("Location");
                });
                header.col(|ui| {
                    ui.label("Content");
                });
            })
            .body(|mut body| {
                for mem in runner.state.memory.get_used().clone().to_owned() {
                    body.row(15.0, |mut row| {
                        row.col(|ui| {
                            let line = runner.code.memory_to_code.get(&mem);
                            match line {
                                Some(line) => {
                                    if line.clone() != last_line {
                                        if runner.state.pc.get() as usize == mem {
                                            ui.label(
                                                egui::RichText::new(format!("{:?}", line))
                                                    .color(RED_TEXT),
                                            );
                                            scroll_row = mem;
                                        } else {
                                            ui.label(egui::RichText::new(format!("{:?}", line)));
                                        }
                                    } else {
                                        ui.label(" ");
                                    }
                                    last_line = line.clone();
                                }
                                None => {}
                            }
                        });
                        row.col(|ui| {
                            let value = format!("{:#06X}", mem).split_at(2).1.to_string();
                            ui.label(egui::RichText::new(value));
                        });
                        row.col(|ui| {
                            let value = format!("{:#06X}", runner.state.memory[mem])
                                .split_at(2)
                                .1
                                .to_string();
                            if runner.state.memory.get_altered_i().contains(&mem) {
                                ui.label(egui::RichText::new(value).color(GREEN_TEXT));
                            } else {
                                ui.label(egui::RichText::new(value));
                            }
                        });
                    });
                }
            });
    });
}

fn make_symbol_table(ui: &mut egui::Ui, runner: &mut CodeRunner) {
    ui.vertical(|ui| {
        ui.set_min_width(160.0);
        ui.heading("Symbol Table");
        TableBuilder::new(ui)
            .id_salt(uuid::Uuid::new_v4())
            .striped(true)
            .columns(Column::remainder(), 3)
            .header(15.0, |mut header| {
                header.col(|ui| {
                    ui.label("Symbol");
                });
                header.col(|ui| {
                    ui.label("Line");
                });
                header.col(|ui| {
                    ui.label("Location");
                });
            })
            .body(|mut body| {
                for (symbol, location) in &runner.code.symbol_table {
                    body.row(15.0, |mut row| {
                        row.col(|ui| {
                            ui.label(symbol);
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:?}",
                                runner.code.memory_to_code.get(location).unwrap()
                            ));
                        });
                        row.col(|ui| {
                            let value = format!("{:#06X}", location).split_at(2).1.to_string();
                            ui.label(value);
                        });
                    });
                }
            });
    });
}

fn make_table_layout() -> egui::Layout {
    egui::Layout::default().with_main_align(egui::Align::Center)
}
