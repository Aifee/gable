use crate::gui::datas::{gable_data::GableData, item_type::ItemType};

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub item_type: ItemType,
    pub display_name: String,
    pub is_open: bool,
    pub fullpath: String,
    pub parent: Option<String>,
    pub children: Vec<TreeItem>,
    /// 存储Sheet类型节点的gable文件内容
    pub gable_content: Option<GableData>,
}
