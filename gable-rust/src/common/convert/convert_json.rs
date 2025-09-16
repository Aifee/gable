use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{esheet_type::ESheetType, tree_data::TreeData},
};
use serde_json::{Map, Value};
use std::io::Error;

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        // 枚举不导出
        return;
    }
    let target_path = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.json", tree_data.file_name));
    let json_data: Vec<Map<String, Value>> = tree_data.to_values(&build_setting.keyword);
    let contents: String = serde_json::to_string_pretty(&json_data).expect("JSON序列化失败");
    let result: Result<(), Error> = std::fs::write(&target_path, contents);
    if result.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "导出【{}】成功:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    }
}
