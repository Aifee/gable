use std::slice::Iter;

use serde::{Deserialize, Serialize};

/// 构建目标类型
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ETargetType {
    /// json
    Json = 0,
    /// csv
    CSV = 1,
    /// xml
    Xml = 2,
    /// yaml
    Yaml = 3,
    /// protobuff
    Protobuff = 4,
}

impl ETargetType {
    pub fn iter() -> Iter<'static, ETargetType> {
        static VARIANTS: &[ETargetType] = &[
            ETargetType::Json,
            ETargetType::CSV,
            ETargetType::Xml,
            ETargetType::Yaml,
            ETargetType::Protobuff,
        ];
        VARIANTS.iter()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            ETargetType::Json => "Json",
            ETargetType::CSV => "CSV",
            ETargetType::Xml => "Xml",
            ETargetType::Yaml => "Yaml",
            ETargetType::Protobuff => "Protobuff",
        }
    }
}
