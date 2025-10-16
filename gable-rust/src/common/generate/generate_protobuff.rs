use crate::{
    common::{
        generate::{
            generate::{self, GenerateFieldInfo, GenerateFieldItem},
            proto_field_info,
        },
        setting::BuildSetting,
        utils,
    },
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
    let field_info: FieldInfo = if let Some(info) = tree_data.to_fields(&build_setting.keyword) {
        info
    } else {
        return;
    };
    let (imports, proto_fields, common_protos) =
        proto_field_info::transition_fields(&field_info, build_setting.is_proto_2);

    let mut tera: Tera = Tera::default();
    let class_key = "templates/proto/class.tpl";
    if let Some(content) = generate::get_template(class_key) {
        tera.add_raw_template(class_key, &content)
            .expect("Proto Failed to add class template");
    }
    let enum_key = "templates/proto/enums.tpl";
    if let Some(content) = generate::get_template(enum_key) {
        tera.add_raw_template(enum_key, &content)
            .expect("Proto Failed to add enum template");
    }

    if common_protos.len() > 0 {
        create_common_proto(&tera, &common_protos, build_setting)
    }

    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("info", &proto_fields);
    context.insert("imports", &imports);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render(class_key, &context)
        }
        ESheetType::Enum => tera.render(enum_key, &context),
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
fn create_common_proto(tera: &Tera, common_protos: &Vec<&EDataType>, build_setting: &BuildSetting) {
    for data_type in common_protos.iter() {
        let class_name;
        let common_fields: Vec<GenerateFieldItem>;
        match data_type {
            EDataType::Vector2 => {
                class_name = "Vector2";
                common_fields = vec![
                    GenerateFieldItem {
                        field_name: "x".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 1,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    GenerateFieldItem {
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
                    GenerateFieldItem {
                        field_name: "x".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 1,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    GenerateFieldItem {
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    GenerateFieldItem {
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
                    GenerateFieldItem {
                        field_name: "x".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 1,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    GenerateFieldItem {
                        field_name: "y".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 2,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    GenerateFieldItem {
                        field_name: "z".to_string(),
                        field_type: "float".to_string(),
                        field_desc: "".to_string(),
                        field_index: 3,
                        field_extend: String::new(),
                        data_type: String::new(),
                    },
                    GenerateFieldItem {
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
        let content_info: GenerateFieldInfo = GenerateFieldInfo {
            primary_num: 0,
            main_fields: vec![],
            fields: common_fields,
        };
        let mut common_context: Context = Context::new();
        common_context.insert("CLASS_NAME", class_name);
        common_context.insert("info", &content_info);
        common_context.insert("imports", &Vec::<String>::new());

        let rendered_result: Result<String, tera::Error> =
            tera.render("templates/proto/class.tpl", &common_context);

        if rendered_result.is_err() {
            log::error!("Template error: {}", rendered_result.unwrap_err());
            continue;
        }
        let rendered: String = rendered_result.unwrap();

        // 写入文件
        let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
            .join(format!("{}.proto", class_name));

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
