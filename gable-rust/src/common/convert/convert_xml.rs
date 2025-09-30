use crate::{
    common::{constant, setting::BuildSetting, utils},
    gui::datas::{cell_data::CellData, esheet_type::ESheetType, tree_data::TreeData},
};
use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
};

/**
 * xml 转换
 * @param build_setting 构建设置
 * @param tree_data 树数据
 * */
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        // 枚举不导出
        return;
    }

    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.xml", tree_data.file_name));
    let xml_data: String = to_xml_data(tree_data, &build_setting.keyword);
    if xml_data.is_empty() {
        log::debug!(
            "Export [{}] skipped: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
        return;
    }
    // 创建XML文件
    let file: Result<File, Error> = File::create(&target_path);
    if file.is_err() {
        log::error!(
            "Export [{}] failed: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
        return;
    }
    let file: File = file.unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);

    // 写入XML数据
    if let Err(e) = writer.write_all(xml_data.as_bytes()) {
        log::error!("Error writing [{}] file: {}", build_setting.display_name, e);
        return;
    }

    if let Err(e) = writer.flush() {
        log::error!(
            "Error flushing [{}] file: {}",
            build_setting.display_name,
            e
        );
        return;
    }

    log::info!(
        "Export [{}] successful: {}",
        build_setting.display_name,
        target_path.to_str().unwrap()
    );
}

/**
 *  转换为xml数据
 *  @param tree_data 树数据
 *  @param keyword 关键字
 * */
fn to_xml_data(tree_data: &TreeData, keyword: &str) -> String {
    match tree_data.gable_type {
        ESheetType::Normal => normal_xml_data(tree_data, keyword),
        ESheetType::KV => kv_xml_data(tree_data, keyword),
        ESheetType::Localize => localize_xml_data(tree_data, keyword),
        _ => {
            log::error!("The enumeration table does not export as XML.");
            String::new()
        }
    }
}

/**
 * 普通表格转换为XML
 * @param tree_data 树数据
 * @param keyword 关键字
*/
fn normal_xml_data(tree_data: &TreeData, keyword: &str) -> String {
    let (valids_main, valids) = tree_data.content.get_valid_normal_heads(keyword);
    if valids_main.is_empty() || valids.is_empty() {
        return String::new();
    }
    let mut xml_content: String = String::new();

    // XML头部
    xml_content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml_content.push_str(&format!("<{}>\n", tree_data.file_name));

    let max_row: usize = tree_data.content.get_max_row() + 1;
    for row_index in constant::TABLE_NORMAL_ROW_TOTAL..=max_row {
        let real_index: usize = row_index - constant::TABLE_NORMAL_ROW_TOTAL;
        let row_data = if let Some(row_data) = tree_data.content.cells.get(real_index) {
            row_data
        } else {
            continue;
        };

        let mut row_valid: bool = true;
        // 检测行数据是否有效，主键没有数据，行数据无效则跳过
        for (col_index, _) in valids_main.iter() {
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                row_valid = false;
                continue;
            };
            if value_cell.value.is_empty() {
                row_valid = false;
                continue;
            };
        }
        // 行数据无效
        if !row_valid {
            continue;
        }

        xml_content.push_str("  <item>\n");

        // 主键字段
        for (col_index, col_data) in valids_main.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_NORMAL_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                continue;
            };
            let field_name: String = field_cell.value.replace("*", "");
            let value: String = escape_xml_value(&value_cell.value);
            xml_content.push_str(&format!("    <{}>{}</{}>\n", field_name, value, field_name));
        }

        // 其他字段
        for (col_index, col_data) in valids.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_NORMAL_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                continue;
            };
            let field_name: &String = &field_cell.value;
            let value: String = escape_xml_value(&value_cell.value);
            xml_content.push_str(&format!("    <{}>{}</{}>\n", field_name, value, field_name));
        }

        xml_content.push_str("  </item>\n");
    }

    xml_content.push_str(&format!("</{}>\n", tree_data.file_name));
    xml_content
}

/**
 * KV表格转换为XML
 * @param tree_data 树数据
 * @param keyword 关键字
*/
fn kv_xml_data(tree_data: &TreeData, keyword: &str) -> String {
    let mut xml_content: String = String::new();

    // XML头部
    xml_content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml_content.push_str(&format!("<{}>\n", tree_data.file_name));

    for row_data in tree_data.content.cells.iter() {
        let field_cell: &CellData =
            if let Some(field_cell) = row_data.get(constant::TABLE_KV_COL_FIELD) {
                field_cell
            } else {
                continue;
            };
        let type_cell: &CellData =
            if let Some(type_cell) = row_data.get(constant::TABLE_KV_COL_TYPE) {
                type_cell
            } else {
                continue;
            };
        let keyword_cell: &CellData =
            if let Some(keyword_cell) = row_data.get(constant::TABLE_KV_COL_KEYWORD) {
                keyword_cell
            } else {
                continue;
            };
        let value_cell: &CellData =
            if let Some(value_cell) = row_data.get(constant::TABLE_KV_COL_VALUE) {
                value_cell
            } else {
                continue;
            };

        // 验证字段是否合法
        if !field_cell.verify_lawful() {
            continue;
        }
        // 验证数据类型是否合法
        if !type_cell.verify_lawful() {
            continue;
        }
        // 验证keyword是否合法
        if !keyword_cell.verify_lawful() {
            continue;
        }
        if !keyword_cell.value.contains(keyword) {
            continue;
        }

        let field_name: &String = &field_cell.value;
        let value: String = escape_xml_value(&value_cell.value);
        xml_content.push_str(&format!("  <{}>{}</{}>\n", field_name, value, field_name));
    }

    xml_content.push_str(&format!("</{}>\n", tree_data.file_name));
    xml_content
}

/**
 * 本地化表格转换为XML
 * @param tree_data 树数据
 * @param keyword 关键字
*/
fn localize_xml_data(tree_data: &TreeData, keyword: &str) -> String {
    let (valids_main, valids) = tree_data.content.get_valid_normal_heads(keyword);
    if valids_main.is_empty() || valids.is_empty() {
        return String::new();
    }
    let mut xml_content: String = String::new();

    // XML头部
    xml_content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml_content.push_str(&format!("<{}>\n", tree_data.file_name));

    let max_row: usize = tree_data.content.get_max_row() + 1;
    for row_index in constant::TABLE_LOCALIZE_ROW_TOTAL..=max_row {
        let real_index: usize = row_index - constant::TABLE_LOCALIZE_ROW_TOTAL;
        let row_data = if let Some(row_data) = tree_data.content.cells.get(real_index) {
            row_data
        } else {
            continue;
        };

        let mut row_valid: bool = true;
        // 检测行数据是否有效，主键没有数据，行数据无效则跳过
        for (col_index, _) in valids_main.iter() {
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                row_valid = false;
                continue;
            };
            if value_cell.value.is_empty() {
                row_valid = false;
                continue;
            };
        }
        // 行数据无效
        if !row_valid {
            continue;
        }

        xml_content.push_str("  <item>\n");

        // 主键字段
        for (col_index, col_data) in valids_main.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                continue;
            };
            let field_name = field_cell.value.replace("*", "");
            let value = escape_xml_value(&value_cell.value);
            xml_content.push_str(&format!("    <{}>{}</{}>\n", field_name, value, field_name));
        }

        // 其他字段
        for (col_index, col_data) in valids.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                continue;
            };
            let field_name: &String = &field_cell.value;
            let value: String = escape_xml_value(&value_cell.value);
            xml_content.push_str(&format!("    <{}>{}</{}>\n", field_name, value, field_name));
        }

        xml_content.push_str("  </item>\n");
    }

    xml_content.push_str(&format!("</{}>\n", tree_data.file_name));
    xml_content
}

/**
 * 转义XML特殊字符
 * @param value 原始值
 * @return 转义后的值
*/
fn escape_xml_value(value: &str) -> String {
    value
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}
