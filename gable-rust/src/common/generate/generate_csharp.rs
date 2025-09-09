use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use std::{fs, io::Error, path::PathBuf};
use tera::{Context, Tera};

pub fn by(build_setting: &BuildSetting, tree_data: &TreeData) {
    let (main_fields, sub_fields) = tree_data.to_csharp_data(&build_setting.keyword);
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
