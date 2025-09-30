use crate::{
    common::{constant, setting::BuildSetting, utils},
    gui::datas::{cell_data::CellData, esheet_type::ESheetType, tree_data::TreeData},
};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
};

/**
 * yaml 转换
 * @param build_setting 构建设置
 * @param tree_data 树数据
 * */
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        // 枚举不导出
        return;
    }

    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.yaml", tree_data.file_name));

    let yaml_data: String = to_yaml_data(tree_data, &build_setting.keyword);
    if yaml_data.is_empty() {
        log::debug!(
            "Export [{}] skipped: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
        return;
    }
    // 创建YAML文件
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

    // 写入YAML数据
    if let Err(e) = writer.write_all(yaml_data.as_bytes()) {
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
 *  转换为yaml数据
 *  @param tree_data 树数据
 *  @param keyword 关键字
 * */
fn to_yaml_data(tree_data: &TreeData, keyword: &str) -> String {
    match tree_data.gable_type {
        ESheetType::Normal => normal_yaml_data(tree_data, keyword),
        ESheetType::KV => kv_yaml_data(tree_data, keyword),
        ESheetType::Localize => localize_yaml_data(tree_data, keyword),
        _ => {
            log::error!("The enumeration table does not export as YAML.");
            String::new()
        }
    }
}

/**
 * 普通表格转换为YAML
 * @param tree_data 树数据
 * @param keyword 关键字
*/
fn normal_yaml_data(tree_data: &TreeData, keyword: &str) -> String {
    let (valids_main, valids) = tree_data.content.get_valid_normal_heads(keyword);
    if valids_main.is_empty() || valids.is_empty() {
        return String::new();
    }
    let mut yaml_root: BTreeMap<String, JsonValue> = BTreeMap::new();
    let mut rows_data: Vec<BTreeMap<String, String>> = Vec::new();

    let max_row: usize = tree_data.content.get_max_row() + 1;
    for row_index in constant::TABLE_NORMAL_ROW_TOTAL..=max_row {
        let real_index: usize = row_index - constant::TABLE_NORMAL_ROW_TOTAL;
        let row_data: &Vec<CellData> =
            if let Some(row_data) = tree_data.content.cells.get(real_index) {
                row_data
            } else {
                continue;
            };

        let mut row_valid: bool = true;
        let mut row_item: BTreeMap<String, String> = BTreeMap::new();

        // 检测行数据是否有效，主键没有数据，行数据无效则跳过
        for (col_index, col_data) in valids_main.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_NORMAL_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                row_valid = false;
                break;
            };
            if value_cell.value.is_empty() {
                row_valid = false;
                break;
            };
            let field_name: String = field_cell.value.replace("*", "");
            row_item.insert(field_name, value_cell.value.clone());
        }
        // 行数据无效
        if !row_valid {
            continue;
        }

        // 其他字段
        for (col_index, col_data) in valids.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_NORMAL_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                continue;
            };
            row_item.insert(field_cell.value.clone(), value_cell.value.clone());
        }

        rows_data.push(row_item);
    }

    yaml_root.insert(
        tree_data.file_name.clone(),
        serde_json::Value::Array(
            rows_data
                .into_iter()
                .map(|map| {
                    serde_json::Value::Object(
                        map.into_iter()
                            .map(|(k, v)| (k, serde_json::Value::String(v)))
                            .collect(),
                    )
                })
                .collect(),
        ),
    );

    serde_yaml::to_string(&yaml_root).unwrap_or_else(|_| String::from("{}\n"))
}

/**
 * KV表格转换为YAML
 * @param tree_data 树数据
 * @param keyword 关键字
*/
fn kv_yaml_data(tree_data: &TreeData, keyword: &str) -> String {
    let mut yaml_data: BTreeMap<String, String> = BTreeMap::new();

    for row_data in tree_data.content.cells.iter() {
        let field_cell: &CellData =
            if let Some(field_cell) = row_data.get(constant::TABLE_KV_COL_FIELD) {
                field_cell
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
        // 验证keyword是否合法
        if !keyword_cell.verify_lawful() {
            continue;
        }
        if !keyword_cell.value.contains(keyword) {
            continue;
        }

        yaml_data.insert(field_cell.value.clone(), value_cell.value.clone());
    }

    let mut root_data: BTreeMap<String, JsonValue> = BTreeMap::new();
    root_data.insert(
        tree_data.file_name.clone(),
        serde_json::Value::Object(
            yaml_data
                .into_iter()
                .map(|(k, v)| (k, serde_json::Value::String(v)))
                .collect(),
        ),
    );

    serde_yaml::to_string(&root_data).unwrap_or_else(|_| String::from("{}\n"))
}

/**
 * 本地化表格转换为YAML
 * @param tree_data 树数据
 * @param keyword 关键字
*/
fn localize_yaml_data(tree_data: &TreeData, keyword: &str) -> String {
    let (valids_main, valids) = tree_data.content.get_valid_normal_heads(keyword);
    if valids_main.is_empty() || valids.is_empty() {
        return String::new();
    }
    let mut yaml_root: BTreeMap<String, JsonValue> = BTreeMap::new();
    let mut rows_data: Vec<BTreeMap<String, String>> = Vec::new();

    let max_row: usize = tree_data.content.get_max_row() + 1;
    for row_index in constant::TABLE_LOCALIZE_ROW_TOTAL..=max_row {
        let real_index: usize = row_index - constant::TABLE_LOCALIZE_ROW_TOTAL;
        let row_data: &Vec<CellData> =
            if let Some(row_data) = tree_data.content.cells.get(real_index) {
                row_data
            } else {
                continue;
            };

        let mut row_valid: bool = true;
        let mut row_item: BTreeMap<String, String> = BTreeMap::new();

        // 检测行数据是否有效，主键没有数据，行数据无效则跳过
        for (col_index, col_data) in valids_main.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                row_valid = false;
                break;
            };
            if value_cell.value.is_empty() {
                row_valid = false;
                break;
            };
            let field_name = field_cell.value.replace("*", "");
            row_item.insert(field_name, value_cell.value.clone());
        }
        // 行数据无效
        if !row_valid {
            continue;
        }

        // 其他字段
        for (col_index, col_data) in valids.iter() {
            let field_cell: &CellData = col_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD).unwrap();
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
                value_cell
            } else {
                continue;
            };
            row_item.insert(field_cell.value.clone(), value_cell.value.clone());
        }

        rows_data.push(row_item);
    }

    yaml_root.insert(
        tree_data.file_name.clone(),
        serde_json::Value::Array(
            rows_data
                .into_iter()
                .map(|map| {
                    serde_json::Value::Object(
                        map.into_iter()
                            .map(|(k, v)| (k, serde_json::Value::String(v)))
                            .collect(),
                    )
                })
                .collect(),
        ),
    );

    serde_yaml::to_string(&yaml_root).unwrap_or_else(|_| String::from("{}\n"))
}
