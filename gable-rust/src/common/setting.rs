use std::sync::Mutex;

// static mut WORKSPACE: &str = "";
pub(crate) static WORKSPACE: Mutex<Option<String>> = Mutex::new(None);

// 提供一个安全的设置方法
pub fn set_workspace(path: String) {
    let mut workspace = WORKSPACE.lock().unwrap();
    *workspace = Some(path);
}
