use crate::gui::datas::{esheet_type::ESheetType, gable_data::GableData};

#[derive(Debug, Clone)]
pub struct TreeData {
    pub gable_type: ESheetType,
    pub content: GableData,
}
