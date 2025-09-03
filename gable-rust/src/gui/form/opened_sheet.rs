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
        Self {
            display_name: item.display_name.clone(),
            data: Self::pairs_data(item.data.as_ref().unwrap()),
        }
    }

    fn pairs_data(data: &TreeData) -> OpenedGableData {
        let data: OpenedGableData = OpenedGableData::new(&data.gable_type, &data.content);
        return data;
    }
}
