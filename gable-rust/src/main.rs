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
    eframe::run_native(
        "Gable", // 固定的窗口标题
        options,
        Box::new(|cc| {
            let app = GableApp::new(cc, "Gable".to_string());
            Ok(Box::new(app))
        }),
    )
}
