use crate::common::{res, utils};
use crate::gui::{
    datas::eitem_type::EItemType, datas::gables, file_watcher::FileWatcher,
    gable_explorer::GableExplorer, gable_form::GableForm, gable_log::GableLog,
    gable_menu::GableMenu, gable_navigation::GableNavigation,
};
use eframe::egui::{
    Context, FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle, ViewportCommand,
};
use eframe::emath::History;
use eframe::{App, CreationContext, Frame};
use std::sync::Arc;

pub(crate) struct GableApp {
    frame_times: History<f32>,
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
    /// 文件监控器
    #[allow(dead_code)]
    file_watcher: Option<FileWatcher>,
}

impl GableApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::init_fonts(cc);
        Self::init_style(cc);
        let max_age: f32 = 1.0;
        let max_len = (max_age * 300.0).round() as usize;
        let mut app: GableApp = Self {
            frame_times: History::new(0..max_len, max_age),
            gable_menu: GableMenu::new(),
            gable_navigation: GableNavigation::new(),
            gable_explorer: GableExplorer::new(),
            gable_form: GableForm::new(),
            gable_log: GableLog::new(),
            file_watcher: None,
        };
        app.gable_menu.set_theme(&cc.egui_ctx, "Dark");
        gables::refresh_gables();
        app.init_watcher();
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

    // 初始化文件监控器
    fn init_watcher(&mut self) {
        // 初始化文件监控器
        match FileWatcher::new() {
            Ok(mut file_watcher) => {
                let temp_path: String = utils::get_temp_path();
                if let Err(e) = file_watcher.watch_temp_directory(temp_path) {
                    log::error!("无法监控临时目录: {}", e);
                } else {
                    file_watcher.start_watching();
                    log::info!("文件监控器已启动");
                    // 保存文件监控器实例，确保它不会被提前释放
                    self.file_watcher = Some(file_watcher);
                }
            }
            Err(e) => {
                log::error!("无法创建文件监控器: {}", e);
            }
        }
    }

    fn on_new_frame(&mut self, now: f64, previous_frame_time: Option<f32>) {
        let previous_frame_time = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = previous_frame_time; // rewrite history now that we know
        }
        self.frame_times.add(now, previous_frame_time); // projected
    }

    // 获取帧率消耗的时长
    fn mean_frame_time(&self) -> f32 {
        self.frame_times.average().unwrap_or_default()
    }
    // app 帧率
    fn fps(&self) -> f32 {
        1.0 / self.frame_times.mean_time_interval().unwrap_or_default()
    }
    /// 绘制窗口标题
    fn gui_title(&mut self, ctx: &Context) {
        let info: String = format!(
            "{}                         CPU usage: {:.2} ms/frame. FPS: {:.1}",
            utils::get_title(),
            1e3 * self.mean_frame_time(),
            self.fps(),
        );
        ctx.send_viewport_cmd(ViewportCommand::Title(info));
    }
}

impl App for GableApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.on_new_frame(ctx.input(|i| i.time), _frame.info().cpu_usage);
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
