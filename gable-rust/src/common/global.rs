///数据文件类型
pub const GABLE_FILE_TYPE: &str = ".gable";
///临时目录
pub const DIR_TEMP: &str = "__Temps";
pub const DIR_DATA: &str = "__Datas";
pub const DIR_LOG: &str = "__Temps/__Logs";
pub const IGNORED_DIRS: &[&str] = &[".vscode", ".git", "_log", DIR_TEMP, DIR_LOG];

///[数据表单]描述行
pub const TABLE_DATA_ROW_DES: usize = 1;
///[数据表单]字段行
pub const TABLE_DATA_ROW_FIELD: usize = 2;
///[数据表单]类型行
pub const TABLE_DATA_ROW_TYPE: usize = 3;
///[数据表单]平台行
pub const TABLE_DATA_ROW_TARGET: usize = 4;
///[数据表单]关联信息行
pub const TABLE_DATA_ROW_LINK: usize = 5;
/// [数据表单]有效数据起始行数
pub const TABLE_DATA_ROW_TOTAL: usize = 6;

pub const TABLE_KV_ROW_TOTAL: usize = 2;

pub const TABLE_ENUM_ROW_TOTAL: usize = 2;
