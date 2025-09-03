use crate::common::constant;

#[derive(Debug, Clone, PartialEq)]
pub enum EDataType {
    /// 未知
    Unknown = 0,
    /// int
    Int = 1,
    /// string
    String = 2,
    /// bool
    Boolean = 3,
    /// float
    Float = 4,
    /// vector2
    Vector2 = 5,
    /// vector3
    Vector3 = 6,
    /// vector4
    Vector4 = 7,
    /// int[]
    IntArr = 8,
    /// string[]
    StringArr = 9,
    /// bool[]
    BooleanArr = 10,
    /// float[]
    FloatArr = 11,
    /// vector2[]
    Vector2Arr = 12,
    /// vector3[]
    Vector3Arr = 13,
    /// vector4[]
    Vector4Arr = 14,
    /// 百分比(两位数的float)
    Percentage = 15,
    /// 千分比(三位数的float)
    Permillage = 16,
    /// 万分比(四位数的float)
    Permian = 17,
    /// 时间(int)，注意此数据类型最大值是1天
    Time = 18,
    /// 日期(int)
    Date = 19,
    /// 枚举，配合链接使用
    Enum = 100,
}

impl EDataType {
    pub fn convert(value: &str) -> EDataType {
        if value.is_empty() {
            return EDataType::String;
        }
        match value {
            constant::DATA_TYPE_KEY_STRING => EDataType::String,
            constant::DATA_TYPE_KEY_INT => EDataType::Int,
            constant::DATA_TYPE_KEY_BOOLEAN => EDataType::Boolean,
            constant::DATA_TYPE_KEY_FLOAT => EDataType::Float,
            constant::DATA_TYPE_KEY_VECTOR2 => EDataType::Vector2,
            constant::DATA_TYPE_KEY_VECTOR3 => EDataType::Vector3,
            constant::DATA_TYPE_KEY_VECTOR4 => EDataType::Vector4,
            constant::DATA_TYPE_KEY_STRING_ARR => EDataType::StringArr,
            constant::DATA_TYPE_KEY_INT_ARR => EDataType::IntArr,
            constant::DATA_TYPE_KEY_BOOLEAN_ARR => EDataType::BooleanArr,
            constant::DATA_TYPE_KEY_FLOAT_ARR => EDataType::FloatArr,
            constant::DATA_TYPE_KEY_VECTOR2_ARR => EDataType::Vector2Arr,
            constant::DATA_TYPE_KEY_VECTOR3_ARR => EDataType::Vector3Arr,
            constant::DATA_TYPE_KEY_VECTOR4_ARR => EDataType::Vector4Arr,
            constant::DATA_TYPE_KEY_PERCENTAGE => EDataType::Percentage,
            constant::DATA_TYPE_KEY_PERMILLAGE => EDataType::Permillage,
            constant::DATA_TYPE_KEY_PERMIAN => EDataType::Permian,
            constant::DATA_TYPE_KEY_TIME => EDataType::Time,
            constant::DATA_TYPE_KEY_DATE => EDataType::Date,
            constant::DATA_TYPE_KEY_ENUM => EDataType::Enum,
            _ => EDataType::Unknown,
        }
    }
}
