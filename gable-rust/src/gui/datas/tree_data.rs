use crate::{
    common::constant,
    gui::datas::{
        cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType, gable_data::GableData,
    },
};
use serde_json::{Map, Value};

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
    pub file_name: String,
    pub content: GableData,
}

impl TreeData {
    /**
     * 将数据转换为值列表
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回值映射列表
     */
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

    /**
     * 将数据转换为字段信息列表
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回字段信息列表
     */
    pub fn to_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        match self.gable_type {
            ESheetType::Normal => self.normal_fields(keyword),
            ESheetType::Localize => self.localize_fields(keyword),
            ESheetType::KV => self.kv_fields(keyword),
            ESheetType::Enum => self.enum_fields(),
        }
    }

    /**
     * 获取普通表数据
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回值映射列表
     */
    fn normal_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        if valids_main.is_empty() || valids.is_empty() {
            return Vec::new();
        }
        let mut items: Vec<Map<String, Value>> = Vec::new();
        let max_row: usize = self.content.get_max_row();
        for row_index in constant::TABLE_NORMAL_ROW_TOTAL..=max_row {
            let real_index: usize = row_index - constant::TABLE_NORMAL_ROW_TOTAL;
            let row_data: &Vec<CellData> =
                if let Some(row_data) = self.content.cells.get(real_index) {
                    row_data
                } else {
                    continue;
                };
            let mut row_valid: bool = true;
            let mut item_data: Map<String, Value> = Map::new();
            // 检测行数据是否有效，主键没有数据，行数据无效则跳过
            for (col_index, head_data) in valids_main.iter() {
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
                let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
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
            if item_data.is_empty() {
                continue;
            }
            items.push(item_data);
        }
        return items;
    }

    /**
     * 获取本地化表数据
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回值映射列表
     */
    fn localize_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        if valids_main.is_empty() || valids.is_empty() {
            return Vec::new();
        }
        let mut items: Vec<Map<String, Value>> = Vec::new();
        let max_row: usize = self.content.get_max_row();
        for row_index in constant::TABLE_LOCALIZE_ROW_TOTAL..=max_row {
            let real_index: usize = row_index - constant::TABLE_LOCALIZE_ROW_TOTAL;
            let row_data = if let Some(row_data) = self.content.cells.get(real_index) {
                row_data
            } else {
                continue;
            };
            let mut row_valid: bool = true;
            let mut item_data: Map<String, Value> = Map::new();
            // 检测行数据是否有效，主键没有数据，行数据无效则跳过
            for (col_index, head_data) in valids_main.iter() {
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
                let value_cell: &CellData = if let Some(value_cell) = row_data.get(*col_index) {
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

    /**
     * 获取键值对表数据
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回值映射列表
     */
    fn kv_data(&self, keyword: &str) -> Vec<Map<String, Value>> {
        let mut items: Map<String, Value> = Map::new();
        for row_data in self.content.cells.iter() {
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
            let keyword_celldata: &CellData =
                if let Some(keyword_celldata) = row_data.get(constant::TABLE_KV_COL_KEYWORD) {
                    keyword_celldata
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

    /**
     * 根据类型和值获取对应的JSON值
     * @param type_cell 类型单元格
     * @param value_cell 值单元格
     * @return 返回对应的JSON值
     */
    fn get_value(type_cell: &CellData, value_cell: &CellData) -> Value {
        let data_type: EDataType = EDataType::convert(&type_cell.value);
        match data_type {
            EDataType::Unknown | EDataType::String | EDataType::Loc => {
                Value::from(value_cell.value.clone())
            }
            EDataType::Int
            | EDataType::Long
            | EDataType::Time
            | EDataType::Date
            | EDataType::Enum => Value::from(value_cell.parse_int()),
            EDataType::Boolean => Value::from(value_cell.parse_bool()),
            EDataType::Float => Value::from(value_cell.parse_float()),
            EDataType::Vector2 => Value::from(value_cell.to_json_vector2()),
            EDataType::Vector3 => Value::from(value_cell.to_json_vector3()),
            EDataType::Vector4 => Value::from(value_cell.to_json_vector4()),
            EDataType::IntArr | EDataType::LongArr => Value::from(value_cell.to_json_int_array()),
            EDataType::StringArr => Value::from(value_cell.to_json_string_array()),
            EDataType::BooleanArr => Value::from(value_cell.to_json_bool_array()),
            EDataType::FloatArr => Value::from(value_cell.to_json_float_array()),
            EDataType::Vector2Arr => Value::from(value_cell.to_json_vector2_array()),
            EDataType::Vector3Arr => Value::from(value_cell.to_json_vector3_array()),
            EDataType::Vector4Arr => Value::from(value_cell.to_json_vector4_array()),
            EDataType::Percentage | EDataType::Permillage | EDataType::Permian => {
                Value::from(value_cell.parse_float())
            }
        }
    }

    /**
     * 获取普通表字段信息
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回字段信息列表
     */
    fn normal_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        if valids_main.is_empty() || valids.is_empty() {
            return vec![];
        };
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

    /**
     * 获取本地化表字段信息
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回字段信息列表
     */
    fn localize_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        let (valids_main, valids) = self.content.get_valid_normal_heads(keyword);
        if valids_main.is_empty() || valids.is_empty() {
            return vec![];
        };
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

    /**
     * 获取键值对表字段信息
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回字段信息列表
     */
    fn kv_fields(&self, keyword: &str) -> Vec<FieldInfo> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        let mut field_index: i32 = 1;
        for head_data in self.content.cells.iter() {
            let field_cell: &CellData =
                if let Some(field_cell) = head_data.get(constant::TABLE_KV_COL_FIELD) {
                    field_cell
                } else {
                    continue;
                };
            let type_cell: &CellData =
                if let Some(type_cell) = head_data.get(constant::TABLE_KV_COL_TYPE) {
                    type_cell
                } else {
                    continue;
                };
            let keyword_cell: &CellData =
                if let Some(keyword_cell) = head_data.get(constant::TABLE_KV_COL_KEYWORD) {
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
            let link_cell: Option<&CellData> = head_data.get(constant::TABLE_KV_COL_LINK);
            let link_value: String = if let Some(link_cell) = link_cell {
                link_cell.value.clone()
            } else {
                String::new()
            };
            let desc_cell: Option<&CellData> = head_data.get(constant::TABLE_KV_COL_DESC);
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

    /**
     * 获取枚举表字段信息
     * @return 返回字段信息列表
     */
    fn enum_fields(&self) -> Vec<FieldInfo> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        for row_data in self.content.cells.iter() {
            let field_cell: &CellData =
                if let Some(field_cell) = row_data.get(constant::TABLE_ENUM_COL_FIELD) {
                    field_cell
                } else {
                    continue;
                };
            let value_cell: &CellData =
                if let Some(value_cell) = row_data.get(constant::TABLE_ENUM_COL_VALUE) {
                    value_cell
                } else {
                    continue;
                };
            let desc_cell: Option<&CellData> = row_data.get(constant::TABLE_ENUM_COL_DESC);
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
