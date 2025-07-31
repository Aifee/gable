#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Folder,
    Excel,
    Sheet,
}

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub id: String,
    pub name: String,
    pub is_open: bool,

    pub item_type: ItemType,
    // fullpath: String,
    // display_name: String,
    // file_name: String,
    // parent: TreeItem,
    pub children: Vec<TreeItem>,
}
