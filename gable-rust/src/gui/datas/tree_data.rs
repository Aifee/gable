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
        let valid_heads: BTreeMap<u16, BTreeMap<u32, &CellData>> =
            self.content.get_valid_normal_heads(keyword);
        let mut items: Vec<Map<String, Value>> = Vec::new();
        let max_row = self.content.max_row + 1;
        for row_index in constant::TABLE_DATA_ROW_TOTAL..=max_row {
            let row_data = if let Some(row_data) = self.content.cells.get(&row_index) {
                row_data
            } else {
                continue;
            };
            let mut item_data: Map<String, Value> = Map::new();
            for (col_index, head_data) in valid_heads.iter() {
                let value_cell = if let Some(value_cell) = row_data.get(col_index) {
                    value_cell
                } else {
                    continue;
                };
                if value_cell.value.is_empty() {
                    continue;
                };
                let type_cell = head_data.get(&constant::TABLE_DATA_ROW_TYPE).unwrap();
                let data_type = EDataType::convert(&type_cell.value);
                let field_cell = head_data.get(&constant::TABLE_DATA_ROW_FIELD).unwrap();
                match data_type {
                    EDataType::Int | EDataType::Time | EDataType::Date | EDataType::Enum => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.parse_int()),
                        );
                    }
                    EDataType::Boolean => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.parse_bool()),
                        );
                    }
                    EDataType::Float => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.parse_float()),
                        );
                    }
                    EDataType::Vector2 => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_vector2()),
                        );
                    }
                    EDataType::Vector3 => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_vector3()),
                        );
                    }
                    EDataType::Vector4 => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_vector4()),
                        );
                    }
                    EDataType::IntArr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_int_array()),
                        );
                    }
                    EDataType::StringArr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_string_array()),
                        );
                    }
                    EDataType::BooleanArr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_bool_array()),
                        );
                    }
                    EDataType::FloatArr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_float_array()),
                        );
                    }
                    EDataType::Vector2Arr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_vector2_array()),
                        );
                    }
                    EDataType::Vector3Arr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_vector3_array()),
                        );
                    }
                    EDataType::Vector4Arr => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.to_json_vector4_array()),
                        );
                    }
                    EDataType::Percentage | EDataType::Permillage | EDataType::Permian => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.parse_float()),
                        );
                    }
                    _ => {
                        item_data.insert(
                            field_cell.value.clone(),
                            Value::from(value_cell.value.clone()),
                        );
                    }
                }
            }
            items.push(item_data);
        }
        return items;
    }
}
