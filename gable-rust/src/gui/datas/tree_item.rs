use crate::gui::datas::{eitem_type::EItemType, gable_data::GableData, tree_data::TreeData};

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub item_type: EItemType,
    pub display_name: String,
    pub is_open: bool,
    pub fullpath: String,
    pub parent: Option<String>,
    pub children: Vec<TreeItem>,
    pub data: Option<TreeData>,
}
