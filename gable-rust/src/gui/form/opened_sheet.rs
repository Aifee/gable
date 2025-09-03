use crate::gui::{
    datas::{
        esheet_type::ESheetType, gable_data::GableData, tree_data::TreeData, tree_item::TreeItem,
    },
    form::opened_gable_data::OpenedGableData,
};

#[derive(Debug, Clone)]
pub struct OpenedSheet {
    pub full_path: String,
    pub display_name: String,
    pub gable_type: ESheetType,
    pub data: OpenedGableData,
}

impl OpenedSheet {
    pub fn new(item: &TreeItem) -> Self {
        Self {
            full_path: item.fullpath.clone(),
            display_name: item.display_name.clone(),
            gable_type: item.data.as_ref().unwrap().gable_type.clone(),
            data: Self::pairs_data(item.data.as_ref().unwrap()),
        }
    }

    fn pairs_data(data: &TreeData) -> OpenedGableData {
        let data: OpenedGableData = OpenedGableData::new(&data.content);
        return data;
    }
}
