use crate::common::constant;

#[derive(Debug, Clone, PartialEq)]
pub enum EDataType {
    /// 未知
    Unknown = 0,
    /// int
    Int = 1,
    /// int64
    Long = 2,
    /// string
    String = 3,
    /// bool
    Boolean = 4,
    /// float
    Float = 5,
    /// vector2
    Vector2 = 6,
    /// vector3
    Vector3 = 7,
    /// vector4
    Vector4 = 8,
    /// int[]
    IntArr = 9,
    /// int64[]
    LongArr = 10,
    /// string[]
    StringArr = 11,
    /// bool[]
    BooleanArr = 12,
    /// float[]
    FloatArr = 13,
    /// vector2[]
    Vector2Arr = 14,
    /// vector3[]
    Vector3Arr = 15,
    /// vector4[]
    Vector4Arr = 16,
    /// 百分比(两位数的float)
    Percentage = 17,
    /// 千分比(三位数的float)
    Permillage = 18,
    /// 万分比(四位数的float)
    Permian = 19,
    /// 时间(int)，注意此数据类型最大值是1天
    Time = 20,
    /// 日期(int)
    Date = 21,
    /// 枚举，配合链接使用
    Enum = 100,
    // 本地化key
    Loc = 101,
}

impl EDataType {
    /**
     * 转换字符串为枚举
     */
    pub fn convert(value: &str) -> EDataType {
        if value.is_empty() {
            return EDataType::String;
        }
        match value {
            constant::DATA_TYPE_KEY_STRING => EDataType::String,
            constant::DATA_TYPE_KEY_INT => EDataType::Int,
            constant::DATA_TYPE_KEY_LONG => EDataType::Long,
            constant::DATA_TYPE_KEY_BOOLEAN => EDataType::Boolean,
            constant::DATA_TYPE_KEY_FLOAT => EDataType::Float,
            constant::DATA_TYPE_KEY_VECTOR2 => EDataType::Vector2,
            constant::DATA_TYPE_KEY_VECTOR3 => EDataType::Vector3,
            constant::DATA_TYPE_KEY_VECTOR4 => EDataType::Vector4,
            constant::DATA_TYPE_KEY_STRING_ARR => EDataType::StringArr,
            constant::DATA_TYPE_KEY_INT_ARR => EDataType::IntArr,
            constant::DATA_TYPE_KEY_LONG_ARR => EDataType::LongArr,
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
            constant::DATA_TYPE_KEY_LOC => EDataType::Loc,
            _ => EDataType::Unknown,
        }
    }
}
