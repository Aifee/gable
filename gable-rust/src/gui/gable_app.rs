use crate::common::convert::convert;
use crate::common::generate::generate;
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
        style.spacing.indent = 22.0;
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
                    log::error!("Unable to monitor temporary directory: {}", e);
                } else {
                    file_watcher.start_watching();
                    log::info!("The file monitor has been activated.");
                    // 保存文件监控器实例，确保它不会被提前释放
                    self.file_watcher = Some(file_watcher);
                }
            }
            Err(e) => {
                log::error!("Unable to create file monitor: {}", e);
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
        let action: ActionCommand = ActionCommand::new(ECommandType::Edit, Some(full_path), None);
        commands.push_back(action);
    }
    /// 打开指令
    pub fn open_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand = ActionCommand::new(ECommandType::Open, Some(full_path), None);
        commands.push_back(action);
    }
    /// 导出配置（根据节点导出）
    pub fn convert_item_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::ConvertItem, Some(full_path), None);
        commands.push_back(action);
    }
    /// 生成代码（根据节点导出）
    pub fn generate_item_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::GenerateItem, Some(full_path), None);
        commands.push_back(action);
    }
    // 导出配置（根据设置导出）
    pub fn convert_target_command(display_name: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::ConvertTarget, Some(display_name), None);
        commands.push_back(action);
    }
    // 生成配置（根据设置生成）
    pub fn generate_target_command(display_name: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::GenerateTarget, Some(display_name), None);
        commands.push_back(action);
    }

    pub fn create_folder_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::CreateFolder, Some(full_path), None);
        commands.push_back(action);
    }

    pub fn create_excel_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::CreateExcel, Some(full_path), None);
        commands.push_back(action);
    }

    pub fn create_sheet_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::CreateSheet, Some(full_path), None);
        commands.push_back(action);
    }

    pub fn editname_command(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::Editname, Some(full_path), None);
        commands.push_back(action);
    }

    pub fn rename_command(full_path: String, new_name: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand =
            ActionCommand::new(ECommandType::Rename, Some(full_path), Some(new_name));
        commands.push_back(action);
    }

    pub fn delete_comand(full_path: String) {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        let action: ActionCommand = ActionCommand::new(ECommandType::Delete, Some(full_path), None);
        commands.push_back(action);
    }

    pub fn refresh_command() {
        let mut commands: MutexGuard<'_, VecDeque<ActionCommand>> = COMMANDS.lock().unwrap();
        commands.push_back(ActionCommand::new(ECommandType::Refresh, None, None));
    }
    /**
     * 更新指令
     */
    pub fn update_command(&mut self) {
        let mut commands = COMMANDS.lock().unwrap();
        while let Some(command) = commands.pop_front() {
            match command.com_type {
                ECommandType::Edit => {
                    if let Some(param) = command.param1 {
                        if let Some(tree_item) = gables::get_item_clone(&param) {
                            gables::command_edit_gable(&tree_item);
                        }
                    }
                }
                ECommandType::Open => {
                    if let Some(param) = command.param1 {
                        if let Some(tree_item) = gables::find_item_clone(&param, EItemType::Excel) {
                            self.gable_form.open(&tree_item);
                        }
                    }
                }
                ECommandType::ConvertItem => {
                    if let Some(param) = command.param1 {
                        if let Some(tree_item) = gables::get_item_clone(&param) {
                            convert::from_items(&tree_item);
                        }
                    }
                }
                ECommandType::GenerateItem => {
                    if let Some(param) = command.param1 {
                        if let Some(tree_item) = gables::get_item_clone(&param) {
                            generate::from_items(&tree_item);
                        }
                    }
                }
                ECommandType::ConvertTarget => {
                    if let Some(param) = command.param1 {
                        if let Some(setting) = setting::get_build_setting_with_name(&param) {
                            convert::from_target(&setting);
                        }
                    }
                }
                ECommandType::GenerateTarget => {
                    if let Some(param) = command.param1 {
                        if let Some(setting) = setting::get_build_setting_with_name(&param) {
                            generate::from_target(&setting);
                        }
                    }
                }
                ECommandType::CreateFolder => {
                    if let Some(full_path) = command.param1 {
                        self.gable_explorer.create_folder(full_path);
                    }
                }
                ECommandType::CreateExcel => {
                    if let Some(full_path) = command.param1 {
                        self.gable_explorer.create_excel(full_path);
                    }
                }
                ECommandType::CreateSheet => {
                    if let Some(full_path) = command.param1 {
                        self.gable_explorer.create_sheet(full_path);
                    }
                }
                ECommandType::Editname => {
                    if let Some(full_path) = command.param1 {
                        self.gable_explorer.edit_name(full_path);
                    }
                }
                ECommandType::Rename => {
                    if let Some(full_path) = command.param1 {
                        if let Some(new_name) = command.param2 {
                            self.gable_explorer.rename(full_path, new_name);
                        }
                    }
                }
                ECommandType::Delete => {
                    if let Some(full_path) = command.param1 {
                        if gables::remove_item_file(&full_path) {
                            gables::remove_tree_item(&full_path);
                        }
                    }
                }
                ECommandType::Refresh => {
                    gables::refresh_gables();
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
