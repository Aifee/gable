use eframe::egui::{Align2, Context, Vec2, Window};

pub struct GableAbout {
    visible: bool,
}
impl GableAbout {
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
        Window::new("关于")
            .open(&mut self.visible)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
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
