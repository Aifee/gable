#[derive(Debug, Clone)]
pub enum ECommandType {
    EDITOR,
    OPEN,
}

#[derive(Debug, Clone)]
pub struct ActionCommand {
    pub com_type: ECommandType,
    pub param: Option<String>,
}

impl ActionCommand {
    pub fn new(com: ECommandType, args: Option<String>) -> Self {
        Self {
            com_type: com,
            param: args,
        }
    }
}
