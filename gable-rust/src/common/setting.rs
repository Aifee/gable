use crate::common::{constant, utils};
use crate::gui::datas::esheet_type::ESheetType;
use crate::gui::datas::{edevelop_type::EDevelopType, etarget_type::ETargetType};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::{fs, io};

/**
 * 构建设置模型
*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildSetting {
    /// 开发环境
    pub dev: EDevelopType,
    /// 显示名字
    pub display_name: String,
    /// 关键字
    pub keyword: String,
    /// 构建目标类型
    pub target_type: ETargetType,
    // 是否是protobuff 2版本
    pub is_proto_2: bool,
    // 是否自定义模板
    pub is_custom: bool,
    // 自定义模板路径
    pub custom_template: PathBuf,
    // 后处理命令
    pub postprocessing: String,
    /// 构建目标路径，相对路径
    pub target_path: PathBuf,
    // 是否生成脚本
    pub generate_script: bool,
    /// 脚本生成路径
    pub script_path: PathBuf,
}

impl Default for BuildSetting {
    fn default() -> Self {
        Self {
            dev: EDevelopType::Cpp,
            display_name: String::new(),
            keyword: String::new(),
            target_type: ETargetType::Json,
            is_proto_2: false,
            is_custom: false,
            custom_template: PathBuf::new(),
            postprocessing: String::new(),
            target_path: PathBuf::new(),
            generate_script: false,
            script_path: PathBuf::new(),
        }
    }
}

/**
 * 应用设置
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub workspace: Option<String>,
    pub build_settings: Vec<BuildSetting>,
}

lazy_static! {
    pub static ref APP_SETTINGS: RwLock<AppSettings> = RwLock::new(AppSettings {
        workspace: None,
        build_settings: Vec::new(),
    });
}

/**
 * 从JSON文件加载BuildSetting列表
*/
pub fn init() {
    let path: PathBuf = get_data_path().join(constant::SETTING_PREFS);
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(json) => match serde_json::from_str::<AppSettings>(&json) {
                Ok(content) => {
                    let mut settings = APP_SETTINGS.write().unwrap();
                    *settings = content;
                }
                Err(e) => {
                    log::error!("Failed to parse settings from JSON: {}", e);
                }
            },
            Err(e) => {
                log::error!("Failed to read settings file: {}", e);
            }
        }
    }
}

/**
 * 获取窗口标题
*/
pub fn get_title() -> String {
    let workspace = get_workspace().to_string_lossy().to_string();
    format!("Gable - {}", workspace)
}

/**
 * 设置工作空间目录
 * @param path 路径
 * @return 设置结果
*/
pub fn set_workspace(path: String) -> io::Result<()> {
    let mut settings = APP_SETTINGS.write().unwrap();
    settings.workspace = Some(path);
    save_build_settings_to_file(&*settings)
}

/**
 * 获取工作空间目录
 * @return 工作空间目录
*/
pub fn get_workspace() -> PathBuf {
    let settings = APP_SETTINGS.read().unwrap();
    let root_path: PathBuf = if let Some(path) = settings.workspace.as_ref() {
        PathBuf::from(path)
    } else {
        PathBuf::from(".")
    };
    root_path
}

/**
 * 获取临时目录
 * @return 临时目录
*/
pub fn get_temp_path() -> PathBuf {
    let path: PathBuf = get_workspace().join(&constant::DIR_TEMP);
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(&path) {
            log::error!("无法创建临时目录: {}", e);
        }
    }
    path
}

/**
 * 获取数据目录
 * @return 数据目录
*/
pub fn get_data_path() -> PathBuf {
    let exe_path: PathBuf = std::env::current_exe().expect("无法获取当前可执行文件路径");
    let exe_dir: &Path = exe_path.parent().expect("无法获取可执行文件所在目录");
    let temp_dir: &str = constant::DIR_DATA;
    let path: PathBuf = exe_dir.join(temp_dir);
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(&path) {
            log::error!("无法创建临时目录: {}", e);
        }
    }
    path
}

pub fn clone_build_settings() -> Vec<BuildSetting> {
    let settings = APP_SETTINGS.read().unwrap();
    let build_settings: Vec<BuildSetting> = settings.build_settings.clone();
    build_settings
}

/**
 * 添加构建设置
 * @param dev_type 开发类型
 */
pub fn add_build_setting(dev_type: EDevelopType) -> Option<usize> {
    let build_setting: BuildSetting = BuildSetting {
        dev: dev_type,
        display_name: dev_type.to_string().to_string(),
        keyword: dev_type.to_keyword().to_string(),
        target_type: ETargetType::Json,
        is_proto_2: false,
        is_custom: false,
        custom_template: PathBuf::new(),
        postprocessing: String::new(),
        target_path: utils::get_env_relative_path(&get_workspace()),
        generate_script: false,
        script_path: PathBuf::new(),
    };
    let mut settings = APP_SETTINGS.write().unwrap();
    settings.build_settings.push(build_setting);
    if let Err(e) = save_build_settings_to_file(&*settings) {
        log::error!("Failed to save build settings: {}", e);
        None
    } else {
        Some(settings.build_settings.len() - 1)
    }
}

/**
 * 删除BuildSetting
 * @param index 索引
 * @return 新的索引
*/
pub fn remove_build_setting(index: usize) -> Option<usize> {
    let mut settings = APP_SETTINGS.write().unwrap();
    settings.build_settings.remove(index);
    if let Err(e) = save_build_settings_to_file(&*settings) {
        log::error!("Failed to save build settings: {}", e);
        None
    } else {
        if settings.build_settings.is_empty() {
            None
        } else if index >= settings.build_settings.len() {
            Some(settings.build_settings.len() - 1)
        } else {
            Some(index)
        }
    }
}

/**
 * 通过索引ID获取构建设置
 * @param index 索引ID
 * @return 构建设置
*/
pub fn get_build_setting(index: usize) -> Option<BuildSetting> {
    let settings = APP_SETTINGS.read().unwrap();
    if index < settings.build_settings.len() {
        Some(settings.build_settings[index].clone())
    } else {
        None
    }
}

/**
 * 通过名称获取构建设置
 * @param name 名称
 * @return 构建设置
*/
pub fn get_build_setting_with_name(name: &str) -> Option<BuildSetting> {
    let settings = APP_SETTINGS.read().unwrap();
    for setting in settings.build_settings.iter() {
        if setting.display_name == name {
            return Some(setting.clone());
        }
    }
    None
}

/**
 * 更新指定索引的BuildSetting
 * @param index
 * @param setting
 */
pub fn update_build_setting(index: usize, setting: BuildSetting) -> io::Result<()> {
    let mut settings = APP_SETTINGS.write().unwrap();
    if index < settings.build_settings.len() {
        settings.build_settings[index] = setting;
        save_build_settings_to_file(&*settings)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Index out of bounds",
        ))
    }
}

/**
 * 保存BuildSetting列表到JSON文件
 * @param settings AppSettings 当前设置项
 */
fn save_build_settings_to_file(settings: &AppSettings) -> io::Result<()> {
    let json: String = serde_json::to_string_pretty(settings)?;
    let path: PathBuf = get_data_path().join(constant::SETTING_PREFS);
    fs::write(path, json)
}

/**
 * 根据文件路径确定ESheetType
 * 以根目录为起点（workspace）下一级的文件夹名字来判断
 * @param path 文件路径
 * @return ESheetType
 */
pub fn determine_sheet_type(path: &Path) -> ESheetType {
    let workspace = get_workspace();
    if let Ok(relative_path) = path.strip_prefix(&workspace) {
        if let Some(first_component) = relative_path.components().next() {
            match first_component.as_os_str().to_string_lossy().as_ref() {
                "kvs" => return ESheetType::KV,
                "enums" => return ESheetType::Enum,
                "localizes" => return ESheetType::Localize,
                _ => return ESheetType::Normal,
            }
        }
    }
    ESheetType::Normal
}
