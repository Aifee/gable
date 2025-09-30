use crate::cli::{cli_export, cli_import};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Gable CLI", version = "1.0.0", author = "Gable")]
#[clap(about = "Gable command-line tool", long_about = None)]
struct GableCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 导入模式
    #[clap(alias = "i")]
    Import {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        args: Vec<String>,
    },

    /// 导出模式（默认）
    #[clap(alias = "e")]
    Export {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        args: Vec<String>,
    },
}

pub fn run(args: Vec<String>) -> Result<(), eframe::Error> {
    let processed_args = if args.is_empty() {
        vec!["gable".to_string(), "export".to_string()]
    } else {
        let mut new_args = vec!["gable".to_string()];
        new_args.extend(args.clone());
        new_args
    };

    let args_str: Vec<&str> = processed_args.iter().map(|s| s.as_str()).collect();
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
            let mut new_args: Vec<String> = vec!["gable".to_string()];
            new_args.extend(args.iter().cloned());
            cli_import::run_import(new_args)
        }
        Some(Commands::Export { args }) => {
            let mut new_args = vec!["gable".to_string()];
            new_args.extend(args.iter().cloned());
            cli_export::run_export(new_args)
        }
        None => {
            let new_args = vec!["gable".to_string()];
            cli_export::run_export(new_args)
        }
    }
}
