use eframe::egui;

use crate::gui::datas::gables;
use crate::gui::datas::gables::{ItemType, TreeItem};

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
    pub fn gui_tree_view(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("my_gables_panel")
            .min_width(150.0) // 设置最小宽度
            .max_width(800.0) // 设置最大宽度
            .resizable(true)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let tree_items = gables::TREE_ITEMS.lock().unwrap();
                        for item in tree_items.iter() {
                            Self::gui_tree_item(
                                ui,
                                item,
                                &mut self.selected_tree_item,
                                &mut self.renaming_item,
                                &mut self.renaming_text,
                            );
                        }
                    });
            });
    }

    /// 带右键菜单的树形结构绘制
    fn gui_tree_item(
        ui: &mut egui::Ui,
        item: &TreeItem,
        selected_id: &mut Option<String>,
        renaming_item: &mut Option<String>,
        renaming_text: &mut String,
    ) {
        let icon = match item.item_type {
            ItemType::Folder => "📁",
            ItemType::Excel => "📄",
            ItemType::Sheet => "📊",
        };

        // 检查是否是当前正在重命名的项目
        let is_renaming = renaming_item
            .as_ref()
            .map_or(false, |id| id == &item.fullpath);

        if is_renaming {
            // 显示重命名输入框
            let response = ui.text_edit_singleline(renaming_text);

            // 处理回车确认重命名
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                Self::finish_renaming(
                    item,
                    std::mem::take(renaming_text),
                    renaming_item,
                    renaming_text,
                );
            }
            // 新增：处理失去焦点时完成重命名（不是通过ESC键）
            else if response.lost_focus() && !ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                Self::finish_renaming(
                    item,
                    std::mem::take(renaming_text),
                    renaming_item,
                    renaming_text,
                );
            }
            // 处理通过ESC键取消重命名
            else if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                *renaming_item = None;
                renaming_text.clear();
            }
        } else {
            let header_text = format!("{} {}", icon, item.display_name);
            // 检查当前项是否被选中
            let is_selected = selected_id
                .as_ref()
                .map_or(false, |id| id == &item.fullpath);

            let header_response = match item.item_type {
                ItemType::Sheet => {
                    // 使用 CollapsingHeader 但禁用展开功能以保持一致的外观和交互
                    egui::CollapsingHeader::new(&header_text)
                        .default_open(item.is_open)
                        .open(Some(false)) // 禁用展开状态
                        .icon(|_, _, _| {}) // 隐藏箭头图标
                        .show(ui, |ui| {})
                        .header_response
                }
                _ => {
                    // 其他类型使用CollapsingHeader
                    egui::CollapsingHeader::new(&header_text)
                        .default_open(item.is_open)
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
                println!("Clicked: {}", item.fullpath.clone())
            }

            // 添加选中状态的视觉反馈
            if is_selected {
                ui.painter().rect_filled(
                    header_response.rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_rgb(0, 120, 200).linear_multiply(0.2),
                );
            }

            // 为header添加右键菜单
            header_response.context_menu(|ui| {
                Self::show_context_menu(ui, item, renaming_item, renaming_text);
            });
        }
    }

    /// 完成重命名操作
    fn finish_renaming(
        item: &TreeItem,
        new_name: String,
        renaming_item: &mut Option<String>,
        renaming_text: &mut String,
    ) {
        if new_name.is_empty() || new_name == item.display_name {
            // 如果名称为空或未更改，则取消重命名
            *renaming_item = None;
            renaming_text.clear();
            return;
        }

        // 进行合法性校验
        if !Self::is_valid_filename(&new_name) {
            println!("文件名包含非法字符");
            // 保留编辑状态，让用户重新输入
            return;
        }

        // 检查同名文件/文件夹是否已存在
        if Self::is_name_exists(item, &new_name) {
            println!("同名文件或文件夹已存在");
            // 保留编辑状态，让用户重新输入
            return;
        }

        let result = match item.item_type {
            ItemType::Excel => {
                // 重命名Excel文件及其所有sheet文件
                Self::rename_excel_item(item, &new_name)
            }
            ItemType::Sheet => {
                // 重命名单个sheet
                Self::rename_sheet_item(item, &new_name)
            }
            ItemType::Folder => {
                // 重命名文件夹
                Self::rename_folder_item(item, &new_name)
            }
        };

        // 清理重命名状态
        *renaming_item = None;
        renaming_text.clear();

        if let Err(e) = result {
            println!("重命名失败: {}", e);
        }

        // 延迟刷新，在下一次update中执行
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(100));
            gables::refresh_gables();
        });
    }

    /// 重命名文件夹项
    fn rename_folder_item(item: &TreeItem, new_folder_name: &str) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            let new_path = parent_path.join(new_folder_name);

            // 检查目标文件夹是否已存在
            if new_path.exists() && path != new_path {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "目标文件夹已存在",
                ));
            }

            // 重命名文件夹
            if path.to_string_lossy() != new_path.to_string_lossy() {
                std::fs::rename(&path, &new_path)?;
            }
        }
        Ok(())
    }

    /// 检查文件名是否合法
    fn is_valid_filename(name: &str) -> bool {
        // 检查是否为空
        if name.is_empty() {
            return false;
        }

        // 检查是否包含非法字符
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        for c in name.chars() {
            if invalid_chars.contains(&c) || c.is_control() {
                return false;
            }
        }

        // 检查是否以点或空格结尾
        if name.ends_with('.') || name.ends_with(' ') {
            return false;
        }

        // 检查是否是保留名称
        let reserved_names = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];

        let upper_name = name.to_uppercase();
        for reserved in &reserved_names {
            if &upper_name == reserved {
                return false;
            }
        }
        true
    }

    /// 检查同名文件/文件夹是否已存在
    fn is_name_exists(item: &TreeItem, new_name: &str) -> bool {
        let path = std::path::Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            let new_path = parent_path.join(new_name);
            new_path.exists()
        } else {
            false
        }
    }

    /// 重命名Excel项及所有相关sheet文件
    pub fn rename_excel_item(item: &TreeItem, new_excel_name: &str) -> Result<(), std::io::Error> {
        // 获取Excel文件所在目录
        if let Some(parent_path) = std::path::Path::new(&item.fullpath).parent() {
            // 查找所有相关的sheet文件
            if let Ok(entries) = std::fs::read_dir(parent_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        let file_name = entry_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // 检查是否为.gable文件
                        if file_name.ends_with(crate::common::global::GABLE_FILE_TYPE) {
                            // 解析文件名
                            if let Some((excel_name, sheet_name)) =
                                gables::parse_gable_filename(&file_name)
                            {
                                // 如果excel名称匹配当前重命名的excel
                                if excel_name == item.display_name {
                                    // 构造新的文件名
                                    let new_file_name = if let Some(sheet) = sheet_name {
                                        format!(
                                            "{}@{}{}",
                                            new_excel_name,
                                            sheet,
                                            crate::common::global::GABLE_FILE_TYPE
                                        )
                                    } else {
                                        format!(
                                            "{}{}",
                                            new_excel_name,
                                            crate::common::global::GABLE_FILE_TYPE
                                        )
                                    };

                                    // 构造新的完整路径
                                    let new_path = parent_path.join(new_file_name);

                                    // 检查目标文件是否已存在
                                    if new_path.exists() && entry_path != new_path {
                                        return Err(std::io::Error::new(
                                            std::io::ErrorKind::AlreadyExists,
                                            "目标文件已存在",
                                        ));
                                    }

                                    // 重命名文件
                                    if entry_path.to_string_lossy() != new_path.to_string_lossy() {
                                        std::fs::rename(&entry_path, &new_path)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// 重命名单个sheet项
    fn rename_sheet_item(item: &TreeItem, new_sheet_name: &str) -> Result<(), std::io::Error> {
        // 从完整路径中提取目录和原始文件名
        let path = std::path::Path::new(&item.fullpath);
        if let Some(parent_path) = path.parent() {
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();

                // 解析原始文件名
                if let Some((excel_name, _)) = gables::parse_gable_filename(&file_name_str) {
                    // 构造新的文件名: excelname@new_sheetname.gable
                    let new_file_name = format!(
                        "{}@{}{}",
                        excel_name,
                        new_sheet_name,
                        crate::common::global::GABLE_FILE_TYPE
                    );
                    let new_path = parent_path.join(new_file_name);

                    // 检查目标文件是否已存在
                    if new_path.exists() && path != new_path {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::AlreadyExists,
                            "目标文件已存在",
                        ));
                    }

                    // 重命名文件
                    if path.to_string_lossy() != new_path.to_string_lossy() {
                        std::fs::rename(&path, &new_path)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// 显示右键菜单
    fn show_context_menu(
        ui: &mut egui::Ui,
        item: &TreeItem,
        renaming_item: &mut Option<String>,
        renaming_text: &mut String,
    ) {
        match item.item_type {
            ItemType::Folder => {
                if ui.button("新建文件").clicked() {
                    // TODO: 实现新建文件逻辑
                    ui.close();
                }
                if ui.button("新建文件夹").clicked() {
                    Self::create_new_folder_and_edit(item, renaming_item, renaming_text);
                    ui.close();
                }
            }
            ItemType::Excel => {
                if ui.button("新建文件").clicked() {
                    // TODO: 实现新建文件逻辑
                    ui.close();
                }
                if ui.button("编辑").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close();
                }
                ui.separator();
                if ui.button("重命名").clicked() {
                    *renaming_item = Some(item.fullpath.clone());
                    *renaming_text = item.display_name.clone();
                    ui.close();
                }
                if ui.button("删除").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close();
                }
            }
            ItemType::Sheet => {
                if ui.button("编辑").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close();
                }
                ui.separator();
                if ui.button("重命名").clicked() {
                    *renaming_item = Some(item.fullpath.clone());
                    *renaming_text = item.display_name.clone();
                    ui.close();
                }
                if ui.button("删除").clicked() {
                    // TODO: 实现打开文件逻辑
                    ui.close();
                }
            }
        }
    }

    /// 创建新文件夹并进入编辑状态
    fn create_new_folder_and_edit(
        parent_item: &TreeItem,
        renaming_item: &mut Option<String>,
        renaming_text: &mut String,
    ) {
        // 确保只在文件夹类型上创建
        if parent_item.item_type != ItemType::Folder {
            return;
        }

        // 标记父节点为展开状态
        gables::set_folder_expanded(&parent_item.fullpath);

        // 构造新文件夹路径
        let new_folder_path = std::path::Path::new(&parent_item.fullpath).join("新建文件夹");

        // 如果文件夹已存在，则添加序号
        let mut new_path = new_folder_path.clone();
        let mut counter = 1;
        while new_path.exists() {
            let new_name = format!("新建文件夹({})", counter);
            new_path = std::path::Path::new(&parent_item.fullpath).join(new_name);
            counter += 1;
        }

        // 创建文件夹
        match std::fs::create_dir_all(&new_path) {
            Ok(_) => {
                // 设置重命名状态，使新建的文件夹进入编辑模式
                if let Some(file_name) = new_path.file_name() {
                    *renaming_item = Some(new_path.to_string_lossy().to_string());
                    *renaming_text = file_name.to_string_lossy().to_string();

                    // 延迟刷新，在下一次update中执行
                    std::thread::spawn(|| {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        gables::refresh_gables();
                    });
                }
            }
            Err(e) => {
                eprintln!("创建文件夹失败: {}", e);
            }
        }
    }
}
