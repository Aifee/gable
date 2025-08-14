use crate::gui::datas::esheet_type::ESheetType;
use eframe::egui;
use std::path::Path;

/// 将列号转换为Excel风格的列名（A, B, ..., Z, AA, AB, ...）
pub fn column_index_to_name(col: u32) -> String {
    let mut result = String::new();
    let mut num = col;

    while num > 0 {
        let remainder = (num - 1) % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        num = (num - 1) / 26;
    }

    result
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

pub fn get_selected_color(ctx: &egui::Context) -> egui::Color32 {
    let style = ctx.style();
    if style.visuals.dark_mode {
        egui::Color32::from_rgb(60, 100, 150)
    } else {
        egui::Color32::from_rgb(173, 216, 230)
    }
}
