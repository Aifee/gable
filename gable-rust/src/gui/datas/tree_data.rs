use crate::{
    common::constant,
    gui::datas::{
        cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType, gable_data::GableData,
    },
};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TreeData {
    pub gable_type: ESheetType,
    pub content: GableData,
}

impl TreeData {
    pub fn to_json_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        match self.gable_type {
            ESheetType::Normal => self.normal_json_data(keyword),
            ESheetType::KV => self.kv_json_data(keyword),
            _ => Vec::new(),
        }
    }
    pub fn to_csv_data(&self, keyword: &str) -> Vec<Vec<String>> {
        match self.gable_type {
            ESheetType::Normal => self.normal_csv_data(keyword),
            _ => Vec::new(),
        }
    }

    fn normal_json_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        let mut items: Vec<Map<String, Value>> = Vec::new();
        let max_row: u32 = self.content.max_row + 1;
        for row_index in constant::TABLE_DATA_ROW_TOTAL..=max_row {
            let row_data: &BTreeMap<u16, CellData> =
                if let Some(row_data) = self.content.cells.get(&row_index) {
                    row_data
                } else {
                    continue;
                };
            let mut row_valid: bool = true;
            let mut item_data: Map<String, Value> = Map::new();
            // 检测行数据是否有效，主键没有数据，行数据无效则跳过
            for (col_index, head_data) in valids_main.iter() {
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
                let type_cell: &&CellData =
                    if let Some(type_cell) = head_data.get(&constant::TABLE_DATA_ROW_TYPE) {
                        type_cell
                    } else {
                        continue;
                    };
                let field_cell: &&CellData =
                    if let Some(field_cell) = head_data.get(&constant::TABLE_DATA_ROW_FIELD) {
                        field_cell
                    } else {
                        continue;
                    };
                let value: Value = Self::get_json_value(type_cell, value_cell);
                let field_value: String = field_cell.value.replace("*", "");
                item_data.insert(field_value, value);
            }
            // 行数据无效
            if !row_valid {
                continue;
            }

            for (col_index, head_data) in valids.iter() {
                let value_cell: &CellData = if let Some(value_cell) = row_data.get(col_index) {
                    value_cell
                } else {
                    continue;
                };
                if value_cell.value.is_empty() {
                    continue;
                };
                let type_cell: &&CellData =
                    if let Some(type_cell) = head_data.get(&constant::TABLE_DATA_ROW_TYPE) {
                        type_cell
                    } else {
                        continue;
                    };
                let field_cell: &&CellData =
                    if let Some(field_cell) = head_data.get(&constant::TABLE_DATA_ROW_FIELD) {
                        field_cell
                    } else {
                        continue;
                    };
                let value: Value = Self::get_json_value(type_cell, value_cell);
                item_data.insert(field_cell.value.clone(), value);
            }
            items.push(item_data);
        }
        return items;
    }

    fn kv_json_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let mut items: Map<String, Value> = Map::new();
        for (_, row_data) in self.content.cells.iter() {
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
            let keyword_celldata: &CellData = if let Some(keyword_celldata) =
                row_data.get(&(constant::TABLE_KV_COL_KEYWORD as u16))
            {
                keyword_celldata
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
            if !keyword_celldata.verify_lawful() {
                continue;
            }
            if !keyword_celldata.value.contains(keyword) {
                continue;
            }
            if value_cell.value.is_empty() {
                continue;
            }
            let value: Value = Self::get_json_value(type_cell, value_cell);
            items.insert(field_cell.value.clone(), value);
        }
        return vec![items];
    }

    fn get_json_value(type_cell: &CellData, value_cell: &CellData) -> Value {
        let data_type: EDataType = EDataType::convert(&type_cell.value);
        match data_type {
            EDataType::Int | EDataType::Time | EDataType::Date | EDataType::Enum => {
                Value::from(value_cell.parse_int())
            }
            EDataType::Boolean => Value::from(value_cell.parse_bool()),
            EDataType::Float => Value::from(value_cell.parse_float()),
            EDataType::Vector2 => Value::from(value_cell.to_json_vector2()),
            EDataType::Vector3 => Value::from(value_cell.to_json_vector3()),
            EDataType::Vector4 => Value::from(value_cell.to_json_vector4()),
            EDataType::IntArr => Value::from(value_cell.to_json_int_array()),
            EDataType::StringArr => Value::from(value_cell.to_json_string_array()),
            EDataType::BooleanArr => Value::from(value_cell.to_json_bool_array()),
            EDataType::FloatArr => Value::from(value_cell.to_json_float_array()),
            EDataType::Vector2Arr => Value::from(value_cell.to_json_vector2_array()),
            EDataType::Vector3Arr => Value::from(value_cell.to_json_vector3_array()),
            EDataType::Vector4Arr => Value::from(value_cell.to_json_vector4_array()),
            EDataType::Percentage | EDataType::Permillage | EDataType::Permian => {
                Value::from(value_cell.parse_float())
            }
            _ => Value::from(value_cell.value.clone()),
        }
    }

    fn normal_csv_data(&self, keyword: &str) -> Vec<Vec<String>> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        let mut items: Vec<Vec<String>> = Vec::new();

        let mut desc_row_item: Vec<String> = Vec::new();
        let mut field_row_item: Vec<String> = Vec::new();
        let mut type_row_item: Vec<String> = Vec::new();
        // 主键表头
        for (_, col_data) in valids_main.iter() {
            let desc_cell = col_data.get(&constant::TABLE_DATA_ROW_DESC).unwrap();
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

        let max_row: u32 = self.content.max_row + 1;
        for row_index in constant::TABLE_DATA_ROW_TOTAL..=max_row {
            let row_data: &BTreeMap<u16, CellData> =
                if let Some(row_data) = self.content.cells.get(&row_index) {
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
}
