use crate::cli::{cli_export, cli_import};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Gable CLI", version = "1.0.0", author = "Gable")]
#[clap(about = "Gable 命令行工具", long_about = None)]
struct GableCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 导入模式
    #[clap(alias = "i")]
    Import {
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// 导出模式（默认）
    #[clap(alias = "e")]
    Export {
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

pub fn run(args: Vec<String>) -> Result<(), eframe::Error> {
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let cli: GableCli = match GableCli::try_parse_from(&args_str) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    // 根据命令执行相应的操作
    match &cli.command {
        Some(Commands::Import { args }) => {
            let mut new_args: Vec<String> = vec![
                args.get(0)
                    .map(|s| s.to_string())
                    .unwrap_or("gable".to_string()),
            ];
            new_args.extend(args.iter().cloned());
            cli_import::run_import(new_args)
        }
        Some(Commands::Export { args }) => {
            let mut new_args = vec![
                args.get(0)
                    .map(|s| s.to_string())
                    .unwrap_or("gable".to_string()),
            ];
            new_args.extend(args.iter().cloned());
            cli_export::run_export(new_args)
        }
        None => {
            let new_args = vec!["gable".to_string()];
            cli_export::run_export(new_args)
        }
    }
}
