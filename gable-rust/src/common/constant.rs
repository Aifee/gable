use lazy_static::lazy_static;
use std::path::PathBuf;

/**
 * 数据文件类型
 */
pub const GABLE_FILE_TYPE: &str = ".gable";
/**
 * 临时目录
*/
pub const DIR_TEMP: &str = "__Temps";
/**
 * 数据目录
 */
pub const DIR_DATA: &str = "__Datas";
/**
 * 日志目录
 */
pub const DIR_LOG: &str = "__Temps/__Logs";
/**
 * 忽略的目录
 */
pub const IGNORED_DIRS: &[&str] = &[".vscode", ".git", "_log", DIR_TEMP, DIR_DATA];
/**
 * 设置文件名
*/
pub const SETTING_PREFS: &str = "appPrefs.json";
/**
 * EXCEL文件扩展名
*/
pub const EXCEL_EXTENSION: &str = ".xlsx";
/**
 * 日期格时间格式
*/
pub const NUMBER_FORMAT_DATE: &str = "YYYY-mm-dd hh:mm:ss";
/**
 * 单元格时间格式
*/
pub const NUMBER_FORMAT_TIME: &str = "hh:mm:ss";

/**
 * 单元格百分比格式
*/
pub const NUMBER_FORMAT_PERCENTAGE: &str = "0%";
/**
 * 单元格千分比格式
 */
pub const NUMBER_FORMAT_PERMILLAGE: &str = "0‰";
/**
 * 单元格万分比格式
 * */
pub const NUMBER_FORMAT_PERMIAN: &str = "0‱";
lazy_static! {
    /**
     * 当前运行目录
    */
    pub static ref EXE_DIR: PathBuf = {
        let exe_path = std::env::current_exe().expect("无法获取当前可执行文件路径");
        exe_path
            .parent()
            .expect("无法获取可执行文件所在目录")
            .to_path_buf()
    };
}

/**
 * 表单最小行数
*/
pub const FORM_MIN_ROW: usize = 47;
/**
 * 表单最小列数
*/
pub const FORM_MIN_COL: usize = 25;

/**
 * [数据表单]描述行
*/
pub const TABLE_NORMAL_ROW_DESC: usize = 0;
/**
 * [数据表单]字段行
*/
pub const TABLE_NORMAL_ROW_FIELD: usize = 1;
/**
 * [数据表单]类型行
*/
pub const TABLE_NORMAL_ROW_TYPE: usize = 2;
/**
 * [数据表单]平台行
*/
pub const TABLE_NORMAL_ROW_KEYWORD: usize = 3;
/**
 * [数据表单]关联信息行
*/
pub const TABLE_NORMAL_ROW_LINK: usize = 4;
/**
 * [数据表单]有效数据起始行数
 */
pub const TABLE_NORMAL_ROW_TOTAL: usize = 5;

/**
 * [本地化表单]描述行
*/
pub const TABLE_LOCALIZE_ROW_DESC: usize = 0;
/**
 * [本地化表单]字段行
*/
pub const TABLE_LOCALIZE_ROW_FIELD: usize = 1;
/**
 * [本地化表单]类型行
*/
pub const TABLE_LOCALIZE_ROW_TYPE: usize = 2;
/**
 * [本地化表单]平台行
*/
// pub const TABLE_LOCALIZE_ROW_KEYWORD: usize = 3;
/**
 * [本地化表单]关联信息行
*/
// pub const TABLE_LOCALIZE_ROW_LINK: usize = 4;
/**
 * [本地化表单]有效数据起始行数
*/
pub const TABLE_LOCALIZE_ROW_TOTAL: usize = 5;

/**
 * [KV表单]字段行
*/
pub const TABLE_KV_COL_FIELD: usize = 0;
/**
 * [KV表单]类型列
*/
pub const TABLE_KV_COL_TYPE: usize = 1;
/**
 * [KV表单]平台行
*/
pub const TABLE_KV_COL_KEYWORD: usize = 2;
/**
 * [KV表单]关联信息列
*/
pub const TABLE_KV_COL_LINK: usize = 3;
/**
 * [KV表单]值列
*/
pub const TABLE_KV_COL_VALUE: usize = 4;
/**
 * [数据表单]描述行
*/
pub const TABLE_KV_COL_DESC: usize = 5;
/**
 * [KV表单]有效数据起始行数
*/
pub const TABLE_KV_ROW_TOTAL: usize = 1;

/**
 * [枚举表单]字段行
*/
pub const TABLE_ENUM_COL_FIELD: usize = 0;
/**
 * [枚举表单]值列
*/
pub const TABLE_ENUM_COL_VALUE: usize = 1;
/**
 * [枚举表单]描述列
*/
pub const TABLE_ENUM_COL_DESC: usize = 2;
/**
 * [枚举表单]有效数据起始行数
*/
pub const TABLE_ENUM_ROW_TOTAL: usize = 1;

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
pub const DATA_TYPE_KEY_PERCENTAGE: &str = "%";
pub const DATA_TYPE_KEY_PERMILLAGE: &str = "‰";
pub const DATA_TYPE_KEY_PERMIAN: &str = "‱";
pub const DATA_TYPE_KEY_TIME: &str = "time";
pub const DATA_TYPE_KEY_DATE: &str = "date";
pub const DATA_TYPE_KEY_ENUM: &str = "enum";
pub const DATA_TYPE_KEY_LOC: &str = "loc";
pub const DATA_TYPE_KEYS: &[&str] = &[
    DATA_TYPE_KEY_INT,
    DATA_TYPE_KEY_STRING,
    DATA_TYPE_KEY_BOOLEAN,
    DATA_TYPE_KEY_FLOAT,
    DATA_TYPE_KEY_VECTOR2,
    DATA_TYPE_KEY_VECTOR3,
    DATA_TYPE_KEY_VECTOR4,
    DATA_TYPE_KEY_INT_ARR,
    DATA_TYPE_KEY_STRING_ARR,
    DATA_TYPE_KEY_BOOLEAN_ARR,
    DATA_TYPE_KEY_FLOAT_ARR,
    DATA_TYPE_KEY_VECTOR2_ARR,
    DATA_TYPE_KEY_VECTOR3_ARR,
    DATA_TYPE_KEY_VECTOR4_ARR,
    DATA_TYPE_KEY_PERCENTAGE,
    DATA_TYPE_KEY_PERMILLAGE,
    DATA_TYPE_KEY_PERMIAN,
    DATA_TYPE_KEY_TIME,
    DATA_TYPE_KEY_DATE,
    DATA_TYPE_KEY_ENUM,
    DATA_TYPE_KEY_LOC,
];
