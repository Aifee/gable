use crate::gui::datas::esheet_type::ESheetType;

#[derive(Debug, Clone)]
pub struct WatcherData {
    // 监控的excel文件目录
    pub file_path: String,
    // gable文件目录
    pub target_path: String,
    // 监控的数据类型
    pub sheet_type: ESheetType,
}
