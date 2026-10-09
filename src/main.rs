#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

#[derive(Default)]
struct Counter {
    value: i64,
}

impl eframe::App for Counter {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Clicker_Game! - Rust-UI");
            ui.heading(format!("Klicks: {}", self.value));

            if self.value < 100 {
                if ui.button("+1").clicked() {
                    self.value += 1;
                }
                if ui.button("-1").clicked() {
                    self.value -= 1;
                }
            }

            if self.value >= 100 && self.value < 200 {
                if ui.button("+10").clicked() {
                    self.value += 10;
                }
                if ui.button("-10").clicked() {
                    self.value -= 10;
                }
            }

            if self.value >= 200 {
                if ui.button("+100").clicked() {
                    self.value += 100;
                }
                if ui.button("-100").clicked() {
                    self.value -= 100;
                }
            }
        });
    }
}

// debuggin stuff
impl Counter {
    pub fn set_value(&mut self) {
        self.value = 90;
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Rust Clicker-Game egui/eframe",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            let mut counter = Counter::default();
            Ok(Box::new(counter))
        }),
    )
}
