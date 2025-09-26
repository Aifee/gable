use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use umya_spreadsheet::Color;

/**
 * 单元格数据结构
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CellData {
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
/// 反序列化字符串的自定义函数
///
/// # 参数
/// * `deserializer` - 用于反序列化的 serde Deserializer
///
/// # 返回值
/// 返回反序列化后的 String 结果，如果反序列化失败则返回错误
///
/// # 说明
/// 此函数将不同类型的 JSON 值转换为字符串：
/// - String 类型直接返回
/// - Number 类型转换为字符串
/// - Boolean 类型转换为字符串
/// - Null 类型返回空字符串
/// - 其他类型转换为其字符串表示形式
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
    /// 创建一个新的 CellData 实例
    ///
    /// @param r 行号
    /// @param c 列号
    /// @param v 单元格值
    /// @param bc 背景颜色（可选）
    /// @param fc 字体颜色（可选）
    /// @return 返回一个新的 CellData 实例
    pub fn new(v: String, bc: Option<&Color>, fc: Option<&Color>) -> Self {
        let data = Self {
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

    /// 获取背景填充类型
    ///
    /// @return -1: 无背景填充, 0: ARGB 颜色填充, 1: 主题色填充
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

    /// 获取背景填充颜色
    ///
    /// @return 背景填充的 ARGB 颜色值，如果无背景或不是 ARGB 类型则返回空字符串
    pub fn get_background_color(&self) -> String {
        if self.bg_fill.is_empty() {
            return String::new();
        }
        if self.bg_fill.starts_with("argb:") {
            return self.bg_fill.replace("argb:", "");
        }
        return String::new();
    }

    /// 获取背景填充主题色和色调
    ///
    /// @return (theme_index, tint) 元组，分别表示主题色索引和色调值
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

    /// 获取字体颜色类型
    ///
    /// @return -1: 无字体颜色, 0: ARGB 颜色, 1: 主题色
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

    /// 获取字体颜色
    ///
    /// @return 字体的 ARGB 颜色值，如果无字体颜色或不是 ARGB 类型则返回空字符串
    pub fn get_font_color(&self) -> String {
        if self.font_fill.is_empty() {
            return String::new();
        }
        if self.font_fill.starts_with("argb:") {
            return self.font_fill.replace("argb:", "");
        }
        return String::new();
    }

    /// 获取字体主题色和色调
    ///
    /// @return (theme_index, tint) 元组，分别表示主题色索引和色调值
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

    /// 检查单元格数据是否为空
    ///
    /// @return 如果值、背景色和字体色都为空则返回 true，否则返回 false
    pub fn is_empty(&self) -> bool {
        self.value.is_empty() && self.bg_fill.is_empty() && self.font_fill.is_empty()
    }

    /// 验证数据合法性
    ///
    /// @return 如果数据合法返回 true，否则返回 false
    ///
    /// 当前实现仅检查值是否为空，未来可以扩展更多验证规则
    pub fn verify_lawful(&self) -> bool {
        if self.value.is_empty() {
            return false;
        }
        // 这里扩展命名合法性
        return true;
    }

    /// 将单元格值解析为整数
    ///
    /// @return 解析后的 i64 整数值，如果解析失败或值为空则返回 0
    pub fn parse_int(&self) -> i64 {
        if self.value.is_empty() {
            return 0;
        }
        return self.value.parse::<i64>().unwrap_or(0);
    }

    /// 将单元格值解析为布尔值
    ///
    /// @return 解析后的布尔值，如果解析失败或值为空则返回 false
    ///
    /// 支持以下值:
    /// - true: "true", "1", "yes", "on"
    /// - false: "false", "0", "no", "off"
    /// - 其他值默认返回 false
    pub fn parse_bool(&self) -> bool {
        if self.value.is_empty() {
            return false;
        }
        let normalized = self.value.trim().to_lowercase();
        match normalized.as_str() {
            "true" | "1" | "yes" | "on" => true,
            "false" | "0" | "no" | "off" => false,
            _ => false, // 对于无法识别的值，返回false
        }
    }

    /// 将单元格值解析为浮点数
    ///
    /// @return 解析后的 f64 浮点数，如果解析失败或值为空则返回 0.0
    pub fn parse_float(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }
        return self.value.parse::<f64>().unwrap_or(0.0);
    }

    /// 将单元格中的值解析为时间格式（Excel/WPS格式）
    ///
    /// @return f64 格式的日期数值，整数部分为天数，小数部分为时间
    ///
    /// 此函数将存储的秒数转换为Excel/WPS格式的日期数值:
    /// - 小数部分：表示一天中的时间比例（1天=1.0）
    pub fn parse_time(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }
        let seconds: f64 = self.value.parse::<f64>().unwrap_or(0.0);
        let fraction: f64 = seconds / 86400.0;
        return fraction;
    }

    /// 将单元格中的时间值转换为 HH:mm:ss 格式的字符串
    ///
    /// @return 格式化后的时间字符串，格式为 HH:mm:ss，如果值为空则返回空字符串
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

    /// 将单元格中的值解析为日期格式（Excel/WPS格式）
    ///
    /// @return f64 格式的日期数值，整数部分为天数，小数部分为时间
    ///
    /// 此函数将存储的秒数转换为Excel/WPS格式的日期数值:
    /// - 整数部分：表示从基准日期（1900年1月0日）开始的天数
    /// - 小数部分：表示一天中的时间比例（1天=1.0）
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

    /// 将单元格中的日期值转换为 YYYY-MM-DD HH:mm:ss 格式的字符串
    ///
    /// @return 格式化后的日期时间字符串，格式为 YYYY-MM-DD HH:mm:ss，如果值为空则返回空字符串
    ///
    /// 处理 Excel 日期系统的一些特殊情况:
    /// 1. Excel 认为 1900 年是闰年（实际上不是）
    /// 2. Excel 有一个错误，将 1900-01-00 作为第 1 天（实际上是不存在的日期）
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

    /// 将单元格值解析为二维向量 (x, y)
    ///
    /// @return 包含 x 和 y 键的 JSON Map，如果解析失败则返回空 Map
    ///
    /// 值应以分号分隔，格式为 "x;y"
    pub fn to_json_vector2(&self) -> Map<String, Value> {
        let mut vector2: Map<String, Value> = Map::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                vector2.insert("x".to_string(), Value::from(x));
                vector2.insert("y".to_string(), Value::from(y));
            }
        }
        return vector2;
    }

    /// 将单元格值解析为三维向量 (x, y, z)
    ///
    /// @return 包含 x、y 和 z 键的 JSON Map，如果解析失败则返回空 Map
    ///
    /// 值应以分号分隔，格式为 "x;y;z"
    pub fn to_json_vector3(&self) -> Map<String, Value> {
        let mut vector3: Map<String, Value> = Map::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        if parts.len() == 3 {
            if let (Ok(x), Ok(y), Ok(z)) = (
                parts[0].parse::<f64>(),
                parts[1].parse::<f64>(),
                parts[2].parse::<f64>(),
            ) {
                vector3.insert("x".to_string(), Value::from(x));
                vector3.insert("y".to_string(), Value::from(y));
                vector3.insert("z".to_string(), Value::from(z));
            }
        }
        return vector3;
    }

    /// 将单元格值解析为四维向量 (x, y, z, w)
    ///
    /// @return 包含 x、y、z 和 w 键的 JSON Map，如果解析失败则返回空 Map
    ///
    /// 值应以分号分隔，格式为 "x;y;z;w"
    pub fn to_json_vector4(&self) -> Map<String, Value> {
        let mut vector4: Map<String, Value> = Map::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        if parts.len() == 4 {
            if let (Ok(x), Ok(y), Ok(z), Ok(w)) = (
                parts[0].parse::<f64>(),
                parts[1].parse::<f64>(),
                parts[2].parse::<f64>(),
                parts[3].parse::<f64>(),
            ) {
                vector4.insert("x".to_string(), Value::from(x));
                vector4.insert("y".to_string(), Value::from(y));
                vector4.insert("z".to_string(), Value::from(z));
                vector4.insert("w".to_string(), Value::from(w));
            }
        }
        return vector4;
    }

    /// 将单元格值解析为整数数组
    ///
    /// @return 包含解析后整数值的 JSON Value 向量，解析失败的值会被忽略
    ///
    /// 值应以分号分隔，每个部分都会被解析为 i64
    pub fn to_json_int_array(&self) -> Vec<Value> {
        let mut arr: Vec<Value> = Vec::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        for part in parts.iter() {
            let value: i64 = part.parse::<i64>().unwrap();
            arr.push(Value::from(value));
        }
        return arr;
    }

    /// 将单元格值解析为字符串数组
    ///
    /// @return 包含原始字符串值的 JSON Value 向量
    ///
    /// 值应以分号分隔，每个部分都会作为字符串保留
    pub fn to_json_string_array(&self) -> Vec<Value> {
        let mut arr: Vec<Value> = Vec::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        for part in parts.iter() {
            arr.push(Value::from(*part));
        }
        return arr;
    }

    /// 将单元格值解析为布尔数组
    ///
    /// @return 包含解析后布尔值的 JSON Value 向量
    ///
    /// 值应以分号分隔，每个部分都会被解析为布尔值
    pub fn to_json_bool_array(&self) -> Vec<Value> {
        let mut arr: Vec<Value> = Vec::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        for part in parts.iter() {
            let value: bool = part.parse::<bool>().unwrap();
            arr.push(Value::from(value));
        }
        return arr;
    }

    /// 将单元格值解析为浮点数数组
    ///
    /// @return 包含解析后浮点数值的 JSON Value 向量
    ///
    /// 值应以分号分隔，每个部分都会被解析为 f64
    pub fn to_json_float_array(&self) -> Vec<Value> {
        let mut arr: Vec<Value> = Vec::new();
        let parts: Vec<&str> = self.value.split(';').collect();
        for part in parts.iter() {
            let value: f64 = part.parse::<f64>().unwrap();
            arr.push(Value::from(value));
        }
        return arr;
    }

    /// 将单元格值解析为二维向量数组
    ///
    /// @return 包含多个二维向量的 JSON Map 向量
    ///
    /// 值应以竖线分隔向量，以分号分隔向量内元素，格式为 "x1;y1|x2;y2"
    pub fn to_json_vector2_array(&self) -> Vec<Map<String, Value>> {
        let mut arr: Vec<Map<String, Value>> = Vec::new();
        let parts: Vec<&str> = self.value.split('|').collect();
        for part in parts.iter() {
            let mut vector2: Map<String, Value> = Map::new();
            let subs: Vec<&str> = part.split(';').collect();
            if subs.len() == 2 {
                if let (Ok(x), Ok(y)) = (subs[0].parse::<f64>(), subs[1].parse::<f64>()) {
                    vector2.insert("x".to_string(), Value::from(x));
                    vector2.insert("y".to_string(), Value::from(y));
                }
            }

            arr.push(vector2);
        }
        return arr;
    }

    /// 将单元格值解析为三维向量数组
    ///
    /// @return 包含多个三维向量的 JSON Map 向量
    ///
    /// 值应以竖线分隔向量，以分号分隔向量内元素，格式为 "x1;y1;z1|x2;y2;z2"
    pub fn to_json_vector3_array(&self) -> Vec<Map<String, Value>> {
        let mut arr: Vec<Map<String, Value>> = Vec::new();
        let parts: Vec<&str> = self.value.split('|').collect();
        for part in parts.iter() {
            let mut vector3: Map<String, Value> = Map::new();
            let subs: Vec<&str> = part.split(';').collect();
            if subs.len() == 3 {
                if let (Ok(x), Ok(y), Ok(z)) = (
                    subs[0].parse::<f64>(),
                    subs[1].parse::<f64>(),
                    subs[2].parse::<f64>(),
                ) {
                    vector3.insert("x".to_string(), Value::from(x));
                    vector3.insert("y".to_string(), Value::from(y));
                    vector3.insert("z".to_string(), Value::from(z));
                }
            }

            arr.push(vector3);
        }
        return arr;
    }

    /// 将单元格值解析为四维向量数组
    ///
    /// @return 包含多个四维向量的 JSON Map 向量
    ///
    /// 值应以竖线分隔向量，以分号分隔向量内元素，格式为 "x1;y1;z1;w1|x2;y2;z2;w2"
    pub fn to_json_vector4_array(&self) -> Vec<Map<String, Value>> {
        let mut arr: Vec<Map<String, Value>> = Vec::new();
        let parts: Vec<&str> = self.value.split('|').collect();
        for part in parts.iter() {
            let mut vector4: Map<String, Value> = Map::new();
            let subs: Vec<&str> = part.split(';').collect();
            if subs.len() == 4 {
                if let (Ok(x), Ok(y), Ok(z), Ok(w)) = (
                    subs[0].parse::<f64>(),
                    subs[1].parse::<f64>(),
                    subs[2].parse::<f64>(),
                    subs[3].parse::<f64>(),
                ) {
                    vector4.insert("x".to_string(), Value::from(x));
                    vector4.insert("y".to_string(), Value::from(y));
                    vector4.insert("z".to_string(), Value::from(z));
                    vector4.insert("w".to_string(), Value::from(w));
                }
            }

            arr.push(vector4);
        }
        return arr;
    }
}
