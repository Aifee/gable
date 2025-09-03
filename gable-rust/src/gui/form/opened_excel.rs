use crate::gui::{datas::tree_item::TreeItem, form::opened_sheet::OpenedSheet};

#[derive(Debug, Clone)]
pub struct OpenedExcel {
    pub item: TreeItem,
    /// 当前选中的Sheet索引
    pub selected_sheet_index: usize,
    pub full_path: String,
    pub display_name: String,
    pub sheets: Vec<OpenedSheet>,
}

impl OpenedExcel {
    pub fn new(item: TreeItem) -> Self {
        Self {
            item: item.clone(),
            selected_sheet_index: 0,
            full_path: item.fullpath,
            display_name: item.display_name,
            sheets: Self::pairs_sheets(item.children),
        }
    }

    fn pairs_sheets(childs: Vec<TreeItem>) -> Vec<OpenedSheet> {
        let mut sheets: Vec<OpenedSheet> = Vec::new();
        for child in childs {
            let sheet = OpenedSheet::new(&child);
            sheets.push(sheet);
        }
        return sheets;
    }
}
