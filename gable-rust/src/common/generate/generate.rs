use crate::{
    common::{
        generate::generate_csharp,
        setting::{self, AppSettings, BuildSetting},
    },
    gui::datas::{edevelop_type::EDevelopType, gables, tree_data::TreeData, tree_item::TreeItem},
};
use std::{collections::HashMap, sync::MutexGuard};

pub fn from_target(setting: &BuildSetting) {
    if !setting.generate_script {
        return;
    }
    let items: MutexGuard<'_, Vec<TreeItem>> = gables::TREE_ITEMS.lock().unwrap();
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
        match setting.dev {
            EDevelopType::Csharp => generate_csharp::to(setting, data),
            _ => {
                log::error!("当前开发环境不支持导出配置:{:?}", setting.dev);
            }
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
        if !setting.generate_script {
            continue;
        }
        for (_, data) in datas.iter() {
            match setting.dev {
                EDevelopType::Csharp => generate_csharp::to(setting, data),
                _ => {
                    log::error!("当前开发环境不支持导出配置:{:?}", setting.dev);
                }
            }
        }
    }
}
