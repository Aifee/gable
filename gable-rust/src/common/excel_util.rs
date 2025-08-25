use crate::{
    common::{global, utils},
    gui::datas::{
        cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType, gable_data::GableData,
    },
};
use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use umya_spreadsheet::{
    Border, Cell, Color, DataValidation, DataValidationValues, DataValidations, PatternValues,
    Spreadsheet, Style, Worksheet, reader, writer,
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
pub fn write_excel(
    excel_name: &str,
    sheet_type: &ESheetType,
    gable_files: Vec<String>,
) -> Result<String, Box<dyn Error>> {
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
            let range = utils::cell_range(
                global::TABLE_DATA_ROW_TYPE,
                1,
                global::TABLE_DATA_ROW_TYPE,
                gable_data.max_col,
            );
            println!("设置单元格格式:{}", range);
            let mut data_validation: DataValidation = DataValidation::default();
            data_validation.set_formula1(format!("\"{}\"", global::DATA_TYPE_KEYS.join(",")));
            data_validation.set_type(DataValidationValues::List);
            data_validation
                .get_sequence_of_references_mut()
                .set_sqref(range);
            let mut data_validations = DataValidations::default();
            data_validations.add_data_validation_list(data_validation);
            worksheet.set_data_validations(data_validations);

            // 预先设置单元格格式，百分率，千分率，万分率，时间，枚举类型的单元格，如果按照数据填充的话有可能会设置不到
            // 但又不能全量遍历所有的单元格，故此只针对这几种类型单独设置单元格格式
            let max_row: u32 = gable_data.max_row + 1;
            let max_col: u16 = gable_data.max_col + 1;

            match sheet_type {
                ESheetType::DATA => {
                    for col_index in 1..max_col {
                        let cell_type_data: Option<&CellData> = gable_data
                            .heads
                            .get(&global::TABLE_DATA_ROW_TYPE)
                            .and_then(|row| row.get(&col_index));
                        let cell_type: EDataType = if let Some(data) = cell_type_data {
                            utils::convert_data_type(&data.value)
                        } else {
                            EDataType::STRING
                        };
                        if cell_type != EDataType::PERCENTAGE
                            && cell_type != EDataType::PERMILLAGE
                            && cell_type != EDataType::PERMIAN
                            && cell_type != EDataType::TIME
                            && cell_type != EDataType::DATE
                            && cell_type != EDataType::ENUM
                        {
                            continue;
                        }
                        for row_index in global::TABLE_DATA_ROW_TOTAL..max_row {
                            let cell: &mut Cell =
                                worksheet.get_cell_mut((&(col_index as u32), &row_index));
                            match cell_type {
                                EDataType::PERCENTAGE => {
                                    cell.get_style_mut()
                                        .get_number_format_mut()
                                        .set_format_code(global::NUMBER_FORMAT_PERCENTAGE);
                                }
                                EDataType::PERMILLAGE => {
                                    cell.get_style_mut()
                                        .get_number_format_mut()
                                        .set_format_code(global::NUMBER_FORMAT_PERMILLAGE);
                                }
                                EDataType::PERMIAN => {
                                    cell.get_style_mut()
                                        .get_number_format_mut()
                                        .set_format_code(global::NUMBER_FORMAT_PERMIAN);
                                }
                                EDataType::TIME => {
                                    cell.get_style_mut()
                                        .get_number_format_mut()
                                        .set_format_code(global::NUMBER_FORMAT_TIME);
                                }
                                EDataType::DATE => {
                                    cell.get_style_mut()
                                        .get_number_format_mut()
                                        .set_format_code(global::NUMBER_FORMAT_DATE);
                                }
                                EDataType::ENUM => {}
                                _ => {}
                            }
                        }
                    }
                }
                ESheetType::KV => {
                    for row_index in global::TABLE_KV_ROW_TOTAL..max_row {
                        let cell_type_data: &mut Cell =
                            worksheet.get_cell_mut((&global::TABLE_KV_COL_TYPE, &row_index));
                        let cell_type: EDataType =
                            utils::convert_data_type(&cell_type_data.get_value());
                        let cell: &mut Cell =
                            worksheet.get_cell_mut((&global::TABLE_KV_COL_VALUE, &row_index));
                        match cell_type {
                            EDataType::PERCENTAGE => {
                                cell.get_style_mut()
                                    .get_number_format_mut()
                                    .set_format_code(global::NUMBER_FORMAT_PERCENTAGE);
                            }
                            EDataType::PERMILLAGE => {
                                cell.get_style_mut()
                                    .get_number_format_mut()
                                    .set_format_code(global::NUMBER_FORMAT_PERMILLAGE);
                            }
                            EDataType::PERMIAN => {
                                cell.get_style_mut()
                                    .get_number_format_mut()
                                    .set_format_code(global::NUMBER_FORMAT_PERMIAN);
                            }
                            EDataType::TIME => {
                                cell.get_style_mut()
                                    .get_number_format_mut()
                                    .set_format_code(global::NUMBER_FORMAT_TIME);
                            }
                            EDataType::DATE => {
                                cell.get_style_mut()
                                    .get_number_format_mut()
                                    .set_format_code(global::NUMBER_FORMAT_DATE);
                            }
                            EDataType::ENUM => {}
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            for (row_index, row_data) in &gable_data.heads {
                for (col_index, cell_data) in row_data {
                    let cell: &mut Cell = worksheet.get_cell_mut((*col_index as u32, *row_index));
                    cell.set_value(&cell_data.value);
                    let style: &mut Style = cell.get_style_mut();
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
                    let color: &mut Color = style
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
    let row_key: u32 = global::TABLE_DATA_ROW_TYPE;
    if let Some(row_data) = heads.get(&row_key) {
        if let Some(cell_type_data) = row_data.get(&col_index) {
            match utils::convert_data_type(&cell_type_data.value) {
                EDataType::INT => cell.set_value_number(cell_data.parse_int()),
                EDataType::BOOLEAN => cell.set_value_bool(cell_data.parse_bool()),
                EDataType::FLOAT => cell.set_value_number(cell_data.parse_float()),
                EDataType::PERCENTAGE => cell.set_value_number(cell_data.parse_float()),
                EDataType::PERMILLAGE => cell.set_value_number(cell_data.parse_float() * 1000.0),
                EDataType::PERMIAN => cell.set_value_number(cell_data.parse_float() * 10000.0),
                EDataType::TIME => cell.set_value_number(cell_data.parse_time()),
                EDataType::DATE => cell.set_value_number(cell_data.parse_date()),
                _ => cell.set_value(cell_data.value.clone()),
            };
        }
    }

    let style: &mut Style = cell.get_style_mut();
    // 边框
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
    // 背景色
    let background_type: i8 = cell_data.get_background_type();
    if background_type == 0 {
        style
            .get_fill_mut()
            .get_pattern_fill_mut()
            .set_pattern_type(PatternValues::Solid)
            .remove_background_color()
            .get_foreground_color_mut()
            .set_argb(cell_data.get_background_color());
    } else if background_type == 1 {
        let (theme, tint) = cell_data.get_background_theme_tint();
        let color: &mut Color = style
            .get_fill_mut()
            .get_pattern_fill_mut()
            .set_pattern_type(PatternValues::Solid)
            .remove_background_color()
            .get_foreground_color_mut();
        color.set_theme_index(theme);
        color.set_tint(tint);
    }

    // 字体颜色
    let font_type: i8 = cell_data.get_font_type();
    if font_type == 0 {
        // 字体颜色
        style
            .get_font_mut()
            .get_color_mut()
            .set_argb(cell_data.get_font_color());
    } else if font_type == 1 {
        let (theme, tint) = cell_data.get_font_theme_tint();
        let color: &mut Color = style.get_font_mut().get_color_mut();
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
        let worksheet: &Worksheet = if let Some(sheet) = workbook.get_sheet(&sheet_index) {
            sheet
        } else {
            log::error!("无法获取工作表索引: {}", sheet_index);
            continue;
        };
        let sheet_name: String = worksheet.get_name().to_string();
        let (max_col, max_row) = worksheet.get_highest_column_and_row();
        let mut gable_data: GableData = GableData {
            sheetname: sheet_name.clone(),
            max_row: max_row,
            max_col: max_col as u16,
            heads: HashMap::new(),
            cells: HashMap::new(),
        };

        let max_row: u32 = max_row + 1;
        let max_col: u32 = max_col + 1;
        // 读取数据并填充到GableData中
        for row_idx in 1..max_row {
            let mut row_data: HashMap<u16, CellData> = HashMap::new();
            let mut cell_type: EDataType = EDataType::STRING;
            if sheet_type == ESheetType::KV && row_idx >= global::TABLE_KV_ROW_TOTAL {
                cell_type = if let Some(cell_type_data) =
                    worksheet.get_cell((&global::TABLE_KV_COL_TYPE, &row_idx))
                {
                    utils::convert_data_type(&cell_type_data.get_value())
                } else {
                    EDataType::STRING
                };
            }
            for col_idx in 0..max_col {
                if sheet_type == ESheetType::DATA && row_idx >= global::TABLE_DATA_ROW_TOTAL {
                    cell_type = if let Some(cell_type_data) =
                        worksheet.get_cell((&col_idx, &global::TABLE_DATA_ROW_TYPE))
                    {
                        utils::convert_data_type(&cell_type_data.get_value())
                    } else {
                        EDataType::STRING
                    };
                }

                if let Some(cell) = worksheet.get_cell((&col_idx, &row_idx)) {
                    let value: Cow<'static, str> = cell.get_value();
                    let style: &Style = cell.get_style();
                    let bc: Option<&Color> = style.get_background_color();
                    let fc: Option<&Color> = if let Some(font) = style.get_font() {
                        Some(font.get_color())
                    } else {
                        None
                    };
                    let cell_data: CellData = if value.is_empty() {
                        CellData::new(row_idx, col_idx as u16, value.to_string(), bc, fc)
                    } else {
                        match cell_type {
                            EDataType::PERMILLAGE => {
                                let permillage_value: f64 = value.parse::<f64>().unwrap() / 1000.0;
                                CellData::new(
                                    row_idx,
                                    col_idx as u16,
                                    format!("{:.3}", permillage_value),
                                    bc,
                                    fc,
                                )
                            }
                            EDataType::PERMIAN => {
                                let permian_value: f64 = value.parse::<f64>().unwrap() / 10000.0;
                                CellData::new(
                                    row_idx,
                                    col_idx as u16,
                                    format!("{:.4}", permian_value),
                                    bc,
                                    fc,
                                )
                            }
                            EDataType::TIME => {
                                let seconds = if value.is_empty() {
                                    0
                                } else {
                                    // excel时间格式的单元格单位是天
                                    match value.parse::<f64>() {
                                        Ok(decimal_time) => {
                                            let total_seconds =
                                                (decimal_time * 86400.0).round() as u32;
                                            total_seconds
                                        }
                                        Err(_) => 0,
                                    }
                                };
                                CellData::new(row_idx, col_idx as u16, seconds.to_string(), bc, fc)
                            }
                            EDataType::DATE => {
                                let seconds: u64 = if value.is_empty() {
                                    0
                                } else {
                                    match value.parse::<f64>() {
                                        Ok(decimal_seconds) => {
                                            // 将Excel/WPS的日期序列号转换为秒基准日期：1900年1月0日（Excel/WPS的起始点）
                                            let days: i64 = decimal_seconds.floor() as i64;
                                            let fraction = decimal_seconds - days as f64;
                                            // log::info!("[excel_util] days: {}", days);
                                            // log::info!("[excel_util] fraction: {}", fraction);
                                            let total_seconds: i64 = ((days - 1) * 86400)
                                                + (fraction * 86400.0).round() as i64;
                                            // log::info!(
                                            //     "[excel_util] total_seconds: {}",
                                            //     total_seconds
                                            // );
                                            total_seconds as u64
                                        }
                                        Err(_) => 0,
                                    }
                                };
                                CellData::new(row_idx, col_idx as u16, seconds.to_string(), bc, fc)
                            }
                            EDataType::ENUM => {
                                CellData::new(row_idx, col_idx as u16, value.to_string(), bc, fc)
                            }
                            _ => CellData::new(row_idx, col_idx as u16, value.to_string(), bc, fc),
                        }
                    };

                    if cell_data.is_empty() {
                        continue;
                    }
                    row_data.insert(col_idx as u16, cell_data);
                }
            }

            match sheet_type {
                ESheetType::KV => {
                    if row_idx < global::TABLE_KV_ROW_TOTAL {
                        gable_data.heads.insert(row_idx, row_data);
                    } else {
                        gable_data.cells.insert(row_idx, row_data);
                    }
                }
                ESheetType::ENUM => {
                    if row_idx < global::TABLE_ENUM_ROW_TOTAL {
                        gable_data.heads.insert(row_idx, row_data);
                    } else {
                        gable_data.cells.insert(row_idx, row_data);
                    }
                }
                _ => {
                    if row_idx < global::TABLE_DATA_ROW_TOTAL {
                        gable_data.heads.insert(row_idx, row_data);
                    } else {
                        gable_data.cells.insert(row_idx, row_data);
                    }
                }
            }
        }
        let gable_file_path: PathBuf =
            PathBuf::from(&target_path).join(format!("{}@{}.gable", file_stem, &sheet_name));
        gable_file_paths.push(gable_file_path.to_string_lossy().to_string());
        let json_data: String = serde_json::to_string_pretty(&gable_data)?;
        fs::write(&gable_file_path, json_data)?;
    }

    Ok(gable_file_paths)
}
