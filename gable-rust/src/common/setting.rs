use crate::common::{constant, utils};
use crate::gui::datas::{edevelop_type::EDevelopType, etarget_type::ETargetType};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use std::{fs, io};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildSetting {
    /// 开发环境
    pub dev: EDevelopType,
    /// 显示名字
    pub display_name: String,
    /// 关键字
    pub keyword: String,
    /// 构建目标类型
    pub target_type: ETargetType,
    /// 构建目标路径，相对路径
    pub target_path: PathBuf,
}

lazy_static! {
    /// 全局存储当前的目录树
    pub static ref WORKSPACE: Mutex<Option<String>> = Mutex::new(Some(String::from("E:\\projects\\gable_project_temp")));
    /// 全局BuildSetting列表
    pub static ref BUILD_SETTINGS: Mutex<Vec<BuildSetting>> = Mutex::new(Vec::new());
}

// 提供一个安全的设置方法
pub fn set_workspace(path: String) {
    let mut workspace: MutexGuard<'_, Option<String>> = WORKSPACE.lock().unwrap();
    *workspace = Some(path);
}

pub fn get_workspace() -> PathBuf {
    let workspace: MutexGuard<'_, Option<String>> = WORKSPACE.lock().unwrap();
    let root_path: PathBuf = if let Some(path) = workspace.as_ref() {
        PathBuf::from(path)
    } else {
        PathBuf::from(".")
    };
    root_path
}

pub fn add_build_setting(dev_type: EDevelopType) -> Option<usize> {
    let build_setting: BuildSetting = BuildSetting {
        dev: dev_type,
        display_name: dev_type.to_string().to_string(),
        keyword: dev_type.to_keyword().to_string(),
        target_type: ETargetType::JSON,
        target_path: get_workspace(),
    };
    let mut build_settings: MutexGuard<'_, Vec<BuildSetting>> = BUILD_SETTINGS.lock().unwrap();
    build_settings.push(build_setting);
    if let Err(e) = save_build_settings_to_file(&*build_settings) {
        log::error!("Failed to save build settings: {}", e);
        None
    } else {
        Some(build_settings.len() - 1)
    }
}

pub fn get_build_setting(index: usize) -> Option<BuildSetting> {
    let build_settings: MutexGuard<'_, Vec<BuildSetting>> = BUILD_SETTINGS.lock().unwrap();
    if index < build_settings.len() {
        Some(build_settings[index].clone())
    } else {
        None
    }
}

/// 更新指定索引的BuildSetting
pub fn update_build_setting(index: usize, setting: BuildSetting) -> io::Result<()> {
    let mut build_settings: MutexGuard<'_, Vec<BuildSetting>> = BUILD_SETTINGS.lock().unwrap();
    if index < build_settings.len() {
        build_settings[index] = setting;
        save_build_settings_to_file(&*build_settings)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Index out of bounds",
        ))
    }
}

/// 删除BuildSetting
pub fn remove_build_setting(display_name: &str) -> io::Result<()> {
    let mut build_settings = BUILD_SETTINGS.lock().unwrap();
    if let Some(index) = build_settings
        .iter()
        .position(|s| s.display_name == display_name)
    {
        build_settings.remove(index);
        save_build_settings_to_file(&*build_settings)
    } else {
        Ok(())
    }
}

/// 保存BuildSetting列表到JSON文件
fn save_build_settings_to_file(settings: &Vec<BuildSetting>) -> io::Result<()> {
    let json: String = serde_json::to_string_pretty(&settings)?;
    let path: PathBuf = utils::get_data_path().join(constant::SETTING_PREFS);
    fs::write(path, json)
}

/// 从JSON文件加载BuildSetting列表
pub fn load_build_settings_from_file() -> io::Result<()> {
    let workspace = WORKSPACE.lock().unwrap();
    let path = if let Some(workspace_path) = workspace.as_ref() {
        PathBuf::from(workspace_path).join("build_settings.json")
    } else {
        PathBuf::from("build_settings.json")
    };

    if path.exists() {
        let json = fs::read_to_string(path)?;
        let settings: Vec<BuildSetting> = serde_json::from_str(&json)?;
        let mut build_settings = BUILD_SETTINGS.lock().unwrap();
        *build_settings = settings;
    }
    Ok(())
}
