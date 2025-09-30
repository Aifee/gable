use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{esheet_type::ESheetType, tree_data::TreeData},
};
use serde_json::{Map, Value};
use std::{io::Error, path::PathBuf};

/**
 * 将数据转换为json
 * @param build_setting 构建设置
 * @param tree_data 树数据
 */
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        // 枚举不导出
        return;
    }
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.json", tree_data.file_name));
    let json_data: Vec<Map<String, Value>> = tree_data.to_values(&build_setting.keyword);
    if json_data.is_empty() {
        log::debug!("No data to export: {}", target_path.to_str().unwrap());
        return;
    }
    let contents: String =
        serde_json::to_string_pretty(&json_data).expect("JSON serialization failed");
    let result: Result<(), Error> = std::fs::write(&target_path, contents);
    if result.is_err() {
        log::error!(
            "Export [{}] failed: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "Export [{}] successful: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    }
}
