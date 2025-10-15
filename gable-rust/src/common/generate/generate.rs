use crate::{
    common::{
        generate::{
            generate_cangjie, generate_cpp, generate_csharp, generate_golang, generate_java,
            generate_javascript, generate_lua, generate_protobuff, generate_python, generate_rust,
            generate_typescript,
        },
        res,
        setting::{self, BuildSetting},
        utils,
    },
    gui::datas::{
        edevelop_type::EDevelopType, etarget_type::ETargetType, gables, tree_data::TreeData,
        tree_item::TreeItem,
    },
};
use std::{collections::HashMap, fs, path::PathBuf, sync::Mutex};
use std::{process::Command, sync::LazyLock};

static TEMPLATES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/**
 * 批量生成代码（所有平台 & 所有表单）
*/
pub fn from_all() {
    let settings = setting::APP_SETTINGS.read().unwrap();
    for setting in settings.build_settings.iter() {
        from_target(setting);
    }
}

/**
 * 批量生成代码（指定平台 & 所有表单）
 * @param setting 指定的平台
*/
pub fn from_target(build_setting: &BuildSetting) {
    if !build_setting.generate_script {
        return;
    }
    let items = gables::TREE_ITEMS.read().unwrap();
    let mut datas: HashMap<String, &TreeData> = HashMap::new();
    for item in items.iter() {
        let item_datas: HashMap<String, &TreeData> = item.get_datas();
        if item_datas.len() > 0 {
            datas.extend(item_datas);
        }
    }
    if datas.len() <= 0 {
        log::error!("No configurations found to export");
        return;
    }
    clear_templates();
    preload_templates(build_setting);
    for (_, data) in datas.iter() {
        execute(build_setting, *data);
    }

    if !build_setting.postprocessing.is_empty() {
        let target_path: PathBuf = utils::get_absolute_path(&setting::get_workspace());
        system_command(&build_setting.postprocessing, &target_path);
    }
}

/**
 * 批量转换（所有平台 & 指定表单）
 * @param item 指定的表单
*/
pub fn from_items(item: &TreeItem) {
    let datas: HashMap<String, &TreeData> = item.get_datas();
    if datas.len() <= 0 {
        log::error!("Obtaining data is empty: {}", item.display_name);
        return;
    }
    clear_templates();
    let settings = setting::APP_SETTINGS.read().unwrap();
    for build_setting in settings.build_settings.iter() {
        if !build_setting.generate_script {
            continue;
        }
        preload_templates(build_setting);
        for (_, data) in datas.iter() {
            execute(build_setting, *data);
        }
        if !build_setting.postprocessing.is_empty() {
            let target_path: PathBuf = utils::get_absolute_path(&setting::get_workspace());
            system_command(&build_setting.postprocessing, &target_path);
        }
    }
}

pub fn get_template(key: &str) -> Option<String> {
    let templates = TEMPLATES.lock().unwrap();
    templates.get(key).cloned()
}

pub fn clear_templates() {
    let mut templates = TEMPLATES.lock().unwrap();
    templates.clear();
}

pub fn preload_templates(build_setting: &BuildSetting) {
    let mut templates = TEMPLATES.lock().unwrap();
    if build_setting.target_type == ETargetType::Protobuff {
    } else {
        let path_keyword = build_setting.dev.path_keyword();
        let template_path = format!("templates/{}/template.tpl", path_keyword);
        templates.insert(
            template_path.clone(),
            load_template(build_setting, &template_path, "template.tpl"),
        );
        let enum_path = format!("templates/{}/enums.tpl", path_keyword);
        templates.insert(
            enum_path.clone(),
            load_template(build_setting, &enum_path, "enums.tpl"),
        );
    }
}

fn load_template(build_setting: &BuildSetting, builtin: &str, file_name: &str) -> String {
    let mut tempalte_content: String = String::new();
    if build_setting.is_custom {
        let template_path: PathBuf =
            utils::get_absolute_path(&build_setting.custom_template).join(file_name);
        if template_path.exists() {
            if let Ok(content) = fs::read_to_string(&template_path) {
                tempalte_content = content;
            }
        }
    }
    if tempalte_content.is_empty() {
        if let Some(file) = res::load_template(builtin) {
            if let Some(content) = file.contents_utf8() {
                tempalte_content = content.to_string();
            }
        }
    }
    tempalte_content
}

/**
 * 执行生成代码
 * @param build_setting 构建设置
 * @param data 树数据
*/
pub fn execute(build_setting: &BuildSetting, data: &TreeData) {
    if build_setting.target_type == ETargetType::Protobuff {
        generate_protobuff::to(build_setting, data);
    } else {
        match build_setting.dev {
            EDevelopType::Cpp => generate_cpp::to(build_setting, data),
            EDevelopType::Csharp => generate_csharp::to(build_setting, data),
            EDevelopType::Cangjie => generate_cangjie::to(build_setting, data),
            EDevelopType::Golang => generate_golang::to(build_setting, data),
            EDevelopType::Java => generate_java::to(build_setting, data),
            EDevelopType::JavaScript => generate_javascript::to(build_setting, data),
            EDevelopType::Lua => generate_lua::to(build_setting, data),
            EDevelopType::Python => generate_python::to(build_setting, data),
            EDevelopType::TypeScript => generate_typescript::to(build_setting, data),
            EDevelopType::Rust => generate_rust::to(build_setting, data),
        }
    }
}

/**
 * 执行系统命令
*/
fn system_command(command: &str, path: &PathBuf) {
    if command.is_empty() {
        return;
    }
    #[cfg(target_os = "windows")]
    {
        // 对于Windows系统，将多行命令写入临时批处理文件执行
        let temp_script = path.join("temp_script.bat");
        if let Err(e) = std::fs::write(&temp_script, command) {
            log::error!("Failed to create temporary script file: {}", e);
            return;
        }

        // 使用spawn启动子进程，但不等待其完成，避免阻塞主线程
        match Command::new("cmd")
            .current_dir(&path)
            .args(&["/C", &temp_script.to_string_lossy()])
            .spawn()
        {
            Ok(mut child) => {
                // 在单独的线程中等待命令执行完成并清理临时文件
                std::thread::spawn(move || {
                    let _ = child.wait();
                    let _ = std::fs::remove_file(&temp_script);
                });
            }
            Err(e) => {
                log::error!("The post-processing command cannot be executed: {}", e);
                let _ = std::fs::remove_file(&temp_script);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // 使用spawn启动子进程，但不等待其完成，避免阻塞主线程
        match Command::new("sh")
            .current_dir(&path)
            .arg("-c")
            .arg(&command)
            .spawn()
        {
            Ok(mut child) => {
                // 在单独的线程中等待命令执行完成
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
            }
            Err(e) => {
                log::error!("The post-processing command cannot be executed: {}", e);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 使用spawn启动子进程，但不等待其完成，避免阻塞主线程
        match Command::new("sh")
            .current_dir(&path)
            .arg("-c")
            .arg(&command)
            .spawn()
        {
            Ok(mut child) => {
                // 在单独的线程中等待命令执行完成
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
            }
            Err(e) => {
                log::error!("The post-processing command cannot be executed: {}", e);
            }
        }
    }
}

/**
 * 首字母大写，遵循go语言命名规则
 * @param s 字符串
 * @return 转换后的字符串
*/
pub fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
