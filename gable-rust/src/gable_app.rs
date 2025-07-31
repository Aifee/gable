use eframe::egui;

pub(crate) struct GableApp {
    title: String,
}

impl Default for GableApp {
    fn default() -> Self {
        Self {
            title: "Gable App".to_owned(),
        }
    }
}

impl GableApp {
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl eframe::App for GableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.title);
            ui.label("Hello World!");
        });
    }
}
