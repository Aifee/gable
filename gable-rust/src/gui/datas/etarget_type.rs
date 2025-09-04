use std::slice::Iter;

use serde::{Deserialize, Serialize};

/// 构建目标类型
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ETargetType {
    /// json
    Json = 0,
    /// csv
    CSV = 1,
    /// protobuff
    Protobuff = 2,
}

impl ETargetType {
    pub fn iter() -> Iter<'static, ETargetType> {
        static VARIANTS: &[ETargetType] =
            &[ETargetType::Json, ETargetType::CSV, ETargetType::Protobuff];
        VARIANTS.iter()
    }
    pub fn to_string(&self) -> &'static str {
        match self {
            ETargetType::Json => "Json",
            ETargetType::CSV => "CSV",
            ETargetType::Protobuff => "Protobuff",
        }
    }
}
