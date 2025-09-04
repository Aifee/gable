use crate::{
    common::{
        setting::{self, AppSettings},
        utils,
    },
    gui::datas::{tree_data::TreeData, tree_item::TreeItem},
};
use serde_json::{Map, Value};
use std::sync::MutexGuard;

// 全量构建
// pub fn all(setting: &BuildSetting) {}

pub fn from_items(item: &TreeItem) {
    let datas = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }

    let settings: MutexGuard<'_, AppSettings> = setting::APP_SETTINGS.lock().unwrap();
    for build_setting in settings.build_settings.iter() {
        log::info!("开始构建:{}", build_setting.display_name);
        for (display_name, data) in datas.iter() {
            let target_path = utils::get_absolute_path(&build_setting.target_path)
                .join(format!("{}.json", display_name));
            let json_content = to_json(data, &build_setting.keyword);
            let result = std::fs::write(&target_path, json_content);
            if result.is_err() {
                log::error!("构建失败:{}", target_path.to_str().unwrap());
            } else {
                log::info!("构建成功:{}", target_path.to_str().unwrap());
            }
        }
    }
}

fn to_json(tree_data: &TreeData, keyword: &str) -> String {
    let json_data: Vec<Map<String, Value>> = tree_data.to_json_data(keyword);
    let result = serde_json::to_string_pretty(&json_data).expect("JSON序列化失败");
    result
}
