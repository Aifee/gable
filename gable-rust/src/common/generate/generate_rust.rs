use std::{fs, io::Error, path::PathBuf};

use crate::{
    common::{generate::generate, setting::BuildSetting, utils},
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use tera::{Context, Tera};

/**
 * Rust 字段信息
*/
#[derive(serde::Serialize)]
struct RustInfo {
    // 是否是主键
    pub is_key: bool,
    // 字段名称
    pub field_name: String,
    // 字段类型
    pub field_type: String,
    // 字段描述
    pub field_desc: String,
    // 字段序号
    pub field_index: i32,
}

/**
 * 生成Rust代码
 * @param build_setting 构建设置
 * @param tree_data 表数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let rust_fields: Vec<RustInfo> = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/rust/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    let struct_name = generate::capitalize_first_letter(&tree_data.file_name);
    context.insert("STRUCT_NAME", &struct_name);
    context.insert("fields", &rust_fields);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render("template.temp", &context)
        }
        ESheetType::Enum => tera.render("enums.temp", &context),
    };
    if rendered_result.is_err() {
        log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.rs", tree_data.file_name));

    let result: Result<(), Error> = fs::write(&target_path, rendered);
    if result.is_err() {
        log::error!(
            "Export [{}] failed: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "Export [{}] successful: {}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    }
}

/**
 * 通用字段转换Rust字段
 * @param fields 字段列表
 * @return 转换后的字段列表
*/
fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<RustInfo> {
    let mut rust_fields: Vec<RustInfo> = Vec::new();
    for field in fields {
        let rust_type = match field.field_type {
            EDataType::Int | EDataType::Time => "i32",
            EDataType::Date => "i64",
            EDataType::String | EDataType::Loc => "String",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "f32",
            EDataType::Vector2 => "(f32, f32)",
            EDataType::Vector3 => "(f32, f32, f32)",
            EDataType::Vector4 => "(f32, f32, f32, f32)",
            EDataType::IntArr => "Vec<i32>",
            EDataType::StringArr => "Vec<String>",
            EDataType::BooleanArr => "Vec<bool>",
            EDataType::FloatArr => "Vec<f32>",
            EDataType::Vector2Arr => "Vec<(f32, f32)>",
            EDataType::Vector3Arr => "Vec<(f32, f32, f32)>",
            EDataType::Vector4Arr => "Vec<(f32, f32, f32, f32)>",
            EDataType::Enum => {
                let mut enum_name = "i32";
                if !field.field_link.is_empty() {
                    if let Some(pos) = field.field_link.find("@") {
                        enum_name = &field.field_link[pos + 1..];
                    } else {
                        enum_name = &field.field_link;
                    };
                }
                enum_name
            }
            _ => "String",
        };

        let rust_field: RustInfo = RustInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: rust_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        rust_fields.push(rust_field);
    }
    return rust_fields;
}
