use serde::{Deserialize, Serialize};

// 开发语言类型类型
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum EDevelopType {
    // C或C++语言
    cpp = 0,
    // C//语言
    csharp = 1,
    // Cangjie语言
    cangjie = 2,
    // golang语言
    go = 3,
    // java语言
    java = 4,
    // javascript语言
    javascript = 5,
    // lua语言
    lua = 6,
    // python语言
    python = 7,
    // typescript语言
    typescript = 8,
}

impl EDevelopType {
    pub fn to_string(&self) -> &'static str {
        match self {
            EDevelopType::cpp => "C/C++",
            EDevelopType::csharp => "C#",
            EDevelopType::cangjie => "Cangjie",
            EDevelopType::go => "Go",
            EDevelopType::java => "Java",
            EDevelopType::javascript => "JavaScript",
            EDevelopType::lua => "Lua",
            EDevelopType::python => "Python",
            EDevelopType::typescript => "TypeScript",
        }
    }
    pub fn iter() -> std::slice::Iter<'static, EDevelopType> {
        static VARIANTS: &[EDevelopType] = &[
            EDevelopType::cpp,
            EDevelopType::csharp,
            EDevelopType::cangjie,
            EDevelopType::go,
            EDevelopType::java,
            EDevelopType::javascript,
            EDevelopType::lua,
            EDevelopType::python,
            EDevelopType::typescript,
        ];
        VARIANTS.iter()
    }
    pub fn to_keyword(&self) -> &'static str {
        match self {
            EDevelopType::cpp => "cpp",
            EDevelopType::csharp => "csharp",
            EDevelopType::cangjie => "cj",
            EDevelopType::go => "go",
            EDevelopType::java => "java",
            EDevelopType::javascript => "js",
            EDevelopType::lua => "lua",
            EDevelopType::python => "py",
            EDevelopType::typescript => "ts",
        }
    }
}
