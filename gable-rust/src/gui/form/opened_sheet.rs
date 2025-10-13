use crate::gui::{
    datas::{tree_data::TreeData, tree_item::TreeItem},
    form::opened_gable_data::OpenedGableData,
};

#[derive(Debug, Clone)]
pub struct OpenedSheet {
    pub display_name: String,
    pub data: OpenedGableData,
}

impl OpenedSheet {
    pub fn new(item: &TreeItem) -> Self {
        let data = match item.data.as_ref() {
            Some(d) => Self::pairs_data(d),
            None => {
                // 创建一个空的默认 OpenedGableData
                OpenedGableData {
                    max_row: 0,
                    max_col: 0,
                    column_headers: vec![],
                    items: std::collections::BTreeMap::new(),
                }
            }
        };

        Self {
            display_name: item.display_name.clone(),
            data,
        }
    }

    fn pairs_data(data: &TreeData) -> OpenedGableData {
        let data: OpenedGableData = OpenedGableData::new(&data.gable_type, &data.content);
        return data;
    }
}
