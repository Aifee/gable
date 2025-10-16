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
 * 生成C/C++代码
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let field_info: FieldInfo = if let Some(info) = tree_data.to_fields(&build_setting.keyword) {
        info
    } else {
        return;
    };

    let cpp_fields: GenerateFieldInfo = transition_fields(&field_info);
    let mut tera: Tera = Tera::default();
    let class_key = "templates/cpp/class.tpl";
    if let Some(content) = generate::get_template(class_key) {
        tera.add_raw_template(class_key, &content)
            .expect("Cpp Failed to add class template");
    }
    let enum_key = "templates/cpp/enums.tpl";
    if let Some(content) = generate::get_template(enum_key) {
        tera.add_raw_template(enum_key, &content)
            .expect("Cpp Failed to add enum template");
    }
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("info", &cpp_fields);

    // 收集需要包含的头文件
    let imports: Vec<String> = collect_imports(&cpp_fields);
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
        .join(format!("{}.h", tree_data.file_name));

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
 * C/C++字段信息转换
 * @param fields 字段列表
 * @return C/C++字段列表
*/
fn transition_fields(info: &FieldInfo) -> GenerateFieldInfo {
    let mut main_fields: Vec<GenerateMainFieldItem> = Vec::new();
    for field in info.main_fields.iter() {
        let field_type = match field.field_type {
            EDataType::Int => "int",
            EDataType::Long => "long",
            EDataType::Float => "float",
            _ => "std::string",
        };
        let main_field: GenerateMainFieldItem = GenerateMainFieldItem {
            field_type: field_type.to_string(),
            field_name: field.field_name.clone(),
        };
        main_fields.push(main_field);
    }

    let mut fields: Vec<GenerateFieldItem> = Vec::new();
    for field in info.fields.iter() {
        let cpp_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date | EDataType::Long => "long",
            EDataType::Unknown | EDataType::String | EDataType::Loc => "std::string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "std::vector<int>",
            EDataType::LongArr => "std::vector<long>",
            EDataType::StringArr => "std::vector<std::string>",
            EDataType::BooleanArr => "std::vector<bool>",
            EDataType::FloatArr => "std::vector<float>",
            EDataType::Vector2Arr => "std::vector<Vector2>",
            EDataType::Vector3Arr => "std::vector<Vector3>",
            EDataType::Vector4Arr => "std::vector<Vector4>",
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
        };

        let cpp_field: GenerateFieldItem = GenerateFieldItem {
            field_name: field.field_name.clone(),
            field_type: cpp_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
            field_extend: String::new(),
            data_type: String::new(),
        };
        fields.push(cpp_field);
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
        // 为 vector 类型添加必要的包含
        if field.field_type.contains("std::vector")
            && !field.field_type.contains("int")
            && !field.field_type.contains("float")
            && !field.field_type.contains("bool")
        {
            // 对于自定义类的 vector 类型，我们可能需要添加相关的包含
            if field.field_type.contains("Vector2") && !imports.contains(&"Vector2".to_string()) {
                imports.push("Vector2".to_string());
            } else if field.field_type.contains("Vector3")
                && !imports.contains(&"Vector3".to_string())
            {
                imports.push("Vector3".to_string());
            } else if field.field_type.contains("Vector4")
                && !imports.contains(&"Vector4".to_string())
            {
                imports.push("Vector4".to_string());
            }
        } else if !field.field_type.starts_with("int")
            && !field.field_type.starts_with("float")
            && !field.field_type.starts_with("bool")
            && !field.field_type.starts_with("long long")
            && !field.field_type.starts_with("std::")
        {
            // 对于自定义类，添加到包含列表
            if !imports.contains(&field.field_type) {
                imports.push(field.field_type.clone());
            }
        }
    }

    imports
}
