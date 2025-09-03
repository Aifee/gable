use crate::common::constant;
use crate::gui::datas::esheet_type::ESheetType;
use eframe::egui::{Color32, Context, Style};
use std::path::{Path, PathBuf};
use std::sync::Arc;

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

/// 绝对路径转换成exe相对路径
pub fn get_env_relative_path(path: &Path) -> PathBuf {
    if let Ok(relative_path) = path.strip_prefix(&*constant::EXE_DIR) {
        relative_path.to_path_buf()
    } else {
        // 尝试计算相对路径
        match (path.canonicalize(), constant::EXE_DIR.canonicalize()) {
            (Ok(canonical_path), Ok(canonical_exe_dir)) => {
                if let Ok(relative_path) = canonical_path.strip_prefix(&canonical_exe_dir) {
                    relative_path.to_path_buf()
                } else {
                    // 计算两个路径之间的相对路径
                    let exe_components: Vec<_> = canonical_exe_dir.components().collect();
                    let path_components: Vec<_> = canonical_path.components().collect();

                    let mut common_prefix_len = 0;
                    for (i, (exe_comp, path_comp)) in exe_components
                        .iter()
                        .zip(path_components.iter())
                        .enumerate()
                    {
                        if exe_comp == path_comp {
                            common_prefix_len = i + 1;
                        } else {
                            break;
                        }
                    }

                    let mut result: PathBuf = PathBuf::new();
                    // 添加向上移动的目录 ".."
                    for _ in common_prefix_len..exe_components.len() {
                        result.push("..");
                    }
                    // 添加剩余的路径部分
                    for comp in path_components.iter().skip(common_prefix_len) {
                        result.push(comp);
                    }

                    result
                }
            }
            _ => path.to_path_buf(),
        }
    }
}

// 获取绝对路径
pub fn get_absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        // 否则将路径与EXE_DIR连接，解析为绝对路径
        let absolute_path = constant::EXE_DIR.join(path);
        // 规范化路径，解析 .. 和 .
        match absolute_path.canonicalize() {
            Ok(canonical_path) => {
                // 在Windows上，去除 \\?\ 前缀以获得更标准的路径格式
                #[cfg(windows)]
                {
                    if let Some(path_str) = canonical_path.to_str() {
                        if path_str.starts_with(r"\\?\") {
                            return PathBuf::from(&path_str[4..]);
                        }
                    }
                }
                canonical_path
            }
            Err(_) => absolute_path, // 如果规范化失败，返回未规范化的路径
        }
    }
}

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
