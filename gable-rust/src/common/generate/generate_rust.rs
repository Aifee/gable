use crate::{
    common::{
        generate::generate::{self, GenerateFieldInfo, GenerateFieldItem, GenerateMainFieldItem},
        setting::BuildSetting,
        utils,
    },
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use std::{fs, io::Error, path::PathBuf};
use tera::{Context, Tera};

/**
 * 生成Rust代码
 * @param build_setting 构建设置
 * @param tree_data 表数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let field_info: FieldInfo = if let Some(info) = tree_data.to_fields(&build_setting.keyword) {
        info
    } else {
        return;
    };
    let rust_fields: GenerateFieldInfo = transition_fields(&field_info);
    let mut tera: Tera = Tera::default();
    let template_key = "templates/rust/template.tpl";
    if let Some(content) = generate::get_template(template_key) {
        tera.add_raw_template(template_key, &content)
            .expect("Rust Failed to add template");
    }
    let enum_key = "templates/rust/enums.tpl";
    if let Some(content) = generate::get_template(enum_key) {
        tera.add_raw_template(enum_key, &content)
            .expect("Rust Failed to add template");
    }
    let mut context: Context = Context::new();
    let struct_name = generate::capitalize_first_letter(&tree_data.file_name);
    context.insert("STRUCT_NAME", &struct_name);
    context.insert("fields", &rust_fields);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render(template_key, &context)
        }
        ESheetType::Enum => tera.render(enum_key, &context),
    };
    if rendered_result.is_err() {
        log::error!("Template error: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap_or(String::new());
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
fn transition_fields(info: &FieldInfo) -> GenerateFieldInfo {
    let mut main_fields: Vec<GenerateMainFieldItem> = Vec::new();
    for field in info.main_fields.iter() {
        let field_type = match field.field_type {
            EDataType::Int => "i32",
            EDataType::Long => "i64",
            EDataType::Float => "f32",
            _ => "String",
        };
        let main_field: GenerateMainFieldItem = GenerateMainFieldItem {
            field_type: field_type.to_string(),
            field_name: field.field_name.clone(),
        };
        main_fields.push(main_field);
    }

    let mut fields: Vec<GenerateFieldItem> = Vec::new();
    for field in info.fields.iter() {
        let rust_type = match field.field_type {
            EDataType::Int | EDataType::Time => "i32",
            EDataType::Date | EDataType::Long => "i64",
            EDataType::Unknown | EDataType::String | EDataType::Loc => "String",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "f32",
            EDataType::Vector2 => "(f32, f32)",
            EDataType::Vector3 => "(f32, f32, f32)",
            EDataType::Vector4 => "(f32, f32, f32, f32)",
            EDataType::IntArr => "Vec<i32>",
            EDataType::LongArr => "Vec<i64>",
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
        };
        let rust_field: GenerateFieldItem = GenerateFieldItem {
            field_name: field.field_name.clone(),
            field_type: rust_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
            field_extend: String::new(),
            data_type: String::new(),
        };
        fields.push(rust_field);
    }
    return GenerateFieldInfo {
        main_fields,
        fields,
    };
}
