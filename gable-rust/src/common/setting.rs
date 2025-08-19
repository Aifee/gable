use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, MutexGuard};

/// 工程目录
pub(crate) static WORKSPACE: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(Some(String::from("E:\\projects\\gable_project_temp"))));

// 提供一个安全的设置方法
pub fn set_workspace(path: String) {
    let mut workspace: MutexGuard<'_, Option<String>> = WORKSPACE.lock().unwrap();
    *workspace = Some(path);
}

pub fn get_workspace() -> PathBuf {
    let workspace: MutexGuard<'_, Option<String>> = WORKSPACE.lock().unwrap();
    let root_path: PathBuf = if let Some(path) = workspace.as_ref() {
        PathBuf::from(path)
    } else {
        PathBuf::from(".")
    };
    root_path
}
