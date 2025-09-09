use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use std::{io::Error, path::PathBuf};
use tera::{Context, Tera};

#[derive(serde::Serialize)]
pub struct ProtoFieldInfo {
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
    let (imports, proto_fields) = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/proto2/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.content.sheetname);
    context.insert("fields", &proto_fields);
    context.insert("imports", &imports);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::KV => tera.render("template.proto", &context),
        ESheetType::Enum => tera.render("enums.proto", &context),
    };
    if rendered_result.is_err() {
        log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();

    // 写入文件
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.proto_target_path)
        .join(format!("{}.proto", tree_data.content.sheetname));

    let result: Result<(), Error> = std::fs::write(&target_path, rendered);
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

pub fn transition_fields(fields: &Vec<FieldInfo>) -> (Vec<String>, Vec<ProtoFieldInfo>) {
    let mut imports: Vec<String> = Vec::new();
    let mut proto_fields: Vec<ProtoFieldInfo> = Vec::new();
    for field in fields {
        let proto_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int32",
            EDataType::Date => "int64",
            EDataType::String => "string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 | EDataType::Vector3 | EDataType::Vector4 => "string", // 简化处理
            EDataType::IntArr => "repeated int32",
            EDataType::StringArr => "repeated string",
            EDataType::BooleanArr => "repeated bool",
            EDataType::FloatArr => "repeated float",
            EDataType::Vector2Arr | EDataType::Vector3Arr | EDataType::Vector4Arr => {
                "repeated string"
            }
            EDataType::Enum => {
                if !field.field_link.is_empty() {
                    imports.push(field.field_link.to_string());
                    field.field_link.as_str()
                } else {
                    "int32"
                }
            }
            _ => "string",
        };

        let proto_field: ProtoFieldInfo = ProtoFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: proto_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        proto_fields.push(proto_field);
    }
    return (imports, proto_fields);
}
