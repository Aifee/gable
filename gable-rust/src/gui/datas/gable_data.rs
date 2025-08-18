use crate::gui::datas::cell_data::CellData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GableData {
    pub sheetname: String,
    pub max_row: u32,
    pub max_column: u32,
    pub heads: HashMap<String, HashMap<String, CellData>>,
    pub cells: HashMap<String, HashMap<String, CellData>>,
}
