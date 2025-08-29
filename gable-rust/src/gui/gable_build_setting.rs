use eframe::egui::{
    Align, CentralPanel, ComboBox, Context, Frame, Layout, SidePanel, Vec2, Window,
};

pub struct GableBuildSetting {
    pub visible: bool,
}
impl GableBuildSetting {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn ongui(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }
        Window::new("构建设置")
            .open(&mut self.visible)
            .resizable(true)
            .collapsible(false)
            .fade_in(true)
            .fade_out(true)
            .min_width(450.0)
            .min_height(600.0)
            .show(ctx, |ui| {
                SidePanel::left("m_buildsetting_panel")
                    .min_width(50.0) // 设置最小宽度
                    .default_width(150.0)
                    .resizable(true)
                    .show_inside(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("配置项");
                            ui.separator();
                            if ui.button("基本设置").clicked() {
                                // 处理基本设置按钮点击
                            }
                            if ui.button("高级设置").clicked() {
                                // 处理高级设置按钮点击
                            }
                            if ui.button("输出配置").clicked() {
                                // 处理输出配置按钮点击
                            }
                        });
                    });
                // 右侧面板（主要内容区域）
                CentralPanel::default().show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("配置详情");
                        ui.separator();
                        ui.label("在这里显示选中配置项的详细内容");
                        ui.add_space(10.0);

                        // 示例内容
                        ui.horizontal(|ui| {
                            ui.label("配置名称:");
                            ui.text_edit_singleline(&mut String::from("示例配置"));
                        });

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.label("输出路径:");
                            ui.text_edit_singleline(&mut String::from("./output/"));
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut true, "启用优化");
                            ui.add_space(10.0);
                            ui.checkbox(&mut false, "生成调试信息");
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.label("构建类型:");
                            ComboBox::from_label("")
                                .selected_text("Release")
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut String::new(),
                                        String::from("Debug"),
                                        "Debug",
                                    );
                                    ui.selectable_value(
                                        &mut String::new(),
                                        String::from("Release"),
                                        "Release",
                                    );
                                    ui.selectable_value(
                                        &mut String::new(),
                                        String::from("Profile"),
                                        "Profile",
                                    );
                                });
                        });

                        ui.add_space(20.0);
                    });
                });
            });
    }
}
