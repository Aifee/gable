use crate::cli::{cli_export, cli_import};

pub fn run(args: Vec<String>) -> Result<(), eframe::Error> {
    let cli_args: &[String] = &args[1..];
    if cli_args.is_empty() {
        _usage();
        return Ok(());
    }

    let mut import_mode: bool = false;
    let mut export_mode: bool = false;
    let mut show_help: bool = false;
    let mut i = 0;

    while i < cli_args.len() {
        let arg: &String = &cli_args[i];
        match arg.as_str() {
            "--import" => {
                import_mode = true;
            }
            "--export" => {
                export_mode = true;
            }
            "--help" => {
                show_help = true;
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Error: 未知参数 '{}'", arg);
                    _usage();
                    return Ok(());
                }
            }
        }
        i += 1;
    }

    if show_help {
        _usage();
        return Ok(());
    }

    if import_mode && export_mode {
        println!("Error: 不能同时指定 --import 和 --export 模式");
        _usage();
        return Ok(());
    }

    if import_mode {
        return cli_import::run_import(cli_args);
    } else {
        return cli_export::run_export(cli_args);
    }
}

fn _usage() {
    println!(
        r#"Gable CLI 工具

用法:
  gable [选项]

模式:
  --import               导入模式
  --export               导出模式（默认）

示例:
  gable --export -i file1.xlsx file2.xlsx
  gable --import --input data.xlsx -t unity
  gable --export --script --input config.xlsx
  gable --help"#
    );
}
