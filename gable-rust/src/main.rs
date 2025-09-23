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
    // 在VSCode调试环境中，第一个参数是可执行文件路径，我们需要将其移除
    let processed_args = if args.len() <= 1 {
        vec![]
    } else {
        // 移除第一个参数（可执行文件路径）
        let mut cli_args = args[1..].to_vec();

        // 处理可能合并在一起的参数
        let mut expanded_args = Vec::new();
        for arg in cli_args {
            if arg.starts_with("--") && arg.contains(' ') && !arg.contains('=') {
                // 如果参数以 -- 开头，包含空格但不包含 =，则可能是多个标志参数合并了
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
