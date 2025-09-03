use crate::common::{res, setting};
use crate::gui::datas::action_command::{ActionCommand, ECommandType};
use crate::gui::form::gable_form::GableForm;
use crate::gui::gable_popup::GablePopup;
use crate::gui::{
    datas::eitem_type::EItemType, datas::gables, file_watcher::FileWatcher,
    gable_explorer::GableExplorer, gable_log::GableLog, gable_menu::GableMenu,
    gable_navigation::GableNavigation,
};
use eframe::egui::{
    Context, FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle, ViewportCommand,
};
use eframe::emath::History;
use eframe::{App, CreationContext, Frame};
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

lazy_static! {
    /// 操作指令
    pub static ref COMMANDS:Arc<Mutex<VecDeque<ActionCommand>>>= Arc::new(Mutex::new(VecDeque::new()));
}

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
    /// 弹窗组件
    gable_popup: GablePopup,
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
            gable_popup: GablePopup::new(),
            file_watcher: None,
        };
        app.gable_menu.set_theme(&cc.egui_ctx, "Dark");
        setting::init();
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
        fonts.font_data.insert(
            "fallback_font".to_owned(),
            Arc::new(FontData::from_static(res::FONT_FALLBACK)),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("fallback_font".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "chinese_font".to_owned());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push("fallback_font".to_owned());

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
                let temp_path: PathBuf = setting::get_temp_path();
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
            setting::get_title(),
            1e3 * self.mean_frame_time(),
            self.fps(),
        );
        ctx.send_viewport_cmd(ViewportCommand::Title(info));
    }

    /// 编辑指令
    pub fn editor_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand = ActionCommand::new(ECommandType::EDITOR, Some(full_path));
        commands.push_back(action);
    }
    pub fn open_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand = ActionCommand::new(ECommandType::OPEN, Some(full_path));
        commands.push_back(action);
    }
    /// 更新指令
    pub fn update_command(&mut self) {
        let mut commands = COMMANDS.lock().unwrap();
        while let Some(command) = commands.pop_front() {
            match command.com_type {
                ECommandType::EDITOR => {
                    if let Some(param) = command.param {
                        if let Some(tree_item) = gables::get_item_clone(&param) {
                            gables::command_edit_gable(&tree_item);
                        }
                    }
                }
                ECommandType::OPEN => {
                    if let Some(param) = command.param {
                        if let Some(tree_item) =
                            gables::find_tree_item_by_path(&param, EItemType::Excel)
                        {
                            self.gable_form.open(&tree_item);
                        }
                    }
                }
                _ => {
                    log::warn!("未知的命令: {:?}", command.com_type);
                }
            }
        }
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
        self.gable_popup.ongui(ctx);
        self.update_command();
    }
}
