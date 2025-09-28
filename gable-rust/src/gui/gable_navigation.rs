use crate::common::utils;
use eframe::egui::{Align, Button, Color32, Context, Layout, RichText, SidePanel, Vec2};
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

    /**
     * 绘制 导航栏
     */
    pub fn ongui(&mut self, ctx: &Context) {
        SidePanel::left("my_left_panel")
            .default_width(40.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    ui.vertical(|ui| {
                        // 设置按钮样式 - 增大按钮和字体大小
                        let button_size: Vec2 = Vec2::new(40.0, 40.0);
                        let tab1_button: Button<'_> = Button::new(RichText::new("🏠").size(24.0))
                            .fill(if self.selected_navigation_index == 0 {
                                utils::get_selected_color(ctx)
                            } else {
                                Color32::TRANSPARENT
                            });

                        if ui.add_sized(button_size, tab1_button).clicked() {
                            self.selected_navigation_index = 0;
                            // Tab1 点击处理逻辑
                        }
                        let tab2_button: Button<'_> = Button::new(RichText::new("🔍").size(24.0))
                            .fill(if self.selected_navigation_index == 1 {
                                utils::get_selected_color(ctx)
                            } else {
                                Color32::TRANSPARENT
                            });

                        if ui.add_sized(button_size, tab2_button).clicked() {
                            self.selected_navigation_index = 1;
                            // Tab2 点击处理逻辑
                        }
                    });

                    // ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    //     let button_size: Vec2 = Vec2::new(40.0, 40.0);
                    //     let bottom_button: Button<'_> =
                    //         Button::new(RichText::new("⚙").size(24.0)) // 增大字体大小
                    //             .fill(Color32::TRANSPARENT);
                    //     if ui.add_sized(button_size, bottom_button).clicked() {
                    //         // 底部按钮点击处理逻辑
                    //     }
                    // });
                });
            });
    }
}
