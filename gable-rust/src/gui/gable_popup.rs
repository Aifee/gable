use crate::gui::{gable_about::GableAbout, gable_build_setting::GableBuildSetting};
use eframe::egui::Context;
use lazy_static::lazy_static;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, Clone)]
pub struct WindowParams {
    pub id: u16,
    pub action: bool,
}

lazy_static! {
    static ref POPUPS: Arc<Mutex<VecDeque<WindowParams>>> = Arc::new(Mutex::new(VecDeque::new()));
}

pub const WINDOW_ABOUT: u16 = 1001;
pub const WINDOW_BUILD_SETTING: u16 = 1002;
pub fn open_window(id: u16) {
    let mut popups: MutexGuard<'_, VecDeque<WindowParams>> = POPUPS.lock().unwrap();
    popups.push_back(WindowParams { id, action: true });
}

pub struct GablePopup {
    pub gable_about: GableAbout,
    pub gable_build_setting: GableBuildSetting,
}
impl GablePopup {
    pub fn new() -> Self {
        Self {
            gable_about: GableAbout::new(),
            gable_build_setting: GableBuildSetting::new(),
        }
    }

    fn update_queue(&mut self) {
        let mut popups: MutexGuard<'_, VecDeque<WindowParams>> = POPUPS.lock().unwrap();
        while let Some(popup) = popups.pop_front() {
            match popup.id {
                WINDOW_ABOUT => {
                    self.gable_about.set_visible(popup.action);
                }
                WINDOW_BUILD_SETTING => {
                    self.gable_build_setting.set_visible(popup.action);
                }
                _ => {}
            }
        }
    }

    pub fn ongui(&mut self, ctx: &Context) {
        self.update_queue();
        self.gable_about.ongui(ctx);
        self.gable_build_setting.ongui(ctx);
    }
}
