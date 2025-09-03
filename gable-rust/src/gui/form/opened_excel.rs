use crate::gui::{datas::tree_item::TreeItem, form::opened_sheet::OpenedSheet};

#[derive(Debug, Clone)]
pub struct OpenedExcel {
    /// 当前选中的Sheet索引
    pub selected_sheet_index: usize,
    /// 文件全路径
    pub full_path: String,
    /// 文件显示名称
    pub display_name: String,
    /// 需要绘制的数据
    pub sheets: Vec<OpenedSheet>,
}

impl OpenedExcel {
    pub fn new(item: TreeItem) -> Self {
        Self {
            selected_sheet_index: 0,
            full_path: item.fullpath,
            display_name: item.display_name,
            sheets: Self::pairs_sheets(item.children),
        }
    }

    fn pairs_sheets(childs: Vec<TreeItem>) -> Vec<OpenedSheet> {
        let mut sheets: Vec<OpenedSheet> = Vec::new();
        for child in childs {
            let sheet: OpenedSheet = OpenedSheet::new(&child);
            sheets.push(sheet);
        }
        return sheets;
    }
}
