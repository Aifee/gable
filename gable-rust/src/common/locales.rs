use crate::common::localization::Localization;
use crate::common::{res, setting};
use std::collections::HashMap;
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

    pub fn from_str(value: &str) -> ELocalizationType {
        match value {
            "zh" => ELocalizationType::Chinese,
            "en" => ELocalizationType::English,
            _ => ELocalizationType::Chinese,
        }
    }
}
pub struct Locales {
    languages: HashMap<String, Localization>,
    current_language: RwLock<ELocalizationType>,
    supported: RwLock<Vec<ELocalizationType>>,
}

impl Locales {
    pub fn new() -> Self {
        let set_lang: String = setting::get_language();
        let lang_type: ELocalizationType = ELocalizationType::from_str(&set_lang);
        Self {
            languages: HashMap::new(),
            current_language: RwLock::new(lang_type),
            supported: RwLock::new(vec![ELocalizationType::Chinese, ELocalizationType::English]),
        }
    }

    /**
     * 从 JSON 文件加载语言包
     */
    pub fn load_language_from_json(
        &mut self,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lang_pack: Vec<Localization> = serde_json::from_str(content)?;
        for v in lang_pack.iter() {
            if !self.languages.contains_key(&v.key.clone()) {
                self.languages.insert(v.key.clone(), v.clone());
            } else {
                log::error!("Repeated key: {}", v.key);
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
            let _ = setting::set_language(&code);
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
    pub static ref LOCALIZATION_MANAGER: Locales = {
        let mut manager = Locales::new();
        match manager.load_language_from_json(res::CONFIG_LOCALIZATION) {
            Ok(_) => {
                log::debug!("Successfully loaded language packs from config");
            }
            Err(e) => {
                log::error!("Failed to load language packs from config {}", e);
            }
        }
        manager
    };
}

// 便捷函数
pub fn t(key: &str) -> String {
    LOCALIZATION_MANAGER.t(key)
}

pub fn set_language(code: &ELocalizationType) -> bool {
    LOCALIZATION_MANAGER.set_language(code)
}

/// 获取可用语言列表
pub fn get_available_languages() -> Vec<ELocalizationType> {
    LOCALIZATION_MANAGER.get_available_languages()
}
