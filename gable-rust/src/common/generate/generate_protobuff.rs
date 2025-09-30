use crate::{
    common::{generate::proto_field_info::ProtoFieldInfo, setting::BuildSetting, utils},
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use std::{io::Error, path::PathBuf};
use tera::{Context, Tera};

/**
 * 生成ProtoBuff文件
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let (imports, proto_fields, common_protos) =
        ProtoFieldInfo::transition_fields(&fields, build_setting.is_proto_2);
    let templat_path = if build_setting.is_proto_2 {
        "assets/templates/proto2/*"
    } else {
        "assets/templates/proto3/*"
    };
    let tera_result: Result<Tera, tera::Error> = Tera::new(templat_path);
    if tera_result.is_err() {
        log::error!(
            "Failed to create Tera template: {}",
            tera_result.unwrap_err()
        );
        return;
    }
    let tera: Tera = tera_result.unwrap();
    if common_protos.len() > 0 {
        create_common_proto(&tera, &common_protos, &build_setting.script_path)
    }
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("fields", &proto_fields);
    context.insert("imports", &imports);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render("template.proto", &context)
        }
        ESheetType::Enum => tera.render("enums.proto", &context),
    };
    if rendered_result.is_err() {
        log::error!("Template error: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();

    // 写入文件
    let proto_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.proto", tree_data.file_name));

    let result: Result<(), Error> = std::fs::write(&proto_path, rendered);
    if result.is_err() {
        log::error!(
            "Export [{}] failed: {}",
            build_setting.display_name,
            proto_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "Export [{}] successful: {}",
            build_setting.display_name,
            proto_path.to_str().unwrap()
        );
    }
}

/**
 * 生成公共的proto文件
 * @param tera 模板
 * @param common_protos 公共字段信息
 * @param target_path 生成目录
*/
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
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                        field_extend: String::new(),
                        data_type: String::new(),
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
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "z".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 3,
                        field_extend: String::new(),
                        data_type: String::new(),
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
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "z".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 3,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    ProtoFieldInfo {
                        is_key: false,
                        field_name: "w".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 4,
                        field_extend: String::new(),
                        data_type: String::new(),
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
            log::error!("Template error: {}", rendered_result.unwrap_err());
            continue;
        }
        let rendered: String = rendered_result.unwrap();

        // 写入文件
        let target_path: PathBuf =
            utils::get_absolute_path(&target_path).join(format!("{}.proto", class_name));

        let result: Result<(), Error> = std::fs::write(&target_path, rendered);
        if result.is_err() {
            log::error!(
                "Export [{}] failed: {}",
                class_name,
                target_path.to_str().unwrap()
            );
        } else {
            log::info!(
                "Export [{}] successful: {}",
                class_name,
                target_path.to_str().unwrap()
            );
        }
    }
}
