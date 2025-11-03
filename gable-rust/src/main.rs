#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod common;
mod gui;
use eframe::egui;
use gui::gable_app::GableApp;
use std::env;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let _ =
        gui::datas::log::LogTrace::init(Some(common::constant::DIR_LOG), log::LevelFilter::Info);
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        #[cfg(windows)]
        attach_parent_console();
        return run_cli(args);
    }
    run_gui()
}

#[cfg(windows)]
fn attach_parent_console() {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn AttachConsole(dw_process_id: u32) -> i32;
    }
    const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_cli(args: Vec<String>) -> Result<(), eframe::Error> {
    let processed_args = if args.len() <= 1 {
        vec![]
    } else {
        let cli_args: Vec<String> = args[1..].to_vec();
        let mut expanded_args = Vec::new();
        for arg in cli_args {
            if arg.starts_with("--") && arg.contains(' ') && !arg.contains('=') {
                let parts: Vec<&str> = arg.split_whitespace().collect();
                expanded_args.extend(parts.iter().map(|s| s.to_string()));
            } else {
                expanded_args.push(arg);
            }
        }
        expanded_args
    };

    return cli::gable_cli::run(processed_args);
}

fn run_gui() -> Result<(), eframe::Error> {
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
