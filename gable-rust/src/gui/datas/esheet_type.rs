#[derive(Debug, Clone, PartialEq)]
pub enum ESheetType {
    /// 普通数据表
    Normal,
    // 本地化表
    Localize,
    /// 键值表
    KV,
    /// 枚举表
    Enum,
}
