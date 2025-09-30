
// localization.rs

/// localization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct localization {
    
    /// 唯一标识
    pub key: String,
    
    /// 中文
    pub zh: String,
    
    /// 英文
    pub en: String,
    
}

impl localization {
    
    /// 获取 唯一标识
    pub fn key(&self) -> &String {
        &self.key
    }
    
    /// 获取 中文
    pub fn zh(&self) -> &String {
        &self.zh
    }
    
    /// 获取 英文
    pub fn en(&self) -> &String {
        &self.en
    }
    
}