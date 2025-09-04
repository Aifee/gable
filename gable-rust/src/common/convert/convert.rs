use std::sync::MutexGuard;

use crate::{
    common::setting::{self, AppSettings, BuildSetting},
    gui::datas::{tree_data::TreeData, tree_item::TreeItem},
};

// 全量构建
// pub fn all(setting: &BuildSetting) {}

pub fn from_items(item: &TreeItem) {
    let datas = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }
    for (display_name, data) in datas.iter() {
        log::info!("开始构建:{}", display_name);
    }
    // let settings: MutexGuard<'_, AppSettings> = setting::APP_SETTINGS.lock().unwrap();
    // for build_setting in settings.build_settings.iter() {
    //     log::info!("开始构建:{}", build_setting.display_name);
    // }
}

// pub fn form_setting(setting: &BuildSetting) {}

// fn to_json(tree_data: &TreeData) -> String {
//     let json: String = serde_json::to_string_pretty(&gable_data.to_json_data())?;
//     json
// }
