use eframe::egui;

pub(crate) struct GableApp {
    title: String,
}

impl GableApp {
    pub fn new(title: String) -> Self {
        Self { title }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl eframe::App for GableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("新建文件").clicked() {}
                    if ui.button("新建文件夹").clicked() {}
                    ui.separator();
                    if ui.button("打开工程目录").clicked() {}
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
                        if ui.button("Auto").clicked() {}
                        if ui.button("Light").clicked() {}
                        if ui.button("Dark").clicked() {}
                    });
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.title);
            ui.label("Hello World!");
        });
    }
}
