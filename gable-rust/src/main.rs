#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gable_app;
use eframe::egui;
use gable_app::GableApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let app = GableApp::new("Gable".to_string());
    let title = app.title().to_string(); // 克隆title避免借用
    eframe::run_native(&title, options, Box::new(|_cc| Ok(Box::new(app))))
}
