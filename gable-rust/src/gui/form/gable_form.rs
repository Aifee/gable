use crate::common::constant;
use crate::gui::component;
use crate::gui::datas::tree_item::TreeItem;
use crate::gui::form::opened_excel::OpenedExcel;
use crate::gui::form::opened_gable_data::OpenedGableData;
use crate::gui::form::opened_sheet::OpenedSheet;
use eframe::egui::{
    Align, CentralPanel, Context, Label, Layout, ScrollArea, Ui, Vec2,
    scroll_area::ScrollBarVisibility, scroll_area::ScrollSource,
};
use eframe::egui::{Color32, Id, Pos2, Rect, Response};
use egui_extras::{Column, TableBuilder};
use egui_table::{CellInfo, HeaderCellInfo, TableDelegate};

#[derive(Debug, Clone)]
pub struct GableForm {
    /// 当前打开excel列表
    excels: Vec<OpenedExcel>,
    /// 当前选中excel索引
    selected_excel_index: Option<usize>,
    default_column: egui_table::Column,
    auto_size_mode: egui_table::AutoSizeMode,
}

impl GableForm {
    pub fn new() -> Self {
        Self {
            excels: Vec::new(),
            selected_excel_index: None,
            default_column: egui_table::Column::new(100.0)
                .range(80.0..=1200.0)
                .resizable(true),
            auto_size_mode: egui_table::AutoSizeMode::OnParentResize,
        }
    }

    /// 打开一个项目
    pub fn open(&mut self, item: &TreeItem) {
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

            let tab_height: f32 = 30.0;

            // 获取可用区域的尺寸
            let rect = ui.available_rect_before_wrap();
            let top_tab_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), tab_height));

            let bottom_pos = rect.max.y;
            let bottom_tab_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, bottom_pos - tab_height),
                Vec2::new(rect.width(), tab_height),
            );

            let table_rect = Rect::from_min_max(
                Pos2::new(rect.min.x, rect.min.y + tab_height),
                Pos2::new(rect.max.x, rect.max.y - tab_height),
            );

            // 绘制顶部标签页
            ui.scope_builder(
                eframe::egui::UiBuilder::new().max_rect(top_tab_rect),
                |ui| {
                    self.ongui_excel_tab(ui, tab_height);
                },
            );

            // 绘制表格
            ui.scope_builder(eframe::egui::UiBuilder::new().max_rect(table_rect), |ui| {
                self.ongui_scroll_table(ui);
            });

            // 绘制底部标签页
            ui.scope_builder(
                eframe::egui::UiBuilder::new().max_rect(bottom_tab_rect),
                |ui| {
                    self.ongui_sheet_tab(ui, tab_height);
                },
            );
        });
    }

    /**
     * 设置选中当前的excel
     */
    fn set_excel_index(&mut self, index: usize) {
        self.selected_excel_index = Some(index);
    }

    /**
     * 获取选中的excel
     */
    fn get_selected_excel(&mut self) -> Option<&mut OpenedExcel> {
        if let Some(index) = self.selected_excel_index {
            if index < self.excels.len() {
                return Some(&mut self.excels[index]);
            }
        }
        None
    }
    /**
     * 删除指定的excel
     */
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

    /**
     * 获取选中的Sheet
     */
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

    /**
     * 数据表 绘制
     */
    fn ongui_scroll_table(&mut self, ui: &mut Ui) {
        let sheet: Option<&OpenedSheet> = self.get_sheet();
        if sheet.is_none() {
            ui.centered_and_justified(|ui| ui.label("请选择要浏览的页签"));
            return;
        }
        let sheet: &OpenedSheet = sheet.unwrap();
        let gable_data: &OpenedGableData = &sheet.data;
        let show_rows: u64 = if gable_data.max_row < constant::FORM_MIN_ROW {
            constant::FORM_MIN_ROW as u64
        } else {
            gable_data.max_row as u64
        };
        let show_cols: usize = if gable_data.max_col < constant::FORM_MIN_COL {
            constant::FORM_MIN_COL
        } else {
            gable_data.max_col
        };

        let id_salt = Id::new("gable_table");
        let mut table = egui_table::Table::new()
            .id_salt(id_salt)
            .num_rows(show_rows)
            .columns(vec![self.default_column; show_cols])
            .num_sticky_cols(1)
            .headers([
                egui_table::HeaderRow {
                    height: 20.0,
                    groups: vec![],
                },
                egui_table::HeaderRow::new(20.0),
            ])
            .auto_size_mode(self.auto_size_mode);

        let mut scroll_to_column: Option<u64> = None;
        if let Some(scroll_to_column) = scroll_to_column {
            table = table.scroll_to_column(0, None);
        }
        let mut scroll_to_row: Option<u64> = None;
        if let Some(scroll_to_row) = scroll_to_row {
            table = table.scroll_to_row(0, None);
        }

        table.show(ui, self);
    }
}

impl TableDelegate for GableForm {
    fn header_cell_ui(&mut self, ui: &mut Ui, cell: &HeaderCellInfo) {
        let sheet: Option<&OpenedSheet> = self.get_sheet();
        if sheet.is_none() {
            return;
        }
        let sheet: &OpenedSheet = sheet.unwrap();
        let gable_data: &OpenedGableData = &sheet.data;

        // 第一个表头行显示列标题
        if cell.row_nr == 0 {
            if cell.group_index == 0 {
                // 左上角空白单元格
                ui.label("");
            } else {
                // 显示列标题，需要减1因为第0列是行号列
                let col_index = cell.group_index - 1;
                if col_index < gable_data.column_headers.len() {
                    ui.centered_and_justified(|ui| {
                        ui.colored_label(Color32::GRAY, &gable_data.column_headers[col_index]);
                    });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.colored_label(Color32::GRAY, "");
                    });
                }
            }
        } else {
            ui.label("");
        }
    }

    fn cell_ui(&mut self, ui: &mut Ui, cell: &CellInfo) {
        let sheet: Option<&OpenedSheet> = self.get_sheet();
        if sheet.is_none() {
            return;
        }
        let sheet: &OpenedSheet = sheet.unwrap();
        let gable_data: &OpenedGableData = &sheet.data;

        if cell.col_nr == 0 {
            // 显示行号，egui_table的行号从0开始，我们需要显示从1开始的行号
            ui.colored_label(Color32::GRAY, (cell.row_nr + 1).to_string());
        } else {
            // 显示数据内容，需要减1因为第0列是行号列
            let col_index = cell.col_nr - 1;
            let row_index = cell.row_nr as usize;

            // 获取对应单元格的数据
            if let Some(row_data) = gable_data.items.get(&row_index) {
                if let Some(cell_data) = row_data.get(&col_index) {
                    ui.add(Label::new(cell_data).truncate());
                } else {
                    ui.label("");
                }
            } else {
                ui.label("");
            }
        }
    }
}
