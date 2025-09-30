use serde::{Deserialize, Serialize};

// 开发语言类型类型
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum EDevelopType {
    // C或C++语言
    Cpp = 0,
    // C#语言
    Csharp = 1,
    // Cangjie语言
    Cangjie = 2,
    // golang语言
    Golang = 3,
    // java语言
    Java = 4,
    // javascript语言
    JavaScript = 5,
    // lua语言
    Lua = 6,
    // python语言
    Python = 7,
    // typescript语言
    TypeScript = 8,
    // Rust语言
    Rust = 9,
}

impl EDevelopType {
    pub fn to_string(&self) -> &'static str {
        match self {
            EDevelopType::Cpp => "C/C++",
            EDevelopType::Csharp => "C#",
            EDevelopType::Cangjie => "Cangjie",
            EDevelopType::Golang => "Go",
            EDevelopType::Java => "Java",
            EDevelopType::JavaScript => "JavaScript",
            EDevelopType::Lua => "Lua",
            EDevelopType::Python => "Python",
            EDevelopType::TypeScript => "TypeScript",
            EDevelopType::Rust => "Rust",
        }
    }
    pub fn iter() -> std::slice::Iter<'static, EDevelopType> {
        static VARIANTS: &[EDevelopType] = &[
            EDevelopType::Cpp,
            EDevelopType::Csharp,
            EDevelopType::Cangjie,
            EDevelopType::Golang,
            EDevelopType::Java,
            EDevelopType::JavaScript,
            EDevelopType::Lua,
            EDevelopType::Python,
            EDevelopType::TypeScript,
            EDevelopType::Rust,
        ];
        VARIANTS.iter()
    }
    pub fn to_keyword(&self) -> &'static str {
        match self {
            EDevelopType::Cpp => "cpp",
            EDevelopType::Csharp => "cs",
            EDevelopType::Cangjie => "cj",
            EDevelopType::Golang => "go",
            EDevelopType::Java => "java",
            EDevelopType::JavaScript => "js",
            EDevelopType::Lua => "lua",
            EDevelopType::Python => "py",
            EDevelopType::TypeScript => "ts",
            EDevelopType::Rust => "rs",
        }
    }
}
