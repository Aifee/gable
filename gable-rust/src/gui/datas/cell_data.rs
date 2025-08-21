use crate::{common::global, gui::datas::edata_type::EDataType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use umya_spreadsheet::Color;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CellData {
    pub row: u32,
    pub column: u16,
    #[serde(
        default = "default_string",
        deserialize_with = "deserialize_string",
        skip_serializing_if = "String::is_empty"
    )]
    pub value: String,
    //前景色色值（argb）
    #[serde(
        default = "default_string",
        deserialize_with = "deserialize_string",
        skip_serializing_if = "String::is_empty"
    )]
    pub bg_color: String,
    //字体颜色（argb）
    #[serde(
        default = "default_string",
        deserialize_with = "deserialize_string",
        skip_serializing_if = "String::is_empty"
    )]
    pub font_color: String,
}

fn default_string() -> String {
    String::new()
}
fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    Ok(match value {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        _ => value.to_string(),
    })
}

impl CellData {
    pub fn new(r: u32, c: u16, v: String, bc: Option<&Color>, fc: Option<&Color>) -> Self {
        let data = Self {
            row: r,
            column: c,
            value: v,
            bg_color: if let Some(color) = bc {
                color.get_argb().to_string()
            } else {
                String::new()
            },
            font_color: if let Some(color) = fc {
                color.get_argb().to_string()
            } else {
                String::new()
            },
        };
        data
    }

    // 数据是否有效
    pub fn is_empty(&self) -> bool {
        self.value.is_empty() && self.bg_color.is_empty() && self.font_color.is_empty()
    }
    // 获取数据类型
    pub fn get_data_type(&self) -> EDataType {
        if self.value.is_empty() {
            return EDataType::STRING;
        }
        match self.value.as_str() {
            global::DATA_TYPE_KEY_STRING => EDataType::STRING,
            global::DATA_TYPE_KEY_INT => EDataType::INT,
            global::DATA_TYPE_KEY_BOOLEAN => EDataType::BOOLEAN,
            global::DATA_TYPE_KEY_FLOAT => EDataType::FLOAT,
            global::DATA_TYPE_KEY_VECTOR2 => EDataType::VECTOR2,
            global::DATA_TYPE_KEY_VECTOR3 => EDataType::VECTOR3,
            global::DATA_TYPE_KEY_VECTOR4 => EDataType::VECTOR4,
            global::DATA_TYPE_KEY_STRING_ARR => EDataType::STRING_ARR,
            global::DATA_TYPE_KEY_INT_ARR => EDataType::INT_ARR,
            global::DATA_TYPE_KEY_BOOLEAN_ARR => EDataType::BOOLEAN_ARR,
            global::DATA_TYPE_KEY_FLOAT_ARR => EDataType::FLOAT_ARR,
            global::DATA_TYPE_KEY_VECTOR2_ARR => EDataType::VECTOR2_ARR,
            global::DATA_TYPE_KEY_VECTOR3_ARR => EDataType::VECTOR3_ARR,
            global::DATA_TYPE_KEY_VECTOR4_ARR => EDataType::VECTOR4_ARR,
            global::DATA_TYPE_KEY_PERCENTAGE => EDataType::PERCENTAGE,
            global::DATA_TYPE_KEY_PERMILLAGE => EDataType::PERMILLAGE,
            global::DATA_TYPE_KEY_PERMIAN => EDataType::PERMIAN,
            global::DATA_TYPE_KEY_TIME => EDataType::TIME,
            global::DATA_TYPE_KEY_ENUM => EDataType::ENUM,
            _ => EDataType::Unknown,
        }
    }

    pub fn parse_int(&self) -> i32 {
        if self.value.is_empty() {
            return 0;
        }
        return self.value.parse::<i32>().unwrap();
    }
    pub fn parse_bool(&self) -> bool {
        if self.value.is_empty() {
            return false;
        }
        return self.value.parse::<bool>().unwrap();
    }
    pub fn parse_float(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }
        return self.value.parse::<f64>().unwrap();
    }
    pub fn parse_vector2(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let list: Vec<&str> = self.value.split(";").collect();
        if list.len() >= 2 {
            format!("x:{},y:{}", list[0], list[1])
        } else {
            String::new()
        }
    }
    pub fn parse_vector3(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let list: Vec<&str> = self.value.split(";").collect();
        if list.len() >= 3 {
            format!("x:{},y:{},z:{}", list[0], list[1], list[2])
        } else {
            String::new()
        }
    }
    pub fn parse_vector4(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let list: Vec<&str> = self.value.split(";").collect();
        if list.len() >= 4 {
            format!("x:{},y:{},z:{},w:{}", list[0], list[1], list[2], list[3])
        } else {
            String::new()
        }
    }
}
