use std::collections::HashMap;

use clap::Parser;

use crate::{
    common::{
        constant,
        convert::convert,
        generate::generate,
        setting::{self, BuildSetting},
    },
    gui::datas::{gables, tree_data::TreeData, tree_item::TreeItem},
};

#[derive(Parser)]
#[clap(name = "Gable Export", version = constant::GABLE_VERSION, author = "Aifei Liu")]
#[clap(about = "Gable Export Tool", long_about = None)]
pub struct ExportArgs {
    /// 指定输入文件（可以指定多个）
    #[clap(short = 'f', long = "files", num_args = 1..)]
    pub files: Vec<String>,

    /// 指定目标平台
    #[clap(short = 't', long = "target")]
    pub target: Option<String>,

    /// 导出数据
    #[clap(long = "data")]
    pub data: bool,

    /// 生成脚本
    #[clap(long = "script")]
    pub script: bool,
}

pub fn run_export(args: Vec<String>) -> Result<(), eframe::Error> {
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let export_args = match ExportArgs::try_parse_from(&args_str) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };
    if !export_args.data && !export_args.script {
        println!(
            "Error: To export data, you need to specify either to export the data or to generate a script."
        );
        let help_args: Vec<&str> = vec![args_str[0], "--help"];
        if let Err(e) = ExportArgs::try_parse_from(&help_args) {
            eprintln!("{}", e);
        }
        return Ok(());
    }
    setting::init();
    gables::refresh_gables();
    if export_args.data {
        execute_convert_command(&export_args.files, &export_args.target);
    }
    if export_args.script {
        execute_script_command(&export_args.files, &export_args.target);
    }
    println!("export successful");
    Ok(())
}

fn execute_convert_command(files: &[String], target: &Option<String>) {
    let build_settings: Vec<BuildSetting> = setting::get_build_settings(target);
    let items: Vec<TreeItem> = gables::get_item_display_name(files);
    for setting in build_settings.iter() {
        for item in items.iter() {
            let datas: HashMap<String, &TreeData> = item.get_datas();
            if datas.len() <= 0 {
                continue;
            }
            for (_, data) in datas.iter() {
                convert::execute(setting, *data)
            }
        }
    }
}
fn execute_script_command(files: &[String], target: &Option<String>) {
    let build_settings: Vec<BuildSetting> = setting::get_build_settings(target);
    let items: Vec<TreeItem> = gables::get_item_display_name(files);
    for setting in build_settings.iter() {
        for item in items.iter() {
            let datas: HashMap<String, &TreeData> = item.get_datas();
            if datas.len() <= 0 {
                continue;
            }
            for (_, data) in datas.iter() {
                generate::execute(setting, *data)
            }
        }
    }
}
