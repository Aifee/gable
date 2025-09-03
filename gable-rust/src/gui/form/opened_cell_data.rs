use crate::gui::datas::cell_data::CellData;

#[derive(Debug, Clone)]
pub struct OpenedCellData {
    pub row: u32,
    pub column: u16,
    pub value: String,
}

impl OpenedCellData {
    pub fn new(cell: &CellData) -> Self {
        Self {
            row: cell.row,
            column: cell.column,
            value: cell.value.clone(),
        }
    }
}
