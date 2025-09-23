#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod common;
mod gui;
use eframe::egui;
use gui::gable_app::GableApp;
use std::env;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return run_cli(args);
    }
    run_gui()
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::Error> {
    // WebAssembly平台只支持GUI模式
    run_gui()
}

#[cfg(not(target_arch = "wasm32"))]
fn run_cli(args: Vec<String>) -> Result<(), eframe::Error> {
    return cli::gable_cli::run(args);
}

fn run_gui() -> Result<(), eframe::Error> {
    let _ =
        gui::datas::log::LogTrace::init(Some(common::constant::DIR_LOG), log::LevelFilter::Info);

    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Gable",
        options,
        Box::new(|cc| {
            let app: GableApp = GableApp::new(cc);
            Ok(Box::new(app))
        }),
    )
}
