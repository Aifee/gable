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
 * golang语言生成
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let field_info: FieldInfo = if let Some(info) = tree_data.to_fields(&build_setting.keyword) {
        info
    } else {
        return;
    };
    let go_fields: GenerateFieldInfo = transition_fields(&field_info);
    let mut tera: Tera = Tera::default();
    let class_key = "templates/golang/class.tpl";
    if let Some(content) = generate::get_template(class_key) {
        tera.add_raw_template(class_key, &content)
            .expect("Golang Failed to add class template");
    }
    let enum_key = "templates/golang/enums.tpl";
    if let Some(content) = generate::get_template(enum_key) {
        tera.add_raw_template(enum_key, &content)
            .expect("Golang Failed to add enum template");
    }
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("info", &go_fields);

    // 收集导入的包
    let imports: Vec<String> = collect_imports(&go_fields);
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
    let rendered: String = rendered_result.unwrap_or(String::new());
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.go", tree_data.file_name));

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
 * 通用字段转换Golang字段
 * @param fields 字段列表
 * @return Golang字段列表
*/
fn transition_fields(info: &FieldInfo) -> GenerateFieldInfo {
    let mut main_fields: Vec<GenerateMainFieldItem> = Vec::new();
    for field in info.main_fields.iter() {
        let field_type = match field.field_type {
            EDataType::Int => "int",
            EDataType::Long => "int64",
            EDataType::Float => "float64",
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
        // Go语言字段名需要首字母大写以保证可导出
        let field_name: String = generate::capitalize_first_letter(&field.field_name);

        let go_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date | EDataType::Long => "int64",
            EDataType::Unknown | EDataType::String | EDataType::Loc => "string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float64",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "[]int",
            EDataType::LongArr => "[]int64",
            EDataType::StringArr => "[]string",
            EDataType::BooleanArr => "[]bool",
            EDataType::FloatArr => "[]float64",
            EDataType::Vector2Arr => "[]Vector2",
            EDataType::Vector3Arr => "[]Vector3",
            EDataType::Vector4Arr => "[]Vector4",
            EDataType::Enum => {
                let mut enum_name = "int".to_string();
                if !field.field_link.is_empty() {
                    if let Some(pos) = field.field_link.find("@") {
                        let part = &field.field_link[pos + 1..];
                        enum_name = generate::capitalize_first_letter(part);
                    } else {
                        enum_name = generate::capitalize_first_letter(&field.field_link);
                    };
                }
                &enum_name.clone()
            }
        };
        let go_field: GenerateFieldItem = GenerateFieldItem {
            field_name: field_name,
            field_type: go_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
            field_extend: String::new(),
            data_type: String::new(),
        };
        fields.push(go_field);
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
        if field.field_type != "int"
            && field.field_type != "int64"
            && field.field_type != "string"
            && field.field_type != "bool"
            && field.field_type != "float64"
            && !field.field_type.starts_with("[]")
        {
            // 对于自定义类型，可能需要添加导入路径
            if !imports.contains(&field.field_type)
                && field.field_type != "Vector2"
                && field.field_type != "Vector3"
                && field.field_type != "Vector4"
            {
                // 这里可以添加实际的导入路径逻辑
                // 暂时使用简单的处理方式
                imports.push(format!(
                    "example.com/project/{}",
                    field.field_type.to_lowercase()
                ));
            }
        } else if field.field_type.starts_with("[]") {
            // 处理数组类型中的自定义类型
            let element_type = &field.field_type[2..]; // 移除 "[]"
            if element_type != "int"
                && element_type != "string"
                && element_type != "bool"
                && element_type != "float64"
                && !imports.contains(&element_type.to_string())
            {
                imports.push(format!(
                    "example.com/project/{}",
                    element_type.to_lowercase()
                ));
            }
        }
    }

    imports
}
