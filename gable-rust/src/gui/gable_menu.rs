use crate::gui::datas::gables;
use crate::{common::setting, gui::gable_popup};
use eframe::egui::{Color32, Context, MenuBar, TopBottomPanel, ViewportCommand, Visuals};
pub(crate) struct GableMenu {}
impl GableMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_theme(&mut self, ctx: &Context, theme: &str) {
        match theme {
            "Light" => {
                let mut visuals_light: Visuals = Visuals::light();
                // 面板背景颜色
                visuals_light.panel_fill = Color32::from_rgb(243, 243, 243);
                // 设置文本颜色（全局文本色）
                visuals_light.override_text_color = Some(Color32::BLACK);
                // 修改浅色背景（表单的交替色）
                visuals_light.faint_bg_color = Color32::from_rgb(230, 230, 230);
                // 编辑框背景色
                visuals_light.text_edit_bg_color = Some(Color32::from_rgb(220, 220, 220));
                // 右键菜单填充色
                visuals_light.window_fill = Color32::from_rgb(230, 230, 230);
                // 非常暗或亮的颜色（对应主题）。用作文本编辑、滚动条和其他需要与其他交互内容区别开来的背景。
                visuals_light.extreme_bg_color = Color32::from_rgb(200, 200, 200);
                // 表单选中内容色
                visuals_light.selection.bg_fill = Color32::from_rgb(173, 216, 230);
                ctx.set_visuals(visuals_light);
            }
            "Dark" => {
                let mut visuals_dark: Visuals = Visuals::dark();
                // 面板背景颜色
                visuals_dark.panel_fill = Color32::from_rgb(40, 44, 51);
                // 设置文本颜色（全局文本色）
                visuals_dark.override_text_color = Some(Color32::WHITE);
                // 表单的交替色
                visuals_dark.faint_bg_color = Color32::from_rgb(45, 49, 59);
                // 编辑框背景色
                visuals_dark.text_edit_bg_color = Some(Color32::from_rgb(29, 31, 35));
                // 右键菜单填充色
                visuals_dark.window_fill = Color32::from_rgb(33, 37, 43);
                // 对应主题。用作文本编辑、滚动条和其他需要与其他交互内容区别开来的背景。
                visuals_dark.extreme_bg_color = Color32::from_rgb(29, 31, 35);
                // 表单选中内容色
                visuals_dark.selection.bg_fill = Color32::from_rgb(60, 100, 150);
                ctx.set_visuals(visuals_dark);
            }
            _ => {
                ctx.set_visuals(Visuals::dark());
            }
        }
    }

    /// 绘制菜单
    pub fn ongui(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
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
                            let path_str: String = path.to_string_lossy().to_string();
                            setting::set_workspace(path_str);
                            gables::refresh_gables();
                        }
                    }
                    ui.separator();
                    if ui.button("设置").clicked() {}
                    if ui.button("退出").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.menu_button("编译", |ui| {
                    if ui.button("编译设置").clicked() {
                        gable_popup::open_window(gable_popup::WINDOW_BUILD_SETTING);
                    }
                    if ui.button("快速编译").clicked() {}
                });
                ui.menu_button("选择", |ui| if ui.button("导入Excel").clicked() {});
                ui.menu_button("帮助", |ui| {
                    if ui.button("关于").clicked() {
                        gable_popup::open_window(gable_popup::WINDOW_ABOUT);
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
    }
}
