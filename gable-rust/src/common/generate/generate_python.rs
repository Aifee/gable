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
 * python 字段信息
*/
#[derive(serde::Serialize)]
struct PythonFieldInfo {
    // 是否是主键
    pub is_key: bool,
    // 字段名称
    pub field_name: String,
    // 字段类型（用于注释）
    pub field_type: String,
    // 字段描述
    pub field_desc: String,
    // 字段序号
    pub field_index: i32,
}
/**
 * 生成python脚本
 * @param build_setting 构建设置
 * @param tree_data 树结构数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let python_fields: Vec<PythonFieldInfo> = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/python/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.file_name);
    context.insert("fields", &python_fields);

    // 收集导入的模块
    let imports: Vec<String> = collect_imports(&python_fields);
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
        .join(format!("{}.py", tree_data.file_name));

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
 * 通用字段转换成python字段
 * @param fields 字段列表
 * @return python字段列表
*/
fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<PythonFieldInfo> {
    let mut python_fields: Vec<PythonFieldInfo> = Vec::new();
    for field in fields {
        // Python 是动态类型语言，不需要转换为特定类型，但保留用于注释
        let python_type = match field.field_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date => "int", // Date类型在Python中通常用时间戳表示
            EDataType::String | EDataType::Loc => "str",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "list", // 数组在Python中是list
            EDataType::StringArr => "list",
            EDataType::BooleanArr => "list",
            EDataType::FloatArr => "list",
            EDataType::Vector2Arr => "list",
            EDataType::Vector3Arr => "list",
            EDataType::Vector4Arr => "list",
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
            _ => "str",
        };

        let python_field: PythonFieldInfo = PythonFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: python_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        python_fields.push(python_field);
    }
    return python_fields;
}
/**
 * 收集需要导入的模块
 * @param fields 字段列表
 * @return 模块列表
*/
fn collect_imports(fields: &Vec<PythonFieldInfo>) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in fields {
        // 检查是否有需要导入的自定义类型
        if field.field_type != "int"
            && field.field_type != "str"
            && field.field_type != "bool"
            && field.field_type != "float"
            && field.field_type != "list"
        {
            // 对于自定义类型，添加到导入列表
            if !imports.contains(&field.field_type)
                && field.field_type != "Vector2"
                && field.field_type != "Vector3"
                && field.field_type != "Vector4"
            {
                imports.push(field.field_type.clone());
            }
        }
    }

    imports
}
