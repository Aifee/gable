use crate::gui::datas::{eitem_type::EItemType, tree_data::TreeData};

#[derive(Debug, Clone)]
pub struct TreeItem {
    /// 文件类型
    pub item_type: EItemType,
    /// 显示名
    pub display_name: String,
    /// 是否被打开
    pub is_open: bool,
    /// 完整路径
    pub fullpath: String,
    /// 父级
    pub parent: Option<String>,
    /// 链接名
    pub link_name: Option<String>,
    /// 子项
    pub children: Vec<TreeItem>,
    /// 数据
    pub data: Option<TreeData>,
}
