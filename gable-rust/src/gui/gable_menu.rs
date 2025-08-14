use crate::common::setting;
use crate::gui::datas::gables;
use eframe::egui;
pub(crate) struct GableMenu {
    show_about: bool, // 添加此字段用于控制关于窗口显示
}
impl GableMenu {
    pub fn new() -> Self {
        Self {
            show_about: false, // 初始化为false
        }
    }

    pub fn set_theme(&mut self, ctx: &egui::Context, theme: &str) {
        match theme {
            "Light" => {
                let mut visuals_light = egui::Visuals::light();
                // 面板背景颜色
                visuals_light.panel_fill = egui::Color32::from_rgb(243, 243, 243);
                // 设置文本颜色（全局文本色）
                visuals_light.override_text_color = Some(egui::Color32::BLACK);
                // 修改浅色背景（表单的交替色）
                visuals_light.faint_bg_color = egui::Color32::from_rgb(230, 230, 230);
                // 编辑框背景色
                visuals_light.text_edit_bg_color = Some(egui::Color32::from_rgb(220, 220, 220));
                // 右键菜单填充色
                visuals_light.window_fill = egui::Color32::from_rgb(230, 230, 230);
                // 非常暗或亮的颜色（对应主题）。用作文本编辑、滚动条和其他需要与其他交互内容区别开来的背景。
                visuals_light.extreme_bg_color = egui::Color32::from_rgb(200, 200, 200);
                ctx.set_visuals(visuals_light);
            }
            "Dark" => {
                let mut visuals_dark = egui::Visuals::dark();
                // 面板背景颜色
                visuals_dark.panel_fill = egui::Color32::from_rgb(40, 44, 51);
                // 设置文本颜色（全局文本色）
                visuals_dark.override_text_color = Some(egui::Color32::WHITE);
                // 表单的交替色
                visuals_dark.faint_bg_color = egui::Color32::from_rgb(45, 49, 59);
                // 编辑框背景色
                visuals_dark.text_edit_bg_color = Some(egui::Color32::from_rgb(29, 31, 35));
                // 右键菜单填充色
                visuals_dark.window_fill = egui::Color32::from_rgb(33, 37, 43);
                // 对应主题。用作文本编辑、滚动条和其他需要与其他交互内容区别开来的背景。
                visuals_dark.extreme_bg_color = egui::Color32::from_rgb(29, 31, 35);
                ctx.set_visuals(visuals_dark);
            }
            _ => {
                ctx.set_visuals(egui::Visuals::dark());
            }
        }
    }

    /// 绘制菜单
    pub fn ongui(&mut self, ctx: &egui::Context) {
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
                    if ui.button("关于").clicked() {
                        self.show_about = true; // 点击时设置为true
                    }
                    ui.menu_button("主题", |ui| {
                        if ui.button("Light").clicked() {
                            self.set_theme(ctx, "Light");
                        }
                        if ui.button("Dark").clicked() {
                            self.set_theme(ctx, "Dark");
                        }
                    });
                });
            });
        });
        if self.show_about {
            egui::Window::new("关于")
                .open(&mut self.show_about) // 由 egui 控制 show_about 状态
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO) // 居中显示
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Gable");
                        ui.label("版本 1.0.0");
                        ui.separator();
                        ui.label("一个用于处理Excel文件的工具");
                        ui.label("© liuaf 2025");
                        ui.label("email:329737941@qq.com");
                    });
                });
        }
    }
}
