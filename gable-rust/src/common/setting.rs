use std::sync::Mutex;

// static mut WORKSPACE: &str = "";
pub(crate) static WORKSPACE: Mutex<Option<String>> = Mutex::new(None);
