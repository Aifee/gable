use crate::gui::datas::cell_data::CellData;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GableData {
    pub sheetname: String,
    pub max_row: u32,
    pub max_col: u16,
    pub heads: BTreeMap<u32, BTreeMap<u16, CellData>>,
    pub cells: BTreeMap<u32, BTreeMap<u16, CellData>>,
}
