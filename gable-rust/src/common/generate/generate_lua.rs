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

#[derive(serde::Serialize)]
struct LuaFieldInfo {
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

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let lua_fields: Vec<LuaFieldInfo> = transition_fields(&fields);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/lua/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.content.sheetname);
    context.insert("fields", &lua_fields);

    // 收集导入的模块
    let imports: Vec<String> = collect_imports(&lua_fields);
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
        .join(format!("{}.lua", tree_data.content.sheetname));

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

fn transition_fields(fields: &Vec<FieldInfo>) -> Vec<LuaFieldInfo> {
    let mut lua_fields: Vec<LuaFieldInfo> = Vec::new();
    for field in fields {
        // Lua 是动态类型语言，不需要转换为特定类型，但保留用于注释
        let lua_type = match field.field_type {
            EDataType::Int | EDataType::Time => "number",
            EDataType::Date => "number", // Date类型在Lua中通常用时间戳表示
            EDataType::String | EDataType::Loc => "string",
            EDataType::Boolean => "boolean",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "number",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "table", // 数组在Lua中是table
            EDataType::StringArr => "table",
            EDataType::BooleanArr => "table",
            EDataType::FloatArr => "table",
            EDataType::Vector2Arr => "table",
            EDataType::Vector3Arr => "table",
            EDataType::Vector4Arr => "table",
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

        let lua_field: LuaFieldInfo = LuaFieldInfo {
            is_key: field.is_key,
            field_name: field.field_name.clone(),
            field_type: lua_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
        };
        lua_fields.push(lua_field);
    }
    return lua_fields;
}

fn collect_imports(fields: &Vec<LuaFieldInfo>) -> Vec<String> {
    let mut imports: Vec<String> = Vec::new();

    for field in fields {
        // 检查是否有需要导入的自定义类型
        if field.field_type != "number"
            && field.field_type != "string"
            && field.field_type != "boolean"
            && field.field_type != "table"
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
