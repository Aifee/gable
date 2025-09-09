use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{esheet_type::ESheetType, tree_data::TreeData},
};
use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
};

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        // 枚举不导出
        return;
    }

    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.csv", tree_data.content.sheetname));
    let csv_data: Vec<Vec<String>> = tree_data.to_csv_data(&build_setting.keyword);
    // 创建CSV文件
    let file: Result<File, Error> = File::create(&target_path);
    if file.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
        return;
    }
    let file = file.unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);
    // 写入CSV数据
    for row_data in csv_data.iter() {
        let mut line: String = String::new();
        let mut is_first: bool = true;
        for col_value in row_data.iter() {
            if !is_first {
                line.push(',');
            }
            // 转义包含逗号或引号的值
            if col_value.contains(',') || col_value.contains('"') || col_value.contains('\n') {
                line.push('"');
                line.push_str(&col_value.replace("\"", "\"\""));
                line.push('"');
            } else {
                line.push_str(col_value);
            }
            is_first = false;
        }

        line.push('\n');
        if let Err(e) = writer.write_all(line.as_bytes()) {
            log::error!("写入【{}】文件时出错:{}", build_setting.display_name, e);
            return;
        }
    }

    if let Err(e) = writer.flush() {
        log::error!("刷新【{}】文件时出错:{}", build_setting.display_name, e);
        return;
    }

    log::info!(
        "导出【{}】成功:{}",
        build_setting.display_name,
        target_path.to_str().unwrap()
    );
}
