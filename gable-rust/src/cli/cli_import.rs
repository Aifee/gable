use std::{fs, path::PathBuf};

use clap::Parser;

use crate::common::excel_util;

#[derive(Parser)]
#[clap(name = "Gable Import", version = "1.0", author = "Aifei Liu")]
#[clap(about = "Gable Import Tool", long_about = None)]
pub struct ImportArgs {
    /// 指定输入文件（可以指定多个）
    #[clap(short = 'f', long = "files", num_args = 1..)]
    pub files: Vec<String>,

    /// 指定输入的目录
    #[clap(short = 'd', long = "dir")]
    pub dir: Option<String>,

    /// 指定目标平台
    #[clap(short = 't', long = "target")]
    pub target_path: String,
}

pub fn run_import(args: Vec<String>) -> Result<(), eframe::Error> {
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let import_args = match ImportArgs::try_parse_from(&args_str) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    if import_args.files.is_empty() && import_args.dir.is_none() {
        println!("Error: A file to be imported needs to be specified.");
        let help_args = vec![args_str[0], "--help"];
        if let Err(e) = ImportArgs::try_parse_from(&help_args) {
            eprintln!("{}", e);
        }
        return Ok(());
    }
    let mut files: Vec<PathBuf> = Vec::new();
    if let Some(dir) = import_args.dir {
        match fs::read_dir(&dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(extension) = path.extension() {
                                let ext_str = extension.to_string_lossy().to_lowercase();
                                if ext_str == "xlsx" || ext_str == "xls" {
                                    files.push(path);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: Unable to read the directory {}: {}", dir, e);
                return Ok(());
            }
        }
    } else {
        for file in import_args.files.iter() {
            let path: PathBuf = PathBuf::from(file);
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();
                if ext_str != "xlsx" && ext_str != "xls" {
                    print!("Error: The file {} is not an Excel file", file);
                    continue;
                }
            } else {
                print!("Error: The file {} is not a valid file", file);
                continue;
            }
            files.push(path);
        }
    }
    excel_util::import_excels(&import_args.target_path, files);
    println!("Import successful");
    Ok(())
}
