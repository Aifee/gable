use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use std::{fs, io::Error, path::PathBuf};
use tera::{Context, Tera};

#[derive(serde::Serialize)]
struct CsharpFieldInfo {
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

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let cs_fields: Vec<CsharpFieldInfo> = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/csharp/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.content.sheetname);
    context.insert("fields", &cs_fields);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::KV => tera.render("template.temp", &context),
        ESheetType::Enum => tera.render("enums.temp", &context),
    };
    if rendered_result.is_err() {
        log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.cs", tree_data.content.sheetname));

    let result: Result<(), Error> = fs::write(&target_path, rendered);
    if result.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "导出【{}】成功:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    }
}

fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<CsharpFieldInfo> {
    let mut cs_fields: Vec<CsharpFieldInfo> = Vec::new();
    for field in fields {
        let cs_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date => "long",
            EDataType::String => "string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "int[]",
            EDataType::StringArr => "string[]",
            EDataType::BooleanArr => "bool[]",
            EDataType::FloatArr => "float[]",
            EDataType::Vector2Arr => "Vector2[]",
            EDataType::Vector3Arr => "Vector3[]",
            EDataType::Vector4Arr => "Vector4[]",
            EDataType::Enum => {
                if !field.field_link.is_empty() {
                    field.field_link.as_str()
                } else {
                    "int"
                }
            }
            _ => "string",
        };

        let cs_field: CsharpFieldInfo = CsharpFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: cs_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        cs_fields.push(cs_field);
    }
    return cs_fields;
}
