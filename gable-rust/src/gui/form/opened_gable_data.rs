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
    pub max_row: usize,
    pub max_col: usize,
    pub column_headers: Vec<String>,
    pub items: BTreeMap<usize, BTreeMap<usize, String>>,
}

impl OpenedGableData {
    pub fn new(sheet_type: &ESheetType, data: &GableData) -> Self {
        let max_row: usize = data.get_max_row();
        let max_col: usize = data.get_max_col();
        Self {
            max_row: max_row,
            max_col: max_col,
            column_headers: Self::pairs_headers(&max_col),
            items: Self::pairs_items(sheet_type, data),
        }
    }

    fn pairs_headers(col: &usize) -> Vec<String> {
        let mut header: Vec<String> = Vec::new();
        for index in 1..=*col {
            header.push(utils::column_index_to_name(&index));
        }
        header
    }

    fn pairs_items(
        sheet_type: &ESheetType,
        data: &GableData,
    ) -> BTreeMap<usize, BTreeMap<usize, String>> {
        match sheet_type {
            ESheetType::Normal => Self::pairs_items_normal(data),
            ESheetType::Localize => Self::pairs_items_localize(data),
            ESheetType::KV => Self::pairs_items_kv(data),
            ESheetType::Enum => Self::pairs_items_enum(data),
        }
    }

    fn pairs_items_normal(data: &GableData) -> BTreeMap<usize, BTreeMap<usize, String>> {
        let mut cell_types: HashMap<usize, EDataType> = HashMap::<usize, EDataType>::new();
        let mut link_cells: HashMap<usize, &String> = HashMap::new();
        let max_row = data.get_max_row();
        // 通过预先获取的行数据查找列数据
        if max_row >= constant::TABLE_NORMAL_ROW_TOTAL {
            let max_col = data.get_max_col();
            for col_index in 0..max_col {
                let row_type_data: Option<&Vec<CellData>> =
                    data.heads.get(constant::TABLE_NORMAL_ROW_TYPE);
                if let Some(row_type_data) = row_type_data {
                    if let Some(type_cell) = row_type_data.get(col_index) {
                        let cell_type: EDataType = EDataType::convert(&type_cell.value);
                        if cell_type == EDataType::Enum || cell_type == EDataType::Loc {
                            let row_link_data = data.heads.get(constant::TABLE_NORMAL_ROW_LINK);
                            if let Some(row_link_data) = row_link_data {
                                let link_cell = row_link_data.get(col_index);
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

        let mut items: BTreeMap<usize, BTreeMap<usize, String>> = BTreeMap::new();
        for row_index in 0..data.heads.len() {
            if let Some(cols) = data.heads.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        let value: String =
                            Self::pairs_value(&EDataType::String, cell, &link_cells, &col_index);
                        cols_items.insert(col_index, value);
                    }
                }
                items.insert(row_index, cols_items);
            }
        }

        for row_index in 0..data.cells.len() {
            if let Some(cols) = data.cells.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        let cell_type: &EDataType =
                            cell_types.get(&col_index).unwrap_or(&EDataType::String);
                        let value: String =
                            Self::pairs_value(cell_type, cell, &link_cells, &col_index);
                        cols_items.insert(col_index, value);
                    }
                }
                let index = row_index + constant::TABLE_NORMAL_ROW_TOTAL;
                items.insert(index, cols_items);
            }
        }
        items
    }

    fn pairs_items_localize(data: &GableData) -> BTreeMap<usize, BTreeMap<usize, String>> {
        let mut items: BTreeMap<usize, BTreeMap<usize, String>> = BTreeMap::new();
        for row_index in 0..data.heads.len() {
            if let Some(cols) = data.heads.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        cols_items.insert(col_index, cell.value.clone());
                    }
                }
                items.insert(row_index, cols_items);
            }
        }

        for row_index in 0..data.cells.len() {
            if let Some(cols) = data.cells.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        cols_items.insert(col_index, cell.value.clone());
                    }
                }
                let index = row_index + constant::TABLE_LOCALIZE_ROW_TOTAL;
                items.insert(index, cols_items);
            }
        }
        items
    }

    fn pairs_items_kv(data: &GableData) -> BTreeMap<usize, BTreeMap<usize, String>> {
        let mut link_cells: HashMap<usize, &String> = HashMap::new();
        let mut items: BTreeMap<usize, BTreeMap<usize, String>> = BTreeMap::new();
        for row_index in 0..data.heads.len() {
            if let Some(cols) = data.heads.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        let value: String =
                            Self::pairs_value(&EDataType::String, cell, &link_cells, &row_index);
                        cols_items.insert(col_index, value);
                    }
                }
                items.insert(row_index, cols_items);
            }
        }
        for row_index in 0..data.cells.len() {
            if let Some(cols) = data.cells.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                let mut cell_type: EDataType = EDataType::String;
                link_cells.clear();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        if col_index == constant::TABLE_KV_COL_TYPE {
                            cell_type = EDataType::convert(&cell.value);
                        }
                        if col_index == constant::TABLE_KV_COL_LINK
                            && (cell_type == EDataType::Enum || cell_type == EDataType::Loc)
                        {
                            let cell_link_value: Option<&String> = Some(&cell.value);
                            if let Some(link_value) = cell_link_value {
                                link_cells.insert(row_index, link_value);
                            }
                        }
                        if col_index == constant::TABLE_KV_COL_VALUE {
                            let value: String =
                                Self::pairs_value(&cell_type, cell, &link_cells, &row_index);
                            cols_items.insert(col_index, value);
                        } else {
                            let value: String = Self::pairs_value(
                                &EDataType::String,
                                cell,
                                &link_cells,
                                &row_index,
                            );
                            cols_items.insert(col_index, value);
                        }
                    }
                }
                let index = row_index + constant::TABLE_KV_ROW_TOTAL;
                items.insert(index, cols_items);
            }
        }
        items
    }

    fn pairs_items_enum(data: &GableData) -> BTreeMap<usize, BTreeMap<usize, String>> {
        let mut items: BTreeMap<usize, BTreeMap<usize, String>> = BTreeMap::new();
        let link_cells: HashMap<usize, &String> = HashMap::new();
        for row_index in 0..data.heads.len() {
            if let Some(cols) = data.heads.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        let value: String =
                            Self::pairs_value(&EDataType::String, cell, &link_cells, &col_index);
                        cols_items.insert(col_index, value);
                    }
                }
                items.insert(row_index, cols_items);
            }
        }
        for row_index in 0..data.cells.len() {
            if let Some(cols) = data.cells.get(row_index) {
                let mut cols_items: BTreeMap<usize, String> = BTreeMap::new();
                for col_index in 0..cols.len() {
                    if let Some(cell) = cols.get(col_index) {
                        let value: String =
                            Self::pairs_value(&EDataType::String, cell, &link_cells, &col_index);
                        cols_items.insert(col_index, value);
                    }
                }
                let index = row_index + constant::TABLE_ENUM_ROW_TOTAL;
                items.insert(index, cols_items);
            }
        }
        items
    }

    fn pairs_value(
        data_type: &EDataType,
        cell: &CellData,
        link_cells: &HashMap<usize, &String>,
        index: &usize,
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
                if let Some(link_name) = link_cells.get(index) {
                    gables::get_enum_cells(link_name, |cell_data| {
                        for link_data in cell_data.cells.iter() {
                            if let Some(enum_value_cell) =
                                link_data.get(constant::TABLE_ENUM_COL_VALUE)
                            {
                                if enum_value_cell.value == cell.value {
                                    if let Some(enum_desc_cell) =
                                        link_data.get(constant::TABLE_ENUM_COL_DESC)
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
            EDataType::Loc => {
                let mut loc_value: String = cell.value.clone();
                if let Some(link_name) = link_cells.get(&index) {
                    gables::get_loc_cells(link_name, |loc_item_cells| {
                        let mut link_key_index: usize = 0;
                        let mut link_value_index: usize = 0;
                        if let Some(link_key_cell) =
                            loc_item_cells.heads.get(constant::TABLE_LOCALIZE_ROW_FIELD)
                        {
                            for col_index in 0..link_key_cell.len() {
                                if let Some(col_cell) = link_key_cell.get(col_index) {
                                    if col_cell.value.contains("*") {
                                        link_key_index = col_index;
                                    }
                                    if col_cell.value.contains("#") {
                                        link_value_index = col_index;
                                    }
                                }
                            }
                        }

                        for loc_row_cell in loc_item_cells.cells.iter() {
                            if let Some(loc_value_cell) = loc_row_cell.get(link_key_index) {
                                if loc_value_cell.value == cell.value {
                                    if let Some(loc_desc_cell) = loc_row_cell.get(link_value_index)
                                    {
                                        loc_value = loc_desc_cell.value.clone();
                                    }
                                    break;
                                }
                            }
                        }
                    });
                }
                loc_value
            }
            _ => cell.value.clone(),
        }
    }
}
