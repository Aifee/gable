use crate::{
    common::{
        convert::{convert_csv, convert_json, convert_protobuff},
        setting::{self, BuildSetting},
    },
    gui::datas::{etarget_type::ETargetType, gables, tree_data::TreeData, tree_item::TreeItem},
};
use std::collections::HashMap;

/**
 * 批量转换（所有平台 & 所有表单）
 */
pub fn from_all() {
    let settings = setting::APP_SETTINGS.read().unwrap();
    for setting in settings.build_settings.iter() {
        from_target(setting);
    }
}

/**
 * 批量转换（指定平台 & 所有表单）
 * @param setting 指定的平台
 */
pub fn from_target(setting: &BuildSetting) {
    let items = gables::TREE_ITEMS.read().unwrap();
    let mut datas: HashMap<String, &TreeData> = HashMap::new();
    for item in items.iter() {
        let item_datas: HashMap<String, &TreeData> = item.get_datas();
        if item_datas.len() > 0 {
            datas.extend(item_datas);
        }
    }
    if datas.len() <= 0 {
        log::error!("未找到要导出的配置");
        return;
    }
    for (_, data) in datas.iter() {
        match setting.target_type {
            ETargetType::Json => convert_json::to(setting, data),
            ETargetType::CSV => convert_csv::to(setting, data),
            ETargetType::Protobuff => convert_protobuff::to(setting, data),
        }
    }
}

/**
 * 批量转换（所有平台 & 指定表单）
 * @param item 指定表单
*/
pub fn from_items(item: &TreeItem) {
    let datas: HashMap<String, &TreeData> = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }

    let settings = setting::APP_SETTINGS.read().unwrap();
    for setting in settings.build_settings.iter() {
        for (_, data) in datas.iter() {
            match setting.target_type {
                ETargetType::Json => convert_json::to(setting, data),
                ETargetType::CSV => convert_csv::to(setting, data),
                ETargetType::Protobuff => convert_protobuff::to(setting, data),
            }
        }
    }
}
