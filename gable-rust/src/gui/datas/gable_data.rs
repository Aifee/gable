use crate::gui::datas::cell_data::CellData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GableData {
    pub sheetname: String,
    pub max_row: u32,
    pub max_col: u16,
    pub heads: HashMap<u32, HashMap<u16, CellData>>,
    pub cells: HashMap<u32, HashMap<u16, CellData>>,
}
