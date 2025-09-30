use crate::common::convert::convert;
use crate::common::generate::generate;
use crate::common::locales;
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
                ui.menu_button(locales::t("file_menu").as_str(), |ui| {
                    if ui.button(locales::t("new_file").as_str()).clicked() {}
                    if ui.button(locales::t("new_folder").as_str()).clicked() {}
                    ui.separator();
                    if ui
                        .button(locales::t("open_project_directory").as_str())
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title(locales::t("select_project_directory").as_str())
                            .pick_folder()
                        {
                            let path_str: String = path.to_string_lossy().to_string();
                            if let Err(e) = setting::set_workspace(path_str) {
                                log::error!("Failed to set workspace: {}", e.to_string());
                            } else {
                                gables::refresh_gables();
                            }
                        }
                    }
                    ui.separator();
                    if ui.button(locales::t("settings").as_str()).clicked() {}
                    if ui.button(locales::t("exit").as_str()).clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.menu_button(locales::t("build").as_str(), |ui| {
                    if ui.button(locales::t("build_settings").as_str()).clicked() {
                        gable_popup::open_window(gable_popup::WINDOW_BUILD_SETTING);
                    }
                    if ui.button(locales::t("quick_build").as_str()).clicked() {
                        convert::from_all();
                        generate::from_all();
                    }
                });
                ui.menu_button(locales::t("select").as_str(), |ui| {
                    if ui.button(locales::t("import_excel").as_str()).clicked() {}
                });
                ui.menu_button(locales::t("help").as_str(), |ui| {
                    if ui.button(locales::t("about").as_str()).clicked() {
                        gable_popup::open_window(gable_popup::WINDOW_ABOUT);
                    }
                    ui.menu_button(locales::t("Language").as_str(), |ui| {
                        for v in locales::get_available_languages().iter() {
                            if ui.button(locales::t(v.as_str())).clicked() {
                                locales::set_language(&v);
                            }
                        }
                    });
                    ui.menu_button(locales::t("theme").as_str(), |ui| {
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
