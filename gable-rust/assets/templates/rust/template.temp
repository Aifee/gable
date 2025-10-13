
// {{STRUCT_NAME}}.rs

/// {{STRUCT_NAME}}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct {{STRUCT_NAME}} {
    {%- for field in fields %}
    /// {{field.field_desc}}
    pub {{ field.field_name }}: {{ field.field_type }},
    {%- endfor %}
}