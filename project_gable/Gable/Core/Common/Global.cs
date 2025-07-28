using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace Gable.Core.Common
{
    public static class Global
    {
        /// <summary>
        /// 临时目录
        /// </summary>
        public static string PROJECT_TEMP_DIR = "__Temps";

        /// <summary>
        /// APP数据目录
        /// </summary>
        public static string PROJECT_DATA_DIR = "__Datas";

        /// <summary>
        /// 配置文件名
        /// </summary>
        public static string SETTING_PREFS = "appPrefs.yaml";

        /// <summary>
        /// excel文件后缀
        /// </summary>
        public static string EXCEL_FILE_TYPE = ".xlsx";

        /// <summary>
        /// 忽略检索目录
        /// </summary>
        public static string[] IGNORE_PATH =
        {
            ".vscode",
            ".git",
            "__pycache__",
            "_log",
            PROJECT_TEMP_DIR,
            PROJECT_DATA_DIR,
        };

        /// <summary>
        /// [数据表单]描述行
        /// </summary>
        public static int TABLE_DATA_ROW_DES = 1;

        /// <summary>
        /// [数据表单]字段行
        /// </summary>
        public static int TABLE_DATA_ROW_FIELD = 2;

        /// <summary>
        /// [数据表单]类型行
        /// </summary>
        public static int TABLE_DATA_ROW_TYPE = 3;

        /// <summary>
        /// [数据表单]平台行
        /// </summary>
        public static int TABLE_DATA_ROW_TARGET = 4;

        /// <summary>
        /// [数据表单]关联信息行
        /// </summary>
        public static int TABLE_DATA_ROW_LINK = 5;

        /// <summary>
        /// [数据表单]有效数据起始行数
        /// </summary>
        public static int TABLE_DATA_ROW_TOTAL = 6;

        //# 表头填充颜色样式
        //        public static Dictionary<string,object> EXCEL_DATA_HEAD_THEME = {
        //    TABLE_DATA_ROW_DES: {"theme": 3, "tint": 0.8},
        //    TABLE_DATA_ROW_FIELD: { "theme": 6, "tint": 0.8},
        //    TABLE_DATA_ROW_TYPE: { "theme": 7, "tint": 0.8},
        //    TABLE_DATA_ROW_TARGET: { "theme": 8, "tint": 0.8},
        //    TABLE_DATA_ROW_LINK: { "theme": 9, "tint": 0.8},
        //}

        /// <summary>
        /// [数据表单]字段列
        /// </summary>
        public static int TABLE_ENUM_COL_FIELD = 1;

        /// <summary>
        /// [数据表单]数值列
        /// </summary>
        public static int TABLE_ENUM_COL_VALUE = 2;

        /// <summary>
        /// [数据表单]描述列
        /// </summary>
        public static int TABLE_ENUM_COL_DES = 3;

        /// <summary>
        /// [枚举表单]有效数据起始行数
        /// </summary>
        public static int TABLE_ENUM_ROW_TOTAL = 2;

        /// <summary>
        ///  [数据表单]字段列
        /// </summary>
        public static int TABLE_KV_COL_FIELD = 1;

        /// <summary>
        /// [数据表单]类型行
        /// </summary>
        public static int TABLE_KV_COL_TYPE = 2;

        /// <summary>
        /// [数据表单]平台列
        /// </summary>
        public static int TABLE_KV_COL_TARGET = 3;

        /// <summary>
        /// [数据表单]值列
        /// </summary>
        public static int TABLE_KV_COL_VALUE = 4;

        /// <summary>
        /// [数据表单]描述列
        /// </summary>
        public static int TABLE_KV_COL_DES = 5;

        /// <summary>
        /// [枚举表单]有效数据起始行数
        /// </summary>
        public static int TABLE_KV_ROW_TOTAL = 2;

        public static string EXCEL_FORMAT_TIME = "%Y-%m-%d %H:%M:%S";

        /// <summary>
        /// 单元格时间格式
        /// </summary>
        public static string NUMBER_FORMAT_TIME = "Y-m-d hh:mm:ss";

        /// <summary>
        /// 单元格百分比格式
        /// </summary>
        public static string NUMBER_FORMAT_PERCENTAGE = "0%";

        /// <summary>
        /// 单元格千分比格式
        /// </summary>
        public static string NUMBER_FORMAT_PERMILLAGE = "0‰";

        /// <summary>
        /// 单元格万分比格式
        /// </summary>
        public static string NUMBER_FORMAT_PERMIAN = "0‱";

        public static string THEME_AUTO = "auto";
        public static string THEME_LIGHT = "light";
        public static string THEME_DARK = "dark";

        /// <summary>
        /// 数据文件类型
        /// </summary>
        public static string GABLE_FILE_TYPE = ".gable";

        /// <summary>
        /// 枚举表目录
        /// </summary>
        public static string ENUM_TABLE_FOLDER = "enums";

        /// <summary>
        /// 键值表目录
        /// </summary>
        public static string KV_TABLE_FOLDER = "kvs";

        /// <summary>
        /// 新建表单默认名字
        /// </summary>
        public static string TABLE_NEW_NAME = "newgable";

        /// <summary>
        /// 新建Sheet默认名字
        /// </summary>
        public static string SHEET_NEW_NAME = "@newsheet.gable";

        /// <summary>
        /// 新建文件夹默认名
        /// </summary>
        public static string FOLDER_NEW_NAME = "folder";

        //THIN_BORDER = Border(
        //    left = Side(style = "thin", color = "FF000000"),
        //    right = Side(style = "thin", color = "FF000000"),
        //    top = Side(style = "thin", color = "FF000000"),
        //    bottom = Side(style = "thin", color = "FF000000"),
        //)

        /// <summary>
        ///  window系统
        /// </summary>
        public static string SYSTEM_WINDOW = "Windows";

        /// <summary>
        /// mac os 系统
        /// </summary>
        public static string SYSTEM_MAC = "Darwin";

        /// <summary>
        /// linux 系统
        /// </summary>
        /// public static string SYSTEM_LINUX = "Linux";
    }
}
