use crate::{
    common::constant,
    gui::datas::{cell_data::CellData, esheet_type::ESheetType},
};
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
    pub fn new(sheetname: String, sheet_type: ESheetType) -> Self {
        match sheet_type {
            ESheetType::Normal => Self::normal_template(sheetname),
            ESheetType::KV => Self::kv_template(sheetname),
            ESheetType::Enum => Self::enum_template(sheetname),
            ESheetType::Localize => Self::localize_template(sheetname),
        }
    }

    fn normal_template(sheetname: String) -> GableData {
        let mut heads: BTreeMap<u32, BTreeMap<u16, CellData>> = BTreeMap::new();
        let mut desc_cols: BTreeMap<u16, CellData> = BTreeMap::new();
        desc_cols.insert(
            1,
            CellData::new(
                constant::TABLE_NORMAL_ROW_DESC,
                1,
                "编号".to_string(),
                None,
                None,
            ),
        );
        heads.insert(constant::TABLE_NORMAL_ROW_DESC, desc_cols);
        let mut field_cols: BTreeMap<u16, CellData> = BTreeMap::new();
        field_cols.insert(
            1,
            CellData::new(
                constant::TABLE_NORMAL_ROW_FIELD,
                1,
                "id".to_string(),
                None,
                None,
            ),
        );
        heads.insert(constant::TABLE_NORMAL_ROW_FIELD, field_cols);
        let mut type_cols: BTreeMap<u16, CellData> = BTreeMap::new();
        type_cols.insert(
            1,
            CellData::new(
                constant::TABLE_NORMAL_ROW_TYPE,
                1,
                "int".to_string(),
                None,
                None,
            ),
        );
        heads.insert(constant::TABLE_NORMAL_ROW_TYPE, type_cols);
        GableData {
            sheetname,
            max_row: constant::TABLE_NORMAL_ROW_TOTAL,
            max_col: 1,
            heads: heads,
            cells: BTreeMap::new(),
        }
    }

    fn kv_template(sheetname: String) -> GableData {
        let mut heads: BTreeMap<u32, BTreeMap<u16, CellData>> = BTreeMap::new();
        let mut cols: BTreeMap<u16, CellData> = BTreeMap::new();
        cols.insert(
            constant::TABLE_KV_COL_FIELD as u16,
            CellData::new(
                1,
                constant::TABLE_KV_COL_FIELD as u16,
                "key".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_KV_COL_TYPE as u16,
            CellData::new(
                1,
                constant::TABLE_KV_COL_TYPE as u16,
                "数据类型".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_KV_COL_KEYWORD as u16,
            CellData::new(
                1,
                constant::TABLE_KV_COL_KEYWORD as u16,
                "导出目标".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_KV_COL_LINK as u16,
            CellData::new(
                1,
                constant::TABLE_KV_COL_LINK as u16,
                "关联信息".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_KV_COL_VALUE as u16,
            CellData::new(
                1,
                constant::TABLE_KV_COL_VALUE as u16,
                "值".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_KV_COL_DESC as u16,
            CellData::new(
                1,
                constant::TABLE_KV_COL_DESC as u16,
                "描述".to_string(),
                None,
                None,
            ),
        );
        heads.insert(1, cols);
        GableData {
            sheetname,
            max_row: constant::TABLE_KV_ROW_TOTAL,
            max_col: (constant::TABLE_KV_COL_DESC + 1) as u16,
            heads: heads,
            cells: BTreeMap::new(),
        }
    }

    fn enum_template(sheetname: String) -> GableData {
        let mut heads: BTreeMap<u32, BTreeMap<u16, CellData>> = BTreeMap::new();
        let mut cols: BTreeMap<u16, CellData> = BTreeMap::new();
        cols.insert(
            constant::TABLE_ENUM_COL_FIELD as u16,
            CellData::new(
                1,
                constant::TABLE_ENUM_COL_FIELD as u16,
                "字段名".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_ENUM_COL_VALUE as u16,
            CellData::new(
                1,
                constant::TABLE_ENUM_COL_VALUE as u16,
                "值".to_string(),
                None,
                None,
            ),
        );
        cols.insert(
            constant::TABLE_ENUM_COL_DESC as u16,
            CellData::new(
                1,
                constant::TABLE_ENUM_COL_DESC as u16,
                "描述".to_string(),
                None,
                None,
            ),
        );
        heads.insert(1, cols);
        GableData {
            sheetname,
            max_row: constant::TABLE_ENUM_ROW_TOTAL,
            max_col: (constant::TABLE_ENUM_COL_DESC + 1) as u16,
            heads: heads,
            cells: BTreeMap::new(),
        }
    }

    fn localize_template(sheetname: String) -> GableData {
        let mut heads: BTreeMap<u32, BTreeMap<u16, CellData>> = BTreeMap::new();
        let mut desc_cols: BTreeMap<u16, CellData> = BTreeMap::new();
        desc_cols.insert(
            1,
            CellData::new(
                constant::TABLE_LOCALIZE_ROW_DESC,
                1,
                "唯一标识".to_string(),
                None,
                None,
            ),
        );
        heads.insert(constant::TABLE_LOCALIZE_ROW_DESC, desc_cols);
        let mut field_cols: BTreeMap<u16, CellData> = BTreeMap::new();
        field_cols.insert(
            1,
            CellData::new(
                constant::TABLE_LOCALIZE_ROW_FIELD,
                1,
                "key".to_string(),
                None,
                None,
            ),
        );
        heads.insert(constant::TABLE_LOCALIZE_ROW_FIELD, field_cols);
        let mut type_cols: BTreeMap<u16, CellData> = BTreeMap::new();
        type_cols.insert(
            1,
            CellData::new(
                constant::TABLE_LOCALIZE_ROW_TYPE,
                1,
                "string".to_string(),
                None,
                None,
            ),
        );
        heads.insert(constant::TABLE_LOCALIZE_ROW_TYPE, type_cols);
        GableData {
            sheetname,
            max_row: constant::TABLE_LOCALIZE_ROW_TOTAL,
            max_col: 1,
            heads: heads,
            cells: BTreeMap::new(),
        }
    }

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
            let desc_celldata: Option<&CellData> = self
                .heads
                .get(&constant::TABLE_NORMAL_ROW_DESC)
                .unwrap()
                .get(&col_index);
            let field_celldata: &CellData =
                if let Some(row_data) = self.heads.get(&constant::TABLE_NORMAL_ROW_FIELD) {
                    if let Some(celldata) = row_data.get(&col_index) {
                        celldata
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
            let type_celldata: &CellData =
                if let Some(row_data) = self.heads.get(&constant::TABLE_NORMAL_ROW_TYPE) {
                    if let Some(celldata) = row_data.get(&col_index) {
                        celldata
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
            let keyword_celldata: &CellData =
                if let Some(row_data) = self.heads.get(&constant::TABLE_NORMAL_ROW_KEYWORD) {
                    if let Some(celldata) = row_data.get(&col_index) {
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

            let mut col_datas: BTreeMap<u32, &CellData> = BTreeMap::new();
            if let Some(desc_celldata) = desc_celldata {
                col_datas.insert(constant::TABLE_NORMAL_ROW_DESC, desc_celldata);
            }
            col_datas.insert(constant::TABLE_NORMAL_ROW_FIELD, field_celldata);
            col_datas.insert(constant::TABLE_NORMAL_ROW_TYPE, type_celldata);
            let link_celldata: Option<&CellData> = self
                .heads
                .get(&constant::TABLE_NORMAL_ROW_LINK)
                .unwrap()
                .get(&col_index);
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
