use eframe::egui;
use std::sync::Arc;

use crate::common::global;
use crate::common::setting;
use crate::gui::datas::gables;
use crate::gui::datas::gables::{ItemType, TreeItem};

pub(crate) struct GableApp {
    /// 当前选中的导航索引
    selected_navigation_index: u8,
    /// 当前选中的treeItem，以fullpath为key
    selected_tree_item: Option<String>,
}

impl GableApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 加载自定义字体
        let mut fonts = egui::FontDefinitions::default();

        // 从文件加载字体（示例使用系统字体路径）
        fonts.font_data.insert(
            "chinese_font".to_owned(),
            Arc::new(egui::FontData::from_static(global::FONT_ASSETS)),
        );

        // 设置字体族，优先使用中文字体
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        // 设置全局样式，调整字体大小
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.indent = 30.0;
        style.text_styles = [
            (
                egui::TextStyle::Small,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(16.0, egui::FontFamily::Monospace),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);
        // 应用字体定义
        let app = Self {
            selected_navigation_index: 0,
            selected_tree_item: None,
        };
        gables::refresh_gables();
        app
    }

    fn get_title(&self) -> String {
        let workspace = setting::WORKSPACE.lock().unwrap();
        format!(
            "Gable - {}",
            workspace.as_ref().unwrap_or(&"Unknown".to_string())
        )
    }

    /// 绘制窗口标题
    fn gui_title(&mut self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.get_title().to_string()));
    }
    /// 绘制菜单
    fn gui_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("新建文件").clicked() {}
                    if ui.button("新建文件夹").clicked() {}
                    ui.separator();
                    if ui.button("打开工程目录").clicked() {
                        // 打开文件选择对话框
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("选择工程目录")
                            .pick_folder()
                        {
                            let path_str = path.to_string_lossy().to_string();
                            setting::set_workspace(path_str);
                            gables::refresh_gables();
                        }
                    }
                    ui.separator();
                    if ui.button("设置").clicked() {}
                    if ui.button("退出").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("编译", |ui| {
                    if ui.button("编译设置").clicked() {}
                    if ui.button("快速编译").clicked() {}
                });
                ui.menu_button("选择", |ui| if ui.button("导入Excel").clicked() {});
                ui.menu_button("帮助", |ui| {
                    if ui.button("关于").clicked() {}
                    ui.menu_button("主题", |ui| {
                        if ui.button("Light").clicked() {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                        if ui.button("Dark").clicked() {
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                    });
                });
            });
        });
    }
    /// 绘制 导航栏
    fn gui_navigation_bar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("my_left_panel")
            .default_width(40.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        // 顶部区域 - 两个互斥的 Tab 按钮
                        ui.vertical(|ui| {
                            // 设置按钮样式 - 增大按钮和字体大小
                            let button_size = egui::Vec2::new(40.0, 40.0);

                            // Tab1 - 房子图标
                            let tab1_button =
                                egui::Button::new(egui::RichText::new("🏠").size(24.0)).fill(
                                    if self.selected_navigation_index == 0 {
                                        // 选中状态背景色
                                        egui::Color32::from_rgb(0, 120, 200)
                                    } else {
                                        // 未选中状态背景色
                                        egui::Color32::TRANSPARENT
                                    },
                                );

                            if ui.add_sized(button_size, tab1_button).clicked() {
                                self.selected_navigation_index = 0;
                                // Tab1 点击处理逻辑
                            }

                            // Tab2 - 搜索图标
                            let tab2_button =
                                egui::Button::new(egui::RichText::new("🔍").size(24.0)).fill(
                                    if self.selected_navigation_index == 1 {
                                        // 选中状态背景色
                                        egui::Color32::from_rgb(0, 120, 200)
                                    } else {
                                        // 未选中状态背景色
                                        egui::Color32::TRANSPARENT
                                    },
                                );

                            if ui.add_sized(button_size, tab2_button).clicked() {
                                self.selected_navigation_index = 1;
                                // Tab2 点击处理逻辑
                            }
                        });

                        // 底部区域 - 一个按钮 (改为设置图标)
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            let button_size = egui::Vec2::new(40.0, 40.0);
                            let bottom_button = egui::Button::new(
                                egui::RichText::new("⚙").size(24.0),
                            ) // 增大字体大小
                            .fill(egui::Color32::TRANSPARENT);
                            if ui.add_sized(button_size, bottom_button).clicked() {
                                // 底部按钮点击处理逻辑
                            }
                        });
                    },
                );
            });
    }

    /// 绘制 treeview
    fn gui_tree_view(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("my_gables_panel")
            .min_width(150.0) // 设置最小宽度
            .max_width(800.0) // 设置最大宽度
            .resizable(true)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let tree_items = gables::TREE_ITEMS.lock().unwrap();
                        for item in tree_items.iter() {
                            Self::gui_tree_item(ui, item, &mut self.selected_tree_item);
                        }
                    });
            });
    }

    /// 带右键菜单的树形结构绘制
    fn gui_tree_item(ui: &mut egui::Ui, item: &TreeItem, selected_id: &mut Option<String>) {
        let icon = match item.item_type {
            ItemType::Folder => "📁",
            ItemType::Excel => "📄",
            ItemType::Sheet => "📊",
        };

        let header_text = format!("{} {}", icon, item.display_name);
        // 检查当前项是否被选中
        let is_selected = selected_id
            .as_ref()
            .map_or(false, |id| id == &item.fullpath);
        if !item.children.is_empty() {
            let header_response = egui::CollapsingHeader::new(&header_text)
                // .icon_style(egui::collapsing_header::IconStyle::OpenClose {
                //     opened: Some(egui::Vec2::new(12.0, 12.0)), // 调整打开状态的箭头大小
                //     closed: Some(egui::Vec2::new(12.0, 12.0)), // 调整关闭状态的箭头大小
                // })
                .default_open(item.is_open)
                .show(ui, |ui| {
                    // 显示子项
                    for child in &item.children {
                        Self::gui_tree_item(ui, child, selected_id);
                    }
                })
                .header_response;

            // 只有点击header文本区域时才选中
            if header_response.clicked() {
                *selected_id = Some(item.fullpath.clone());
                println!("Clicked: {}", item.fullpath.clone())
            }

            // 添加选中状态的视觉反馈
            if is_selected {
                ui.painter().rect_filled(
                    header_response.rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_rgb(0, 120, 200).linear_multiply(0.2),
                );
            }

            // 为header添加右键菜单
            header_response.context_menu(|ui| {
                Self::show_context_menu(ui, item);
            });
        } else {
            let response = ui.label(&header_text);
            // 处理点击事件
            if response.clicked() {
                *selected_id = Some(item.fullpath.clone());
            }
            // 添加选中状态的视觉反馈
            if is_selected {
                ui.painter().rect_filled(
                    response.rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_rgb(0, 120, 200).linear_multiply(0.2),
                );
            }
            // 为文件添加右键菜单
            response.context_menu(|ui| {
                Self::show_context_menu(ui, item);
            });
        }
    }

    /// 显示右键菜单
    fn show_context_menu(ui: &mut egui::Ui, item: &TreeItem) {
        match item.item_type {
            ItemType::Folder => {
                if ui.button("新建文件").clicked() {
                    // TODO: 实现新建文件逻辑
                    ui.close_menu();
                }
                if ui.button("新建文件夹").clicked() {
                    // TODO: 实现新建文件夹逻辑
                    ui.close_menu();
                }
            }
            ItemType::Excel => {
                if ui.button("新建文件").clicked() {
                    // TODO: 实现新建文件逻辑
                    ui.close_menu();
                }
                if ui.button("编辑").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("重命名").clicked() {
                    // TODO: 实现重命名逻辑
                    ui.close_menu();
                }
                if ui.button("删除").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close_menu();
                }
            }
            ItemType::Sheet => {
                if ui.button("编辑").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("删除").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close_menu();
                }
            }
        }
    }
}

impl eframe::App for GableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.gui_title(ctx);
        self.gui_menu(ctx);
        self.gui_navigation_bar(ctx);
        self.gui_tree_view(ctx);
        egui::TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show_animated(ctx, true, |ui| {
                ui.heading("LeftPanelLeftPanelLeftPanelLeftPanelLeftPanel");
                if ui.button("按钮1").clicked() {}
                if ui.button("按钮2").clicked() {}
            });
        // 中央主内容面板
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Main Content");
                ui.label("这是中央主要内容区域");
            });
        });
    }
}
