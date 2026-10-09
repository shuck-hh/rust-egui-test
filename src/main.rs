use eframe::egui;

#[derive(Default)]
struct Counter {
    value: u32,
}

impl eframe::App for Counter {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Meine erste Rust-UI");
            ui.label(format!("Zähler: {}", self.value));

            if ui.button("Weiterzählen").clicked() {
                self.value += 1;
            }
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Rust UI",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Counter::default()))),
    )
}