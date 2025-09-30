use crate::common::localization::Localization;
use crate::common::res;
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

#[derive(PartialEq, Copy, Clone)]
pub enum ELocalizationType {
    // 中文
    Chinese = 0,
    // 英文
    English = 1,
}

#[allow(dead_code)]
impl ELocalizationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ELocalizationType::Chinese => "zh",
            ELocalizationType::English => "en",
        }
    }
}
pub struct LocalizationManager {
    languages: HashMap<String, Localization>,
    current_language: RwLock<ELocalizationType>,
    supported: RwLock<Vec<ELocalizationType>>,
}

impl LocalizationManager {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
            current_language: RwLock::new(ELocalizationType::Chinese),
            supported: RwLock::new(vec![ELocalizationType::Chinese, ELocalizationType::English]),
        }
    }

    /**
     * 从 JSON 文件加载语言包
     */
    pub fn load_language_from_json(
        &mut self,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let lang_pack: Vec<Localization> = serde_json::from_str(&content)?;
        for v in lang_pack.iter() {
            if !self.languages.contains_key(&v.key.clone()) {
                self.languages.insert(v.key.clone(), v.clone());
            } else {
                log::error!("重复的key:{}", v.key);
            }
        }
        Ok(())
    }
    /**
     * 设置当前语言
     */
    pub fn set_language(&self, code: &ELocalizationType) -> bool {
        let supported = self.supported.read().unwrap();
        if supported.contains(code) {
            let mut current = self.current_language.write().unwrap();
            *current = code.clone();
            true
        } else {
            false
        }
    }

    pub fn get_current_language(&self) -> ELocalizationType {
        self.current_language
            .read()
            .map(|guard| *guard)
            .unwrap_or(ELocalizationType::Chinese)
    }

    pub fn t(&self, key: &str) -> String {
        let current_lang: ELocalizationType = self.get_current_language();
        if let Some(lang_pack) = self.languages.get(key) {
            match current_lang {
                ELocalizationType::Chinese => {
                    return lang_pack.zh.clone();
                }
                ELocalizationType::English => {
                    return lang_pack.en.clone();
                }
            }
        };
        key.to_owned()
    }

    pub fn t_with_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.t(key);

        for (placeholder, value) in args {
            let placeholder_str = format!("{{{}}}", placeholder);
            result = result.replace(&placeholder_str, value);
        }

        result
    }
    /**
     * 获取可用语言列表
     */
    pub fn get_available_languages(&self) -> Vec<ELocalizationType> {
        let mut list: Vec<ELocalizationType> = vec![];
        let supported = self.supported.read().unwrap();
        for lang in supported.iter() {
            list.push(*lang);
        }
        list
    }
}

// 全局本地化管理器实例
lazy_static::lazy_static! {
    pub static ref LOCALIZATION_MANAGER: LocalizationManager = {
        let mut manager = LocalizationManager::new();
        match manager.load_language_from_json(res::CONFIG_LOCALIZATION) {
            Ok(_) => {
                println!("Successfully loaded language packs from {}", res::CONFIG_LOCALIZATION);
            }
            Err(e) => {
                eprintln!("Failed to load language packs from {}: {}", res::CONFIG_LOCALIZATION, e);
                eprintln!("Using default language packs instead");
            }
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

pub fn set_language(code: &ELocalizationType) -> bool {
    LOCALIZATION_MANAGER.set_language(code)
}

/// 获取可用语言列表
pub fn get_available_languages() -> Vec<ELocalizationType> {
    LOCALIZATION_MANAGER.get_available_languages()
}
