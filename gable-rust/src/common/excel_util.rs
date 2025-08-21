use crate::{
    common::{global, utils},
    gui::datas::{
        cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType, gable_data::GableData,
    },
};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use umya_spreadsheet::{
    Border, Cell, PatternValues, Spreadsheet, Style, Worksheet, reader, writer,
};

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

// 写入Excel文件
pub fn write_excel(excel_name: &str, gable_files: Vec<String>) -> Result<String, Box<dyn Error>> {
    let file_name: &str = &format!("{}{}", &excel_name, &global::EXCEL_EXTENSION);
    let tem_path: String = utils::get_temp_path();
    let excel_file_path_tem: String = PathBuf::from(&tem_path)
        .join(&format!("~${}", &file_name))
        .to_string_lossy()
        .to_string();
    let excel_file_path: String = PathBuf::from(&tem_path)
        .join(&file_name)
        .to_string_lossy()
        .to_string();
    if Path::new(&excel_file_path_tem).exists() {
        log::error!("Excel文件 '{}' 已经打开，无法写入", excel_file_path);
        return Err("Excel文件已经打开，无法写入".into());
    }
    if Path::new(&excel_file_path).exists() {
        match fs::remove_file(&excel_file_path) {
            Ok(_) => {}
            Err(e) => {
                log::error!("无法删除已存在的Excel文件 '{}': {}", excel_file_path, e);
                return Err(e.into());
            }
        }
    }
    let mut workbook: Spreadsheet = umya_spreadsheet::new_file();
    let sheet_counts = workbook.get_sheet_count();
    for _ in 0..sheet_counts {
        match workbook.remove_sheet(0) {
            Ok(_) => {}
            Err(_) => {
                log::error!("无法删除工作表");
            }
        }
    }
    for file_path in gable_files.iter() {
        if let Some(gable_data) = read_gable_file(file_path) {
            let worksheet: &mut Worksheet = match workbook.new_sheet(&gable_data.sheetname) {
                Ok(sheet) => sheet,
                Err(e) => {
                    log::error!("无法添加工作表到Excel文件: {}", e);
                    continue;
                }
            };
            for (row_index, row_data) in &gable_data.heads {
                for (col_index, cell_data) in row_data {
                    let cell: &mut Cell = worksheet.get_cell_mut((*col_index as u32, *row_index));
                    cell.set_value(&cell_data.value);
                    let style = cell.get_style_mut();
                    let borders = style.get_borders_mut();
                    borders
                        .get_left_border_mut()
                        .set_border_style(Border::BORDER_THIN);
                    borders
                        .get_right_border_mut()
                        .set_border_style(Border::BORDER_THIN);
                    borders
                        .get_top_border_mut()
                        .set_border_style(Border::BORDER_THIN);
                    borders
                        .get_bottom_border_mut()
                        .set_border_style(Border::BORDER_THIN);
                    let color = style
                        .get_fill_mut()
                        .get_pattern_fill_mut()
                        .set_pattern_type(PatternValues::Solid)
                        .remove_background_color()
                        .get_foreground_color_mut();

                    if row_index % 2 == 0 {
                        color.set_theme_index(7);
                        color.set_tint(0.8);
                    } else {
                        color.set_theme_index(9);
                        color.set_tint(0.8);
                    }
                }
            }

            for (row_index, row_data) in &gable_data.cells {
                for (col_index, cell_data) in row_data {
                    let cell: &mut Cell = worksheet.get_cell_mut((*col_index as u32, *row_index));
                    write_excel_cell_value(cell, &gable_data.heads, &col_index, &cell_data);
                }
            }
        } else {
            log::error!("无法读取或解析文件: {}", file_path);
        }
    }
    match writer::xlsx::write(&workbook, &excel_file_path) {
        Ok(_) => {}
        Err(e) => {
            log::error!("无法写入Excel文件 '{}': {}", excel_file_path, e);
            return Err(e.into());
        }
    };
    Ok(excel_file_path)
}

// excel 单元格数据类型写入
fn write_excel_cell_value(
    cell: &mut Cell,
    heads: &HashMap<u32, HashMap<u16, CellData>>,
    col_index: &u16,
    cell_data: &CellData,
) {
    let row_key = global::TABLE_DATA_ROW_TYPE;
    if let Some(row_data) = heads.get(&row_key) {
        if let Some(cell_type_data) = row_data.get(&col_index) {
            match cell_type_data.get_data_type() {
                EDataType::INT => cell.set_value_number(cell_data.parse_int()),
                EDataType::BOOLEAN => cell.set_value_bool(cell_data.parse_bool()),
                EDataType::FLOAT => cell.set_value_number(cell_data.parse_float()),
                EDataType::VECTOR2 => cell.set_value(cell_data.parse_vector2()),
                _ => cell.set_value(cell_data.value.clone()),
            };
        }
    }
    let fill_type = cell_data.get_fill_type();
    let style = cell.get_style_mut();
    let borders = style.get_borders_mut();
    borders
        .get_left_border_mut()
        .set_border_style(Border::BORDER_NONE);
    borders
        .get_right_border_mut()
        .set_border_style(Border::BORDER_NONE);
    borders
        .get_top_border_mut()
        .set_border_style(Border::BORDER_NONE);
    borders
        .get_bottom_border_mut()
        .set_border_style(Border::BORDER_NONE);
    if fill_type == 0 {
        style
            .get_fill_mut()
            .get_pattern_fill_mut()
            .set_pattern_type(PatternValues::Solid)
            .remove_background_color()
            .get_foreground_color_mut()
            .set_argb(cell_data.get_fill_color());
    } else if fill_type == 1 {
        let (theme, tint) = cell_data.get_fill_theme_tint();
        let color = style
            .get_fill_mut()
            .get_pattern_fill_mut()
            .set_pattern_type(PatternValues::Solid)
            .remove_background_color()
            .get_foreground_color_mut();
        color.set_theme_index(theme);
        color.set_tint(tint);
    }
}

// Excel数据写入gable文件
pub fn write_gable(
    excel_file: String,
    target_path: String,
    sheet_type: ESheetType,
) -> Result<Vec<String>, Box<dyn Error>> {
    let workbook: Spreadsheet = reader::xlsx::read(&excel_file).unwrap();
    let sheet_counts: usize = workbook.get_sheet_count();
    let file_path: &Path = Path::new(&excel_file);
    let file_stem: &str = file_path.file_stem().unwrap().to_str().unwrap();
    let mut gable_file_paths: Vec<String> = Vec::new();

    for sheet_index in 0..sheet_counts {
        let worksheet: &umya_spreadsheet::Worksheet =
            if let Some(sheet) = workbook.get_sheet(&sheet_index) {
                sheet
            } else {
                log::error!("无法获取工作表索引: {}", sheet_index);
                continue;
            };
        let sheet_name = worksheet.get_name().to_string();
        let (max_col, max_row) = worksheet.get_highest_column_and_row();
        let mut gable_data: GableData = GableData {
            sheetname: sheet_name.clone(),
            max_row: max_row,
            max_column: max_col as u16,
            heads: HashMap::new(),
            cells: HashMap::new(),
        };

        // 读取数据并填充到GableData中
        for row_idx in 0..max_row {
            let row_key: u32 = row_idx + 1;
            let mut row_data: HashMap<u16, CellData> = HashMap::new();
            for col_idx in 0..max_col {
                let col_key: u32 = col_idx + 1;
                if let Some(cell) = worksheet.get_cell((&col_key, &row_key)) {
                    let value: std::borrow::Cow<'static, str> = cell.get_value();
                    let style: &umya_spreadsheet::Style = cell.get_style();
                    let bc: Option<&umya_spreadsheet::Color> = style.get_background_color();
                    let fc: Option<&umya_spreadsheet::Color> = if let Some(font) = style.get_font()
                    {
                        Some(font.get_color())
                    } else {
                        None
                    };

                    let cell_data: CellData =
                        CellData::new(row_key, col_key as u16, value.to_string(), bc, fc);
                    if cell_data.is_empty() {
                        continue;
                    }
                    row_data.insert(col_key as u16, cell_data);
                }
            }

            match sheet_type {
                ESheetType::KV => {
                    if row_key < global::TABLE_KV_ROW_TOTAL {
                        gable_data.heads.insert(row_key, row_data);
                    } else {
                        gable_data.cells.insert(row_key, row_data);
                    }
                }
                ESheetType::ENUM => {
                    if row_key < global::TABLE_ENUM_ROW_TOTAL {
                        gable_data.heads.insert(row_key, row_data);
                    } else {
                        gable_data.cells.insert(row_key, row_data);
                    }
                }
                _ => {
                    if row_key < global::TABLE_DATA_ROW_TOTAL {
                        gable_data.heads.insert(row_key, row_data);
                    } else {
                        gable_data.cells.insert(row_key, row_data);
                    }
                }
            }
        }
        // 创建.gable文件路径
        let gable_file_path: PathBuf =
            PathBuf::from(&target_path).join(format!("{}@{}.gable", file_stem, &sheet_name));
        // 将路径添加到返回列表中
        gable_file_paths.push(gable_file_path.to_string_lossy().to_string());
        let json_data: String = serde_json::to_string_pretty(&gable_data)?;
        // log::info!("{}", json_data);
        fs::write(&gable_file_path, json_data)?;
    }

    Ok(gable_file_paths)
}
