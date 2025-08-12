use crate::common::global;
use crate::common::utils;
use crate::gui::component;
use crate::gui::datas::tree_item::TreeItem;
use eframe::egui;

#[derive(Debug, Clone)]
pub struct OpenedExcel {
    /// 当前excel中的sheet信息
    pub item: TreeItem,
    /// 当前选中的Sheet索引
    pub selected_sheet_index: usize,
}

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
        // 检查项目是否已经打开
        if let Some(index) = self
            .excels
            .iter()
            .position(|opened_item| opened_item.item.fullpath == item.fullpath)
        {
            // 如果已经打开，直接选中它
            self.selected_excel_index = Some(index);
        } else {
            let mut items: Vec<TreeItem> = Vec::new();
            for (_, v) in item.children.iter().enumerate() {
                items.push(v.clone());
            }
            // 如果未打开，添加到打开列表并选中它
            let opened_item = OpenedExcel {
                item,
                selected_sheet_index: 0,
            };
            self.excels.push(opened_item);
            self.selected_excel_index = Some(self.excels.len() - 1);
        }
    }

    pub fn ongui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    egui::Vec2::new(ui.available_width(), table_height),
                    egui::Layout::top_down(egui::Align::Min),
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
    fn get_sheet(&mut self) -> Option<&mut TreeItem> {
        if let Some(oe) = self.get_selected_excel() {
            if oe.selected_sheet_index < oe.item.children.len() {
                return Some(&mut oe.item.children[oe.selected_sheet_index]);
            }
        }
        None
    }

    fn ongui_excel_tab(&mut self, ui: &mut egui::Ui, height: f32) {
        let tab_padding = egui::Vec2::new(8.0, 4.0);
        ui.push_id("excel_tab_scroll", |ui| {
            egui::ScrollArea::horizontal()
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource::ALL)
                .wheel_scroll_multiplier(egui::Vec2::new(1.0, 1.0))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .max_height(height)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            let mut selected_index = None;
                            let mut close_index = None;
                            for (index, opened_item) in self.excels.iter().enumerate() {
                                let is_selected = self.selected_excel_index == Some(index);
                                let info = &opened_item.item.display_name;
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

    fn ongui_sheet_tab(&mut self, ui: &mut egui::Ui, height: f32) {
        let excel = self.get_selected_excel().unwrap();
        let tab_padding = egui::Vec2::new(8.0, 4.0);
        ui.push_id("sheet_tab_scroll", |ui| {
            egui::ScrollArea::horizontal()
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource::ALL)
                .wheel_scroll_multiplier(egui::Vec2::new(1.0, 1.0))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .max_height(height)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            for index in 0..excel.item.children.len() {
                                let sheet_item = &excel.item.children[index];
                                let is_selected = excel.selected_sheet_index == index;
                                let info = &sheet_item.display_name;
                                let response =
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

    fn ongui_table(&mut self, ui: &mut egui::Ui) {
        let sheet = self.get_sheet();
        if sheet.is_none() {
            ui.centered_and_justified(|ui| ui.label("请选择要浏览的页签"));
            return;
        }
        let sheet = sheet.unwrap();
        let content = &sheet.data.as_ref().unwrap().content;
        let max_row = content.max_row as usize;
        let max_col = content.max_column as usize;

        ui.push_id("table_scroll", |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(egui_extras::Column::auto())
                    // 列宽度
                    .columns(
                        egui_extras::Column::initial(100.0).range(40.0..=300.0),
                        max_col,
                    )
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("");
                        });
                        // 显示列号 (A, B, C, ...)
                        for col in 1..=max_col {
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label(utils::column_index_to_name(col as u32));
                                });
                            });
                        }
                    })
                    .body(|body| {
                        // 表头和数据都在body中显示
                        // 总行数 = 表头行数(5) + 数据行数(max_row)
                        body.rows(20.0, max_row, |mut row| {
                            // excel索引从1开始的，表现层的索引从0开始
                            let row_index = row.index() + 1;
                            // 显示行号
                            row.col(|ui| {
                                ui.label(&row_index.to_string());
                            });
                            for col in 1..=max_col {
                                row.col(|ui| {
                                    // 前5行显示heads数据（表头）
                                    if row_index < global::TABLE_DATA_ROW_TOTAL {
                                        if let Some(row_data) =
                                            content.heads.get(&row_index.to_string())
                                        {
                                            if let Some(col_data) = row_data.get(&col.to_string()) {
                                                ui.style_mut().wrap_mode =
                                                    Some(egui::TextWrapMode::Extend);
                                                ui.add(
                                                    egui::Label::new(&col_data.value).truncate(),
                                                );
                                            } else {
                                                ui.label("");
                                            }
                                        } else {
                                            ui.label("");
                                        }
                                    }
                                    // 从第6行开始显示cells数据
                                    else {
                                        if let Some(row_data) =
                                            content.cells.get(&row_index.to_string())
                                        {
                                            if let Some(col_data) = row_data.get(&col.to_string()) {
                                                ui.style_mut().wrap_mode =
                                                    Some(egui::TextWrapMode::Extend);
                                                ui.add(
                                                    egui::Label::new(&col_data.value).truncate(),
                                                );
                                            } else {
                                                ui.label("");
                                            }
                                        } else {
                                            ui.label("");
                                        }
                                    }
                                });
                            }
                        });
                    });
            });
        });
    }
}
