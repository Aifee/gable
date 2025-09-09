use crate::{
    common::{constant, setting::BuildSetting, utils},
    gui::datas::{
        cell_data::CellData,
        edata_type::EDataType,
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use std::{fs, io::Error, path::PathBuf};
use tera::{Context, Tera};

pub fn by(build_setting: &BuildSetting, tree_data: &TreeData) {
    let (main_fields, sub_fields) = to_csharp_data(tree_data, &build_setting.keyword);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/csharp/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let all_fields: Vec<FieldInfo> = main_fields
        .into_iter()
        .chain(sub_fields.into_iter())
        .collect::<Vec<_>>();
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.content.sheetname);
    context.insert("fields", &all_fields);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal | ESheetType::KV => tera.render("template.temp", &context),
        ESheetType::Enum => tera.render("enums.temp", &context),
    };
    if rendered_result.is_err() {
        log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.script_path)
        .join(format!("{}.cs", tree_data.content.sheetname));

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

fn to_csharp_data(tree_data: &TreeData, keyword: &str) -> (Vec<FieldInfo>, Vec<FieldInfo>) {
    match tree_data.gable_type {
        ESheetType::Normal => normal_csharp_data(tree_data, keyword),
        ESheetType::KV => (Vec::new(), Vec::new()),
        ESheetType::Enum => (Vec::new(), Vec::new()),
    }
}

fn normal_csharp_data(tree_data: &TreeData, keyword: &str) -> (Vec<FieldInfo>, Vec<FieldInfo>) {
    let (valids_main, valids) = tree_data.content.get_valid_normal_heads(keyword);
    let mut main_fields: Vec<FieldInfo> = Vec::new();
    let mut fields: Vec<FieldInfo> = Vec::new();
    let mut field_index: i32 = 1;
    let mut row_valid: bool = true;
    for (_, head_data) in valids_main.iter() {
        let field_cell: &&CellData =
            if let Some(field_cell) = head_data.get(&constant::TABLE_DATA_ROW_FIELD) {
                field_cell
            } else {
                row_valid = false;
                continue;
            };
        let type_cell: &&CellData =
            if let Some(type_cell) = head_data.get(&constant::TABLE_DATA_ROW_TYPE) {
                type_cell
            } else {
                row_valid = false;
                continue;
            };

        let data_type: EDataType = EDataType::convert(&type_cell.value);
        let proto_type = match data_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date => "long",
            EDataType::String => "string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "int[]",
            EDataType::StringArr => "string[]",
            EDataType::BooleanArr => "bool[]",
            EDataType::FloatArr => "float[]",
            EDataType::Vector2Arr => "Vector2[]",
            EDataType::Vector3Arr => "Vector3[]",
            EDataType::Vector4Arr => "Vector4[]",
            EDataType::Enum => {
                let link_cell: Option<&&CellData> = head_data.get(&constant::TABLE_DATA_ROW_LINK);
                if let Some(link_cell) = link_cell {
                    let link_name = if let Some(pos) = link_cell.value.find('@') {
                        &link_cell.value[pos + 1..]
                    } else {
                        link_cell.value.as_str()
                    };
                    link_name
                } else {
                    "int"
                }
            }
            _ => "string",
        };
        let desc_cell: Option<&&CellData> = head_data.get(&constant::TABLE_DATA_ROW_DESC);
        let desc_value: String = if let Some(desc_cell) = desc_cell {
            desc_cell.value.clone()
        } else {
            String::new()
        };
        let field_value: String = field_cell.value.replace("*", "");
        let field_info: FieldInfo = FieldInfo {
            field_type: proto_type.to_string(),
            field_name: field_value,
            field_desc: desc_value,
            field_index,
        };
        main_fields.push(field_info);
        field_index += 1;
    }
    // 行数据无效
    if !row_valid {
        return (Vec::new(), Vec::new());
    }

    for (_, head_data) in valids.iter() {
        let field_cell: &&CellData =
            if let Some(field_cell) = head_data.get(&constant::TABLE_DATA_ROW_FIELD) {
                field_cell
            } else {
                continue;
            };
        let type_cell: &&CellData =
            if let Some(type_cell) = head_data.get(&constant::TABLE_DATA_ROW_TYPE) {
                type_cell
            } else {
                continue;
            };
        let data_type: EDataType = EDataType::convert(&type_cell.value);
        let proto_type = match data_type {
            EDataType::Int | EDataType::Time => "int",
            EDataType::Date => "long",
            EDataType::String => "string",
            EDataType::Boolean => "bool",
            EDataType::Float
            | EDataType::Percentage
            | EDataType::Permillage
            | EDataType::Permian => "float",
            EDataType::Vector2 => "Vector2",
            EDataType::Vector3 => "Vector3",
            EDataType::Vector4 => "Vector4",
            EDataType::IntArr => "int[]",
            EDataType::StringArr => "string[]",
            EDataType::BooleanArr => "bool[]",
            EDataType::FloatArr => "float[]",
            EDataType::Vector2Arr => "Vector2[]",
            EDataType::Vector3Arr => "Vector3[]",
            EDataType::Vector4Arr => "Vector4[]",
            EDataType::Enum => {
                if let Some(link_cell) = head_data.get(&constant::TABLE_DATA_ROW_LINK) {
                    let link_name: &str = if let Some(pos) = link_cell.value.find('@') {
                        &link_cell.value[pos + 1..]
                    } else {
                        link_cell.value.as_str()
                    };
                    link_name
                } else {
                    "int"
                }
            }
            _ => "string",
        };
        let desc_cell: Option<&&CellData> = head_data.get(&constant::TABLE_DATA_ROW_DESC);
        let desc_value: String = if let Some(desc_cell) = desc_cell {
            desc_cell.value.clone()
        } else {
            String::new()
        };
        let field_info: FieldInfo = FieldInfo {
            field_type: proto_type.to_string(),
            field_name: field_cell.value.clone(),
            field_desc: desc_value,
            field_index,
        };
        fields.push(field_info);
        field_index += 1;
    }
    return (main_fields, fields);
}
