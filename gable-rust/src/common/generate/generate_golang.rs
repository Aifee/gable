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
 * golang语言字段信息
*/
#[derive(serde::Serialize)]
struct GolangFieldInfo {
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
 * golang语言生成
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let go_fields: Vec<GolangFieldInfo> = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/golang/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("fields", &go_fields);

    // 收集导入的包
    let imports: Vec<String> = collect_imports(&go_fields);
    context.insert("imports", &imports);

    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize | ESheetType::KV => {
            tera.render("template.temp", &context)
        }
        ESheetType::Enum => tera.render("enums.temp", &context),
    };
    if rendered_result.is_err() {
        log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.go", tree_data.file_name));

    let result: Result<(), Error> = fs::write(&target_path, rendered);
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

/**
 * 通用字段转换Golang字段
 * @param fields 字段列表
 * @return Golang字段列表
*/
fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<GolangFieldInfo> {
    let mut go_fields: Vec<GolangFieldInfo> = Vec::new();
    for field in fields {
        // Go语言字段名需要首字母大写以保证可导出
        let field_name: String = capitalize_first_letter(&field.field_name);

        let go_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date => "int64",
            EDataType::String | EDataType::Loc => "string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float64",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "[]int",
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
                        enum_name = capitalize_first_letter(part);
                    } else {
                        enum_name = capitalize_first_letter(&field.field_link);
                    };
                }
                &enum_name.clone()
            }
            _ => "string",
        };

        let go_field: GolangFieldInfo = GolangFieldInfo {
            is_key: field.is_key,
            field_name: field_name,
            field_type: go_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        go_fields.push(go_field);
    }
    return go_fields;
}

/**
 * 收集导入的模块
 * @param fields 字段列表
 * @return 导入的模块列表
*/
fn collect_imports(fields: &Vec<GolangFieldInfo>) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in fields {
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

/**
 * 首字母大写，遵循go语言命名规则
 * @param s 字符串
 * @return 转换后的字符串
*/
fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
