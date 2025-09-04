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
                let type_cell: &&CellData = head_data.get(&constant::TABLE_DATA_ROW_TYPE).unwrap();
                let field_cell: &&CellData =
                    head_data.get(&constant::TABLE_DATA_ROW_FIELD).unwrap();
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
                let type_cell: &&CellData = head_data.get(&constant::TABLE_DATA_ROW_TYPE).unwrap();
                let field_cell: &&CellData =
                    head_data.get(&constant::TABLE_DATA_ROW_FIELD).unwrap();
                let value: Value = Self::get_json_value(type_cell, value_cell);
                item_data.insert(field_cell.value.clone(), value);
            }
            items.push(item_data);
        }
        return items;
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
}
