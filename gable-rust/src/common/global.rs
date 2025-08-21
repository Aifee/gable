///数据文件类型
pub const GABLE_FILE_TYPE: &str = ".gable";
///临时目录
pub const DIR_TEMP: &str = "__Temps";
pub const DIR_DATA: &str = "__Datas";
pub const DIR_LOG: &str = "__Temps/__Logs";
pub const IGNORED_DIRS: &[&str] = &[".vscode", ".git", "_log", DIR_TEMP, DIR_DATA];
/// EXCEL文件扩展名
pub const EXCEL_EXTENSION: &str = ".xlsx";
/// 单元格百分比格式
pub const NUMBER_FORMAT_PERCENTAGE: &str = "0%";
/// 单元格千分比格式
pub const NUMBER_FORMAT_PERMILLAGE: &str = "0‰";
/// 单元格万分比格式
pub const NUMBER_FORMAT_PERMIAN: &str = "0‱";

///[数据表单]描述行
pub const TABLE_DATA_ROW_DES: u32 = 1;
///[数据表单]字段行
pub const TABLE_DATA_ROW_FIELD: u32 = 2;
///[数据表单]类型行
pub const TABLE_DATA_ROW_TYPE: u32 = 3;
///[数据表单]平台行
pub const TABLE_DATA_ROW_TARGET: u32 = 4;
///[数据表单]关联信息行
pub const TABLE_DATA_ROW_LINK: u32 = 5;
/// [数据表单]有效数据起始行数
pub const TABLE_DATA_ROW_TOTAL: u32 = 6;

///[KV表单]类型列
pub const TABLE_KV_COL_TYPE: u32 = 2;
///[KV表单]有效数据起始行数
pub const TABLE_KV_ROW_TOTAL: u32 = 2;
///[枚举表单]有效数据起始行数
pub const TABLE_ENUM_ROW_TOTAL: u32 = 2;

pub const DATA_TYPE_KEY_INT: &str = "int";
pub const DATA_TYPE_KEY_STRING: &str = "string";
pub const DATA_TYPE_KEY_BOOLEAN: &str = "bool";
pub const DATA_TYPE_KEY_FLOAT: &str = "float";
pub const DATA_TYPE_KEY_VECTOR2: &str = "vector2";
pub const DATA_TYPE_KEY_VECTOR3: &str = "vector3";
pub const DATA_TYPE_KEY_VECTOR4: &str = "vector4";
pub const DATA_TYPE_KEY_INT_ARR: &str = "int[]";
pub const DATA_TYPE_KEY_STRING_ARR: &str = "string[]";
pub const DATA_TYPE_KEY_BOOLEAN_ARR: &str = "bool[]";
pub const DATA_TYPE_KEY_FLOAT_ARR: &str = "float[]";
pub const DATA_TYPE_KEY_VECTOR2_ARR: &str = "vector2[]";
pub const DATA_TYPE_KEY_VECTOR3_ARR: &str = "vector3[]";
pub const DATA_TYPE_KEY_VECTOR4_ARR: &str = "vector4[]";
pub const DATA_TYPE_KEY_PERCENTAGE: &str = "percentage";
pub const DATA_TYPE_KEY_PERMILLAGE: &str = "permillage";
pub const DATA_TYPE_KEY_PERMIAN: &str = "permian";
pub const DATA_TYPE_KEY_TIME: &str = "time";
pub const DATA_TYPE_KEY_ENUM: &str = "enum";
