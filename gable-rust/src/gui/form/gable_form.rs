use crate::common::constant;
use crate::gui::component;
use crate::gui::datas::tree_item::TreeItem;
use crate::gui::form::opened_excel::OpenedExcel;
use crate::gui::form::opened_gable_data::OpenedGableData;
use crate::gui::form::opened_sheet::OpenedSheet;
use eframe::egui::Response;
use eframe::egui::{
    Align, CentralPanel, Context, Label, Layout, ScrollArea, Ui, Vec2,
    scroll_area::ScrollBarVisibility, scroll_area::ScrollSource,
};
use egui_extras::{Column, TableBuilder};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct GableForm {
    /// 当前打开excel列表
    excels: Vec<OpenedExcel>,
    /// 当前选中excel索引
    selected_excel_index: Option<usize>,
}

impl GableForm {
    pub fn new() -> Self {
        Self {
            excels: Vec::new(),
            selected_excel_index: None,
        }
    }

    /// 打开一个项目
    pub fn open(&mut self, item: TreeItem) {
        if let Some(index) = self
            .excels
            .iter()
            .position(|opened_item| opened_item.full_path == item.fullpath)
        {
            self.selected_excel_index = Some(index);
        } else {
            let mut items: Vec<TreeItem> = Vec::new();
            for (_, v) in item.children.iter().enumerate() {
                items.push(v.clone());
            }
            let opened_item: OpenedExcel = OpenedExcel::new(item);
            self.excels.push(opened_item);
            self.selected_excel_index = Some(self.excels.len() - 1);
        }
    }

    pub fn ongui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.set_min_height(100.0);
            if self.excels.is_empty() {
                ui.centered_and_justified(|ui| ui.label("双击左侧文件树中的项目以打开"));
                return;
            }
            ui.vertical(|ui| {
                let tab_height: f32 = 30.0;
                self.ongui_excel_tab(ui, tab_height);
                let table_height: f32 =
                    ui.available_height() - tab_height - ui.spacing().item_spacing.y;
                // 表格区域填充剩余空间
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), table_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        self.ongui_table(ui);
                    },
                );
                self.ongui_sheet_tab(ui, tab_height);
            });
        });
    }

    /// 设置选中当前的excel
    fn set_excel_index(&mut self, index: usize) {
        self.selected_excel_index = Some(index);
    }

    /// 获取选中的excel
    fn get_selected_excel(&mut self) -> Option<&mut OpenedExcel> {
        if let Some(index) = self.selected_excel_index {
            if index < self.excels.len() {
                return Some(&mut self.excels[index]);
            }
        }
        None
    }
    /// 删除指定的excel
    fn remove_item(&mut self, index: usize) {
        if index < self.excels.len() {
            self.excels.remove(index);
        }
        if self.excels.len() <= 0 {
            self.selected_excel_index = None;
        } else {
            if self.selected_excel_index == Some(index) {
                if index < self.excels.len() {
                    self.selected_excel_index = Some(index);
                } else {
                    self.selected_excel_index = Some(self.excels.len() - 1);
                }
            }
        }
    }

    /// 获取选中的Sheet
    fn get_sheet(&mut self) -> Option<&OpenedSheet> {
        if let Some(oe) = self.get_selected_excel() {
            if oe.selected_sheet_index < oe.sheets.len() {
                return Some(&oe.sheets[oe.selected_sheet_index]);
            }
        }
        None
    }

    fn ongui_excel_tab(&mut self, ui: &mut Ui, height: f32) {
        let tab_padding: Vec2 = Vec2::new(8.0, 4.0);
        ui.push_id("excel_tab_scroll", |ui| {
            ScrollArea::horizontal()
                .auto_shrink(false)
                .scroll_source(ScrollSource::ALL)
                .wheel_scroll_multiplier(Vec2::new(1.0, 1.0))
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .max_height(height)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            let mut selected_index: Option<usize> = None;
                            let mut close_index: Option<usize> = None;
                            for (index, opened_item) in self.excels.iter().enumerate() {
                                let is_selected: bool = self.selected_excel_index == Some(index);
                                let info: &String = &opened_item.display_name;
                                let (response, close_response) =
                                    component::excel_tap(ui, info, is_selected, tab_padding);
                                if response.clicked() {
                                    selected_index = Some(index);
                                }
                                if let Some(close_resp) = close_response {
                                    if close_resp.clicked() {
                                        close_index = Some(index);
                                    }
                                }
                            }
                            if let Some(index) = selected_index {
                                self.set_excel_index(index)
                            }
                            if let Some(index) = close_index {
                                self.remove_item(index);
                            }
                        });
                    });
                });
        });
    }

    fn ongui_sheet_tab(&mut self, ui: &mut Ui, height: f32) {
        if let Some(excel) = self.get_selected_excel() {
            let tab_padding = Vec2::new(8.0, 4.0);
            ui.push_id("sheet_tab_scroll", |ui| {
                ScrollArea::horizontal()
                    .auto_shrink(false)
                    .scroll_source(ScrollSource::ALL)
                    .wheel_scroll_multiplier(Vec2::new(1.0, 1.0))
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .max_height(height)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                for index in 0..excel.sheets.len() {
                                    let sheet_item: &OpenedSheet = &excel.sheets[index];
                                    let is_selected: bool = excel.selected_sheet_index == index;
                                    let info: &String = &sheet_item.display_name;
                                    let response: Response =
                                        component::sheet_tab(ui, info, is_selected, tab_padding);
                                    if response.clicked() {
                                        excel.selected_sheet_index = index;
                                    }
                                }
                            });
                        });
                    });
            });
        }
    }

    /// 数据表 绘制
    fn ongui_table(&mut self, ui: &mut Ui) {
        let sheet: Option<&OpenedSheet> = self.get_sheet();
        if sheet.is_none() {
            ui.centered_and_justified(|ui| ui.label("请选择要浏览的页签"));
            return;
        }
        let sheet: &OpenedSheet = sheet.unwrap();
        // let sheet_type: &ESheetType = &sheet.gable_type;
        let gable_data: &OpenedGableData = &sheet.data;

        let show_rows: u32 = if gable_data.max_row < constant::FORM_MIN_ROW {
            constant::FORM_MIN_ROW
        } else {
            gable_data.max_row
        };
        let show_cols: u16 = if gable_data.max_col < constant::FORM_MIN_COL {
            constant::FORM_MIN_COL
        } else {
            gable_data.max_col
        };

        ui.push_id("table_scroll", |ui| {
            ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::auto())
                    .columns(
                        Column::initial(100.0).range(40.0..=300.0),
                        show_cols as usize,
                    )
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("");
                        });
                        for header_index in gable_data.column_headers.iter() {
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label(header_index);
                                });
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(20.0, show_rows as usize, |mut row| {
                            let row_index: u32 = (row.index() + 1) as u32;
                            row.col(|ui| {
                                ui.label(&row_index.to_string());
                            });
                            let row_data: Option<&BTreeMap<u16, String>> =
                                gable_data.items.get(&row_index);
                            for col_index in 1..show_cols + 1 {
                                row.col(|ui| {
                                    if let Some(row_data) = row_data {
                                        if let Some(cell_data) = row_data.get(&col_index) {
                                            ui.add(Label::new(cell_data).truncate());
                                        } else {
                                            ui.label("");
                                        }
                                    } else {
                                        ui.label("");
                                    }
                                });
                            }
                        });
                    });
            });
        });
    }
}
