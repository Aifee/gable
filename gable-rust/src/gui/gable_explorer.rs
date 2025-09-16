use crate::{
    common::{constant, excel_util, setting, utils},
    gui::{
        datas::{eitem_type::EItemType, gables, tree_item::TreeItem},
        gable_app::GableApp,
    },
};
use eframe::egui::{
    CollapsingHeader, Color32, Context, CornerRadius, Frame, InputState, Key, Response, ScrollArea,
    SidePanel, Ui,
};
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
}

impl GableExplorer {
    pub fn new() -> Self {
        Self {
            selected_tree_item: None,
            renaming_item: None,
            renaming_text: String::new(),
        }
    }

    /// 绘制 treeview
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
                            Self::gui_tree_item(
                                ui,
                                item,
                                &mut self.selected_tree_item,
                                &mut self.renaming_item,
                                &mut self.renaming_text,
                            );
                        }
                        // 添加空白区域右键菜单
                        ui.allocate_rect(
                            ui.available_rect_before_wrap(),
                            eframe::egui::Sense::click_and_drag(),
                        )
                        .context_menu(|ui| {
                            if ui.button("新建文件").clicked() {
                                GableApp::create_excel_command(
                                    setting::get_workspace().to_string_lossy().to_string(),
                                );
                                ui.close();
                            }
                            if ui.button("新建文件夹").clicked() {
                                GableApp::create_folder_command(
                                    setting::get_workspace().to_string_lossy().to_string(),
                                );
                                ui.close();
                            }
                            if ui.button("在资源管理器中显示").clicked() {
                                if let Some(path) = setting::get_workspace().to_str() {
                                    if let Err(e) = utils::open_in_explorer(path) {
                                        log::error!("无法打开资源管理器: {}", e);
                                    }
                                }
                                ui.close();
                            }
                        });
                    });
            });
    }

    /// 带右键菜单的树形结构绘制
    fn gui_tree_item(
        ui: &mut Ui,
        item: &TreeItem,
        selected_id: &mut Option<String>,
        renaming_item: &mut Option<String>,
        renaming_text: &mut String,
    ) {
        let icon: &'static str = match item.item_type {
            EItemType::Folder => "📁",
            EItemType::Excel => "📄",
            EItemType::Sheet => "📊",
        };

        // 检查是否是当前正在重命名的项目
        let is_renaming: bool = renaming_item
            .as_ref()
            .map_or(false, |id| id == &item.fullpath);

        if is_renaming {
            // 显示重命名输入框
            let response: Response = ui.text_edit_singleline(renaming_text);

            // 处理回车确认重命名
            if response.lost_focus() && ui.input(|i: &InputState| i.key_pressed(Key::Enter)) {
                // 注意：这里需要创建一个可变版本的item用于重命名
                // 但由于我们使用的是引用，这里只是传递信息给重命名函数
                // 实际的重命名逻辑不会修改当前引用的item
                if !renaming_text.is_empty() && *renaming_text != item.display_name {
                    // 这里我们不直接修改item，而是触发重命名操作
                    let new_name = mem::take(renaming_text);
                    *renaming_item = None;
                    // 执行重命名逻辑
                    // Self::execute_rename(item, new_name);
                    GableApp::rename_command(item.fullpath.clone(), new_name);
                } else {
                    *renaming_item = None;
                    renaming_text.clear();
                }
            }
            // 新增：处理失去焦点时完成重命名（不是通过ESC键）
            else if response.lost_focus()
                && !ui.input(|i: &InputState| i.key_pressed(Key::Escape))
            {
                if !renaming_text.is_empty() && *renaming_text != item.display_name {
                    let new_name = mem::take(renaming_text);
                    *renaming_item = None;
                    // Self::execute_rename(item, new_name);
                    GableApp::rename_command(item.fullpath.clone(), new_name);
                } else {
                    *renaming_item = None;
                    renaming_text.clear();
                }
            }
            // 处理通过ESC键取消重命名
            else if response.lost_focus() && ui.input(|i: &InputState| i.key_pressed(Key::Escape))
            {
                *renaming_item = None;
                renaming_text.clear();
            }
        } else {
            let header_text: String = format!("{} {}", icon, item.display_name);
            // 检查当前项是否被选中
            let is_selected: bool = selected_id
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
                    // 其他类型使用CollapsingHeader
                    CollapsingHeader::new(&header_text)
                        .default_open(item.is_open)
                        .id_salt(format!(
                            "{}{}",
                            &item.fullpath,
                            if item.is_open { "_open" } else { "" }
                        ))
                        .show(ui, |ui| {
                            // 显示子项（如果有的话）
                            for child in &item.children {
                                Self::gui_tree_item(
                                    ui,
                                    child,
                                    selected_id,
                                    renaming_item,
                                    renaming_text,
                                );
                            }
                        })
                        .header_response
                }
            };

            // 只有点击header文本区域时才选中
            if header_response.clicked() {
                *selected_id = Some(item.fullpath.clone());
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

            // 为header添加右键菜单
            header_response.context_menu(|ui| {
                // 为上下文菜单创建一个可变的副本
                let mut item_clone = item.clone();
                Self::show_context_menu(ui, &mut item_clone, renaming_item, renaming_text);
            });
        }
    }

    pub fn rename(&mut self, full_path: String, new_name: String) {
        if new_name.is_empty() {
            return;
        }
        // 进行合法性校验
        if !utils::is_valid_filename(&new_name) {
            log::error!("文件名包含非法字符:{}", &new_name);
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
                log::error!("同名文件或文件夹已存在:{}", &new_name);
                return;
            }

            let result: Result<Option<String>, Error> = match item.item_type {
                EItemType::Excel => self.rename_excel_item(&item, &new_name),
                EItemType::Sheet => self.rename_sheet_item(&item, &new_name),
                EItemType::Folder => self.rename_folder_item(&item, &new_name),
            };

            match result {
                Err(e) => {
                    log::error!("重命名失败:{}", e);
                }
                Ok(new_fullpath) => {
                    let new_fullpath = new_fullpath.unwrap_or(item.fullpath.clone());
                    let fullpath_clone = item.fullpath.clone();
                    gables::update_item_display_name(fullpath_clone, new_fullpath, new_name);
                }
            }
        }
    }

    /// 重命名文件夹项
    fn rename_folder_item(
        &self,
        item: &TreeItem,
        new_folder_name: &str,
    ) -> Result<Option<String>, Error> {
        let path: &Path = Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            let new_path: PathBuf = parent_path.join(new_folder_name);

            // 检查目标文件夹是否已存在
            if new_path.exists() && path != new_path {
                return Err(Error::new(ErrorKind::AlreadyExists, "目标文件夹已存在"));
            }

            // 重命名文件夹
            if path.to_string_lossy() != new_path.to_string_lossy() {
                fs::rename(&path, &new_path)?;
                return Ok(Some(new_path.to_string_lossy().to_string()));
            }
        }
        Ok(None)
    }

    /// 重命名Excel项及所有相关sheet文件
    fn rename_excel_item(
        &self,
        item: &TreeItem,
        new_excel_name: &str,
    ) -> Result<Option<String>, Error> {
        let mut new_main_path: Option<String> = None;
        // 获取Excel文件所在目录
        if let Some(parent_path) = Path::new(&item.fullpath).parent() {
            // 查找所有相关的sheet文件
            if let Ok(entries) = fs::read_dir(parent_path) {
                for entry in entries.filter_map(|e: Result<DirEntry, Error>| e.ok()) {
                    let entry_path: PathBuf = entry.path();
                    if entry_path.is_file() {
                        let file_name: String = entry_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // 检查是否为.gable文件
                        if file_name.ends_with(constant::GABLE_FILE_TYPE) {
                            // 解析文件名
                            if let Some((excel_name, sheet_name)) =
                                utils::parse_gable_filename(&file_name)
                            {
                                // 如果excel名称匹配当前重命名的excel
                                if excel_name == item.display_name {
                                    // 构造新的文件名
                                    let new_file_name: String = if let Some(sheet) = &sheet_name {
                                        format!(
                                            "{}@{}{}",
                                            new_excel_name,
                                            sheet,
                                            constant::GABLE_FILE_TYPE
                                        )
                                    } else {
                                        format!("{}{}", new_excel_name, constant::GABLE_FILE_TYPE)
                                    };

                                    // 构造新的完整路径
                                    let new_path: PathBuf = parent_path.join(new_file_name);

                                    // 检查目标文件是否已存在
                                    if new_path.exists() && entry_path != new_path {
                                        return Err(Error::new(
                                            ErrorKind::AlreadyExists,
                                            "目标文件已存在",
                                        ));
                                    }

                                    // 重命名文件
                                    if entry_path.to_string_lossy() != new_path.to_string_lossy() {
                                        fs::rename(&entry_path, &new_path)?;
                                        // 如果这是主Excel文件（没有sheet部分），记录新路径
                                        if sheet_name.is_none() {
                                            new_main_path =
                                                Some(new_path.to_string_lossy().to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(new_main_path)
    }
    /// 重命名单个sheet项
    fn rename_sheet_item(
        &self,
        item: &TreeItem,
        new_sheet_name: &str,
    ) -> Result<Option<String>, Error> {
        // 从完整路径中提取目录和原始文件名
        let path: &Path = Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            if let Some(file_name) = path.file_name() {
                let file_name_str: Cow<'_, str> = file_name.to_string_lossy();

                // 解析原始文件名
                if let Some((excel_name, _)) = utils::parse_gable_filename(&file_name_str) {
                    // 构造新的文件名: excelname@new_sheetname.gable
                    let new_file_name: String = format!(
                        "{}@{}{}",
                        excel_name,
                        new_sheet_name,
                        constant::GABLE_FILE_TYPE
                    );
                    let new_path: PathBuf = parent_path.join(new_file_name);

                    // 检查目标文件是否已存在
                    if new_path.exists() && path != new_path {
                        return Err(Error::new(ErrorKind::AlreadyExists, "目标文件已存在"));
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

    /// 显示右键菜单
    fn show_context_menu(
        ui: &mut Ui,
        item: &mut TreeItem,
        renaming_item: &mut Option<String>,
        renaming_text: &mut String,
    ) {
        match item.item_type {
            EItemType::Folder => {
                if ui.button("新建文件").clicked() {
                    GableApp::create_excel_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button("新建文件夹").clicked() {
                    GableApp::create_folder_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button("在资源管理器中显示").clicked() {
                    if let Err(e) = utils::open_in_explorer(&item.fullpath) {
                        log::error!("无法打开资源管理器: {}", e);
                    }
                    ui.close();
                }
            }
            EItemType::Excel => {
                if ui.button("新建文件").clicked() {
                    GableApp::create_sheet_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button("编辑").clicked() {
                    GableApp::editor_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button("导出").clicked() {
                    GableApp::convert_item_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button("生成代码").clicked() {
                    GableApp::generate_item_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button("重命名").clicked() {
                    *renaming_item = Some(item.fullpath.clone());
                    *renaming_text = item.display_name.clone();
                    ui.close();
                }
                if ui.button("删除").clicked() {
                    if gables::remove_item(&item.fullpath, &item.item_type) {
                        gables::request_remove_item_from_tree(item.fullpath.clone());
                    }
                    ui.close();
                }
                ui.separator();
                if ui.button("在资源管理器中显示").clicked() {
                    if let Some(path) = &item.parent {
                        if let Err(e) = utils::open_in_explorer(&path) {
                            log::error!("无法打开资源管理器: {}", e);
                        }
                    }
                    ui.close();
                }
            }
            EItemType::Sheet => {
                if ui.button("编辑").clicked() {
                    GableApp::editor_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button("导出").clicked() {
                    GableApp::convert_item_command(item.fullpath.clone());
                    ui.close();
                }
                if ui.button("生成代码").clicked() {
                    GableApp::generate_item_command(item.fullpath.clone());
                    ui.close();
                }
                ui.separator();
                if ui.button("重命名").clicked() {
                    *renaming_item = Some(item.fullpath.clone());
                    *renaming_text = item.display_name.clone();
                    ui.close();
                }
                if ui.button("删除").clicked() {
                    if gables::remove_item(&item.fullpath, &item.item_type) {
                        gables::request_remove_item_from_tree(item.fullpath.clone());
                    }
                    ui.close();
                }
                ui.separator();
                if ui.button("在资源管理器中显示").clicked() {
                    if let Err(e) = utils::open_in_explorer(&item.fullpath) {
                        log::error!("无法打开资源管理器: {}", e);
                    }
                    ui.close();
                }
            }
        }
    }

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
            let new_folder_path: PathBuf = Path::new(&root_path).join("新建文件夹");
            let mut new_path: PathBuf = new_folder_path.clone();
            let mut counter: i32 = 1;
            while new_path.exists() {
                let new_name: String = format!("新建文件夹({})", counter);
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
                    log::error!("创建文件夹失败:{}", e);
                }
            }
        }
    }
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
            let mut new_excel_path: PathBuf = parent_path.join("新建Excel@新建Sheet.gable");
            while new_excel_path.exists() {
                let new_name: String = format!("新建Excel_{}@新建Sheet.gable", counter);
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
                        "新建Excel".to_string()
                    };
                    let excel_virtual_path = Path::new(&parent_path)
                        .join(&excel_name)
                        .to_string_lossy()
                        .to_string();
                    self.renaming_item = Some(excel_virtual_path);
                    self.renaming_text = excel_name;
                }
                Err(e) => {
                    log::error!("创建Excel文件失败: {}", e);
                }
            }
        }
    }

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
                "{}@新建Sheet{}",
                excel_name,
                constant::GABLE_FILE_TYPE
            ));
            while new_sheet_path.exists() {
                let new_name: String = format!(
                    "{}@新建Sheet_{}{}",
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
                    self.renaming_text = "新建Sheet".to_string();
                }
                Err(e) => {
                    log::error!("创建Sheet文件失败: {}", e);
                }
            }
        }
    }
}
