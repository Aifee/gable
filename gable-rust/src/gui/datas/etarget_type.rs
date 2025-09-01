use serde::{Deserialize, Serialize};

/// 构建目标类型
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ETargetType {
    /// json
    JSON = 0,
    /// csv
    CSV = 1,
    /// protobuff
    PROTOBUFF = 2,
}
