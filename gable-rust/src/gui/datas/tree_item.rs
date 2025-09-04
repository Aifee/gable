use crate::gui::datas::{eitem_type::EItemType, esheet_type::ESheetType, tree_data::TreeData};
use std::collections::HashMap;

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

impl TreeItem {
    pub fn get_datas(&self) -> HashMap<String, &TreeData> {
        let mut cache: HashMap<String, &TreeData> = HashMap::new();
        if self.item_type == EItemType::Sheet {
            if let Some(data) = &self.data {
                if data.gable_type != ESheetType::Enum {
                    cache.insert(self.display_name.clone(), data);
                }
            }
        }

        for item in &self.children {
            let child_cache = item.get_datas();
            cache.extend(child_cache);
        }

        cache
    }
}
