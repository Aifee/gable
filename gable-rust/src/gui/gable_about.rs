use eframe::egui::{Align2, Context, Vec2, Window};

use crate::common::{constant, locales};

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
        Window::new(locales::t("about"))
            .open(&mut self.visible)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                let version_info: String =
                    format!("{} - {}", locales::t("version"), constant::GABLE_VERSION);
                let tool_desc: String = locales::t("tool_description");
                ui.vertical_centered(|ui| {
                    ui.heading("Gable");
                    ui.label(version_info);
                    ui.separator();
                    ui.label(tool_desc);
                    ui.label("© Aifei Liu 2025");
                    ui.label("Email:329737941@qq.com");
                });
            });
    }
}
