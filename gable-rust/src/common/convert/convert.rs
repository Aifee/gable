use crate::{
    common::{
        setting::{self, AppSettings, BuildSetting},
        utils,
    },
    gui::datas::{etarget_type::ETargetType, gables, tree_data::TreeData, tree_item::TreeItem},
};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
    sync::MutexGuard,
};

pub fn from_target(setting: &BuildSetting) {
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
        match setting.target_type {
            ETargetType::JSON => to_json(setting, data),
            ETargetType::CSV => to_csv(setting, data),
            _ => {
                log::info!("暂未实现：{:?}", setting.target_type)
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
    for build_setting in settings.build_settings.iter() {
        for (_, data) in datas.iter() {
            match build_setting.target_type {
                ETargetType::JSON => to_json(build_setting, data),
                ETargetType::CSV => to_csv(build_setting, data),
                _ => {
                    log::info!("暂未实现：{:?}", build_setting.target_type)
                }
            }
        }
    }
}

fn to_json(build_setting: &BuildSetting, tree_data: &TreeData) {
    let target_path = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.json", tree_data.content.sheetname));
    let json_data: Vec<Map<String, Value>> = tree_data.to_json_data(&build_setting.keyword);
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

fn to_csv(build_setting: &BuildSetting, tree_data: &TreeData) {
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
