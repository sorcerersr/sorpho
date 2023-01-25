#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "sorpho",
        options,
        Box::new(|_cc| Box::new(Sorpho::default())),
    )
}

struct Sorpho {
    
}

impl Default for Sorpho {
    fn default() -> Self {
        Self {        }
    }
}

impl eframe::App for Sorpho {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sorpho");

        });
    }
}