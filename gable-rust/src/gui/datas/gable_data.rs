use crate::{
    common::constant,
    gui::datas::{cell_data::CellData, esheet_type::ESheetType},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GableData {
    pub heads: Vec<Vec<CellData>>,
    pub cells: Vec<Vec<CellData>>,
}

impl GableData {
    /**
     * 创建一个新的GableData实例
     * @param sheet_type 表格类型
     * @return 返回对应类型的GableData模板实例
     */
    pub fn new(sheet_type: ESheetType) -> Self {
        match sheet_type {
            ESheetType::Normal => Self::normal_template(),
            ESheetType::KV => Self::kv_template(),
            ESheetType::Enum => Self::enum_template(),
            ESheetType::Localize => Self::localize_template(),
        }
    }
    /**
     * 获取表格的行数
     * @return 返回表格的最大行数
     */
    pub fn get_max_row(&self) -> usize {
        let head_row = self.heads.len();
        let cells_row = self.cells.len();
        return head_row + cells_row;
    }
    /**
     * 获取表格的列数
     * @return 最大列数
     */
    pub fn get_max_col(&self) -> usize {
        let mut max_col = 0;
        for row in self.heads.iter() {
            let col_len = row.len();
            if max_col < col_len {
                max_col = col_len;
            }
        }
        for row in self.cells.iter() {
            let col_len = row.len();
            if max_col < col_len {
                max_col = col_len;
            }
        }
        return max_col;
    }

    /**
     * 创建普通表格模板
     * @return 返回普通表格的GableData模板
     */
    fn normal_template() -> GableData {
        let mut heads: Vec<Vec<CellData>> = Vec::new();
        let mut desc_cols: Vec<CellData> = Vec::new();
        desc_cols.push(CellData::new("ID".to_string(), None, None));
        heads.push(desc_cols);
        let mut field_cols: Vec<CellData> = Vec::new();
        field_cols.push(CellData::new("*id".to_string(), None, None));
        heads.push(field_cols);
        let mut type_cols: Vec<CellData> = Vec::new();
        type_cols.push(CellData::new("int".to_string(), None, None));
        heads.push(type_cols);
        heads.push(Vec::new());
        heads.push(Vec::new());

        let mut cells: Vec<Vec<CellData>> = Vec::new();
        let mut value_cols: Vec<CellData> = Vec::new();
        value_cols.push(CellData::new("1".to_string(), None, None));
        cells.push(value_cols);
        GableData {
            heads: heads,
            cells: cells,
        }
    }

    /**
     * 创建键值对表格模板
     * @return 返回键值对表格的GableData模板
     */
    fn kv_template() -> GableData {
        let mut heads: Vec<Vec<CellData>> = Vec::new();
        let mut cols: Vec<CellData> = Vec::new();
        cols.push(CellData::new("key".to_string(), None, None));
        cols.push(CellData::new("Data Type".to_string(), None, None));
        cols.push(CellData::new("Target".to_string(), None, None));
        cols.push(CellData::new("Link Info".to_string(), None, None));
        cols.push(CellData::new("Value".to_string(), None, None));
        cols.push(CellData::new("Description".to_string(), None, None));
        heads.push(cols);
        GableData {
            heads: heads,
            cells: Vec::new(),
        }
    }

    /**
     * 创建枚举表格模板
     * @return 返回枚举表格的GableData模板
     */
    fn enum_template() -> GableData {
        let mut heads: Vec<Vec<CellData>> = Vec::new();
        let mut cols: Vec<CellData> = Vec::new();
        cols.push(CellData::new("Field Name".to_string(), None, None));
        cols.push(CellData::new("Value".to_string(), None, None));
        cols.push(CellData::new("Description".to_string(), None, None));
        heads.push(cols);
        GableData {
            heads: heads,
            cells: Vec::new(),
        }
    }

    /**
     * 创建本地化表格模板
     * @return 返回本地化表格的GableData模板
     */
    fn localize_template() -> GableData {
        let mut heads: Vec<Vec<CellData>> = Vec::new();
        let mut desc_cols: Vec<CellData> = Vec::new();
        desc_cols.push(CellData::new("ID".to_string(), None, None));
        heads.push(desc_cols);
        let mut field_cols: Vec<CellData> = Vec::new();
        field_cols.push(CellData::new("*key".to_string(), None, None));
        heads.push(field_cols);
        let mut type_cols: Vec<CellData> = Vec::new();
        type_cols.push(CellData::new("string".to_string(), None, None));
        heads.push(type_cols);
        heads.push(Vec::new()); // 空行(关键字)
        heads.push(Vec::new()); // 空行(链接)
        GableData {
            heads: heads,
            cells: Vec::new(),
        }
    }

    /**
     * 获取普通表的有效的数据头,列，行
     * @param keyword 关键字，用于筛选包含该关键字的数据
     * @return 返回一个元组，包含主键表头数据和其他表头数据
     */
    pub fn get_valid_normal_heads(
        &self,
        keyword: &str,
    ) -> (
        BTreeMap<usize, BTreeMap<usize, &CellData>>,
        BTreeMap<usize, BTreeMap<usize, &CellData>>,
    ) {
        // 主键表头
        let mut valids_main: BTreeMap<usize, BTreeMap<usize, &CellData>> = BTreeMap::new();
        // 除主键外的其他表头数据
        let mut valids: BTreeMap<usize, BTreeMap<usize, &CellData>> = BTreeMap::new();
        let max_col: usize = self.get_max_col() + 1;
        for col_index in 0..max_col {
            let desc_celldata: Option<&CellData> = self
                .heads
                .get(constant::TABLE_NORMAL_ROW_DESC)
                .and_then(|row| row.get(col_index));
            let field_celldata: &CellData =
                if let Some(row_data) = self.heads.get(constant::TABLE_NORMAL_ROW_FIELD) {
                    if let Some(celldata) = row_data.get(col_index) {
                        celldata
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
            let type_celldata: &CellData =
                if let Some(row_data) = self.heads.get(constant::TABLE_NORMAL_ROW_TYPE) {
                    if let Some(celldata) = row_data.get(col_index) {
                        celldata
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
            let keyword_celldata: &CellData =
                if let Some(row_data) = self.heads.get(constant::TABLE_NORMAL_ROW_KEYWORD) {
                    if let Some(celldata) = row_data.get(col_index) {
                        celldata
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
            // 验证字段是否合法
            if !field_celldata.verify_lawful() {
                continue;
            }

            // 验证数据类型是否合法
            if !type_celldata.verify_lawful() {
                continue;
            }
            // 验证keyword是否合法
            if !keyword_celldata.verify_lawful() {
                continue;
            }
            if !keyword_celldata.value.contains(keyword) {
                continue;
            }

            let mut col_datas: BTreeMap<usize, &CellData> = BTreeMap::new();
            if let Some(desc_celldata) = desc_celldata {
                col_datas.insert(constant::TABLE_NORMAL_ROW_DESC, desc_celldata);
            }
            col_datas.insert(constant::TABLE_NORMAL_ROW_FIELD, field_celldata);
            col_datas.insert(constant::TABLE_NORMAL_ROW_TYPE, type_celldata);
            let link_celldata: Option<&CellData> = self
                .heads
                .get(constant::TABLE_NORMAL_ROW_LINK)
                .and_then(|row| row.get(col_index));
            if let Some(link_celldata) = link_celldata {
                col_datas.insert(constant::TABLE_NORMAL_ROW_LINK, link_celldata);
            }
            if field_celldata.value.contains("*") {
                valids_main.insert(col_index, col_datas);
            } else {
                valids.insert(col_index, col_datas);
            }
        }
        return (valids_main, valids);
    }
}
