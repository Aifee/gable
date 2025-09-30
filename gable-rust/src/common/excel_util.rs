use crate::{
    common::{constant, setting, utils},
    gui::{
        datas::{
            cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType,
            gable_data::GableData, gables,
        },
        gable_app::GableApp,
    },
};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use umya_spreadsheet::{
    Border, Cell, Color, DataValidation, DataValidationValues, DataValidations, PatternValues,
    Spreadsheet, Style, Worksheet, reader, writer,
};

/**
 * 读取并解析gable文件
 * @param file_path gable文件路径
 * @return gable数据
*/
pub fn read_gable_file(file_path: &str) -> Option<GableData> {
    match fs::read_to_string(file_path) {
        Ok(content) => match serde_json::from_str::<GableData>(&content) {
            Ok(json_value) => Some(json_value),
            Err(e) => {
                log::error!("Failed to parse JSON file: '{}' : {}", file_path, e);
                None
            }
        },
        Err(e) => {
            log::error!("Failed to read file: '{}' : {}", file_path, e);
            None
        }
    }
}

/**
 * 写入Excel文件
 * @param excel_name 文件名
 * @param sheet_type 表单类型
 * @param gable_files 数据文件列表
 * @return 是否成功
*/
pub fn write_excel(
    excel_name: &str,
    sheet_type: &ESheetType,
    gable_files: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let file_name: &str = &format!("{}{}", &excel_name, &constant::EXCEL_EXTENSION);
    let tem_path: PathBuf = setting::get_temp_path();
    let excel_file_path_tem: String = utils::get_temp_path(&tem_path, &file_name);
    let excel_file_path: String = PathBuf::from(&tem_path)
        .join(&file_name)
        .to_string_lossy()
        .to_string();
    if Path::new(&excel_file_path_tem).exists() {
        log::error!(
            "Excel file '{}' is already open, unable to write",
            excel_file_path
        );
        return Err("Excel file is already open, unable to write".into());
    }
    if Path::new(&excel_file_path).exists() {
        match fs::remove_file(&excel_file_path) {
            Ok(_) => {}
            Err(e) => {
                log::error!(
                    "Unable to delete existing Excel file '{}': {}",
                    excel_file_path,
                    e
                );
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
                log::error!("Unable to delete worksheet");
            }
        }
    }
    for file_path in gable_files.iter() {
        if let Some(gable_data) = read_gable_file(file_path) {
            let filename = Path::new(file_path)
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let (_, sheet_name) =
                if let Some((excel_name, sheet_name)) = utils::parse_gable_filename(filename) {
                    (excel_name, sheet_name.unwrap_or_default())
                } else {
                    log::error!("Unable to parse filename: {}", filename);
                    continue;
                };
            let worksheet: &mut Worksheet = match workbook.new_sheet(&sheet_name) {
                Ok(sheet) => sheet,
                Err(e) => {
                    log::error!("Unable to add worksheet to Excel file: {}", e);
                    continue;
                }
            };
            let max_row: usize = match sheet_type {
                ESheetType::Normal => constant::TABLE_NORMAL_ROW_TOTAL,
                ESheetType::Localize => constant::TABLE_LOCALIZE_ROW_TOTAL,
                ESheetType::KV => constant::TABLE_KV_ROW_TOTAL,
                ESheetType::Enum => constant::TABLE_ENUM_ROW_TOTAL,
            };
            // 预先设置单元格格式，百分率，千分率，万分率，时间，枚举类型的单元格，如果按照数据填充的话有可能会设置不到
            // 但又不能全量遍历所有的单元格，故此只针对这几种类型单独设置单元格格式
            for row_index in 0..max_row {
                if let Some(row_data) = gable_data.heads.get(row_index) {
                    for col_index in 0..row_data.len() {
                        if let Some(cell_data) = row_data.get(col_index) {
                            let sheet_row: u32 = (row_index + 1) as u32;
                            let sheet_col: u32 = (col_index + 1) as u32;
                            let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
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
                }
            }
            match sheet_type {
                ESheetType::Normal => write_excel_normal(worksheet, &gable_data),
                ESheetType::Localize => write_excel_localize(worksheet, &gable_data),
                ESheetType::KV => write_excel_kv(worksheet, &gable_data),
                ESheetType::Enum => write_excel_enum(worksheet, &gable_data),
            }
        } else {
            log::error!("Unable to read or parse file: {}", file_path);
        }
    }
    match writer::xlsx::write(&workbook, &excel_file_path) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to write to Excel file '{}': {}", excel_file_path, e);
            return Err(e.into());
        }
    };
    Ok(excel_file_path)
}

/**
 * 普通表单写入Excel文件
 * @param worksheet 工作表
 * @param gable_data Gable数据
*/
fn write_excel_normal(worksheet: &mut Worksheet, gable_data: &GableData) {
    let max_row: usize = gable_data.get_max_row();
    let max_col: usize = gable_data.get_max_col();

    // 数据类型下拉框
    let range: String = utils::cell_range(
        &(constant::TABLE_NORMAL_ROW_TYPE + 1),
        &1,
        &(constant::TABLE_NORMAL_ROW_TYPE + 1),
        &(max_col + 1),
    );
    let mut data_validation: DataValidation = DataValidation::default();
    data_validation.set_formula1(format!("\"{}\"", constant::DATA_TYPE_KEYS.join(",")));
    data_validation.set_type(DataValidationValues::List);
    data_validation
        .get_sequence_of_references_mut()
        .set_sqref(range);
    let mut data_validations = DataValidations::default();
    data_validations.add_data_validation_list(data_validation);

    let mut enum_cells: BTreeMap<usize, &String> = BTreeMap::new();
    let mut loc_cells: BTreeMap<usize, &String> = BTreeMap::new();
    // 数据类型数据
    for col_index in 0..max_col {
        let cell_type_data: Option<&CellData> = gable_data
            .heads
            .get(constant::TABLE_NORMAL_ROW_TYPE)
            .and_then(|row| row.get(col_index));
        let cell_type: EDataType = if let Some(data) = cell_type_data {
            EDataType::convert(&data.value)
        } else {
            EDataType::String
        };
        if cell_type != EDataType::Percentage
            && cell_type != EDataType::Permillage
            && cell_type != EDataType::Permian
            && cell_type != EDataType::Time
            && cell_type != EDataType::Date
            && cell_type != EDataType::Enum
            && cell_type != EDataType::Loc
        {
            continue;
        }

        // 枚举单独设置
        if cell_type == EDataType::Enum {
            let cell_link_data: Option<&CellData> = gable_data
                .heads
                .get(constant::TABLE_NORMAL_ROW_LINK)
                .and_then(|row| row.get(col_index));
            if let Some(cell_link_data) = cell_link_data {
                gables::get_enum_cells(&cell_link_data.value, |enum_datas| {
                    let mut formula_vec: Vec<String> = Vec::new();
                    for r_d in enum_datas.cells.iter() {
                        if let Some(r_c) = r_d.get(constant::TABLE_ENUM_COL_DESC) {
                            if !r_c.value.is_empty() {
                                formula_vec.push(r_c.value.clone());
                            }
                        }
                    }
                    let range: String = utils::cell_range(
                        &(constant::TABLE_NORMAL_ROW_TOTAL + 1),
                        &(col_index + 1),
                        &(max_col + 1),
                        &(col_index + 1),
                    );
                    let mut enum_validation: DataValidation = DataValidation::default();
                    enum_validation.set_formula1(format!("\"{}\"", formula_vec.join(",")));
                    enum_validation.set_type(DataValidationValues::List);
                    enum_validation
                        .get_sequence_of_references_mut()
                        .set_sqref(range);
                    data_validations.add_data_validation_list(enum_validation);
                });
                enum_cells.insert(col_index, &cell_link_data.value);
            }
        }

        // 多语言单独设置
        if cell_type == EDataType::Loc {
            let cell_link_data: Option<&CellData> = gable_data
                .heads
                .get(constant::TABLE_NORMAL_ROW_LINK)
                .and_then(|row| row.get(col_index));
            if let Some(cell_link_data) = cell_link_data {
                loc_cells.insert(col_index, &cell_link_data.value);
            }
        }
        for row_index in constant::TABLE_NORMAL_ROW_TOTAL..max_row {
            let sheet_row: u32 = (row_index + 1) as u32;
            let sheet_col: u32 = (col_index + 1) as u32;
            let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
            match cell_type {
                EDataType::Percentage => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_PERCENTAGE);
                }
                EDataType::Permillage => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_PERMILLAGE);
                }
                EDataType::Permian => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_PERMIAN);
                }
                EDataType::Time => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_TIME);
                }
                EDataType::Date => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_DATE);
                }
                _ => {}
            }
        }
    }

    // 数据验证填充
    worksheet.set_data_validations(data_validations);

    // 数据内容处理
    for row_index in 0..gable_data.cells.len() {
        if let Some(row_data) = gable_data.cells.get(row_index) {
            for col_index in 0..row_data.len() {
                if let Some(cell_data) = row_data.get(col_index) {
                    let sheet_row: u32 = (row_index + 1 + constant::TABLE_NORMAL_ROW_TOTAL) as u32;
                    let sheet_col: u32 = (col_index + 1) as u32;
                    let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
                    if let Some(row_data) = &gable_data.heads.get(constant::TABLE_NORMAL_ROW_TYPE) {
                        if let Some(cell_type_data) = row_data.get(col_index) {
                            match EDataType::convert(&cell_type_data.value) {
                                EDataType::Int => {
                                    cell.set_value_number(cell_data.parse_int() as f64)
                                }
                                EDataType::Boolean => cell.set_value_bool(cell_data.parse_bool()),
                                EDataType::Float => cell.set_value_number(cell_data.parse_float()),
                                EDataType::Percentage => {
                                    cell.set_value_number(cell_data.parse_float())
                                }
                                EDataType::Permillage => {
                                    cell.set_value_number(cell_data.parse_float() * 1000.0)
                                }
                                EDataType::Permian => {
                                    cell.set_value_number(cell_data.parse_float() * 10000.0)
                                }
                                EDataType::Time => cell.set_value_number(cell_data.parse_time()),
                                EDataType::Date => cell.set_value_number(cell_data.parse_date()),
                                EDataType::Enum => {
                                    let mut cell_value = cell_data.value.clone();
                                    if let Some(enum_item_key) = enum_cells.get(&col_index) {
                                        gables::get_enum_cells(enum_item_key, |enum_item_cells| {
                                            for enum_row_cell in enum_item_cells.cells.iter() {
                                                if let Some(enum_value_cell) = enum_row_cell
                                                    .get(constant::TABLE_ENUM_COL_VALUE)
                                                {
                                                    if enum_value_cell.value == cell_data.value {
                                                        if let Some(enum_desc_cell) = enum_row_cell
                                                            .get(constant::TABLE_ENUM_COL_DESC)
                                                        {
                                                            cell_value =
                                                                enum_desc_cell.value.clone();
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    cell.set_value(cell_value)
                                }
                                EDataType::Loc => {
                                    let mut cell_value = cell_data.value.clone();
                                    if let Some(loc_item_key) = loc_cells.get(&col_index) {
                                        gables::get_loc_cells(loc_item_key, |loc_item_cells| {
                                            let mut link_key_index: usize = 0;
                                            let mut link_value_index: usize = 0;
                                            if let Some(link_key_cell) = loc_item_cells
                                                .heads
                                                .get(constant::TABLE_LOCALIZE_ROW_FIELD)
                                            {
                                                for link_index in 0..link_key_cell.len() {
                                                    if let Some(col_cell) =
                                                        link_key_cell.get(link_index)
                                                    {
                                                        if col_cell.value.contains("*") {
                                                            link_key_index = col_index;
                                                        }
                                                        if col_cell.value.contains("#") {
                                                            link_value_index = col_index;
                                                        }
                                                    }
                                                }
                                            }

                                            for loc_row_cell in loc_item_cells.cells.iter() {
                                                if let Some(loc_value_cell) =
                                                    loc_row_cell.get(link_key_index)
                                                {
                                                    if loc_value_cell.value == cell_data.value {
                                                        if let Some(loc_desc_cell) =
                                                            loc_row_cell.get(link_value_index)
                                                        {
                                                            cell_value =
                                                                loc_desc_cell.value.clone();
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    cell.set_value(cell_value)
                                }
                                _ => cell.set_value(&cell_data.value),
                            };
                        }
                    }

                    write_excel_cell_style(cell, &cell_data);
                }
            }
        }
    }
}

/**
 * 本地化表单写入Excel文件
 * @param worksheet 工作表
 * @param gable_data Gable数据
*/
fn write_excel_localize(worksheet: &mut Worksheet, gable_data: &GableData) {
    let max_col = gable_data.get_max_col();
    // 数据类型下拉框
    let range: String = utils::cell_range(
        &(constant::TABLE_LOCALIZE_ROW_TYPE + 1),
        &1,
        &(constant::TABLE_LOCALIZE_ROW_TYPE + 1),
        &(max_col + 1),
    );
    let mut data_validation: DataValidation = DataValidation::default();
    data_validation.set_formula1(format!(
        "\"{}\"",
        vec![constant::DATA_TYPE_KEY_STRING].join(",")
    ));
    data_validation.set_type(DataValidationValues::List);
    data_validation
        .get_sequence_of_references_mut()
        .set_sqref(range);
    let mut data_validations = DataValidations::default();
    data_validations.add_data_validation_list(data_validation);
    // 数据验证填充
    worksheet.set_data_validations(data_validations);

    // 数据内容处理
    for row_index in 0..gable_data.cells.len() {
        if let Some(row_data) = gable_data.cells.get(row_index) {
            for col_index in 0..row_data.len() {
                if let Some(cell_data) = row_data.get(col_index) {
                    if cell_data.is_empty() {
                        continue;
                    }
                    let sheet_row = (row_index + 1 + constant::TABLE_LOCALIZE_ROW_TOTAL) as u32;
                    let sheet_col = (col_index + 1) as u32;
                    let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
                    cell.set_value(&cell_data.value);
                    write_excel_cell_style(cell, &cell_data);
                }
            }
        }
    }
}

/**
 * KV表单写入Excel文件
 * @param worksheet 工作表
 * @param gable_data Gable数据
*/
fn write_excel_kv(worksheet: &mut Worksheet, gable_data: &GableData) {
    let max_row = gable_data.get_max_row() + 1;

    // 数据类型下拉框
    let range: String = utils::cell_range(
        &(constant::TABLE_KV_ROW_TOTAL + 1),
        &(constant::TABLE_KV_COL_TYPE + 1),
        &(max_row + 1),
        &(constant::TABLE_KV_COL_TYPE + 1),
    );
    let mut data_validation: DataValidation = DataValidation::default();
    data_validation.set_formula1(format!("\"{}\"", constant::DATA_TYPE_KEYS.join(",")));
    data_validation.set_type(DataValidationValues::List);
    data_validation
        .get_sequence_of_references_mut()
        .set_sqref(range);
    let mut data_validations = DataValidations::default();
    data_validations.add_data_validation_list(data_validation);

    let mut enum_cell_links: BTreeMap<usize, &String> = BTreeMap::new();
    let mut loc_cell_links: BTreeMap<usize, &String> = BTreeMap::new();
    // 数据类型处理
    for row_index in constant::TABLE_KV_ROW_TOTAL..max_row {
        if let Some(cell_type_data) = gable_data
            .cells
            .get(row_index)
            .and_then(|row| row.get(constant::TABLE_KV_COL_TYPE))
        {
            let cell_type_value = &cell_type_data.value;
            let cell_type: EDataType = EDataType::convert(&cell_type_value);
            let sheet_row = (row_index + 1) as u32;
            let sheet_col = (constant::TABLE_KV_COL_VALUE + 1) as u32;
            let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
            match cell_type {
                EDataType::Percentage => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_PERCENTAGE);
                }
                EDataType::Permillage => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_PERMILLAGE);
                }
                EDataType::Permian => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_PERMIAN);
                }
                EDataType::Time => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_TIME);
                }
                EDataType::Date => {
                    cell.get_style_mut()
                        .get_number_format_mut()
                        .set_format_code(constant::NUMBER_FORMAT_DATE);
                }
                EDataType::Enum => {
                    let cell_link_data: Option<&CellData> = gable_data
                        .cells
                        .get(row_index)
                        .and_then(|row| row.get(constant::TABLE_KV_COL_LINK));
                    if let Some(cell_link_data) = cell_link_data {
                        gables::get_enum_cells(&cell_link_data.value, |cell_gable| {
                            let mut formula_vec = Vec::new();
                            for r_d in cell_gable.cells.iter() {
                                if let Some(r_c) = r_d.get(constant::TABLE_ENUM_COL_DESC) {
                                    if !r_c.value.is_empty() {
                                        formula_vec.push(r_c.value.clone());
                                    }
                                }
                            }

                            let range: String = utils::cell_range(
                                &(row_index + 1),
                                &(constant::TABLE_KV_COL_VALUE + 1),
                                &(row_index + 1),
                                &(constant::TABLE_KV_COL_VALUE + 1),
                            );
                            let mut enum_validation = DataValidation::default();
                            enum_validation.set_formula1(format!("\"{}\"", formula_vec.join(",")));
                            enum_validation.set_type(DataValidationValues::List);
                            enum_validation
                                .get_sequence_of_references_mut()
                                .set_sqref(range);
                            data_validations.add_data_validation_list(enum_validation);
                        });
                        enum_cell_links.insert(row_index, &cell_link_data.value);
                    }
                }
                EDataType::Loc => {
                    let cell_link_data: Option<&CellData> = gable_data
                        .cells
                        .get(row_index)
                        .and_then(|row| row.get(constant::TABLE_KV_COL_LINK));
                    if let Some(cell_link_data) = cell_link_data {
                        loc_cell_links.insert(row_index, &cell_link_data.value);
                    }
                }
                _ => {}
            }
        }
    }
    // 数据验证填充
    worksheet.set_data_validations(data_validations);
    let mut cell_type_data_temp: Option<&CellData> = None;
    // 数据内容处理
    for row_index in 0..gable_data.cells.len() {
        if let Some(row_data) = gable_data.cells.get(row_index) {
            for col_index in 0..row_data.len() {
                if let Some(cell_data) = row_data.get(col_index) {
                    let sheet_row = (row_index + 1 + constant::TABLE_KV_ROW_TOTAL) as u32;
                    let sheet_col = (col_index + 1) as u32;
                    let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
                    if col_index == constant::TABLE_KV_COL_TYPE {
                        cell_type_data_temp = Some(cell_data);
                    }
                    if col_index == constant::TABLE_KV_COL_VALUE {
                        if let Some(cell_type_data) = cell_type_data_temp {
                            match EDataType::convert(&cell_type_data.value) {
                                EDataType::Int => {
                                    cell.set_value_number(cell_data.parse_int() as f64)
                                }
                                EDataType::Boolean => cell.set_value_bool(cell_data.parse_bool()),
                                EDataType::Float => cell.set_value_number(cell_data.parse_float()),
                                EDataType::Percentage => {
                                    cell.set_value_number(cell_data.parse_float())
                                }
                                EDataType::Permillage => {
                                    cell.set_value_number(cell_data.parse_float() * 1000.0)
                                }
                                EDataType::Permian => {
                                    cell.set_value_number(cell_data.parse_float() * 10000.0)
                                }
                                EDataType::Time => cell.set_value_number(cell_data.parse_time()),
                                EDataType::Date => cell.set_value_number(cell_data.parse_date()),
                                EDataType::Enum => {
                                    let mut cell_value: String = cell_data.value.clone();
                                    if let Some(link_name) = enum_cell_links.get(&row_index) {
                                        gables::get_enum_cells(link_name, |link_cell| {
                                            for enum_row_cell in link_cell.cells.iter() {
                                                if let Some(enum_value_cell) = enum_row_cell
                                                    .get(constant::TABLE_ENUM_COL_VALUE)
                                                {
                                                    if enum_value_cell.value == cell_data.value {
                                                        if let Some(enum_desc_cell) = enum_row_cell
                                                            .get(constant::TABLE_ENUM_COL_DESC)
                                                        {
                                                            cell_value =
                                                                enum_desc_cell.value.clone();
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    cell.set_value(cell_value)
                                }
                                EDataType::Loc => {
                                    let mut cell_value = cell_data.value.clone();
                                    if let Some(link_name) = loc_cell_links.get(&row_index) {
                                        gables::get_loc_cells(link_name, |loc_item_cells| {
                                            let mut link_key_index: usize = 0;
                                            let mut link_value_index: usize = 0;
                                            if let Some(link_key_cell) = loc_item_cells
                                                .heads
                                                .get(constant::TABLE_LOCALIZE_ROW_FIELD)
                                            {
                                                for link_index in 0..link_key_cell.len() {
                                                    if let Some(col_cell) =
                                                        link_key_cell.get(link_index)
                                                    {
                                                        if col_cell.value.contains("*") {
                                                            link_key_index = col_index;
                                                        }
                                                        if col_cell.value.contains("#") {
                                                            link_value_index = col_index;
                                                        }
                                                    }
                                                }
                                            }

                                            for loc_row_cell in loc_item_cells.cells.iter() {
                                                if let Some(loc_value_cell) =
                                                    loc_row_cell.get(link_key_index)
                                                {
                                                    if loc_value_cell.value == cell_data.value {
                                                        if let Some(loc_desc_cell) =
                                                            loc_row_cell.get(link_value_index)
                                                        {
                                                            cell_value =
                                                                loc_desc_cell.value.clone();
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    cell.set_value(cell_value)
                                }
                                _ => cell.set_value(&cell_data.value),
                            };
                        }
                    } else {
                        cell.set_value(&cell_data.value);
                    }
                    write_excel_cell_style(cell, &cell_data);
                }
            }
        }
    }
}

/**
 * 枚举表单写入Excel文件
 * @param worksheet 工作表
 * @param gable_data Gable数据
*/
fn write_excel_enum(worksheet: &mut Worksheet, gable_data: &GableData) {
    // 数据内容处理
    for row_index in 0..gable_data.cells.len() {
        if let Some(row_data) = gable_data.cells.get(row_index) {
            for col_index in 0..row_data.len() {
                if let Some(cell_data) = row_data.get(col_index) {
                    let sheet_row = (row_index + 1 + constant::TABLE_ENUM_ROW_TOTAL) as u32;
                    let sheet_col = (col_index + 1) as u32;
                    let cell: &mut Cell = worksheet.get_cell_mut((sheet_col, sheet_row));
                    cell.set_value(&cell_data.value);
                    write_excel_cell_style(cell, &cell_data);
                }
            }
        }
    }
}

/**
 * excel 单元格数据类型写入
 * @param cell 单元格
 * @param cell_data 单元格数据
*/
fn write_excel_cell_style(cell: &mut Cell, cell_data: &CellData) {
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

/**
 * 新建gable文件
 * @param gable_path 文件路径
 * */
pub fn write_gable_new(gable_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if gable_path.exists() {
        return Err("Gable file already exists".into());
    }
    let sheet_type: ESheetType = setting::determine_sheet_type(Path::new(&gable_path));
    let gable_data: GableData = GableData::new(sheet_type);
    let json_data: String = serde_json::to_string_pretty(&gable_data)?;
    fs::write(&gable_path, json_data)?;
    Ok(())
}

/**
 * Excel数据写入gable文件
 * @param excel_file: Excel文件路径
 * @param target_path: gable文件所在目录
 * @param sheet_type: 表类型
 * @return: Result<(), String>
 * */
pub fn write_gable(
    excel_file: &PathBuf,
    target_path: &str,
    sheet_type: &ESheetType,
) -> Result<Vec<String>, Box<dyn Error>> {
    let workbook: Spreadsheet = reader::xlsx::read(excel_file).unwrap();
    let sheet_counts: usize = workbook.get_sheet_count();
    let file_path: &Path = Path::new(excel_file);
    let file_stem: &str = file_path.file_stem().unwrap().to_str().unwrap();
    let mut gable_file_paths: Vec<String> = Vec::new();

    for sheet_index in 0..sheet_counts {
        let worksheet: &Worksheet = if let Some(sheet) = workbook.get_sheet(&sheet_index) {
            sheet
        } else {
            log::error!("Unable to obtain the worksheet index: {}", sheet_index);
            continue;
        };
        let sheet_name: String = worksheet.get_name().to_string();
        let (max_col, max_row) = worksheet.get_highest_column_and_row();
        let mut gable_data: GableData = GableData {
            heads: Vec::new(),
            cells: Vec::new(),
        };

        let max_row: usize = max_row as usize;
        let max_col: usize = max_col as usize;
        match sheet_type {
            ESheetType::Normal => write_gable_normal(worksheet, &mut gable_data, max_row, max_col),
            ESheetType::Localize => {
                write_gable_localize(worksheet, &mut gable_data, max_row, max_col)
            }
            ESheetType::KV => write_gable_kv(worksheet, &mut gable_data, max_row, max_col),
            ESheetType::Enum => write_gable_enum(worksheet, &mut gable_data, max_row, max_col),
        }
        let gable_file_path: PathBuf =
            PathBuf::from(&target_path).join(format!("{}@{}.gable", file_stem, &sheet_name));
        gable_file_paths.push(gable_file_path.to_string_lossy().to_string());
        let json_data: String = serde_json::to_string_pretty(&gable_data)?;
        fs::write(&gable_file_path, json_data)?;
    }

    Ok(gable_file_paths)
}

/**
 * 将excel以普通表单写入gable文件
 * @param worksheet 工作表
 * @param gable_data gable数据
 * @param max_row 最大行数
 * @param max_col 最大列数
*/
fn write_gable_normal(
    worksheet: &Worksheet,
    gable_data: &mut GableData,
    max_row: usize,
    max_col: usize,
) {
    // 收集所有enum的link信息
    let mut links: BTreeMap<usize, String> = BTreeMap::new();
    if max_row >= constant::TABLE_NORMAL_ROW_TOTAL {
        for col_idx in 0..max_col {
            let sheet_row = (constant::TABLE_NORMAL_ROW_LINK + 1) as u32;
            let sheet_col = (col_idx + 1) as u32;
            if let Some(cell_link_cell) = worksheet.get_cell((sheet_col, sheet_row)) {
                links.insert(col_idx, cell_link_cell.get_value().to_string());
            }
        }
    }

    for row_idx in 0..max_row {
        let mut row_data: Vec<CellData> = Vec::new();
        let mut cell_type: EDataType = EDataType::String;
        for col_idx in 0..max_col {
            if row_idx >= constant::TABLE_NORMAL_ROW_TOTAL {
                let sheet_row = (constant::TABLE_NORMAL_ROW_TYPE + 1) as u32;
                let sheet_col = (col_idx + 1) as u32;
                cell_type = if let Some(cell_type_data) = worksheet.get_cell((sheet_col, sheet_row))
                {
                    EDataType::convert(&cell_type_data.get_value())
                } else {
                    EDataType::String
                };
            }
            let sheet_row = (row_idx + 1) as u32;
            let sheet_col = (col_idx + 1) as u32;
            if let Some(cell) = worksheet.get_cell((sheet_col, sheet_row)) {
                let value: Cow<'static, str> = cell.get_value();
                let style: &Style = cell.get_style();
                let bc: Option<&Color> = style.get_background_color();
                let fc: Option<&Color> = if let Some(font) = style.get_font() {
                    Some(font.get_color())
                } else {
                    None
                };
                let cell_value: String = if value.is_empty() {
                    String::new()
                } else {
                    match cell_type {
                        EDataType::Permillage => {
                            let permillage_value: f64 = value.parse::<f64>().unwrap() / 1000.0;
                            format!("{:.3}", permillage_value)
                        }
                        EDataType::Permian => {
                            let permian_value: f64 = value.parse::<f64>().unwrap() / 10000.0;
                            format!("{:.4}", permian_value)
                        }
                        EDataType::Time => match value.parse::<f64>() {
                            Ok(decimal_time) => {
                                let total_seconds = (decimal_time * 86400.0).round() as u32;
                                total_seconds.to_string()
                            }
                            Err(_) => String::new(),
                        },
                        EDataType::Date => {
                            match value.parse::<f64>() {
                                Ok(decimal_seconds) => {
                                    // 将Excel/WPS的日期序列号转换为秒基准日期：1900年1月0日（Excel/WPS的起始点）
                                    let days: i64 = decimal_seconds.floor() as i64;
                                    let fraction = decimal_seconds - days as f64;
                                    let total_seconds: i64 =
                                        ((days - 1) * 86400) + (fraction * 86400.0).round() as i64;
                                    total_seconds.to_string()
                                }
                                Err(_) => String::new(),
                            }
                        }
                        EDataType::Enum => {
                            let mut cell_value: String = value.to_string();
                            if let Some(link_name) = links.get(&col_idx) {
                                gables::get_enum_cells(link_name, |link_cell| {
                                    for data_row in link_cell.cells.iter() {
                                        if let Some(data) =
                                            data_row.get(constant::TABLE_ENUM_COL_DESC)
                                        {
                                            if data.value == value {
                                                if let Some(value_data) =
                                                    data_row.get(constant::TABLE_ENUM_COL_VALUE)
                                                {
                                                    cell_value = value_data.value.clone();
                                                }
                                                break;
                                            }
                                        }
                                    }
                                });
                            };
                            cell_value
                        }
                        EDataType::Loc => {
                            let mut cell_value: String = value.to_string();
                            if let Some(link_name) = links.get(&col_idx) {
                                gables::get_loc_cells(link_name, |loc_item_cells| {
                                    let mut link_key_index: usize = 0;
                                    let mut link_value_index: usize = 0;
                                    if let Some(link_key_cell) =
                                        loc_item_cells.heads.get(constant::TABLE_LOCALIZE_ROW_FIELD)
                                    {
                                        for link_index in 0..link_key_cell.len() {
                                            if let Some(col_cell) = link_key_cell.get(link_index) {
                                                if col_cell.value.contains("*") {
                                                    link_key_index = link_index;
                                                }
                                                if col_cell.value.contains("#") {
                                                    link_value_index = link_index;
                                                }
                                            }
                                        }
                                    }

                                    for loc_row_cell in loc_item_cells.cells.iter() {
                                        if let Some(loc_desc_cell) =
                                            loc_row_cell.get(link_value_index)
                                        {
                                            if loc_desc_cell.value == value {
                                                if let Some(loc_key_cell) =
                                                    loc_row_cell.get(link_key_index)
                                                {
                                                    cell_value = loc_key_cell.value.clone();
                                                }
                                                break;
                                            }
                                        }
                                    }
                                });
                            };
                            cell_value
                        }
                        _ => value.to_string(),
                    }
                };
                let cell_data: CellData = CellData::new(cell_value, bc, fc);
                row_data.push(cell_data);
            } else {
                let cell_data: CellData = CellData::new(String::new(), None, None);
                row_data.push(cell_data);
            }
        }
        if row_idx < constant::TABLE_NORMAL_ROW_TOTAL {
            gable_data.heads.push(row_data);
        } else {
            gable_data.cells.push(row_data);
        }
    }
}

/**
 * 将excel以本地化表单写入gable文件
 * @param worksheet 工作表
 * @param gable_data gable数据
 * @param max_row 最大行数
 * @param max_col 最大列数
*/
fn write_gable_localize(
    worksheet: &Worksheet,
    gable_data: &mut GableData,
    max_row: usize,
    max_col: usize,
) {
    for row_idx in 0..max_row {
        let mut row_data: Vec<CellData> = Vec::new();
        for col_idx in 0..max_col {
            let sheet_row = (row_idx + 1) as u32;
            let sheet_col = (col_idx + 1) as u32;
            if let Some(cell) = worksheet.get_cell((sheet_col, sheet_row)) {
                let value: Cow<'static, str> = cell.get_value();
                let style: &Style = cell.get_style();
                let bc: Option<&Color> = style.get_background_color();
                let fc: Option<&Color> = if let Some(font) = style.get_font() {
                    Some(font.get_color())
                } else {
                    None
                };
                let cell_data: CellData = CellData::new(value.to_string(), bc, fc);
                row_data.push(cell_data);
            } else {
                let cell_data: CellData = CellData::new(String::new(), None, None);
                row_data.push(cell_data);
            }
        }
        if row_idx < constant::TABLE_LOCALIZE_ROW_TOTAL {
            gable_data.heads.push(row_data);
        } else {
            gable_data.cells.push(row_data);
        }
    }
}

/**
 * 将excel以KV表单写入gable文件
 * @param worksheet 工作表
 * @param gable_data gable数据
 * @param max_row 最大行数
 * @param max_col 最大列数
*/
fn write_gable_kv(
    worksheet: &Worksheet,
    gable_data: &mut GableData,
    max_row: usize,
    max_col: usize,
) {
    // 读取数据并填充到GableData中
    for row_idx in 0..max_row {
        let mut row_data: Vec<CellData> = Vec::new();
        let cell_type: EDataType = if row_idx >= constant::TABLE_KV_ROW_TOTAL {
            let sheet_row = (row_idx + 1) as u32;
            let sheet_col = (constant::TABLE_KV_COL_TYPE + 1) as u32;
            if let Some(cell_type_data) = worksheet.get_cell((sheet_col, sheet_row)) {
                EDataType::convert(&cell_type_data.get_value())
            } else {
                EDataType::String
            }
        } else {
            EDataType::String
        };
        let mut link_name: Option<String> = None;
        for col_idx in 0..max_col {
            let sheet_row = (row_idx + 1) as u32;
            let sheet_col = (col_idx + 1) as u32;
            if let Some(cell) = worksheet.get_cell((sheet_col, sheet_row)) {
                let value: Cow<'static, str> = cell.get_value();
                let style: &Style = cell.get_style();
                let bc: Option<&Color> = style.get_background_color();
                let fc: Option<&Color> = if let Some(font) = style.get_font() {
                    Some(font.get_color())
                } else {
                    None
                };
                if col_idx == constant::TABLE_KV_COL_LINK {
                    if !value.is_empty() {
                        link_name = Some(value.to_string());
                    }
                }
                if col_idx == constant::TABLE_KV_COL_VALUE {
                    let cell_value: String = if value.is_empty() {
                        String::new()
                    } else {
                        match cell_type {
                            EDataType::Permillage => {
                                let permillage_value: f64 = value.parse::<f64>().unwrap() / 1000.0;
                                format!("{:.3}", permillage_value)
                            }
                            EDataType::Permian => {
                                let permian_value: f64 = value.parse::<f64>().unwrap() / 10000.0;
                                format!("{:.4}", permian_value)
                            }
                            EDataType::Time => match value.parse::<f64>() {
                                Ok(decimal_time) => (decimal_time * 86400.0).round().to_string(),
                                Err(_) => String::new(),
                            },
                            EDataType::Date => {
                                match value.parse::<f64>() {
                                    Ok(decimal_seconds) => {
                                        // 将Excel/WPS的日期序列号转换为秒基准日期：1900年1月0日（Excel/WPS的起始点）
                                        let days: i64 = decimal_seconds.floor() as i64;
                                        let fraction = decimal_seconds - days as f64;
                                        let total_seconds: i64 = ((days - 1) * 86400)
                                            + (fraction * 86400.0).round() as i64;
                                        total_seconds.to_string()
                                    }
                                    Err(_) => String::new(),
                                }
                            }
                            EDataType::Enum => {
                                let mut cell_value: String = value.to_string();
                                if let Some(link_name) = &link_name {
                                    gables::get_enum_cells(link_name, |link_cell| {
                                        for enum_row in link_cell.cells.iter() {
                                            if let Some(enum_cell) =
                                                enum_row.get(constant::TABLE_ENUM_COL_DESC)
                                            {
                                                if enum_cell.value == value {
                                                    if let Some(value_data) =
                                                        enum_row.get(constant::TABLE_ENUM_COL_VALUE)
                                                    {
                                                        cell_value = value_data.value.clone();
                                                    }
                                                }
                                            }
                                        }
                                    });
                                }
                                cell_value
                            }
                            EDataType::Loc => {
                                let mut cell_value: String = value.to_string();
                                if let Some(link_name) = &link_name {
                                    gables::get_loc_cells(link_name, |loc_item_cells| {
                                        let mut link_key_index: usize = 0;
                                        let mut link_value_index: usize = 0;
                                        if let Some(link_key_cell) = loc_item_cells
                                            .heads
                                            .get(constant::TABLE_LOCALIZE_ROW_FIELD)
                                        {
                                            for link_index in 0..link_key_cell.len() {
                                                if let Some(col_cell) =
                                                    link_key_cell.get(link_index)
                                                {
                                                    if col_cell.value.contains("*") {
                                                        link_key_index = link_index;
                                                    }
                                                    if col_cell.value.contains("#") {
                                                        link_value_index = link_index;
                                                    }
                                                }
                                            }
                                        }

                                        for loc_row_cell in loc_item_cells.cells.iter() {
                                            if let Some(loc_desc_cell) =
                                                loc_row_cell.get(link_value_index)
                                            {
                                                if loc_desc_cell.value == value {
                                                    if let Some(loc_key_cell) =
                                                        loc_row_cell.get(link_key_index)
                                                    {
                                                        cell_value = loc_key_cell.value.clone();
                                                    }
                                                    break;
                                                }
                                            }
                                        }
                                    });
                                }
                                cell_value
                            }
                            _ => value.to_string(),
                        }
                    };
                    let cell_data: CellData = CellData::new(cell_value, bc, fc);
                    row_data.push(cell_data);
                } else {
                    if !value.is_empty() {
                        let cell_data: CellData = CellData::new(value.to_string(), bc, fc);
                        row_data.push(cell_data);
                    } else {
                        let cell_data: CellData = CellData::new(String::new(), bc, fc);
                        row_data.push(cell_data);
                    }
                }
            }
        }
        if row_idx < constant::TABLE_KV_ROW_TOTAL {
            gable_data.heads.push(row_data);
        } else {
            gable_data.cells.push(row_data);
        }
    }
}

/**
 * 将excel以枚举表单写入gable文件
 * @param worksheet 工作表
 * @param gable_data gable数据
 * @param max_row 最大行数
 * @param max_col 最大列数
*/
fn write_gable_enum(
    worksheet: &Worksheet,
    gable_data: &mut GableData,
    max_row: usize,
    max_col: usize,
) {
    // 读取数据并填充到GableData中
    for row_idx in 0..max_row {
        let mut row_data: Vec<CellData> = Vec::new();
        for col_idx in 0..max_col {
            let sheet_row = (row_idx + 1) as u32;
            let sheet_col = (col_idx + 1) as u32;
            if let Some(cell) = worksheet.get_cell((sheet_col, sheet_row)) {
                let value: Cow<'static, str> = cell.get_value();
                let style: &Style = cell.get_style();
                let bc: Option<&Color> = style.get_background_color();
                let fc: Option<&Color> = if let Some(font) = style.get_font() {
                    Some(font.get_color())
                } else {
                    None
                };

                let cell_data: CellData = CellData::new(value.to_string(), bc, fc);
                row_data.push(cell_data);
            } else {
                let cell_data: CellData = CellData::new(String::new(), None, None);
                row_data.push(cell_data);
            }
        }
        if row_idx < constant::TABLE_ENUM_ROW_TOTAL {
            gable_data.heads.push(row_data);
        } else {
            gable_data.cells.push(row_data);
        }
    }
}

/**
 * 导入Excel文件
 * @param targe_path 导入到工程目录
 * @param files 文件列表
*/
pub fn import_excels(targe_path: &str, files: Vec<PathBuf>) {
    if files.len() <= 0 {
        return;
    }
    let sheet_type: ESheetType = setting::determine_sheet_type(Path::new(targe_path));
    for file in files {
        if let Some(extension) = file.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            if ext_str != "xlsx" && ext_str != "xls" {
                // 跳过非Excel文件
                continue;
            }
        } else {
            // 跳过无扩展名的文件
            continue;
        }

        match write_gable(&file, targe_path, &sheet_type) {
            Ok(gable_file_paths) => {
                for gable_file_path in gable_file_paths {
                    log::info!("import gable file: {}", gable_file_path);
                }
            }
            Err(_) => {
                log::error!("import gable file failed: {:?}", file);
            }
        };
    }
    GableApp::refresh_command();
}
