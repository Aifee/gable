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
    let (imports, proto_fields, common_protos) = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/proto2/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    if common_protos.len() > 0 {
        create_common_proto(&tera, &common_protos, &build_setting.proto_target_path)
    }
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

pub fn transition_fields(
    fields: &Vec<FieldInfo>,
) -> (Vec<String>, Vec<ProtoFieldInfo>, Vec<&EDataType>) {
    let mut common_proto: Vec<&EDataType> = Vec::new();
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
            EDataType::Vector2 => {
                if !common_proto.contains(&&field.field_type) {
                    common_proto.push(&field.field_type);
                }
                "Vector2"
            }
            EDataType::Vector3 => {
                if !common_proto.contains(&&field.field_type) {
                    common_proto.push(&field.field_type);
                }
                "Vector3"
            }
            EDataType::Vector4 => {
                if !common_proto.contains(&&field.field_type) {
                    common_proto.push(&field.field_type);
                }
                "Vector4"
            }
            EDataType::IntArr => "repeated int32",
            EDataType::StringArr => "repeated string",
            EDataType::BooleanArr => "repeated bool",
            EDataType::FloatArr => "repeated float",
            EDataType::Vector2Arr => {
                if !common_proto.contains(&&EDataType::Vector2) {
                    common_proto.push(&EDataType::Vector2);
                }
                "repeated Vector2"
            }
            EDataType::Vector3Arr => {
                if !common_proto.contains(&&EDataType::Vector3) {
                    common_proto.push(&EDataType::Vector3);
                }
                "repeated Vector2"
            }
            EDataType::Vector4Arr => {
                if !common_proto.contains(&&EDataType::Vector4) {
                    common_proto.push(&EDataType::Vector4);
                }
                "repeated Vector4"
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
    if common_proto.len() > 0 {
        for common_type in common_proto.iter() {
            match common_type {
                EDataType::Vector2 => {
                    imports.push("Vector2".to_string());
                }
                EDataType::Vector3 => {
                    imports.push("Vector3".to_string());
                }
                EDataType::Vector4 => {
                    imports.push("Vector4".to_string());
                }
                _ => {}
            }
        }
    }
    return (imports, proto_fields, common_proto);
}

fn create_common_proto(tera: &Tera, common_protos: &Vec<&EDataType>, target_path: &PathBuf) {
    for data_type in common_protos.iter() {
        let class_name;
        let common_fields: Vec<ProtoFieldInfo>;
        match data_type {
            EDataType::Vector2 => {
                class_name = "Vector2";
                common_fields = vec![
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "x".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 1,
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                    },
                ];
            }
            EDataType::Vector3 => {
                class_name = "Vector3";
                common_fields = vec![
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "x".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 1,
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "z".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 3,
                    },
                ];
            }
            EDataType::Vector4 => {
                class_name = "Vector4";
                common_fields = vec![
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "x".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 1,
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "z".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 3,
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "w".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 4,
                    },
                ];
            }
            _ => {
                continue;
            }
        }
        let mut common_context: Context = Context::new();
        common_context.insert("CLASS_NAME", class_name);
        common_context.insert("fields", &common_fields);
        common_context.insert("imports", &Vec::<String>::new());

        let rendered_result: Result<String, tera::Error> =
            tera.render("template.proto", &common_context);
        if rendered_result.is_err() {
            log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
            continue;
        }
        let rendered: String = rendered_result.unwrap();

        // 写入文件
        let target_path: PathBuf =
            utils::get_absolute_path(&target_path).join(format!("{}.proto", class_name));

        let result: Result<(), Error> = std::fs::write(&target_path, rendered);
        if result.is_err() {
            log::error!(
                "导出【{}】失败:{}",
                class_name,
                target_path.to_str().unwrap()
            );
        } else {
            log::info!(
                "导出【{}】成功:{}",
                class_name,
                target_path.to_str().unwrap()
            );
        }
    }
}
