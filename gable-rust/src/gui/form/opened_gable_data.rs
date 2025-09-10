use crate::{
    common::{constant, utils},
    gui::datas::{
        cell_data::CellData, edata_type::EDataType, esheet_type::ESheetType, gable_data::GableData,
        gables,
    },
};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct OpenedGableData {
    pub max_row: u32,
    pub max_col: u16,
    pub column_headers: Vec<String>,
    pub items: BTreeMap<u32, BTreeMap<u16, String>>,
}

impl OpenedGableData {
    pub fn new(sheet_type: &ESheetType, data: &GableData) -> Self {
        Self {
            max_row: data.max_row,
            max_col: data.max_col,
            column_headers: Self::pairs_headers(data.max_col as u32),
            items: Self::pairs_items(sheet_type, data),
        }
    }

    fn pairs_headers(col: u32) -> Vec<String> {
        let mut header: Vec<String> = Vec::new();
        for index in 1..=col {
            header.push(utils::column_index_to_name(index));
        }
        header
    }

    fn pairs_items(
        sheet_type: &ESheetType,
        data: &GableData,
    ) -> BTreeMap<u32, BTreeMap<u16, String>> {
        match sheet_type {
            ESheetType::Normal => Self::pairs_items_normal(data),
            ESheetType::KV => Self::pairs_items_kv(data),
            ESheetType::Enum => Self::pairs_items_enum(data),
        }
    }

    fn pairs_items_normal(data: &GableData) -> BTreeMap<u32, BTreeMap<u16, String>> {
        let total_rows: u32 = data.max_row;
        let total_cols: u16 = data.max_col;
        let mut cell_types: HashMap<u16, EDataType> = HashMap::<u16, EDataType>::new();
        let mut link_cells: HashMap<u16, &String> = HashMap::new();
        // 通过预先获取的行数据查找列数据
        if total_rows >= constant::TABLE_NORMAL_ROW_TOTAL {
            for col_index in 1..total_cols + 1 {
                let row_type_data: Option<&BTreeMap<u16, CellData>> =
                    data.heads.get(&constant::TABLE_NORMAL_ROW_TYPE);
                if let Some(row_type_data) = row_type_data {
                    let type_cell = row_type_data.get(&col_index);
                    if let Some(type_cell) = type_cell {
                        let cell_type: EDataType = EDataType::convert(&type_cell.value);
                        if cell_type == EDataType::Enum {
                            let row_link_data: Option<&BTreeMap<u16, CellData>> =
                                data.heads.get(&constant::TABLE_NORMAL_ROW_LINK);
                            if let Some(row_link_data) = row_link_data {
                                let link_cell = row_link_data.get(&col_index);
                                if let Some(link_cell) = link_cell {
                                    if !link_cell.value.is_empty() {
                                        link_cells.insert(col_index, &link_cell.value);
                                    }
                                }
                            }
                        }
                        cell_types.insert(col_index, cell_type);
                    }
                }
            }
        }

        let mut items: BTreeMap<u32, BTreeMap<u16, String>> = BTreeMap::new();
        for (row, cols) in data.heads.iter() {
            let mut cols_items: BTreeMap<u16, String> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let value: String = Self::pairs_value(&EDataType::String, cell, &link_cells, false);
                cols_items.insert(*col, value);
            }
            items.insert(*row, cols_items);
        }

        for (row, cols) in data.cells.iter() {
            let mut cols_items: BTreeMap<u16, String> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let cell_type: &EDataType = cell_types.get(&col).unwrap();
                let value: String = Self::pairs_value(cell_type, cell, &link_cells, false);
                cols_items.insert(*col, value);
            }
            items.insert(*row, cols_items);
        }
        items
    }
    fn pairs_items_kv(data: &GableData) -> BTreeMap<u32, BTreeMap<u16, String>> {
        let mut link_cells: HashMap<u16, &String> = HashMap::new();
        let mut items: BTreeMap<u32, BTreeMap<u16, String>> = BTreeMap::new();
        for (row, cols) in data.heads.iter() {
            let mut cols_items: BTreeMap<u16, String> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let value: String = Self::pairs_value(&EDataType::String, cell, &link_cells, true);
                cols_items.insert(*col, value);
            }
            items.insert(*row, cols_items);
        }
        for (row, cols) in data.cells.iter() {
            let mut cols_items: BTreeMap<u16, String> = BTreeMap::new();
            let mut cell_type: EDataType = EDataType::String;
            link_cells.clear();
            for (col, cell) in cols.iter() {
                if col == &(constant::TABLE_KV_COL_TYPE as u16) {
                    cell_type = EDataType::convert(&cell.value);
                }
                if col == &(constant::TABLE_KV_COL_LINK as u16) && cell_type == EDataType::Enum {
                    let cell_link_value: Option<&String> = Some(&cell.value);
                    if let Some(link_value) = cell_link_value {
                        link_cells.insert(*row as u16, link_value);
                    }
                }
                if col == &(constant::TABLE_KV_COL_VALUE as u16) {
                    let value: String = Self::pairs_value(&cell_type, cell, &link_cells, true);
                    cols_items.insert(*col, value);
                } else {
                    let value: String =
                        Self::pairs_value(&EDataType::String, cell, &link_cells, true);
                    cols_items.insert(*col, value);
                }
            }
            items.insert(*row, cols_items);
        }
        items
    }
    fn pairs_items_enum(data: &GableData) -> BTreeMap<u32, BTreeMap<u16, String>> {
        let mut items: BTreeMap<u32, BTreeMap<u16, String>> = BTreeMap::new();
        let link_cells: HashMap<u16, &String> = HashMap::new();
        for (row, cols) in data.heads.iter() {
            let mut cols_items: BTreeMap<u16, String> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let value: String = Self::pairs_value(&EDataType::String, cell, &link_cells, false);
                cols_items.insert(*col, value);
            }
            items.insert(*row, cols_items);
        }
        for (row, cols) in data.cells.iter() {
            let mut cols_items: BTreeMap<u16, String> = BTreeMap::new();
            for (col, cell) in cols.iter() {
                let value: String = Self::pairs_value(&EDataType::String, cell, &link_cells, false);
                cols_items.insert(*col, value);
            }
            items.insert(*row, cols_items);
        }
        items
    }

    fn pairs_value(
        data_type: &EDataType,
        cell: &CellData,
        link_cells: &HashMap<u16, &String>,
        iskv: bool,
    ) -> String {
        match data_type {
            EDataType::Percentage => {
                format!("{:.0}%", cell.parse_float() * 100.0)
            }
            EDataType::Permillage => {
                format!("{:.0}‰", cell.parse_float() * 1000.0)
            }
            EDataType::Permian => {
                format!("{:.0}‱", cell.parse_float() * 10000.0)
            }
            EDataType::Time => cell.convert_time(),
            EDataType::Date => cell.convert_date(),
            EDataType::Enum => {
                let mut enum_value: String = cell.value.clone();
                let index: u16 = if iskv { cell.row as u16 } else { cell.column };
                if let Some(link_name) = link_cells.get(&index) {
                    gables::get_enum_cells(link_name, |cell_data| {
                        for (_, link_data) in cell_data.cells.iter() {
                            if let Some(enum_value_cell) =
                                link_data.get(&constant::TABLE_ENUM_COL_VALUE)
                            {
                                if enum_value_cell.value == cell.value {
                                    if let Some(enum_desc_cell) =
                                        link_data.get(&constant::TABLE_ENUM_COL_DESC)
                                    {
                                        enum_value = enum_desc_cell.value.clone();
                                    }
                                    break;
                                }
                            }
                        }
                    });
                }
                enum_value
            }
            _ => cell.value.clone(),
        }
    }
}
