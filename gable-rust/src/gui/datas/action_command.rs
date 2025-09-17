#[derive(Debug, Clone)]
pub enum ECommandType {
    // 编辑
    Edit,
    // 打开
    Open,
    // 根据表单导出数据
    ConvertItem,
    // 根据源表单生成代码
    GenerateItem,
    // 根据目标表单导出数据
    ConvertTarget,
    // 创建文件夹
    CreateFolder,
    // 创建Excel
    CreateExcel,
    // 创建Sheet
    CreateSheet,
    // 编辑名称
    Editname,
    // 重命名
    Rename,
    // 删除
    Delete,
}

#[derive(Debug, Clone)]
pub struct ActionCommand {
    pub com_type: ECommandType,
    pub param1: Option<String>,
    pub param2: Option<String>,
}

impl ActionCommand {
    pub fn new(com: ECommandType, arg1: Option<String>, arg2: Option<String>) -> Self {
        Self {
            com_type: com,
            param1: arg1,
            param2: arg2,
        }
    }
}
