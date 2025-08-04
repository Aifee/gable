use eframe::egui;
use std::sync::Arc;

use crate::common::res;
use crate::common::setting;
use crate::gui::datas::gables;
use crate::gui::gable_explorer::GableExplorer;
use crate::gui::gable_form::GableForm;
use crate::gui::gable_menu::GableMenu;
use crate::gui::gable_navigation::GableNavigation;

pub(crate) struct GableApp {
    /// 菜单组件
    gable_menu: GableMenu,
    /// 导航组件
    gable_navigation: GableNavigation,
    /// 文件浏览器组件
    gable_explorer: GableExplorer,
    /// 表格展示组件
    gable_form: GableForm,
}

impl GableApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 加载自定义字体
        let mut fonts = egui::FontDefinitions::default();

        // 从文件加载字体（示例使用系统字体路径）
        fonts.font_data.insert(
            "chinese_font".to_owned(),
            Arc::new(egui::FontData::from_static(res::FONT_ASSETS)),
        );

        // 设置字体族，优先使用中文字体
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        // 设置全局样式，调整字体大小
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.indent = 30.0;
        style.text_styles = [
            (
                egui::TextStyle::Small,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(16.0, egui::FontFamily::Monospace),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);
        // 应用字体定义
        let app = Self {
            gable_menu: GableMenu::new(),
            gable_navigation: GableNavigation::new(),
            gable_explorer: GableExplorer::new(),
            gable_form: GableForm::new(),
        };
        gables::refresh_gables();
        app
    }

    fn get_title(&self) -> String {
        let workspace = setting::WORKSPACE.lock().unwrap();
        format!(
            "Gable - {}",
            workspace.as_ref().unwrap_or(&"Unknown".to_string())
        )
    }

    /// 绘制窗口标题
    fn gui_title(&mut self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.get_title().to_string()));
    }
}

impl eframe::App for GableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.gui_title(ctx);
        self.gable_menu.gui_menu(ctx);
        self.gable_navigation.gui_navigation_bar(ctx);
        self.gable_explorer.gui_tree_view(ctx);
        egui::TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            .min_height(100.0)
            .max_height(200.0)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show_animated(ctx, true, |ui| {
                ui.heading("LeftPanelLeftPanelLeftPanelLeftPanelLeftPanel");
                if ui.button("按钮1").clicked() {}
                if ui.button("按钮2").clicked() {}
            });

        if let Some(double_clicked_path) = &self.gable_explorer.double_clicked_item {
            // println!("double_clicked_path: {}", &double_clicked_path);
            // 从TREE_ITEMS中查找对应的TreeItem
            if let Some(tree_item) = gables::find_tree_item_by_path(double_clicked_path) {
                // 直接打开项目
                self.gable_form.open(tree_item);
            }
            // 重置双击项，避免重复处理
            self.gable_explorer.double_clicked_item = None;
        }
        // 中央主内容面板
        egui::CentralPanel::default().show(ctx, |ui| {
            self.gable_form.gui_form(ui);
        });
    }
}
