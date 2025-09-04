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
    /// 获取普通表的有效的数据头
    pub fn get_valid_normal_heads(&self, keyword: &str) -> BTreeMap<u16, BTreeMap<u32, &CellData>> {
        let mut valids: BTreeMap<u16, BTreeMap<u32, &CellData>> = BTreeMap::new();

        let max_col = self.max_col + 1;
        for col_index in 1..max_col {
            let field_celldata =
                if let Some(row_data) = self.heads.get(&constant::TABLE_DATA_ROW_FIELD) {
                    row_data.get(&col_index)
                } else {
                    None
                };
            let type_celldata =
                if let Some(row_data) = self.heads.get(&constant::TABLE_DATA_ROW_TYPE) {
                    row_data.get(&col_index)
                } else {
                    None
                };
            let keyword_celldata =
                if let Some(row_data) = self.heads.get(&constant::TABLE_DATA_ROW_TARGET) {
                    row_data.get(&col_index)
                } else {
                    None
                };
            // 验证字段是否合法
            if let Some(field_celldata) = field_celldata {
                if !field_celldata.verify_lawful() {
                    continue;
                }
            } else {
                continue;
            }

            // 验证数据类型是否合法
            if let Some(type_celldata) = type_celldata {
                if !type_celldata.verify_lawful() {
                    continue;
                }
            } else {
                continue;
            }
            // 验证keyword是否合法
            if let Some(keyword_celldata) = keyword_celldata {
                if !keyword_celldata.verify_lawful() {
                    continue;
                }
                if !keyword_celldata.value.contains(keyword) {
                    continue;
                }
            } else {
                continue;
            }

            let mut col_datas: BTreeMap<u32, &CellData> = BTreeMap::new();
            col_datas.insert(constant::TABLE_DATA_ROW_FIELD, field_celldata.unwrap());
            col_datas.insert(constant::TABLE_DATA_ROW_TYPE, type_celldata.unwrap());
            valids.insert(col_index, col_datas);
        }
        return valids;
    }
}
