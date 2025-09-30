use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePack {
    pub name: String,
    pub code: String,
    pub translations: HashMap<String, String>,
}

impl LanguagePack {
    pub fn new(code: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            translations: HashMap::new(),
        }
    }

    pub fn add_translation(&mut self, key: &str, value: &str) {
        self.translations.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
}

pub struct LocalizationManager {
    languages: HashMap<String, LanguagePack>,
    current_language: RwLock<String>,
}

impl LocalizationManager {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
            current_language: RwLock::new("en".to_string()),
        }
    }

    pub fn add_language(&mut self, lang: LanguagePack) {
        self.languages.insert(lang.code.clone(), lang);
    }

    /// 从 JSON 文件加载语言包
    pub fn load_language_from_json(
        &mut self,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let lang_pack: LanguagePack = serde_json::from_str(&content)?;
        self.add_language(lang_pack);
        Ok(())
    }

    /// 从目录批量加载语言包
    pub fn load_languages_from_directory(
        &mut self,
        dir_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir_path = Path::new(dir_path);

        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(format!(
                "Directory {} does not exist or is not a directory",
                dir_path.display()
            )
            .into());
        }

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "json") {
                match self.load_language_from_json(&path.to_string_lossy()) {
                    Ok(_) => {
                        println!("Loaded language pack from {}", path.display());
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to load language pack from {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// 初始化默认语言包（以防文件加载失败）
    pub fn init_default_languages(&mut self) {
        // English
        let mut en = LanguagePack::new("en", "English");
        en.add_translation("app_title", "Gable");
        en.add_translation("file_menu", "File");
        en.add_translation("edit_menu", "Edit");
        en.add_translation("view_menu", "View");
        en.add_translation("help_menu", "Help");
        en.add_translation("settings", "Settings");
        en.add_translation("exit", "Exit");
        self.add_language(en);

        // 设置默认语言
        let mut current = self.current_language.write().unwrap();
        *current = "en".to_string();
    }

    pub fn set_language(&self, code: &str) -> bool {
        if self.languages.contains_key(code) {
            let mut current = self.current_language.write().unwrap();
            *current = code.to_string();
            true
        } else {
            false
        }
    }

    pub fn get_current_language(&self) -> String {
        self.current_language.read().unwrap().clone()
    }

    pub fn t(&self, key: &str) -> String {
        let current_lang = self.current_language.read().unwrap();
        if let Some(lang_pack) = self.languages.get(current_lang.as_str()) {
            if let Some(translation) = lang_pack.get(key) {
                return translation.clone();
            }
        }

        // 回退到默认语言(英语)
        if let Some(lang_pack) = self.languages.get("en") {
            if let Some(translation) = lang_pack.get(key) {
                return translation.clone();
            }
        }

        // 如果找不到翻译，则返回键本身
        key.to_string()
    }

    pub fn t_with_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.t(key);

        for (placeholder, value) in args {
            let placeholder_str = format!("{{{}}}", placeholder);
            result = result.replace(&placeholder_str, value);
        }

        result
    }

    pub fn get_available_languages(&self) -> Vec<(&String, &String)> {
        self.languages
            .iter()
            .map(|(code, pack)| (code, &pack.name))
            .collect()
    }
}

// 全局本地化管理器实例
lazy_static::lazy_static! {
    pub static ref LOCALIZATION_MANAGER: LocalizationManager = {
        let mut manager = LocalizationManager::new();

        // 尝试从 assets/locales 目录加载语言包
        let locales_dir = "assets/locales";
        match manager.load_languages_from_directory(locales_dir) {
            Ok(_) => {
                println!("Successfully loaded language packs from {}", locales_dir);
            }
            Err(e) => {
                eprintln!("Failed to load language packs from {}: {}", locales_dir, e);
                eprintln!("Using default language packs instead");
                manager.init_default_languages();
            }
        }

        // 如果没有任何语言包被加载，初始化默认语言包
        if manager.languages.is_empty() {
            manager.init_default_languages();
        }

        manager
    };
}

// 便捷函数
pub fn t(key: &str) -> String {
    LOCALIZATION_MANAGER.t(key)
}

pub fn t_with_args(key: &str, args: &[(&str, &str)]) -> String {
    LOCALIZATION_MANAGER.t_with_args(key, args)
}

pub fn set_language(code: &str) -> bool {
    LOCALIZATION_MANAGER.set_language(code)
}

/// 获取可用语言列表
pub fn get_available_languages() -> Vec<(&'static String, &'static String)> {
    LOCALIZATION_MANAGER.get_available_languages()
}
