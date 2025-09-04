#[derive(Debug, Clone, PartialEq)]
pub enum ESheetType {
    /// 普通数据表
    Normal,
    /// 键值表
    KV,
    /// 枚举表
    Enum,
}
