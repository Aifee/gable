use crate::{
    common::{
        generate::{
            generate_cangjie, generate_cpp, generate_csharp, generate_golang, generate_java,
            generate_javascript, generate_lua, generate_protobuff, generate_python,
            generate_typescript,
        },
        setting::{self, BuildSetting},
        utils,
    },
    gui::datas::{
        edevelop_type::EDevelopType, etarget_type::ETargetType, gables, tree_data::TreeData,
        tree_item::TreeItem,
    },
};
use std::process::Command;
use std::{collections::HashMap, path::PathBuf};

pub fn from_target(setting: &BuildSetting) {
    if !setting.generate_script {
        return;
    }
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
        if setting.target_type == ETargetType::Protobuff {
            generate_protobuff::to(setting, data);
        } else {
            match setting.dev {
                EDevelopType::Cpp => generate_cpp::to(setting, data),
                EDevelopType::Csharp => generate_csharp::to(setting, data),
                EDevelopType::Cangjie => generate_cangjie::to(setting, data),
                EDevelopType::Golang => generate_golang::to(setting, data),
                EDevelopType::Java => generate_java::to(setting, data),
                EDevelopType::JavaScript => generate_javascript::to(setting, data),
                EDevelopType::Lua => generate_lua::to(setting, data),
                EDevelopType::Python => generate_python::to(setting, data),
                EDevelopType::TypeScript => generate_typescript::to(setting, data),
                // _ => {
                //     log::error!("当前开发环境不支持导出配置:{:?}", setting.dev);
                // }
            }
        }
    }
    if setting.target_type == ETargetType::Protobuff {
        let target_path: PathBuf = utils::get_absolute_path(&setting.proto_target_path);
        system_command(&setting.postprocessing, &target_path);
    }
}

pub fn from_items(item: &TreeItem) {
    let datas: HashMap<String, &TreeData> = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }

    let settings = setting::APP_SETTINGS.read().unwrap();
    for setting in settings.build_settings.iter() {
        if !setting.generate_script {
            continue;
        }
        for (_, data) in datas.iter() {
            if setting.target_type == ETargetType::Protobuff {
                generate_protobuff::to(setting, data);
            } else {
                match setting.dev {
                    EDevelopType::Csharp => generate_csharp::to(setting, data),
                    _ => {
                        log::error!("当前开发环境不支持导出配置:{:?}", setting.dev);
                    }
                }
            }
        }
        if setting.target_type == ETargetType::Protobuff {
            let target_path: PathBuf = utils::get_absolute_path(&setting.proto_target_path);
            system_command(&setting.postprocessing, &target_path);
        }
    }
}

fn system_command(command: &str, path: &PathBuf) {
    if command.is_empty() {
        return;
    }
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = Command::new("cmd")
            .current_dir(&path)
            .args(&["/C", &command])
            .spawn()
        {
            log::error!("无法执行后处理命令: {}", e);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Err(e) = Command::new("sh")
            .current_dir(&path)
            .arg("-c")
            .arg(&command)
            .spawn()
        {
            log::error!("无法执行后处理命令: {}", e);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = Command::new("sh")
            .current_dir(&path)
            .arg("-c")
            .arg(&command)
            .spawn()
        {
            log::error!("无法执行后处理命令: {}", e);
        }
    }
}
