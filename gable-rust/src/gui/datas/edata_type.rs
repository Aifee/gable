#[derive(Debug, Clone, PartialEq)]
pub enum EDataType {
    // 未知
    Unknown = 0,
    // int
    INT = 1,
    // string
    STRING = 2,
    // bool
    BOOLEAN = 3,
    // float
    FLOAT = 4,
    // vector2
    VECTOR2 = 5,
    // vector3
    VECTOR3 = 6,
    // vector4
    VECTOR4 = 7,
    // int[]
    INT_ARR = 8,
    // string[]
    STRING_ARR = 9,
    // bool[]
    BOOLEAN_ARR = 10,
    // float[]
    FLOAT_ARR = 11,
    // vector2[]
    VECTOR2_ARR = 12,
    // vector3[]
    VECTOR3_ARR = 13,
    // vector4[]
    VECTOR4_ARR = 14,
    // 百分比(两位数的float)
    PERCENTAGE = 15,
    // 千分比(三位数的float)
    PERMILLAGE = 16,
    // 万分比(四位数的float)
    PERMIAN = 17,
    // 时间(int)
    TIME = 18,
    // 枚举，配合链接使用
    ENUM = 100,
}
