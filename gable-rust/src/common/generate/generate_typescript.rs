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
 * typescript 字段信息
*/
#[derive(serde::Serialize)]
struct TypescriptFieldInfo {
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
 * 生成typescript代码
 * @param build_setting 构建设置
 * @param tree_data 表数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let typescript_fields: Vec<TypescriptFieldInfo> = transition_fields(&fields);
    let mut tera: Tera = Tera::default();
    if let Some(file) = res::load_template("templates/typescript/template.temp") {
        let template_content = file
            .contents_utf8()
            .expect("Failed to read template content");
        tera.add_raw_template("template.temp", template_content)
            .expect("Failed to add template");
    }
    if let Some(file) = res::load_template("templates/typescript/enums.temp") {
        let enum_content = file
            .contents_utf8()
            .expect("Failed to read template content");
        tera.add_raw_template("enums.temp", enum_content)
            .expect("Failed to add template");
    }
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("fields", &typescript_fields);

    // 收集导入的模块
    let imports: Vec<String> = collect_imports(&typescript_fields);
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
        .join(format!("{}.ts", tree_data.file_name));

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
 * 通用字段转换typescript字段
 * @param fields 字段列表
 * @return 转换后的字段列表
*/
fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<TypescriptFieldInfo> {
    let mut typescript_fields: Vec<TypescriptFieldInfo> = Vec::new();
    for field in fields {
        let typescript_type = match field.field_type {
            EDataType::Int | EDataType::Time => "number",
            EDataType::Date => "number", // Date类型在TypeScript中通常用时间戳表示
            EDataType::String | EDataType::Loc => "string",
            EDataType::Boolean => "boolean",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "number",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "number[]",
            EDataType::StringArr => "string[]",
            EDataType::BooleanArr => "boolean[]",
            EDataType::FloatArr => "number[]",
            EDataType::Vector2Arr => "Vector2[]",
            EDataType::Vector3Arr => "Vector3[]",
            EDataType::Vector4Arr => "Vector4[]",
            EDataType::Enum => {
                let mut enum_name = "number";
                if !field.field_link.is_empty() {
                    if let Some(pos) = field.field_link.find("@") {
                        enum_name = &field.field_link[pos + 1..];
                    } else {
                        enum_name = &field.field_link;
                    };
                }
                enum_name
            }
            _ => "string",
        };

        let typescript_field: TypescriptFieldInfo = TypescriptFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: typescript_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        typescript_fields.push(typescript_field);
    }
    return typescript_fields;
}

/**
 * 收集导入的模块
 * @param fields 字段列表
 * @return 导入的模块列表
*/
fn collect_imports(fields: &Vec<TypescriptFieldInfo>) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in fields {
        // 检查是否有需要导入的自定义类型
        if field.field_type != "number"
            && field.field_type != "string"
            && field.field_type != "boolean"
            && !field.field_type.ends_with("[]")
        {
            // 对于自定义类型，添加到导入列表
            if !imports.contains(&field.field_type)
                && field.field_type != "Vector2"
                && field.field_type != "Vector3"
                && field.field_type != "Vector4"
            {
                imports.push(field.field_type.clone());
            }
        } else if field.field_type.ends_with("[]") {
            // 处理数组类型中的自定义类型
            let element_type = &field.field_type[..field.field_type.len() - 2]; // 移除 "[]"
            if element_type != "number"
                && element_type != "string"
                && element_type != "boolean"
                && !imports.contains(&element_type.to_string())
            {
                imports.push(element_type.to_string());
            }
        }
    }

    imports
}
