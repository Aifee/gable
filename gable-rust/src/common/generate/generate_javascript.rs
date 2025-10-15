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
 * 生成JavaScript代码
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let field_info: FieldInfo = if let Some(info) = tree_data.to_fields(&build_setting.keyword) {
        info
    } else {
        return;
    };
    let javascript_fields: GenerateFieldInfo = transition_fields(&field_info);
    let mut tera: Tera = Tera::default();
    let template_key = "templates/javascript/template.tpl";
    if let Some(content) = generate::get_template(template_key) {
        tera.add_raw_template(template_key, &content)
            .expect("JavaScript Failed to add template");
    }
    let enum_key = "templates/javascript/enums.tpl";
    if let Some(content) = generate::get_template(enum_key) {
        tera.add_raw_template(enum_key, &content)
            .expect("JavaScript Failed to add template");
    }
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("info", &javascript_fields);

    // 收集导入的模块
    let imports: Vec<String> = collect_imports(&javascript_fields);
    context.insert("imports", &imports);

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
        .join(format!("{}.js", tree_data.file_name));

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
 * 通用字段转换成JavaScript字段
 * @param fields 通用字段
 * @return JavaScript字段
*/
fn transition_fields(info: &FieldInfo) -> GenerateFieldInfo {
    let mut main_fields: Vec<GenerateMainFieldItem> = Vec::new();
    for field in info.main_fields.iter() {
        let field_type = match field.field_type {
            EDataType::Int | EDataType::Long | EDataType::Float => "number",
            _ => "string",
        };
        let main_field: GenerateMainFieldItem = GenerateMainFieldItem {
            field_type: field_type.to_string(),
            field_name: field.field_name.clone(),
        };
        main_fields.push(main_field);
    }

    let mut fields: Vec<GenerateFieldItem> = Vec::new();
    for field in info.fields.iter() {
        let js_type = match field.field_type {
            EDataType::Unknown | EDataType::String | EDataType::Loc => "string",
            EDataType::Boolean => "boolean",
            EDataType::Int
            | EDataType::Long
            | EDataType::Time
            | EDataType::Date
            | EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "number",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr | EDataType::LongArr | EDataType::FloatArr => "number[]",
            EDataType::StringArr => "string[]",
            EDataType::BooleanArr => "boolean[]",
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
        };

        let javascript_field: GenerateFieldItem = GenerateFieldItem {
            field_name: field.field_name.clone(),
            field_type: js_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
            field_extend: String::new(),
            data_type: String::new(),
        };
        fields.push(javascript_field);
    }
    return GenerateFieldInfo {
        primary_num: main_fields.len(),
        main_fields,
        fields,
    };
}

/**
 * 收集导入的模块
 * @param fields 字段列表
 * @return 导入的模块列表
*/
fn collect_imports(info: &GenerateFieldInfo) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in info.fields.iter() {
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
