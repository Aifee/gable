use crate::{
    common::{constant, excel_util, localization_manager, setting, utils},
    gui::{
        datas::{eitem_type::EItemType, gables, tree_item::TreeItem},
        gable_app::GableApp,
    },
};
use eframe::egui::{
    CollapsingHeader, Color32, Context, CornerRadius, Frame, InputState, Key, Response, ScrollArea,
    SidePanel, Ui,
};
use rfd::FileDialog;
use std::{
    borrow::Cow,
    fs::{self, DirEntry},
    io::{Error, ErrorKind},
    mem,
    path::{Path, PathBuf},
};

pub struct GableExplorer {
    /// 当前选中的treeItem，以fullpath为key
    selected_tree_item: Option<String>,
    /// 当前正在重命名的项目路径
    renaming_item: Option<String>,
    /// 重命名时的临时名称
    renaming_text: String,
    // 自动获取焦点
    auto_focus: bool,
}

impl GableExplorer {
    pub fn new() -> Self {
        Self {
            selected_tree_item: None,
            renaming_item: None,
            renaming_text: String::new(),
            auto_focus: false,
        }
    }

    /**
     * 绘制 treeview
     */
    pub fn ongui(&mut self, ctx: &Context) {
        SidePanel::left("m_gables_panel")
            .min_width(150.0) // 设置最小宽度
            .max_width(800.0) // 设置最大宽度
            .resizable(true)
            .frame(Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let tree_items = gables::TREE_ITEMS.read().unwrap();
                        for item in tree_items.iter() {
                            self.gui_tree_item(ui, item);
                        }
                        // 添加空白区域右键菜单
                        ui.allocate_rect(
                            ui.available_rect_before_wrap(),
                            eframe::egui::Sense::click_and_drag(),
                        )
                        .context_menu(|ui| {
                            if ui.button(localization_manager::t("new_file")).clicked() {
                                GableApp::create_excel_command(
                                    setting::get_workspace().to_string_lossy().to_string(),
                                );
                                ui.close();
                            }
                            if ui.button(localization_manager::t("new_folder")).clicked() {
                                GableApp::create_folder_command(
                                    setting::get_workspace().to_string_lossy().to_string(),
                                );
                                ui.close();
                            }
                            if ui
                                .button(localization_manager::t("show_in_resource_manager"))
                                .clicked()
                            {
                                if let Some(path) = setting::get_workspace().to_str() {
                                    if let Err(e) = utils::open_in_explorer(path) {
                                        log::error!("Cannot open the Resource Manager: {}", e);
                                    }
                                }
                                ui.close();
                            }
                        });
                    });
            });
    }

    /**
     * 带右键菜单的树形结构绘制
     */
    fn gui_tree_item(&mut self, ui: &mut Ui, item: &TreeItem) {
        let icon: &'static str = match item.item_type {
            EItemType::Folder => "📁",
            EItemType::Excel => "📄",
            EItemType::Sheet => "📊",
        };

        // 检查是否是当前正在重命名的项目
        let is_renaming: bool = self
            .renaming_item
            .as_ref()
            .map_or(false, |id| id == &item.fullpath);

        if is_renaming {
            let response: Response = ui.text_edit_singleline(&mut self.renaming_text);
            if self.auto_focus {
                response.request_focus();
                self.auto_focus = false;
            }
            // 处理回车确认重命名
            if response.lost_focus() && ui.input(|i: &InputState| i.key_pressed(Key::Enter)) {
                if !self.renaming_text.is_empty() && *self.renaming_text != item.display_name {
                    let new_name: String = mem::take(&mut self.renaming_text);
                    self.renaming_item = None;
                    GableApp::rename_command(item.fullpath.clone(), new_name);
                } else {
                    self.renaming_item = None;
                    self.renaming_text.clear();
                }
            }
            // 失去焦点
            else if response.lost_focus()
                && !ui.input(|i: &InputState| i.key_pressed(Key::Escape))
            {
                if !self.renaming_text.is_empty() && *self.renaming_text != item.display_name {
                    let new_name: String = mem::take(&mut self.renaming_text);
                    self.renaming_item = None;
                    GableApp::rename_command(item.fullpath.clone(), new_name);
                } else {
                    self.renaming_item = None;
                    self.renaming_text.clear();
                }
            }
            // 处理通过ESC键取消重命名
            else if response.lost_focus() && ui.input(|i: &InputState| i.key_pressed(Key::Escape))
            {
                self.renaming_item = None;
                self.renaming_text.clear();
            }
        } else {
            let header_text: String = format!("{} {}", icon, item.display_name);
            let is_selected: bool = self
                .selected_tree_item
                .as_ref()
                .map_or(false, |id: &String| id == &item.fullpath);

            let header_response: Response = match item.item_type {
                EItemType::Sheet => {
                    // 使用 CollapsingHeader 但禁用展开功能以保持一致的外观和交互
                    CollapsingHeader::new(&header_text)
                        .default_open(item.is_open)
                        .open(Some(false)) // 禁用展开状态
                        .icon(|_, _, _| {}) // 隐藏箭头图标
                        .show(ui, |_| {})
                        .header_response
                }
                _ => {
                    CollapsingHeader::new(&header_text)
                        .default_open(item.is_open)
                        .id_salt(format!(
                            "{}{}",
                            &item.fullpath,
                            if item.is_open { "_open" } else { "" }
                        ))
                        .show(ui, |ui| {
                            for child in &item.children {
                                self.gui_tree_item(ui, child);
                            }
                        })
                        .header_response
                }
            };

            // 只有点击header文本区域时才选中
            if header_response.clicked() {
                self.selected_tree_item = Some(item.fullpath.clone());
            }

            // 处理双击事件
            if header_response.double_clicked() {
                GableApp::open_command(item.fullpath.clone());
            }

            // 添加选中状态的视觉反馈
            if is_selected {
                ui.painter().rect_filled(
                    header_response.rect,
                    CornerRadius::ZERO,
                    Color32::from_rgb(0, 120, 200).linear_multiply(0.2),
                );
            }

            header_response.context_menu(|ui| {
                let mut item_clone = item.clone();
                Self::show_context_menu(ui, &mut item_clone);
            });
        }
    }

    /**
     * 显示右键菜单
     */
    fn show_context_menu(ui: &mut Ui, item: &mut TreeItem) {
        match item.item_type {
            EItemType::Folder => {
                if ui.button(localization_manager::t("new_file")).clicked() {
                    GableApp::create_excel_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button(localization_manager::t("new_folder")).clicked() {
                    GableApp::create_folder_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button(localization_manager::t("import")).clicked() {
                    ui.close();
                    if let Some(files) = FileDialog::new()
                        .set_title(localization_manager::t("select_import_file"))
                        .add_filter("Excel Files", &["xlsx", "xls"])
                        .pick_files()
                    {
                        excel_util::import_excels(&item.fullpath, files);
                    }
                }
                ui.separator();
                if ui.button(localization_manager::t("delete")).clicked() {
                    GableApp::delete_comand(item.fullpath.clone());
                    ui.close();
                }
                if ui
                    .button(localization_manager::t("show_in_resource_manager"))
                    .clicked()
                {
                    if let Err(e) = utils::open_in_explorer(&item.fullpath) {
                        log::error!("Cannot open the Resource Manager: {}", e);
                    }
                    ui.close();
                }
            }
            EItemType::Excel => {
                if ui.button(localization_manager::t("new_file")).clicked() {
                    GableApp::create_sheet_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button(localization_manager::t("edit")).clicked() {
                    GableApp::editor_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button(localization_manager::t("export")).clicked() {
                    GableApp::convert_item_command(item.fullpath.clone());
                    ui.close();
                }
                if ui
                    .button(localization_manager::t("generate_script"))
                    .clicked()
                {
                    GableApp::generate_item_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button(localization_manager::t("rename")).clicked() {
                    GableApp::editname_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button(localization_manager::t("delete")).clicked() {
                    GableApp::delete_comand(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui
                    .button(localization_manager::t("show_in_resource_manager"))
                    .clicked()
                {
                    if let Some(path) = &item.parent {
                        if let Err(e) = utils::open_in_explorer(&path) {
                            log::error!("Cannot open the Resource Manager: {}", e);
                        }
                    }
                    ui.close();
                }
            }
            EItemType::Sheet => {
                if ui.button(localization_manager::t("edit")).clicked() {
                    GableApp::editor_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button(localization_manager::t("export")).clicked() {
                    GableApp::convert_item_command(item.fullpath.clone());
                    ui.close();
                }
                if ui
                    .button(localization_manager::t("generate_script"))
                    .clicked()
                {
                    GableApp::generate_item_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button(localization_manager::t("rename")).clicked() {
                    GableApp::editname_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button(localization_manager::t("delete")).clicked() {
                    GableApp::delete_comand(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui
                    .button(localization_manager::t("show_in_resource_manager"))
                    .clicked()
                {
                    if let Err(e) = utils::open_in_explorer(&item.fullpath) {
                        log::error!("Cannot open the Resource Manager: {}", e);
                    }
                    ui.close();
                }
            }
        }
    }
    /**
     * 创建文件夹
     */
    pub fn create_folder(&mut self, full_path: String) {
        let tree_items = gables::TREE_ITEMS.read().unwrap();
        let (is_folder, target_path) =
            if let Some(parent_item) = gables::find_item_by_path(&tree_items, &full_path) {
                if parent_item.item_type == EItemType::Folder {
                    (true, Some(parent_item.fullpath.clone()))
                } else {
                    (false, None)
                }
            } else {
                // 当找不到路径时，表示在根目录创建
                (
                    true,
                    Some(setting::get_workspace().to_string_lossy().to_string()),
                )
            };

        // 释放读锁后再获取写锁
        drop(tree_items);

        if is_folder {
            let root_path: PathBuf = PathBuf::from(&target_path.unwrap());
            let new_folder: String = localization_manager::t("new_folder");
            let new_folder_path: PathBuf = Path::new(&root_path).join(&new_folder);
            let mut new_path: PathBuf = new_folder_path.clone();
            let mut counter: i32 = 1;
            while new_path.exists() {
                let new_name: String = format!("{}({})", new_folder, counter);
                new_path = Path::new(&root_path).join(new_name);
                counter += 1;
            }

            match fs::create_dir_all(&new_path) {
                Ok(_) => {
                    if let Some(file_name) = new_path.file_name() {
                        gables::add_new_item(&new_path, EItemType::Folder);
                        self.renaming_item = Some(new_path.to_string_lossy().to_string());
                        self.renaming_text = file_name.to_string_lossy().to_string();
                    }
                }
                Err(e) => {
                    log::error!("Failed to create the folder:{}", e);
                }
            }
        }
    }
    /**
     * 创建excel
     */
    pub fn create_excel(&mut self, full_path: String) {
        let tree_items = gables::TREE_ITEMS.read().unwrap();
        let (is_folder, target_path) =
            if let Some(parent_item) = gables::find_item_by_path(&tree_items, &full_path) {
                if parent_item.item_type == EItemType::Folder {
                    (true, Some(parent_item.fullpath.clone()))
                } else {
                    (false, None)
                }
            } else {
                // 当找不到路径时，表示在根目录创建
                (
                    true,
                    Some(setting::get_workspace().to_string_lossy().to_string()),
                )
            };

        // 释放读锁后再获取写锁
        drop(tree_items);

        if is_folder {
            let parent_path: PathBuf = PathBuf::from(&target_path.unwrap());
            let mut counter: i32 = 1;
            let mut new_excel_path: PathBuf = parent_path.join("Excel_New@Sheet_New.gable");
            while new_excel_path.exists() {
                let new_name: String = format!("Excel_New{}@Sheet_New.gable", counter);
                new_excel_path = parent_path.join(new_name);
                counter += 1;
            }

            match excel_util::write_gable_new(&new_excel_path) {
                Ok(_) => {
                    gables::add_new_item(&new_excel_path, EItemType::Excel);
                    gables::add_new_item(&new_excel_path, EItemType::Sheet);
                    let file_name_str = new_excel_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let excel_name = if let Some((excel_name, _)) =
                        utils::parse_gable_filename(&file_name_str)
                    {
                        excel_name
                    } else {
                        "Excel_New".to_string()
                    };
                    let excel_virtual_path = Path::new(&parent_path)
                        .join(&excel_name)
                        .to_string_lossy()
                        .to_string();
                    self.renaming_item = Some(excel_virtual_path);
                    self.renaming_text = excel_name;
                }
                Err(e) => {
                    log::error!("Failed to create the Excel file: {}", e);
                }
            }
        }
    }
    /**
     * 创建Excel文件
     */
    pub fn create_sheet(&mut self, full_path: String) {
        let tree_items = gables::TREE_ITEMS.read().unwrap();
        let is_excel = if let Some(parent_item) = gables::find_item_by_path(&tree_items, &full_path)
        {
            parent_item.item_type == EItemType::Excel
        } else {
            false
        };

        // 释放读锁后再获取写锁
        drop(tree_items);

        if is_excel {
            let parent_path: PathBuf = PathBuf::from(&full_path).parent().unwrap().to_path_buf();
            let tree_items = gables::TREE_ITEMS.read().unwrap();
            let excel_name = gables::find_item_by_path(&tree_items, &full_path)
                .unwrap()
                .display_name
                .clone();
            drop(tree_items);

            let mut counter: i32 = 1;
            let mut new_sheet_path: PathBuf = parent_path.join(format!(
                "{}@Sheet_New{}",
                excel_name,
                constant::GABLE_FILE_TYPE
            ));
            while new_sheet_path.exists() {
                let new_name: String = format!(
                    "{}@Sheet_New{}{}",
                    excel_name,
                    counter,
                    constant::GABLE_FILE_TYPE
                );
                new_sheet_path = parent_path.join(new_name);
                counter += 1;
            }

            match excel_util::write_gable_new(&new_sheet_path) {
                Ok(_) => {
                    gables::add_new_item(&new_sheet_path, EItemType::Sheet);
                    self.renaming_item = Some(new_sheet_path.to_string_lossy().to_string());
                    self.renaming_text = "Sheet_New".to_string();
                }
                Err(e) => {
                    log::error!("Failed to create the Sheet file: {}", e);
                }
            }
        }
    }

    pub fn edit_name(&mut self, full_path: String) {
        let item_info = {
            let tree_items = gables::TREE_ITEMS.read().unwrap();
            gables::find_item_by_path(&tree_items, &full_path)
                .map(|item| Some(item.clone()))
                .unwrap_or(None)
        };
        if let Some(item) = item_info {
            self.renaming_item = Some(item.fullpath);
            self.renaming_text = item.display_name;
            self.auto_focus = true;
        }
    }

    /**
     * 重命名
     */
    pub fn rename(&mut self, full_path: String, new_name: String) {
        if new_name.is_empty() {
            return;
        }
        // 进行合法性校验
        if !utils::is_valid_filename(&new_name) {
            log::error!("The file name contains illegal characters.:{}", &new_name);
            return;
        }
        // 先获取item信息，然后释放锁
        let item_info = {
            let tree_items = gables::TREE_ITEMS.read().unwrap();
            gables::find_item_by_path(&tree_items, &full_path)
                .map(|item| {
                    // 检查名称是否更改
                    if new_name == item.display_name {
                        None // 名称未更改，直接返回
                    } else {
                        // 克隆完整项目并释放锁
                        Some(item.clone())
                    }
                })
                .unwrap_or(None)
        };

        if let Some(item) = item_info {
            // 检查同名文件/文件夹是否已存在
            if utils::is_name_exists(&item.fullpath, &new_name) {
                log::error!(
                    "A file or folder with the same name already exists:{}",
                    &new_name
                );
                return;
            }

            let result: Result<Option<String>, Error> = match item.item_type {
                EItemType::Excel => self.rename_excel_item(&item, &new_name),
                EItemType::Sheet => self.rename_sheet_item(&item, &new_name),
                EItemType::Folder => self.rename_folder_item(&item, &new_name),
            };

            match result {
                Err(e) => {
                    log::error!("Renaming failed:{}", e);
                }
                Ok(new_path) => {
                    let new_fullpath: String = new_path.unwrap_or(item.fullpath.clone());
                    gables::refresh_item(&item.fullpath, &new_fullpath);
                }
            }
        }
    }
    /**
     * 重命名文件夹项
     */
    fn rename_folder_item(&self, item: &TreeItem, new_name: &str) -> Result<Option<String>, Error> {
        let path: &Path = Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            let new_path: PathBuf = parent_path.join(new_name);

            // 检查目标文件夹是否已存在
            if new_path.exists() && path != new_path {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    "The target folder already exists.",
                ));
            }

            // 重命名文件夹
            if path.to_string_lossy() != new_path.to_string_lossy() {
                fs::rename(&path, &new_path)?;
                return Ok(Some(new_path.to_string_lossy().to_string()));
            }
        }
        Ok(None)
    }
    /**
     * 重命名Excel项及所有相关sheet文件
     */
    fn rename_excel_item(&self, item: &TreeItem, new_name: &str) -> Result<Option<String>, Error> {
        let mut new_path: Option<String> = None;
        // 获取Excel文件所在目录
        if let Some(parent_path) = Path::new(&item.fullpath).parent() {
            if let Ok(entries) = fs::read_dir(parent_path) {
                for entry in entries.filter_map(|e: Result<DirEntry, Error>| e.ok()) {
                    let entry_path: PathBuf = entry.path();
                    if entry_path.is_file() {
                        if let Some(file_name) = entry_path.file_name() {
                            let file_name_str: String = file_name.to_string_lossy().to_string();

                            // 检查是否为.gable文件
                            if file_name_str.ends_with(constant::GABLE_FILE_TYPE) {
                                // 解析文件名
                                if let Some((excel_name, sheet_name)) =
                                    utils::parse_gable_filename(&file_name_str)
                                {
                                    // 如果excel名称匹配当前重命名的excel
                                    if excel_name == item.display_name {
                                        // 构造新的文件名
                                        let new_file_name: String = if let Some(sheet) = &sheet_name
                                        {
                                            format!(
                                                "{}@{}{}",
                                                new_name,
                                                sheet,
                                                constant::GABLE_FILE_TYPE
                                            )
                                        } else {
                                            format!("{}{}", new_name, constant::GABLE_FILE_TYPE)
                                        };

                                        // 构造新的完整路径
                                        let new_excel_path: PathBuf =
                                            parent_path.join(new_file_name);

                                        // 检查目标文件是否已存在
                                        if new_excel_path.exists() && entry_path != new_excel_path {
                                            return Err(Error::new(
                                                ErrorKind::AlreadyExists,
                                                "The target file already exists.",
                                            ));
                                        }

                                        // 重命名文件
                                        if entry_path.to_string_lossy()
                                            != new_excel_path.to_string_lossy()
                                        {
                                            fs::rename(&entry_path, &new_excel_path)?;
                                            new_path = Some(
                                                parent_path
                                                    .join(new_name)
                                                    .to_string_lossy()
                                                    .to_string(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(new_path)
    }
    /**
     * 重命名单个sheet项
     */
    fn rename_sheet_item(&self, item: &TreeItem, new_name: &str) -> Result<Option<String>, Error> {
        let path: &Path = Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            if let Some(file_name) = path.file_name() {
                let file_name_str: Cow<'_, str> = file_name.to_string_lossy();

                // 解析原始文件名
                if let Some((excel_name, _)) = utils::parse_gable_filename(&file_name_str) {
                    // 构造新的文件名: excelname@new_sheetname.gable
                    let new_file_name: String =
                        format!("{}@{}{}", excel_name, new_name, constant::GABLE_FILE_TYPE);
                    let new_path: PathBuf = parent_path.join(new_file_name);

                    // 检查目标文件是否已存在
                    if new_path.exists() && path != new_path {
                        return Err(Error::new(
                            ErrorKind::AlreadyExists,
                            "The target file already exists.",
                        ));
                    }

                    // 重命名文件
                    if path.to_string_lossy() != new_path.to_string_lossy() {
                        fs::rename(&path, &new_path)?;
                        return Ok(Some(new_path.to_string_lossy().to_string()));
                    }
                }
            }
        }
        Ok(None)
    }
}
