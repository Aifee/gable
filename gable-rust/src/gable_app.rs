use eframe::egui;
use std::sync::Arc;

pub(crate) struct GableApp {
    title: String,
}

impl GableApp {
    pub fn new(cc: &eframe::CreationContext<'_>, title: String) -> Self {
        // 加载自定义字体
        let mut fonts = egui::FontDefinitions::default();

        // 从文件加载字体（示例使用系统字体路径）
        fonts.font_data.insert(
            "chinese_font".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/fonts/NotoSansSC-VariableFont_wght.ttf"
            ))),
        );

        // 设置字体族，优先使用中文字体
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());

        // 应用字体定义
        cc.egui_ctx.set_fonts(fonts);
        Self { title: title }
    }
    pub fn title(&self) -> String {
        format!("{} Project Path", self.title)
    }

    // 添加设置标题的方法
    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }
}

impl eframe::App for GableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 动态更新窗口标题
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.title().to_string()));

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
