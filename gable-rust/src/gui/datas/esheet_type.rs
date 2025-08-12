#[derive(Debug, Clone, PartialEq)]
pub enum ESheetType {
    /// 普通数据表
    DATA,
    /// 键值表
    KV,
    /// 枚举表
    ENUM,
}
