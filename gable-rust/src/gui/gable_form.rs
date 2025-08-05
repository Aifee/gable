use crate::common::global;
use crate::gui::datas::gables::GableData;
use crate::gui::datas::gables::{ItemType, TreeItem};
use eframe::egui;

#[derive(Debug, Clone)]
pub struct OpenedItem {
    /// 打开的项目
    pub item: TreeItem,
    /// 当前选中的Sheet索引
    pub selected_sheet_index: usize,
}

#[derive(Debug, Clone)]
pub struct GableForm {
    /// 当前打开的项目列表
    opened_items: Vec<OpenedItem>,
    /// 当前选中的项目索引
    selected_item_index: Option<usize>,
}

impl GableForm {
    pub fn new() -> Self {
        Self {
            opened_items: Vec::new(),
            selected_item_index: None,
        }
    }

    /// 打开一个项目
    pub fn open(&mut self, item: TreeItem) {
        // 检查项目是否已经打开
        if let Some(index) = self
            .opened_items
            .iter()
            .position(|opened_item| opened_item.item.fullpath == item.fullpath)
        {
            // 如果已经打开，直接选中它
            self.selected_item_index = Some(index);
        } else {
            // 如果未打开，添加到打开列表并选中它
            let opened_item = OpenedItem {
                item,
                selected_sheet_index: 0,
            };
            self.opened_items.push(opened_item);
            self.selected_item_index = Some(self.opened_items.len() - 1);
        }
    }

    /// 获取当前选中的项目
    fn get_selected_item(&self) -> Option<&OpenedItem> {
        if let Some(index) = self.selected_item_index {
            if index < self.opened_items.len() {
                return Some(&self.opened_items[index]);
            }
        }
        None
    }

    /// 获取当前选中的项目（可变引用）
    fn get_selected_item_mut(&mut self) -> Option<&mut OpenedItem> {
        if let Some(index) = self.selected_item_index {
            if index < self.opened_items.len() {
                return Some(&mut self.opened_items[index]);
            }
        }
        None
    }

    /// 获取指定项目的所有Sheet子项（静态版本）
    fn get_sheet_items_static(item: &TreeItem) -> Vec<&TreeItem> {
        item.children
            .iter()
            .filter(|child| child.item_type == ItemType::Sheet)
            .collect()
    }
    /// 渲染表格内容
    fn render_table(&self, ui: &mut egui::Ui, gable_content: &GableData) {
        let max_row = gable_content.max_row as usize;
        let max_column = gable_content.max_column as usize;

        // 创建表格（不使用header）
        egui_extras::TableBuilder::new(ui)
            // 是否显示边框
            .striped(true)
            // 是否可拖动列宽
            .resizable(true)
            // 单元格默认布局
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            // 行号
            .column(egui_extras::Column::auto())
            // 列宽度
            .columns(
                egui_extras::Column::initial(100.0).range(40.0..=300.0),
                max_column,
            )
            .header(20.0, |mut header| {
                // 表头行
                header.col(|ui| {
                    // 左上角空白单元格（行号列和列号行的交汇处）
                    ui.label("");
                });

                // 显示列号 (A, B, C, ...)
                for col in 1..=max_column {
                    header.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(self.column_number_to_name(col as u32));
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
                    for col in 1..=max_column {
                        row.col(|ui| {
                            // 前5行显示heads数据（表头）
                            if row_index < global::TABLE_DATA_ROW_TOTAL {
                                if let Some(row_data) =
                                    gable_content.heads.get(&row_index.to_string())
                                {
                                    if let Some(col_data) = row_data.get(&col.to_string()) {
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                                        ui.add(egui::Label::new(&col_data.value).truncate());
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
                                    gable_content.cells.get(&row_index.to_string())
                                {
                                    if let Some(col_data) = row_data.get(&col.to_string()) {
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                                        ui.add(egui::Label::new(&col_data.value).truncate());
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
    }

    /// 将列号转换为Excel风格的列名（A, B, ..., Z, AA, AB, ...）
    fn column_number_to_name(&self, column_number: u32) -> String {
        let mut result = String::new();
        let mut num = column_number;

        while num > 0 {
            let remainder = (num - 1) % 26;
            result.insert(0, (b'A' + remainder as u8) as char);
            num = (num - 1) / 26;
        }

        result
    }
    /// 渲染顶部标签页
    fn render_item_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut tab_to_close: Option<usize> = None;

            // 创建临时索引和项目列表的副本以避免借用冲突
            let items_count = self.opened_items.len();
            for index in 0..items_count {
                // 确保索引仍然有效
                if index >= self.opened_items.len() {
                    break;
                }

                let opened_item = &self.opened_items[index];
                let is_selected = self.selected_item_index == Some(index);

                ui.horizontal(|ui| {
                    // 标签文本
                    let label_response =
                        ui.selectable_label(is_selected, &opened_item.item.display_name);

                    // 关闭按钮，使用较小的按钮
                    let close_response = ui.add(egui::Button::new("✕").small().frame(false));
                    if close_response.clicked() {
                        tab_to_close = Some(index);
                    }

                    if label_response.clicked() {
                        self.selected_item_index = Some(index);
                    }
                });

                // 在标签之间添加一点间距
                ui.add_space(2.0);
            }

            // 处理标签关闭
            if let Some(index) = tab_to_close {
                // 确保索引仍然有效
                if index < self.opened_items.len() {
                    self.close_item_tab(index);
                }
            }
        });
    }

    /// 关闭一个项目标签页
    fn close_item_tab(&mut self, index: usize) {
        // 检查索引是否有效
        if index >= self.opened_items.len() {
            return;
        }

        // 从打开的项目列表中移除指定索引的项目
        self.opened_items.remove(index);

        // 调整选中项索引
        match self.selected_item_index {
            // 如果关闭的是当前选中的标签页
            Some(selected_index) if selected_index == index => {
                // 如果还有其他标签页
                if !self.opened_items.is_empty() {
                    // 如果关闭的不是第一个标签页，选中前面一个标签页
                    if index > 0 {
                        self.selected_item_index = Some(index - 1);
                    }
                    // 如果关闭的是第一个标签页，选中新的第一个标签页（索引0）
                    else {
                        self.selected_item_index = Some(0);
                    }
                }
                // 如果没有其他标签页，取消选中
                else {
                    self.selected_item_index = None;
                }
            }
            // 如果关闭的是选中项之前的标签页，需要调整选中项索引
            Some(selected_index) if selected_index > index => {
                self.selected_item_index = Some(selected_index - 1);
            }
            // 其他情况保持当前选中项不变
            _ => {}
        }

        // 最后确保选中的索引不会超出范围
        if let Some(selected_index) = self.selected_item_index {
            if selected_index >= self.opened_items.len() {
                if self.opened_items.is_empty() {
                    self.selected_item_index = None;
                } else {
                    self.selected_item_index = Some(self.opened_items.len() - 1);
                }
            }
        }
    }

    /// 渲染底部Sheet标签页
    fn render_sheet_tabs(&mut self, ui: &mut egui::Ui, sheet_items: &[&TreeItem]) {
        ui.horizontal(|ui| {
            for (index, sheet_item) in sheet_items.iter().enumerate() {
                let is_selected = if let Some(selected_item) = self.get_selected_item() {
                    selected_item.selected_sheet_index == index
                } else {
                    false
                };

                if ui
                    .selectable_label(is_selected, &sheet_item.display_name)
                    .clicked()
                {
                    if let Some(selected_item) = self.get_selected_item_mut() {
                        selected_item.selected_sheet_index = index;
                    }
                }
            }
        });
    }

    /// 主渲染函数
    pub fn gui_form(&mut self, ui: &mut egui::Ui) {
        // 如果没有选中的项目或没有打开的项目，显示提示信息
        if self.selected_item_index.is_none() || self.opened_items.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("双击左侧文件树中的项目以打开");
            });
            return;
        }

        // 确保选中的索引是有效的
        let selected_index = match self.selected_item_index {
            Some(index) if index < self.opened_items.len() => index,
            _ => {
                // 索引无效，重置选择
                if self.opened_items.is_empty() {
                    self.selected_item_index = None;
                    ui.centered_and_justified(|ui| {
                        ui.label("双击左侧文件树中的项目以打开");
                    });
                    return;
                } else {
                    // 选择第一个有效的项目
                    let new_index = 0;
                    self.selected_item_index = Some(new_index);
                    new_index
                }
            }
        };

        // 渲染顶部标签页
        self.render_item_tabs(ui);

        // 再次检查，确保在渲染标签页后opened_items仍然不为空
        if self.opened_items.is_empty() {
            self.selected_item_index = None;
            ui.centered_and_justified(|ui| {
                ui.label("双击左侧文件树中的项目以打开");
            });
            return;
        }

        // 确保selected_index仍然有效
        if selected_index >= self.opened_items.len() {
            self.selected_item_index = if self.opened_items.is_empty() {
                None
            } else {
                Some(0)
            };

            // 如果仍然没有有效项目，显示提示信息
            if self.selected_item_index.is_none() {
                ui.centered_and_justified(|ui| {
                    ui.label("双击左侧文件树中的项目以打开");
                });
                return;
            }
        }

        // 获取当前有效的selected_index
        let selected_index = self.selected_item_index.unwrap();

        // 克隆当前项目以避免借用冲突
        let current_item = self.opened_items[selected_index].item.clone();

        // 获取当前项目的所有Sheet项
        let sheet_items = Self::get_sheet_items_static(&current_item);

        // 如果没有Sheet子项，但项目本身有内容，则直接显示项目内容
        let has_sheet_children = !sheet_items.is_empty();

        // 创建一个垂直布局，为底部标签页预留空间
        ui.vertical(|ui| {
            // 上部区域：表格内容（占据大部分空间）
            let available_height = ui.available_height();
            // 为底部标签页预留大约30像素空间
            let table_height = if has_sheet_children {
                available_height - 30.0
            } else {
                available_height
            };

            // 渲染表格内容区域
            egui::ScrollArea::both()
                .max_height(table_height)
                .show(ui, |ui| {
                    if has_sheet_children {
                        let selected_sheet_index = if self.opened_items[selected_index]
                            .selected_sheet_index
                            < sheet_items.len()
                        {
                            self.opened_items[selected_index].selected_sheet_index
                        } else {
                            0
                        };

                        // 获取当前选中的Sheet
                        let selected_sheet = sheet_items[selected_sheet_index];

                        // 显示Sheet内容
                        if let Some(content) = &selected_sheet.gable_content {
                            self.render_table(ui, content);
                        } else {
                            ui.label("没有可显示的Sheet内容");
                        }
                    } else {
                        // 显示项目内容（当项目没有Sheet子项时）
                        if let Some(content) = &self.opened_items[selected_index].item.gable_content
                        {
                            self.render_table(ui, content);
                        } else {
                            ui.label("没有可显示的内容");
                        }
                    }
                });

            // 如果有Sheet子项，渲染底部Sheet标签页
            if has_sheet_children {
                ui.separator();
                self.render_sheet_tabs(ui, &sheet_items);
            }
        });
    }
}
