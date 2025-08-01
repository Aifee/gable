use eframe::egui;
pub(crate) struct GableNavigation {
    /// 当前选中的导航索引
    selected_navigation_index: u8,
}

impl GableNavigation {
    pub fn new() -> Self {
        Self {
            selected_navigation_index: 0,
        }
    }

    /// 绘制 导航栏
    pub fn gui_navigation_bar(&mut self, ctx: &egui::Context) {
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
}
