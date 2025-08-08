use eframe::egui;
pub(crate) struct GableLog {}

impl GableLog {
    pub fn new() -> Self {
        Self {}
    }
    pub fn ongui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            .min_height(100.0)
            .max_height(200.0)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("LeftPanelLeftPanelLeftPanelLeftPanelLeftPanel");
                    if ui.button("按钮1").clicked() {
                        println!("点击了按钮1");
                    }
                    if ui.button("按钮2").clicked() {
                        println!("点击了按钮2");
                    }
                })
            });
    }
}
