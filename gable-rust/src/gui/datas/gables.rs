use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Folder,
    Excel,
    Sheet,
}

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub item_type: ItemType,
    pub display_name: String,
    pub is_open: bool,
    pub fullpath: String,
    // file_name: String,
    // parent: TreeItem,
    pub children: Vec<TreeItem>,
}

lazy_static! {
    pub static ref TREE_ITEMS: Arc<Mutex<Vec<TreeItem>>> = Arc::new(Mutex::new(Vec::new()));
}

// pub fn get_tree_items() -> &'static Arc<Mutex<Vec<TreeItem>>> {
//     &TREE_ITEMS
// }
