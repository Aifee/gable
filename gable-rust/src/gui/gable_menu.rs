use crate::common::setting;
use crate::gui::datas::gables;
use eframe::egui;
pub(crate) struct GableMenu {}
impl GableMenu {
    pub fn new() -> Self {
        Self {}
    }
    /// 绘制菜单
    pub fn gui_menu(&mut self, ctx: &egui::Context) {
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
}
