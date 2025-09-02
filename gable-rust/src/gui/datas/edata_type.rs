use crate::common::constant;

#[derive(Debug, Clone, PartialEq)]
pub enum EDataType {
    /// 未知
    Unknown = 0,
    /// int
    INT = 1,
    /// string
    STRING = 2,
    /// bool
    BOOLEAN = 3,
    /// float
    FLOAT = 4,
    /// vector2
    VECTOR2 = 5,
    /// vector3
    VECTOR3 = 6,
    /// vector4
    VECTOR4 = 7,
    /// int[]
    INT_ARR = 8,
    /// string[]
    STRING_ARR = 9,
    /// bool[]
    BOOLEAN_ARR = 10,
    /// float[]
    FLOAT_ARR = 11,
    /// vector2[]
    VECTOR2_ARR = 12,
    /// vector3[]
    VECTOR3_ARR = 13,
    /// vector4[]
    VECTOR4_ARR = 14,
    /// 百分比(两位数的float)
    PERCENTAGE = 15,
    /// 千分比(三位数的float)
    PERMILLAGE = 16,
    /// 万分比(四位数的float)
    PERMIAN = 17,
    /// 时间(int)，注意此数据类型最大值是1天
    TIME = 18,
    /// 日期(int)
    DATE = 19,
    /// 枚举，配合链接使用
    ENUM = 100,
}

impl EDataType {
    pub fn convert(value: &str) -> EDataType {
        if value.is_empty() {
            return EDataType::STRING;
        }
        match value {
            constant::DATA_TYPE_KEY_STRING => EDataType::STRING,
            constant::DATA_TYPE_KEY_INT => EDataType::INT,
            constant::DATA_TYPE_KEY_BOOLEAN => EDataType::BOOLEAN,
            constant::DATA_TYPE_KEY_FLOAT => EDataType::FLOAT,
            constant::DATA_TYPE_KEY_VECTOR2 => EDataType::VECTOR2,
            constant::DATA_TYPE_KEY_VECTOR3 => EDataType::VECTOR3,
            constant::DATA_TYPE_KEY_VECTOR4 => EDataType::VECTOR4,
            constant::DATA_TYPE_KEY_STRING_ARR => EDataType::STRING_ARR,
            constant::DATA_TYPE_KEY_INT_ARR => EDataType::INT_ARR,
            constant::DATA_TYPE_KEY_BOOLEAN_ARR => EDataType::BOOLEAN_ARR,
            constant::DATA_TYPE_KEY_FLOAT_ARR => EDataType::FLOAT_ARR,
            constant::DATA_TYPE_KEY_VECTOR2_ARR => EDataType::VECTOR2_ARR,
            constant::DATA_TYPE_KEY_VECTOR3_ARR => EDataType::VECTOR3_ARR,
            constant::DATA_TYPE_KEY_VECTOR4_ARR => EDataType::VECTOR4_ARR,
            constant::DATA_TYPE_KEY_PERCENTAGE => EDataType::PERCENTAGE,
            constant::DATA_TYPE_KEY_PERMILLAGE => EDataType::PERMILLAGE,
            constant::DATA_TYPE_KEY_PERMIAN => EDataType::PERMIAN,
            constant::DATA_TYPE_KEY_TIME => EDataType::TIME,
            constant::DATA_TYPE_KEY_DATE => EDataType::DATE,
            constant::DATA_TYPE_KEY_ENUM => EDataType::ENUM,
            _ => EDataType::Unknown,
        }
    }
}
