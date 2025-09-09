use crate::{common::constant, gui::datas::cell_data::CellData};
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

impl GableData {
    /// 获取普通表的有效的数据头,列，行
    pub fn get_valid_normal_heads(
        &self,
        keyword: &str,
    ) -> (
        BTreeMap<u16, BTreeMap<u32, &CellData>>,
        BTreeMap<u16, BTreeMap<u32, &CellData>>,
    ) {
        // 主键表头
        let mut valids_main: BTreeMap<u16, BTreeMap<u32, &CellData>> = BTreeMap::new();
        // 除主键外的其他表头数据
        let mut valids: BTreeMap<u16, BTreeMap<u32, &CellData>> = BTreeMap::new();
        let max_col = self.max_col + 1;
        for col_index in 1..max_col {
            let desc_celldata = self
                .heads
                .get(&constant::TABLE_DATA_ROW_DESC)
                .unwrap()
                .get(&col_index);
            let field_celldata =
                if let Some(row_data) = self.heads.get(&constant::TABLE_DATA_ROW_FIELD) {
                    row_data.get(&col_index).unwrap()
                } else {
                    continue;
                };
            let type_celldata =
                if let Some(row_data) = self.heads.get(&constant::TABLE_DATA_ROW_TYPE) {
                    row_data.get(&col_index).unwrap()
                } else {
                    continue;
                };
            let keyword_celldata =
                if let Some(row_data) = self.heads.get(&constant::TABLE_DATA_ROW_KEYWORD) {
                    row_data.get(&col_index).unwrap()
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

            let mut col_datas: BTreeMap<u32, &CellData> = BTreeMap::new();
            if let Some(desc_celldata) = desc_celldata {
                col_datas.insert(constant::TABLE_DATA_ROW_DESC, desc_celldata);
            }
            col_datas.insert(constant::TABLE_DATA_ROW_FIELD, field_celldata);
            col_datas.insert(constant::TABLE_DATA_ROW_TYPE, type_celldata);
            let link_celldata = self
                .heads
                .get(&constant::TABLE_DATA_ROW_LINK)
                .unwrap()
                .get(&col_index);
            if let Some(link_celldata) = link_celldata {
                col_datas.insert(constant::TABLE_DATA_ROW_LINK, link_celldata);
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
