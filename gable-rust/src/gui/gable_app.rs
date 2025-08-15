use crate::common::res;
use crate::common::setting;
use crate::gui::datas::eitem_type::EItemType;
use crate::gui::datas::gables;
use crate::gui::gable_explorer::GableExplorer;
use crate::gui::gable_form::GableForm;
use crate::gui::gable_log::GableLog;
use crate::gui::gable_menu::GableMenu;
use crate::gui::gable_navigation::GableNavigation;
use eframe::egui::{
    Context, FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle, ViewportCommand,
};
use eframe::{App, CreationContext, Frame};
use std::sync::{Arc, MutexGuard};

pub(crate) struct GableApp {
    /// 菜单组件
    gable_menu: GableMenu,
    /// 导航组件
    gable_navigation: GableNavigation,
    /// 文件浏览器组件
    gable_explorer: GableExplorer,
    /// 表格展示组件
    gable_form: GableForm,
    /// 日志组件
    gable_log: GableLog,
}

impl GableApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::init_fonts(cc);
        Self::init_style(cc);
        // 应用字体定义
        let mut app: GableApp = Self {
            gable_menu: GableMenu::new(),
            gable_navigation: GableNavigation::new(),
            gable_explorer: GableExplorer::new(),
            gable_form: GableForm::new(),
            gable_log: GableLog::new(),
        };
        app.gable_menu.set_theme(&cc.egui_ctx, "Dark");
        gables::refresh_gables();
        app
    }

    /// 初始化字体
    fn init_fonts(cc: &CreationContext<'_>) {
        // 加载自定义字体
        let mut fonts: FontDefinitions = FontDefinitions::default();

        // 从文件加载字体（示例使用系统字体路径）
        fonts.font_data.insert(
            "chinese_font".to_owned(),
            Arc::new(FontData::from_static(res::FONT_ASSETS)),
        );
        // 设置字体族，优先使用中文字体
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "chinese_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);
    }

    /// 初始化样式
    fn init_style(cc: &CreationContext<'_>) {
        // 设置全局样式，调整字体大小
        let mut style: Style = (*cc.egui_ctx.style()).clone();
        style.spacing.indent = 30.0;
        style.text_styles = [
            (
                TextStyle::Small,
                FontId::new(14.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
            (
                TextStyle::Button,
                FontId::new(16.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(20.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(16.0, FontFamily::Monospace),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);
    }

    fn get_title(&self) -> String {
        let workspace: MutexGuard<'_, Option<String>> = setting::WORKSPACE.lock().unwrap();
        format!(
            "Gable - {}",
            workspace.as_ref().unwrap_or(&"Unknown".to_string())
        )
    }

    /// 绘制窗口标题
    fn gui_title(&mut self, ctx: &Context) {
        ctx.send_viewport_cmd(ViewportCommand::Title(self.get_title().to_string()));
    }
}

impl App for GableApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.gui_title(ctx);
        self.gable_menu.ongui(ctx);
        self.gable_navigation.ongui(ctx);
        self.gable_explorer.ongui(ctx);
        self.gable_log.ongui(ctx);
        self.gable_form.ongui(ctx);

        if let Some(double_clicked_path) = &self.gable_explorer.double_clicked_item {
            // 从TREE_ITEMS中查找对应的TreeItem
            if let Some(tree_item) =
                gables::find_tree_item_by_path(double_clicked_path, EItemType::Excel)
            {
                self.gable_form.open(tree_item);
            }
            // 重置双击项，避免重复处理
            self.gable_explorer.double_clicked_item = None;
        }
    }
}
