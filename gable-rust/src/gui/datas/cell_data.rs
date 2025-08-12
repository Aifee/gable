use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CellData {
    pub row: u32,
    pub column: u32,
    #[serde(default = "default_string", deserialize_with = "deserialize_string")]
    pub value: String,
}

fn default_string() -> String {
    String::new()
}

fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::value::Value::deserialize(deserializer)?;
    Ok(match value {
        serde_json::value::Value::String(s) => s,
        serde_json::value::Value::Number(n) => n.to_string(),
        serde_json::value::Value::Bool(b) => b.to_string(),
        serde_json::value::Value::Null => String::new(),
        _ => value.to_string(),
    })
}
