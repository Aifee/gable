use crate::common::{global, setting};
use crate::gui::datas::{
    edata_type::EDataType, edevelop_type::EDevelopType, esheet_type::ESheetType,
};
use eframe::egui::{Color32, Context, Style, TextBuffer};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, MutexGuard};

/// 将列号转换为Excel风格的列名（A, B, ..., Z, AA, AB, ...）
pub fn column_index_to_name(col: u32) -> String {
    let mut result: String = String::new();
    let mut num: u32 = col;

    while num > 0 {
        let remainder: u32 = (num - 1) % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        num = (num - 1) / 26;
    }

    result
}

pub fn cell_range(start_row: u32, start_col: u32, end_row: u32, end_col: u16) -> String {
    format!(
        "{}{}:{}{}",
        column_index_to_name(start_col),
        start_row,
        column_index_to_name(end_col as u32),
        end_row
    )
}

///根据文件路径确定ESheetType
pub fn determine_sheet_type(path: &Path) -> ESheetType {
    // 获取父目录名称
    if let Some(parent) = path.parent() {
        if let Some(parent_name) = parent.file_name() {
            match parent_name.to_string_lossy().as_ref() {
                "kvs" => return ESheetType::KV,
                "enums" => return ESheetType::ENUM,
                _ => return ESheetType::DATA,
            }
        }
    }
    // 默认类型
    ESheetType::DATA
}

pub fn get_selected_color(ctx: &Context) -> Color32 {
    let style: Arc<Style> = ctx.style();
    if style.visuals.dark_mode {
        Color32::from_rgb(60, 100, 150)
    } else {
        Color32::from_rgb(173, 216, 230)
    }
}

/// 获取窗口标题
pub fn get_title() -> String {
    let workspace: MutexGuard<'_, Option<String>> = setting::WORKSPACE.lock().unwrap();
    format!(
        "Gable - {}",
        workspace.as_ref().unwrap_or(&"Unknown".to_string())
    )
}

/// 获取临时目录
pub fn get_temp_path() -> String {
    let workspace: MutexGuard<'_, Option<String>> = setting::WORKSPACE.lock().unwrap();
    let temp_dir = global::DIR_TEMP;
    let path: String = PathBuf::from(workspace.as_ref().unwrap())
        .join(temp_dir)
        .to_string_lossy()
        .to_string();
    let path_temp = Path::new(&path);
    if !path_temp.exists() {
        if let Err(e) = fs::create_dir_all(path_temp) {
            log::error!("无法创建临时目录: {}", e);
        }
    }

    path
}

// pub fn convert_data_type(value: &str, dt: EDevelopType) -> EDataType {
//     match dt {
//         EDevelopType::c => EDataType::Unknown,
//         EDevelopType::csharp => convert_data_csharp(value),
//         EDevelopType::cangjie => EDataType::Unknown,
//         EDevelopType::go => EDataType::Unknown,
//         EDevelopType::java => EDataType::Unknown,
//         EDevelopType::javascript => EDataType::Unknown,
//         EDevelopType::lua => EDataType::Unknown,
//         EDevelopType::python => EDataType::Unknown,
//         EDevelopType::typescript => EDataType::Unknown,
//         _ => EDataType::Unknown,
//     }
// }

// fn convert_data_csharp(value: &str) -> EDataType {
//     match value {
//         "int" => EDataType::INT,
//         "string" => EDataType::STRING,
//         "bool" => EDataType::BOOLEAN,
//         "float" => EDataType::FLOAT,
//         "vector2" => EDataType::VECTOR2,
//         "vector3" => EDataType::VECTOR3,
//         "vector4" => EDataType::VECTOR4,
//         "int[]" => EDataType::INT_ARR,
//         "string[]" => EDataType::STRING_ARR,
//         "bool[]" => EDataType::BOOLEAN_ARR,
//         "float[]" => EDataType::FLOAT_ARR,
//         "vector2[]" => EDataType::VECTOR2_ARR,
//         "vector3[]" => EDataType::VECTOR3_ARR,
//         "vector4[]" => EDataType::VECTOR4_ARR,
//         "percentage" => EDataType::PERCENTAGE,
//         "permillage" => EDataType::PERMILLAGE,
//         "permian" => EDataType::PERMIAN,
//         "time" => EDataType::TIME,
//         "enum" => EDataType::ENUM,
//         _ => EDataType::Unknown,
//     }
// }

/// 检查文件名是否合法
pub fn is_valid_filename(name: &str) -> bool {
    // 检查是否为空
    if name.is_empty() {
        return false;
    }

    // 检查是否包含非法字符
    let invalid_chars: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    for c in name.chars() {
        if invalid_chars.contains(&c) || c.is_control() {
            return false;
        }
    }

    // 检查是否以点或空格结尾
    if name.ends_with('.') || name.ends_with(' ') {
        return false;
    }

    // 检查是否是保留名称
    let reserved_names: [&'static str; 22] = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    let upper_name: String = name.to_uppercase();
    for reserved in &reserved_names {
        if &upper_name == reserved {
            return false;
        }
    }
    true
}

/// 检查同名文件/文件夹是否已存在
pub fn is_name_exists(full_path: &str, new_name: &str) -> bool {
    let path: &Path = Path::new(&full_path);
    if let Some(parent_path) = path.parent() {
        let new_path: PathBuf = parent_path.join(new_name);
        new_path.exists()
    } else {
        false
    }
}
pub fn open_in_explorer(path: &str) -> std::io::Result<()> {
    let path_obj = Path::new(path);
    let (explorer_path, select_path) = if path_obj.is_file() {
        let parent = path_obj.parent().unwrap_or(Path::new("."));
        (parent, Some(path_obj))
    } else {
        (path_obj, None)
    };
    #[cfg(target_os = "windows")]
    {
        if let Some(file_to_select) = select_path {
            // 在Windows上，使用 /select 参数来选中特定文件
            std::process::Command::new("explorer")
                .arg("/select,")
                .arg(file_to_select)
                .spawn()?;
        } else {
            std::process::Command::new("explorer")
                .arg(explorer_path)
                .spawn()?;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(file_to_select) = select_path {
            // 在macOS上，使用 -R 参数来选中特定文件
            std::process::Command::new("open")
                .arg("-R")
                .arg(file_to_select)
                .spawn()?;
        } else {
            std::process::Command::new("open")
                .arg(explorer_path)
                .spawn()?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(explorer_path)
            .spawn()?;
    }

    Ok(())
}

// 获取数据类型
pub fn convert_data_type(value: &str) -> EDataType {
    if value.is_empty() {
        return EDataType::STRING;
    }
    match value {
        global::DATA_TYPE_KEY_STRING => EDataType::STRING,
        global::DATA_TYPE_KEY_INT => EDataType::INT,
        global::DATA_TYPE_KEY_BOOLEAN => EDataType::BOOLEAN,
        global::DATA_TYPE_KEY_FLOAT => EDataType::FLOAT,
        global::DATA_TYPE_KEY_VECTOR2 => EDataType::VECTOR2,
        global::DATA_TYPE_KEY_VECTOR3 => EDataType::VECTOR3,
        global::DATA_TYPE_KEY_VECTOR4 => EDataType::VECTOR4,
        global::DATA_TYPE_KEY_STRING_ARR => EDataType::STRING_ARR,
        global::DATA_TYPE_KEY_INT_ARR => EDataType::INT_ARR,
        global::DATA_TYPE_KEY_BOOLEAN_ARR => EDataType::BOOLEAN_ARR,
        global::DATA_TYPE_KEY_FLOAT_ARR => EDataType::FLOAT_ARR,
        global::DATA_TYPE_KEY_VECTOR2_ARR => EDataType::VECTOR2_ARR,
        global::DATA_TYPE_KEY_VECTOR3_ARR => EDataType::VECTOR3_ARR,
        global::DATA_TYPE_KEY_VECTOR4_ARR => EDataType::VECTOR4_ARR,
        global::DATA_TYPE_KEY_PERCENTAGE => EDataType::PERCENTAGE,
        global::DATA_TYPE_KEY_PERMILLAGE => EDataType::PERMILLAGE,
        global::DATA_TYPE_KEY_PERMIAN => EDataType::PERMIAN,
        global::DATA_TYPE_KEY_TIME => EDataType::TIME,
        global::DATA_TYPE_KEY_ENUM => EDataType::ENUM,
        _ => EDataType::Unknown,
    }
}
