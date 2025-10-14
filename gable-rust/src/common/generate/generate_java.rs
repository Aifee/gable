use std::{fs, io::Error, path::PathBuf};

use crate::{
    common::{res, setting::BuildSetting, utils},
    gui::datas::{
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use tera::{Context, Tera};

/**
 * Java字段信息
*/
#[derive(serde::Serialize)]
struct JavaFieldInfo {
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
 * 生成Java代码
 * @param build_setting 构建设置
 * @param tree_data 树结构数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let java_fields: Vec<JavaFieldInfo> = transition_fields(&fields);
    let mut tera: Tera = Tera::default();
    if let Some(file) = res::load_template("templates/java/template.tpl") {
        let template_content = file
            .contents_utf8()
            .expect("Failed to read template content");
        tera.add_raw_template("template.tpl", template_content)
            .expect("Failed to add template");
    }
    if let Some(file) = res::load_template("templates/java/enums.tpl") {
        let enum_content = file
            .contents_utf8()
            .expect("Failed to read template content");
        tera.add_raw_template("enums.tpl", enum_content)
            .expect("Failed to add template");
    }
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("fields", &java_fields);

    // 收集导入的类
    let imports: Vec<String> = collect_imports(&java_fields);
    context.insert("imports", &imports);

    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render("template.tpl", &context)
        }
        ESheetType::Enum => tera.render("enums.tpl", &context),
    };
    if rendered_result.is_err() {
        log::error!("Template error: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.java", tree_data.file_name));

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
 * 通用字段转换Java字段
 * @param fields 字段列表
 * @return Java字段列表
*/
fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<JavaFieldInfo> {
    let mut java_fields: Vec<JavaFieldInfo> = Vec::new();
    for field in fields {
        let java_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date => "long",
            EDataType::String | EDataType::Loc => "String",
            EDataType::Boolean => "boolean",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "int[]",
            EDataType::StringArr => "String[]",
            EDataType::BooleanArr => "boolean[]",
            EDataType::FloatArr => "float[]",
            EDataType::Vector2Arr => "Vector2[]",
            EDataType::Vector3Arr => "Vector3[]",
            EDataType::Vector4Arr => "Vector4[]",
            EDataType::Enum => {
                let mut enum_name = "int";
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

        let java_field: JavaFieldInfo = JavaFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: java_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        java_fields.push(java_field);
    }
    return java_fields;
}

/**
 * 收集导入的模块
 * @param fields 字段列表
 * @return 模块列表
*/
fn collect_imports(fields: &Vec<JavaFieldInfo>) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in fields {
        // 为数组类型添加必要的导入
        if field.field_type.contains("[]")
            && !field.field_type.starts_with("int")
            && !field.field_type.starts_with("float")
            && !field.field_type.starts_with("boolean")
        {
            // 对于自定义类的数组类型，我们可能需要添加相关的导入
            let base_type = &field.field_type[..field.field_type.len() - 2]; // 移除 "[]"
            if base_type != "String" && !imports.contains(&base_type.to_string()) {
                imports.push(base_type.to_string());
            }
        } else if !field.field_type.starts_with("int")
            && !field.field_type.starts_with("float")
            && !field.field_type.starts_with("boolean")
            && !field.field_type.starts_with("String")
            && !field.field_type.starts_with("long")
        {
            // 对于自定义类，添加到导入列表
            if !imports.contains(&field.field_type) {
                imports.push(field.field_type.clone());
            }
        }
    }

    imports
}
