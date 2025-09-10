use crate::{
    common::{
        convert::{convert_csv, convert_json, convert_protobuff},
        setting::{self, AppSettings, BuildSetting},
    },
    gui::datas::{etarget_type::ETargetType, gables, tree_data::TreeData, tree_item::TreeItem},
};
use std::{collections::HashMap, sync::MutexGuard};

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

pub fn from_items(item: &TreeItem) {
    let datas: HashMap<String, &TreeData> = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }

    let settings: MutexGuard<'_, AppSettings> = setting::APP_SETTINGS.lock().unwrap();
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
