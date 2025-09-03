use crate::gui::{datas::gable_data::GableData, form::opened_cell_data::OpenedCellData};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct OpenedGableData {
    pub sheetname: String,
    pub max_row: u32,
    pub max_col: u16,
    pub items: BTreeMap<u32, BTreeMap<u16, OpenedCellData>>,
}

impl OpenedGableData {
    pub fn new(data: &GableData) -> Self {
        Self {
            sheetname: data.sheetname.clone(),
            max_row: data.max_row,
            max_col: data.max_col,
            items: Self::pairs_items(data),
        }
    }

    fn pairs_items(data: &GableData) -> BTreeMap<u32, BTreeMap<u16, OpenedCellData>> {
        let mut items: BTreeMap<u32, BTreeMap<u16, OpenedCellData>> = BTreeMap::new();
        for (row, cols) in data.heads.iter() {
            let mut cols_items: BTreeMap<u16, OpenedCellData> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let item: OpenedCellData = OpenedCellData::new(cell);
                cols_items.insert(*col, item);
            }
            items.insert(*row, cols_items);
        }

        for (row, cols) in data.cells.iter() {
            let mut cols_items: BTreeMap<u16, OpenedCellData> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let item = OpenedCellData::new(cell);
                cols_items.insert(*col, item);
            }
            items.insert(*row, cols_items);
        }
        return items;
    }
}
