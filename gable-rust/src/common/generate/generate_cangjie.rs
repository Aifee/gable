use std::{fs, io::Error, path::PathBuf};

use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use tera::{Context, Tera};

/**
 * 仓颉语言字段信息
*/
#[derive(serde::Serialize)]
struct CangjieFieldInfo {
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
 * 生成仓颉语言脚本
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let cangjie_fields: Vec<CangjieFieldInfo> = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/cangjie/*");
    if tera_result.is_err() {
        log::error!(
            "Failed to create Tera template: {}",
            tera_result.unwrap_err()
        );
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("fields", &cangjie_fields);

    // 收集导入的模块
    let imports: Vec<String> = collect_imports(&cangjie_fields);
    context.insert("imports", &imports);

    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render("template.temp", &context)
        }
        ESheetType::Enum => tera.render("enums.temp", &context),
    };
    if rendered_result.is_err() {
        log::error!("Template error: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.cj", tree_data.file_name));

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
 * 通用字段转换仓颉语法字段
 * @param fields 通用字段信息
 * @return 仓颉语法字段信息
*/
fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<CangjieFieldInfo> {
    let mut cangjie_fields: Vec<CangjieFieldInfo> = Vec::new();
    for field in fields {
        let cangjie_type = match field.field_type {
            EDataType::Int | EDataType::Time => "Int32",
            EDataType::Date => "Int64",
            EDataType::String | EDataType::Loc => "String",
            EDataType::Boolean => "Bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "Float32",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "Array<Int32>",
            EDataType::StringArr => "Array<String>",
            EDataType::BooleanArr => "Array<Bool>",
            EDataType::FloatArr => "Array<Float32>",
            EDataType::Vector2Arr => "Array<Vector2>",
            EDataType::Vector3Arr => "Array<Vector3>",
            EDataType::Vector4Arr => "Array<Vector4>",
            EDataType::Enum => {
                let mut enum_name = "Int32";
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

        let cangjie_field: CangjieFieldInfo = CangjieFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: cangjie_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        cangjie_fields.push(cangjie_field);
    }
    return cangjie_fields;
}

/**
 * 收集要导入的模块
 * @param fields 字段信息
 * @return 导入的模块列表
*/
fn collect_imports(fields: &Vec<CangjieFieldInfo>) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in fields {
        // 检查是否需要导入自定义类型
        if field.field_type != "Int32"
            && field.field_type != "Int64"
            && field.field_type != "String"
            && field.field_type != "Bool"
            && field.field_type != "Float32"
            && !field.field_type.starts_with("Array<")
        {
            // 对于自定义类型，添加到导入列表
            if !imports.contains(&field.field_type)
                && field.field_type != "Vector2"
                && field.field_type != "Vector3"
                && field.field_type != "Vector4"
            {
                imports.push(format!("gable.{}", field.field_type));
            }
        } else if field.field_type.starts_with("Array<") {
            // 处理数组类型中的自定义类型
            let element_type = &field.field_type[6..field.field_type.len() - 1]; // 移除 "Array<>"
            if element_type != "Int32"
                && element_type != "String"
                && element_type != "Bool"
                && element_type != "Float32"
                && !imports.contains(&element_type.to_string())
            {
                imports.push(format!("gable.{}", element_type));
            }
        }
    }

    imports
}
