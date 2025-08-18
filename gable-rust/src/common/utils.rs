use crate::common::{global, setting};
use crate::gui::datas::esheet_type::ESheetType;
use crate::gui::datas::gable_data::GableData;
use eframe::egui::{Color32, Context, Style};
use rust_xlsxwriter::{Color, Format, FormatBorder, workbook::Workbook, worksheet::Worksheet};
use std::error::Error;
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

/// 读取并解析gable文件
pub fn read_gable_file(file_path: &str) -> Option<GableData> {
    match fs::read_to_string(file_path) {
        Ok(content) => match serde_json::from_str::<GableData>(&content) {
            Ok(json_value) => Some(json_value),
            Err(e) => {
                log::error!("解析JSON文件失败:'{}': {}", file_path, e);
                None
            }
        },
        Err(e) => {
            log::error!("读取文件失败:'{}': {}", file_path, e);
            None
        }
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
    path
}

// 写入Excel文件
pub fn write_excel(excel_name: &str, gable_files: Vec<String>) -> Result<String, Box<dyn Error>> {
    let file_name: &str = &format!("{}{}", &excel_name, &global::EXCEL_EXTENSION);
    let tem_path = get_temp_path();
    let excel_file_path_tem: String = PathBuf::from(&tem_path)
        .join(&format!("~${}", &file_name))
        .to_string_lossy()
        .to_string();
    let excel_file_path: String = PathBuf::from(&tem_path)
        .join(&file_name)
        .to_string_lossy()
        .to_string();
    // 检查临时文件是否存在（表示Excel文件已打开）
    if Path::new(&excel_file_path_tem).exists() {
        log::error!("Excel文件 '{}' 已经打开，无法写入", excel_file_path);
        return Err("Excel文件已经打开，无法写入".into());
    }
    // 如果Excel文件已存在，则删除它
    if Path::new(&excel_file_path).exists() {
        match fs::remove_file(&excel_file_path) {
            Ok(_) => log::info!("已删除旧的Excel文件 '{}'", excel_file_path),
            Err(e) => {
                log::error!("无法删除已存在的Excel文件 '{}': {}", excel_file_path, e);
                return Err(e.into());
            }
        }
    }
    let mut workbook: Workbook = Workbook::new();
    let header_format_1: Format = Format::new()
        .set_background_color(Color::Theme(3, 1))
        .set_border(FormatBorder::Thin);
    let header_format_2: Format = Format::new()
        .set_background_color(Color::Theme(6, 1))
        .set_border(FormatBorder::Thin);

    for file_path in gable_files.iter() {
        if let Some(gable_data) = read_gable_file(file_path) {
            let worksheet: &mut Worksheet = workbook.add_worksheet();
            worksheet.set_name(&gable_data.sheetname)?;
            for (row_key, row_data) in &gable_data.heads {
                // 为了保证和excel工具统一，起始行从1开始
                let row_index: u32 = row_key.parse().unwrap_or(0) - 1;
                for (col_key, cell_data) in row_data {
                    // 为了保证和excel工具统一，起始列从1开始
                    let col_index: u16 = col_key.parse().unwrap_or(0) - 1;
                    worksheet.write_string(row_index, col_index, &cell_data.value)?;
                    if row_index % 2 == 0 {
                        worksheet.set_cell_format(row_index, col_index, &header_format_1)?;
                    } else {
                        worksheet.set_cell_format(row_index, col_index, &header_format_2)?;
                    }
                }
            }

            for (row_key, row_data) in &gable_data.cells {
                let row_index: u32 = row_key.parse().unwrap_or(0) - 1;
                for (col_key, cell_data) in row_data {
                    let col_index: u16 = col_key.parse().unwrap_or(0) - 1;
                    worksheet.write_string(row_index, col_index, &cell_data.value)?;
                }
            }
        } else {
            log::error!("无法读取或解析文件: {}", file_path);
        }
    }
    workbook.save(&excel_file_path)?;
    log::info!("成功写入Excel文件: {}", excel_file_path);
    Ok(excel_file_path)
}
