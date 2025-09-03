use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
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
        return self.value.parse::<i32>().unwrap_or(0);
    }
    pub fn parse_bool(&self) -> bool {
        if self.value.is_empty() {
            return false;
        }
        return self.value.parse::<bool>().unwrap_or(false);
    }
    pub fn parse_float(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }
        return self.value.parse::<f64>().unwrap_or(0.0);
    }
    /**
     * 将单元格中的值解析为时间格式：
     *
     * 此函数将存储的秒数转换为Excel/WPS格式的日期数值。
     * Excel/WPS日期格式说明：
     * - 小数部分：表示一天中的时间比例（1天=1.0）
     *
     * 返回值：f64格式的日期数值，整数部分为天数，小数部分为时间
     */
    pub fn parse_time(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }
        let seconds: f64 = self.value.parse::<f64>().unwrap_or(0.0);
        let fraction: f64 = seconds / 86400.0;
        return fraction;
    }
    pub fn convert_time(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let seconds: f64 = self.value.parse::<f64>().unwrap_or(0.0);
        let total_seconds = seconds as u32;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let secs = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }
    /**
     * 将单元格中的值解析为日期格式
     *
     * 此函数将存储的秒数转换为Excel/WPS格式的日期数值。
     * Excel/WPS日期格式说明：
     * - 整数部分：表示从基准日期（1900年1月0日）开始的天数
     * - 小数部分：表示一天中的时间比例（1天=1.0）
     *
     * 返回值：f64格式的日期数值，整数部分为天数，小数部分为时间
     */
    pub fn parse_date(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }
        // 尝试解析存储的值为秒数
        let seconds: u64 = self.value.parse::<u64>().unwrap_or(0);
        let days: u64 = seconds / 86400;
        let fraction: f64 = (seconds % 86400) as f64;
        let cell_value: f64 = ((days + 1) as f64) + fraction / 86400.0;
        cell_value
    }

    pub fn convert_date(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }
        let seconds: u64 = self.value.parse::<u64>().unwrap_or(0);
        // 根据Excel日期系统处理日期转换
        // Excel基准日期是1900-01-01，但存在一个特殊问题：
        // 1. Excel认为1900年是闰年（实际上不是）
        // 2. Excel有一个错误，将1900-01-00作为第1天（实际上是不存在的日期）
        // 3. 为了兼容Excel，需要特殊处理
        // 先计算天数和当天的秒数
        let days: u64 = seconds / 86400;
        let remaining_seconds: u64 = seconds % 86400;
        // 根据parse_date函数的逻辑，需要将天数加1来补偿Excel的偏移
        // 但同时需要处理1900年2月29日这个不存在的日期问题
        let excel_days = days + 1;
        // 从1900-01-01开始计算日期
        let base_date = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        // 如果天数超过60天（1900-02-28之后），需要减去1天来补偿1900年非闰年的问题
        let date: NaiveDate = if excel_days <= 60 {
            // 60对应的是1900-02-28（在Excel中被认为是1900-02-29）
            base_date + Duration::days(excel_days as i64 - 1)
        } else {
            // 超过60天后，需要补偿1900年不是闰年的问题
            base_date + Duration::days(excel_days as i64 - 2)
        };
        // 计算时间部分
        let hours: u32 = (remaining_seconds / 3600) as u32;
        let minutes: u32 = ((remaining_seconds % 3600) / 60) as u32;
        let secs: u32 = (remaining_seconds % 60) as u32;
        let time: NaiveTime = NaiveTime::from_hms_opt(hours, minutes, secs).unwrap();
        // 组合日期和时间
        let datetime: NaiveDateTime = date.and_time(time);
        // 格式化为 YYYY-mm-dd hh:mm:ss 格式
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
