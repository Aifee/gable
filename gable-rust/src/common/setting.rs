use crate::gui::datas::{edevelop_type::EDevelopType, etarget_type::ETargetType};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use std::{fs, io};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn add_build_setting(dev_type: EDevelopType) -> usize {
    let build_setting: BuildSetting = BuildSetting {
        dev: dev_type,
        display_name: dev_type.to_string().to_string(),
        keyword: dev_type.to_string().to_string(),
        target_type: ETargetType::JSON,
        target_path: get_workspace(),
    };
    let mut build_settings: MutexGuard<'_, Vec<BuildSetting>> = BUILD_SETTINGS.lock().unwrap();
    build_settings.push(build_setting);
    build_settings.len() - 1
}

/// 设置BuildSetting列表
pub fn set_build_settings(settings: Vec<BuildSetting>) -> io::Result<()> {
    let mut build_settings: MutexGuard<'_, Vec<BuildSetting>> = BUILD_SETTINGS.lock().unwrap();
    *build_settings = settings;
    save_build_settings_to_file()
}

/// 添加或更新BuildSetting
pub fn add_or_update_build_setting(setting: BuildSetting) -> io::Result<()> {
    let mut build_settings: MutexGuard<'_, Vec<BuildSetting>> = BUILD_SETTINGS.lock().unwrap();
    // 检查是否已存在相同的设置（以display_name为标识）
    if let Some(index) = build_settings
        .iter()
        .position(|s| s.display_name == setting.display_name)
    {
        build_settings[index] = setting;
    } else {
        build_settings.push(setting);
    }
    save_build_settings_to_file()
}

/// 删除BuildSetting
pub fn remove_build_setting(display_name: &str) -> io::Result<()> {
    let mut build_settings = BUILD_SETTINGS.lock().unwrap();
    if let Some(index) = build_settings
        .iter()
        .position(|s| s.display_name == display_name)
    {
        build_settings.remove(index);
        save_build_settings_to_file()
    } else {
        Ok(())
    }
}

/// 获取BuildSetting列表
pub fn get_build_settings() -> MutexGuard<'static, Vec<BuildSetting>> {
    BUILD_SETTINGS.lock().unwrap()
}

/// 保存BuildSetting列表到JSON文件
fn save_build_settings_to_file() -> io::Result<()> {
    let build_settings = BUILD_SETTINGS.lock().unwrap();
    let json = serde_json::to_string_pretty(&*build_settings)?;
    let workspace = WORKSPACE.lock().unwrap();
    let path = if let Some(workspace_path) = workspace.as_ref() {
        PathBuf::from(workspace_path).join("build_settings.json")
    } else {
        PathBuf::from("build_settings.json")
    };
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
