use crate::{
    common::{constant, setting::BuildSetting, utils},
    gui::datas::{cell_data::CellData, esheet_type::ESheetType, tree_data::TreeData},
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
};

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        // 枚举不导出
        return;
    }

    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.csv", tree_data.content.sheetname));
    let csv_data: Vec<Vec<String>> = to_csv_data(tree_data, &build_setting.keyword);
    // 创建CSV文件
    let file: Result<File, Error> = File::create(&target_path);
    if file.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
        return;
    }
    let file = file.unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);
    // 写入CSV数据
    for row_data in csv_data.iter() {
        let mut line: String = String::new();
        let mut is_first: bool = true;
        for col_value in row_data.iter() {
            if !is_first {
                line.push(',');
            }
            // 转义包含逗号或引号的值
            if col_value.contains(',') || col_value.contains('"') || col_value.contains('\n') {
                line.push('"');
                line.push_str(&col_value.replace("\"", "\"\""));
                line.push('"');
            } else {
                line.push_str(col_value);
            }
            is_first = false;
        }

        line.push('\n');
        if let Err(e) = writer.write_all(line.as_bytes()) {
            log::error!("写入【{}】文件时出错:{}", build_setting.display_name, e);
            return;
        }
    }

    if let Err(e) = writer.flush() {
        log::error!("刷新【{}】文件时出错:{}", build_setting.display_name, e);
        return;
    }

    log::info!(
        "导出【{}】成功:{}",
        build_setting.display_name,
        target_path.to_str().unwrap()
    );
}

fn to_csv_data(tree_data: &TreeData, keyword: &str) -> Vec<Vec<String>> {
    match tree_data.gable_type {
        ESheetType::Normal => normal_csv_data(tree_data, keyword),
        ESheetType::KV => kv_csv_data(tree_data, keyword),
        _ => {
            log::error!("The enumeration table does not export as CSV.");
            Vec::new()
        }
    }
}

fn normal_csv_data(tree_data: &TreeData, keyword: &str) -> Vec<Vec<String>> {
    let (valids_main, valids) = tree_data.content.get_valid_normal_heads(keyword);
    let mut items: Vec<Vec<String>> = Vec::new();

    let mut desc_row_item: Vec<String> = Vec::new();
    let mut field_row_item: Vec<String> = Vec::new();
    let mut type_row_item: Vec<String> = Vec::new();
    // 主键表头
    for (_, col_data) in valids_main.iter() {
        let desc_cell: &&CellData = col_data.get(&constant::TABLE_DATA_ROW_DESC).unwrap();
        let field_cell: &CellData =
            if let Some(field_cell) = col_data.get(&constant::TABLE_DATA_ROW_FIELD) {
                field_cell
            } else {
                return items;
            };
        if field_cell.value.is_empty() {
            return items;
        };
        let type_cell: &CellData =
            if let Some(type_cell) = col_data.get(&constant::TABLE_DATA_ROW_TYPE) {
                type_cell
            } else {
                return items;
            };
        if type_cell.value.is_empty() {
            return items;
        };
        desc_row_item.push(desc_cell.value.clone());
        let field_value: String = field_cell.value.replace("*", "");
        field_row_item.push(field_value);
        type_row_item.push(type_cell.value.clone());
    }
    // 表头
    for (_, col_data) in valids.iter() {
        let desc_cell = col_data.get(&constant::TABLE_DATA_ROW_DESC).unwrap();
        let field_cell: &&CellData = col_data.get(&constant::TABLE_DATA_ROW_FIELD).unwrap();
        let type_cell: &&CellData = col_data.get(&constant::TABLE_DATA_ROW_TYPE).unwrap();
        desc_row_item.push(desc_cell.value.clone());
        field_row_item.push(field_cell.value.clone());
        type_row_item.push(type_cell.value.clone());
    }
    items.push(desc_row_item);
    items.push(field_row_item);
    items.push(type_row_item);

    let max_row: u32 = tree_data.content.max_row + 1;
    for row_index in constant::TABLE_DATA_ROW_TOTAL..=max_row {
        let row_data: &BTreeMap<u16, CellData> =
            if let Some(row_data) = tree_data.content.cells.get(&row_index) {
                row_data
            } else {
                continue;
            };
        let mut row_valid: bool = true;
        let mut item_data: Vec<String> = Vec::new();
        // 检测行数据是否有效，主键没有数据，行数据无效则跳过
        for (col_index, _) in valids_main.iter() {
            let value_cell: &CellData = if let Some(value_cell) = row_data.get(col_index) {
                value_cell
            } else {
                row_valid = false;
                continue;
            };
            if value_cell.value.is_empty() {
                row_valid = false;
                continue;
            };
            item_data.push(value_cell.value.clone());
        }
        // 行数据无效
        if !row_valid {
            continue;
        }

        for (col_index, _) in valids.iter() {
            let value_cell = if let Some(value_cell) = row_data.get(col_index) {
                value_cell.value.clone()
            } else {
                String::new()
            };
            item_data.push(value_cell);
        }
        items.push(item_data);
    }
    return items;
}

fn kv_csv_data(tree_data: &TreeData, keyword: &str) -> Vec<Vec<String>> {
    let mut items: Vec<Vec<String>> = Vec::new();
    for row_data in tree_data.content.heads.values() {
        let mut head_item: Vec<String> = Vec::new();
        let field_value: String =
            if let Some(field_cell) = row_data.get(&(constant::TABLE_KV_COL_FIELD as u16)) {
                field_cell.value.clone()
            } else {
                String::new()
            };
        let type_value: String =
            if let Some(type_cell) = row_data.get(&(constant::TABLE_KV_COL_TYPE as u16)) {
                type_cell.value.clone()
            } else {
                String::new()
            };
        let value_value: String =
            if let Some(value_cell) = row_data.get(&(constant::TABLE_KV_COL_VALUE as u16)) {
                value_cell.value.clone()
            } else {
                String::new()
            };
        head_item.push(field_value);
        head_item.push(type_value);
        head_item.push(value_value);
        items.push(head_item);
    }
    for (_, row_data) in tree_data.content.cells.iter() {
        let mut row_item: Vec<String> = Vec::new();
        let field_cell: &CellData =
            if let Some(field_cell) = row_data.get(&(constant::TABLE_KV_COL_FIELD as u16)) {
                field_cell
            } else {
                continue;
            };
        let type_cell: &CellData =
            if let Some(type_cell) = row_data.get(&(constant::TABLE_KV_COL_TYPE as u16)) {
                type_cell
            } else {
                continue;
            };
        let keyword_cell: &CellData =
            if let Some(keyword_cell) = row_data.get(&(constant::TABLE_KV_COL_KEYWORD as u16)) {
                keyword_cell
            } else {
                continue;
            };
        let value_cell: &CellData =
            if let Some(value_cell) = row_data.get(&(constant::TABLE_KV_COL_VALUE as u16)) {
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
        row_item.push(field_cell.value.clone());
        row_item.push(type_cell.value.clone());
        row_item.push(value_cell.value.clone());
        items.push(row_item);
    }
    return items;
}
