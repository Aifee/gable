#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod gui;
use eframe::egui;
use gui::gable_app::GableApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Gable", // 固定的窗口标题
        options,
        Box::new(|cc| {
            let app = GableApp::new(cc);
            Ok(Box::new(app))
        }),
    )
}
