// localization.rs

/// localization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Localization {
    /// 唯一标识
    pub key: String,

    /// 中文
    pub zh: String,

    /// 英文
    pub en: String,
}
