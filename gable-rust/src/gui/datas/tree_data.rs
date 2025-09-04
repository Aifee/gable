use crate::gui::datas::{esheet_type::ESheetType, gable_data::GableData};

#[derive(Debug, Clone)]
pub struct TreeData {
    pub gable_type: ESheetType,
    pub content: GableData,
}

impl TreeData {
    pub fn to_json_data(&self, keyword: &str) {}

    // fn data_json_data(&self) -> String {}
}
