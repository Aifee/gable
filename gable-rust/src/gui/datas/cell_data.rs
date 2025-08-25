use chrono::NaiveTime;
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
    //背景色值（argb）
    #[serde(
        default = "default_string",
        deserialize_with = "deserialize_string",
        skip_serializing_if = "String::is_empty"
    )]
    pub bg_fill: String,
    //字体颜色（argb）
    #[serde(
        default = "default_string",
        deserialize_with = "deserialize_string",
        skip_serializing_if = "String::is_empty"
    )]
    pub font_fill: String,
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
            bg_fill: if let Some(color) = bc {
                let theme_index: &u32 = color.get_theme_index();
                let tint: &f64 = color.get_tint();
                let color_argb: String = color.get_argb().to_string();
                if !color_argb.is_empty() {
                    format!("argb:{}", color_argb)
                } else if *theme_index != 0 && *tint != 0.0 {
                    format!("theme:{},tint:{}", theme_index, tint)
                } else {
                    String::new()
                }
            } else {
                String::new()
            },
            font_fill: if let Some(color) = fc {
                let theme_index: &u32 = color.get_theme_index();
                let tint: &f64 = color.get_tint();
                let color_argb: String = color.get_argb().to_string();
                if !color_argb.is_empty() {
                    format!("argb:{}", color_argb)
                } else if *theme_index != 0 && *tint != 0.0 {
                    format!("theme:{},tint:{}", theme_index, tint)
                } else {
                    String::new()
                }
            } else {
                String::new()
            },
        };
        data
    }
    // 获取填充类型:0-颜色剔重,1-主题填充，其他不填充
    pub fn get_background_type(&self) -> i8 {
        if self.bg_fill.is_empty() {
            return -1;
        }
        if self.bg_fill.starts_with("argb:") {
            return 0;
        }
        if self.bg_fill.starts_with("theme:") {
            return 1;
        }
        return -1;
    }

    pub fn get_background_color(&self) -> String {
        if self.bg_fill.is_empty() {
            return String::new();
        }
        if self.bg_fill.starts_with("argb:") {
            return self.bg_fill.replace("argb:", "");
        }
        return String::new();
    }

    pub fn get_background_theme_tint(&self) -> (u32, f64) {
        if self.bg_fill.is_empty() || !self.bg_fill.starts_with("theme:") {
            return (0, 0.0);
        }
        let parts: Vec<&str> = self.bg_fill.split(',').collect();
        if parts.len() < 2 {
            return (0, 0.0);
        }
        let theme_index: u32 = parts[0].replace("theme:", "").parse().unwrap_or(0);
        let tint: f64 = parts[1].replace("tint:", "").parse().unwrap_or(0.0);
        (theme_index, tint)
    }

    pub fn get_font_type(&self) -> i8 {
        if self.font_fill.is_empty() {
            return -1;
        }
        if self.font_fill.starts_with("argb:") {
            return 0;
        }
        if self.font_fill.starts_with("theme:") {
            return 1;
        }
        return -1;
    }

    pub fn get_font_color(&self) -> String {
        if self.font_fill.is_empty() {
            return String::new();
        }
        if self.font_fill.starts_with("argb:") {
            return self.font_fill.replace("argb:", "");
        }
        return String::new();
    }

    pub fn get_font_theme_tint(&self) -> (u32, f64) {
        if self.font_fill.is_empty() || !self.font_fill.starts_with("theme:") {
            return (0, 0.0);
        }
        let parts: Vec<&str> = self.font_fill.split(',').collect();
        if parts.len() < 2 {
            return (0, 0.0);
        }
        let theme_index: u32 = parts[0].replace("theme:", "").parse().unwrap_or(0);
        let tint: f64 = parts[1].replace("tint:", "").parse().unwrap_or(0.0);
        (theme_index, tint)
    }

    // 数据是否有效
    pub fn is_empty(&self) -> bool {
        self.value.is_empty() && self.bg_fill.is_empty() && self.font_fill.is_empty()
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

    pub fn parse_time(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let seconds = self.value.parse::<u32>().unwrap();
        let time = NaiveTime::from_num_seconds_from_midnight_opt(seconds, 0)
            .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        return time.format("%H:%M:%S").to_string();
    }
    pub fn parse_date(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let seconds = self.value.parse::<u32>().unwrap();
        let time = NaiveTime::from_num_seconds_from_midnight_opt(seconds, 0)
            .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        return time.format("%Y-%m-%d %H:%M:%S").to_string();
    }
}
