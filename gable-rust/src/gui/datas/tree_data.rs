use crate::{
    common::constant,
    gui::datas::{
        cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType, gable_data::GableData,
    },
};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

// #[derive(serde::Serialize)]
pub struct FieldInfo {
    // 是否是主键
    pub is_key: bool,
    // 字段名称
    pub field_name: String,
    // 字段类型
    pub field_type: EDataType,
    // 字段描述
    pub field_desc: String,
    // 字段链接
    pub field_link: String,
    // 字段序号
    pub field_index: i32,
}

#[derive(Debug, Clone)]
pub struct TreeData {
    pub gable_type: ESheetType,
    pub content: GableData,
}

impl TreeData {
    pub fn to_values(&self, keyword: &str) -> Vec<Map<String, Value>> {
        match self.gable_type {
            ESheetType::Normal => self.normal_data(keyword),
            ESheetType::Localize => self.localize_data(keyword),
            ESheetType::KV => self.kv_data(keyword),
            _ => {
                log::error!("The enumeration table does not export as JSON.");
                Vec::new()
            }
        }
    }

    pub fn to_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        match self.gable_type {
            ESheetType::Normal => self.normal_fields(keyword),
            ESheetType::Localize => self.localize_fields(keyword),
            ESheetType::KV => self.kv_fields(keyword),
            ESheetType::Enum => self.enum_fields(),
        }
    }

    fn normal_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        let mut items: Vec<Map<String, Value>> = Vec::new();
        let max_row: u32 = self.content.max_row + 1;
        for row_index in constant::TABLE_NORMAL_ROW_TOTAL..=max_row {
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
                    if let Some(type_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_TYPE) {
                        type_cell
                    } else {
                        continue;
                    };
                let field_cell: &&CellData =
                    if let Some(field_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_FIELD) {
                        field_cell
                    } else {
                        continue;
                    };
                let value: Value = Self::get_value(type_cell, value_cell);
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
                    if let Some(type_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_TYPE) {
                        type_cell
                    } else {
                        continue;
                    };
                let field_cell: &&CellData =
                    if let Some(field_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_FIELD) {
                        field_cell
                    } else {
                        continue;
                    };
                let value: Value = Self::get_value(type_cell, value_cell);
                item_data.insert(field_cell.value.clone(), value);
            }
            items.push(item_data);
        }
        return items;
    }

    fn localize_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        let mut items: Vec<Map<String, Value>> = Vec::new();
        let max_row: u32 = self.content.max_row + 1;
        for row_index in constant::TABLE_LOCALIZE_ROW_TOTAL..=max_row {
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
                    if let Some(type_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_TYPE) {
                        type_cell
                    } else {
                        continue;
                    };
                let field_cell: &&CellData =
                    if let Some(field_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD) {
                        field_cell
                    } else {
                        continue;
                    };
                let value: Value = Self::get_value(type_cell, value_cell);
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
                    if let Some(type_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_TYPE) {
                        type_cell
                    } else {
                        continue;
                    };
                let field_cell: &&CellData =
                    if let Some(field_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD) {
                        field_cell
                    } else {
                        continue;
                    };
                let value: Value = Self::get_value(type_cell, value_cell);
                item_data.insert(field_cell.value.clone(), value);
            }
            items.push(item_data);
        }
        return items;
    }

    fn kv_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
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
            let value: Value = Self::get_value(type_cell, value_cell);
            items.insert(field_cell.value.clone(), value);
        }
        return vec![items];
    }

    fn get_value(type_cell: &CellData, value_cell: &CellData) -> Value {
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

    fn normal_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        let mut fields: Vec<FieldInfo> = Vec::new();
        let mut field_index: i32 = 1;
        let mut row_valid: bool = true;
        for (_, head_data) in valids_main.iter() {
            let field_cell: &&CellData =
                if let Some(field_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_FIELD) {
                    field_cell
                } else {
                    row_valid = false;
                    continue;
                };
            let type_cell: &&CellData =
                if let Some(type_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_TYPE) {
                    type_cell
                } else {
                    row_valid = false;
                    continue;
                };
            if !field_cell.verify_lawful() {
                row_valid = false;
                continue;
            };
            if !type_cell.verify_lawful() {
                row_valid = false;
                continue;
            };
            let field_value: String = field_cell.value.replace("*", "");
            let data_type: EDataType = EDataType::convert(&type_cell.value);
            let desc_cell: Option<&&CellData> = head_data.get(&constant::TABLE_NORMAL_ROW_DESC);
            let desc_value: String = if let Some(desc_cell) = desc_cell {
                desc_cell.value.clone()
            } else {
                String::new()
            };
            let link_cell: Option<&&CellData> = head_data.get(&constant::TABLE_NORMAL_ROW_LINK);
            let link_value: String = if let Some(link_cell) = link_cell {
                link_cell.value.clone()
            } else {
                String::new()
            };

            let field_info: FieldInfo = FieldInfo {
                is_key: true,
                field_name: field_value,
                field_type: data_type,
                field_desc: desc_value,
                field_link: link_value,
                field_index,
            };
            fields.push(field_info);
            field_index += 1;
        }
        // 列数据无效
        if !row_valid {
            return Vec::new();
        }

        for (_, head_data) in valids.iter() {
            let field_cell: &&CellData =
                if let Some(field_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_FIELD) {
                    field_cell
                } else {
                    continue;
                };
            let type_cell: &&CellData =
                if let Some(type_cell) = head_data.get(&constant::TABLE_NORMAL_ROW_TYPE) {
                    type_cell
                } else {
                    continue;
                };
            if !field_cell.verify_lawful() {
                continue;
            };
            if !type_cell.verify_lawful() {
                continue;
            };
            let data_type: EDataType = EDataType::convert(&type_cell.value);
            let desc_cell: Option<&&CellData> = head_data.get(&constant::TABLE_NORMAL_ROW_DESC);
            let desc_value: String = if let Some(desc_cell) = desc_cell {
                desc_cell.value.clone()
            } else {
                String::new()
            };
            let link_cell: Option<&&CellData> = head_data.get(&constant::TABLE_NORMAL_ROW_LINK);
            let link_value: String = if let Some(link_cell) = link_cell {
                link_cell.value.clone()
            } else {
                String::new()
            };
            let field_info: FieldInfo = FieldInfo {
                is_key: false,
                field_name: field_cell.value.clone(),
                field_type: data_type,
                field_desc: desc_value,
                field_link: link_value,
                field_index,
            };
            fields.push(field_info);
            field_index += 1;
        }
        return fields;
    }

    fn localize_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        let mut fields: Vec<FieldInfo> = Vec::new();
        let mut field_index: i32 = 1;
        let mut row_valid: bool = true;
        for (_, head_data) in valids_main.iter() {
            let field_cell: &&CellData =
                if let Some(field_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD) {
                    field_cell
                } else {
                    row_valid = false;
                    continue;
                };
            let type_cell: &&CellData =
                if let Some(type_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_TYPE) {
                    type_cell
                } else {
                    row_valid = false;
                    continue;
                };
            if !field_cell.verify_lawful() {
                row_valid = false;
                continue;
            };
            if !type_cell.verify_lawful() {
                row_valid = false;
                continue;
            };
            let field_value: String = field_cell.value.replace("*", "");
            let desc_cell: Option<&&CellData> = head_data.get(&constant::TABLE_LOCALIZE_ROW_DESC);
            let desc_value: String = if let Some(desc_cell) = desc_cell {
                desc_cell.value.clone()
            } else {
                String::new()
            };
            let field_info: FieldInfo = FieldInfo {
                is_key: true,
                field_name: field_value,
                field_type: EDataType::String,
                field_desc: desc_value,
                field_link: String::new(),
                field_index,
            };
            fields.push(field_info);
            field_index += 1;
        }
        // 列数据无效
        if !row_valid {
            return Vec::new();
        }

        for (_, head_data) in valids.iter() {
            let field_cell: &&CellData =
                if let Some(field_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_FIELD) {
                    field_cell
                } else {
                    continue;
                };
            let type_cell: &&CellData =
                if let Some(type_cell) = head_data.get(&constant::TABLE_LOCALIZE_ROW_TYPE) {
                    type_cell
                } else {
                    continue;
                };
            if !field_cell.verify_lawful() {
                continue;
            };
            if !type_cell.verify_lawful() {
                continue;
            };
            let desc_cell: Option<&&CellData> = head_data.get(&constant::TABLE_LOCALIZE_ROW_DESC);
            let desc_value: String = if let Some(desc_cell) = desc_cell {
                desc_cell.value.clone()
            } else {
                String::new()
            };
            let field_info: FieldInfo = FieldInfo {
                is_key: false,
                field_name: field_cell.value.clone(),
                field_type: EDataType::String,
                field_desc: desc_value,
                field_link: String::new(),
                field_index,
            };
            fields.push(field_info);
            field_index += 1;
        }
        return fields;
    }

    fn kv_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        let mut field_index: i32 = 1;
        for (_, head_data) in self.content.cells.iter() {
            let field_cell: &CellData =
                if let Some(field_cell) = head_data.get(&(constant::TABLE_KV_COL_FIELD as u16)) {
                    field_cell
                } else {
                    continue;
                };
            let type_cell: &CellData =
                if let Some(type_cell) = head_data.get(&(constant::TABLE_KV_COL_TYPE as u16)) {
                    type_cell
                } else {
                    continue;
                };
            let keyword_cell: &CellData = if let Some(keyword_cell) =
                head_data.get(&(constant::TABLE_KV_COL_KEYWORD as u16))
            {
                keyword_cell
            } else {
                continue;
            };
            if !field_cell.verify_lawful() {
                continue;
            };

            if !type_cell.verify_lawful() {
                continue;
            };

            if !keyword_cell.verify_lawful() {
                continue;
            };

            if !keyword_cell.value.contains(keyword) {
                continue;
            }

            let data_type: EDataType = EDataType::convert(&type_cell.value);
            let link_cell: Option<&CellData> = head_data.get(&(constant::TABLE_KV_COL_LINK as u16));
            let link_value: String = if let Some(link_cell) = link_cell {
                link_cell.value.clone()
            } else {
                String::new()
            };
            let desc_cell: Option<&CellData> = head_data.get(&(constant::TABLE_KV_COL_DESC as u16));
            let desc_value: String = if let Some(desc_cell) = desc_cell {
                desc_cell.value.clone()
            } else {
                String::new()
            };
            let field_value: String = field_cell.value.replace("*", "");
            let field_info: FieldInfo = FieldInfo {
                is_key: false,
                field_name: field_value,
                field_type: data_type,
                field_desc: desc_value,
                field_link: link_value,
                field_index,
            };
            fields.push(field_info);
            field_index += 1;
        }

        return fields;
    }

    fn enum_fields(&self) -> Vec<FieldInfo> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        for (_, row_data) in self.content.cells.iter() {
            let field_cell: &CellData =
                if let Some(field_cell) = row_data.get(&(constant::TABLE_ENUM_COL_FIELD as u16)) {
                    field_cell
                } else {
                    continue;
                };
            let value_cell: &CellData =
                if let Some(value_cell) = row_data.get(&(constant::TABLE_ENUM_COL_VALUE as u16)) {
                    value_cell
                } else {
                    continue;
                };
            let desc_cell: Option<&CellData> =
                row_data.get(&(constant::TABLE_ENUM_COL_DESC as u16));
            // 验证字段是否合法
            if !field_cell.verify_lawful() {
                continue;
            }
            // 验证数据类型是否合法
            if !value_cell.verify_lawful() {
                continue;
            }
            let value_value: i32 = value_cell.parse_int() as i32;
            let desc_value: String = if let Some(desc_cell) = desc_cell {
                desc_cell.value.clone()
            } else {
                String::new()
            };
            let field_info: FieldInfo = FieldInfo {
                is_key: false,
                field_name: field_cell.value.clone(),
                field_type: EDataType::String,
                field_desc: desc_value,
                field_link: String::new(),
                field_index: value_value,
            };
            fields.push(field_info);
        }
        return fields;
    }
}
