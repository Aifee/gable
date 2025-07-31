use eframe::egui;
use std::sync::Arc;

use crate::common::global;
use crate::common::setting;

pub(crate) struct GableApp {
    selected_mode: u8,
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

        // 应用字体定义
        cc.egui_ctx.set_fonts(fonts);
        Self { selected_mode: 0 }
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
                            // 设置 WORKSPACE 值
                            setting::set_workspace(path_str);
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
}

impl eframe::App for GableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 调用独立的函数来更新窗口标题
        self.gui_title(ctx);
        self.gui_menu(ctx);
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
                            let tab1_button = egui::Button::new(
                                egui::RichText::new("🏠").size(24.0),
                            )
                            .fill(if self.selected_mode == 0 {
                                // 选中状态背景色
                                egui::Color32::from_rgb(0, 120, 255)
                            } else {
                                // 未选中状态背景色
                                egui::Color32::TRANSPARENT
                            });

                            if ui.add_sized(button_size, tab1_button).clicked() {
                                self.selected_mode = 0;
                                // Tab1 点击处理逻辑
                            }

                            // Tab2 - 搜索图标
                            let tab2_button = egui::Button::new(
                                egui::RichText::new("🔍").size(24.0),
                            )
                            .fill(if self.selected_mode == 1 {
                                // 选中状态背景色
                                egui::Color32::from_rgb(0, 120, 255)
                            } else {
                                // 未选中状态背景色
                                egui::Color32::TRANSPARENT
                            });

                            if ui.add_sized(button_size, tab2_button).clicked() {
                                self.selected_mode = 1;
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
        egui::SidePanel::left("my_gables_panel")
            .resizable(true)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                // ui.vertical(|ui| {
                //     ui.heading("Left Panel");
                //     if ui.button("按钮1").clicked() {}
                //     if ui.button("按钮2").clicked() {}
                // });
                ui.heading("LeftPanelLeftPanelLeftPanelLeftPanelLeftPanel");
                if ui.button("按钮1").clicked() {}
                if ui.button("按钮2").clicked() {}
            });
        egui::TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show_animated(ctx, true, |ui| {
                // ui.vertical(|ui| {
                //     ui.heading("Left Panel");
                //     if ui.button("按钮1").clicked() {}
                //     if ui.button("按钮2").clicked() {}
                // });
                ui.heading("LeftPanelLeftPanelLeftPanelLeftPanelLeftPanel");
                if ui.button("按钮1").clicked() {}
                if ui.button("按钮2").clicked() {}
            });
        // 中央主内容面板
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.vertical(|ui| {
            //     ui.heading("Main Content");
            //     ui.label("这是中央主要内容区域");
            // });
        });
    }
}
