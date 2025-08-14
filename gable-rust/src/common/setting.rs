use std::sync::{LazyLock, Mutex, MutexGuard};

/// 工程目录
// pub(crate) static WORKSPACE: Mutex<Option<String>> = Mutex::new(None);
pub(crate) static WORKSPACE: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(Some(String::from("E:\\projects\\gable_project"))));

// 提供一个安全的设置方法
pub fn set_workspace(path: String) {
    let mut workspace: MutexGuard<'_, Option<String>> = WORKSPACE.lock().unwrap();
    *workspace = Some(path);
}
