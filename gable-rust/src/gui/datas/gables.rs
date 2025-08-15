use crate::common::{global, setting, utils};
use crate::gui::datas::eitem_type::EItemType;
use crate::gui::datas::{
    esheet_type::ESheetType, gable_data::GableData, tree_data::TreeData, tree_item::TreeItem,
};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::{
    cmp::Ordering, collections::HashMap, collections::HashSet, fs, io::Error, path::Path,
    path::PathBuf, sync::Arc, sync::Mutex, sync::MutexGuard,
};

lazy_static! {
    pub static ref TREE_ITEMS: Arc<Mutex<Vec<TreeItem>>> = Arc::new(Mutex::new(Vec::new()));
    // 跟踪需要展开的节点路径
    pub static ref EXPANDED_FOLDERS: Arc<Mutex<HashSet<String>>> =
        Arc::new(Mutex::new(HashSet::new()));
}

// 添加设置展开状态的函数
pub fn set_folder_expanded(path: &str) {
    let _ = EXPANDED_FOLDERS.lock().unwrap().insert(path.to_string());
}

/// 解析 .gable 文件名，返回 (excel_name, sheet_name) 或仅 excel_name
pub(crate) fn parse_gable_filename(filename: &str) -> Option<(String, Option<String>)> {
    if !filename.ends_with(global::GABLE_FILE_TYPE) {
        return None;
    }

    let name_without_ext: &str = &filename[..filename.len() - global::GABLE_FILE_TYPE.len()];

    if let Some(pos) = name_without_ext.find('@') {
        // 格式为 excelname@sheetname
        let excel_name: String = name_without_ext[..pos].to_string();
        let sheet_name: String = name_without_ext[pos + 1..].to_string();
        Some((excel_name, Some(sheet_name)))
    } else {
        // 格式为 excelname
        Some((name_without_ext.to_string(), None))
    }
}

/// 并行读取所有gable文件
fn read_all_gable_files_parallel(
    gable_files: &HashMap<String, Vec<(String, String)>>,
) -> HashMap<String, Option<GableData>> {
    // 收集所有文件路径
    let file_paths: Vec<String> = gable_files
        .values()
        .flat_map(|sheets| sheets.iter().map(|(path, _)| path.clone()))
        .collect();

    // 使用rayon并行处理所有文件读取
    file_paths
        .into_par_iter()
        .map(|file_path| {
            let content: Option<GableData> = utils::read_gable_file(&file_path);
            (file_path, content)
        })
        .collect()
}

/// 递归构建目录树
fn build_tree_from_path(path: &Path) -> Vec<TreeItem> {
    let mut items: Vec<TreeItem> = Vec::new();

    if !path.exists() || !path.is_dir() {
        return items;
    }

    // 收集目录项和文件项
    let mut directories: Vec<(PathBuf, String)> = Vec::new();
    let mut gable_files: HashMap<String, Vec<(String, String)>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e: Result<fs::DirEntry, Error>| e.ok()) {
            let entry_path: PathBuf = entry.path();
            let entry_name: String = entry.file_name().to_string_lossy().to_string();

            if entry_path.is_dir() {
                if !global::IGNORED_DIRS.contains(&entry_name.as_str()) {
                    directories.push((entry_path, entry_name));
                }
            } else if let Some((excel_name, sheet_name)) = parse_gable_filename(&entry_name) {
                gable_files
                    .entry(excel_name)
                    .or_insert_with(Vec::new)
                    .push((
                        entry_path.to_string_lossy().to_string(),
                        sheet_name.unwrap_or_default(),
                    ));
            }
        }
    }

    // 并行读取所有gable文件内容
    let file_contents: HashMap<String, Option<GableData>> =
        read_all_gable_files_parallel(&gable_files);

    // 处理目录
    for (dir_path, dir_name) in directories {
        let children: Vec<TreeItem> = build_tree_from_path(&dir_path);

        // 检查此路径是否应该展开
        let should_be_expanded: bool = {
            let expanded_folders: MutexGuard<'_, HashSet<String>> =
                EXPANDED_FOLDERS.lock().unwrap();
            expanded_folders.contains(&dir_path.to_string_lossy().to_string())
        };

        // 创建目录项
        items.push(TreeItem {
            item_type: EItemType::Folder,
            display_name: dir_name,
            is_open: should_be_expanded,
            fullpath: dir_path.to_string_lossy().to_string(),
            parent: Some(path.to_string_lossy().to_string()),
            children,
            data: None,
        });
    }

    // 处理 .gable 文件
    for (excel_name, sheets) in gable_files {
        if sheets.len() == 1 && sheets[0].1.is_empty() {
            // 读取文件内容
            let gable_content: Option<GableData> =
                file_contents.get(&sheets[0].0).cloned().unwrap_or(None);
            // 确定文件类型
            let sheet_type: ESheetType = utils::determine_sheet_type(Path::new(&sheets[0].0));
            let tree_data: Option<TreeData> = gable_content.map(|content| TreeData {
                gable_type: sheet_type,
                content,
            });
            items.push(TreeItem {
                item_type: EItemType::Excel,
                display_name: excel_name,
                is_open: false,
                fullpath: sheets[0].0.clone(),
                parent: Some(path.to_string_lossy().to_string()),
                children: vec![],
                data: tree_data,
            });
        } else {
            // 有多个 sheet 或有 sheet 部分
            let excel_fullpath: String = format!("{}/{}", path.to_string_lossy(), excel_name);

            // 创建子项
            let mut children: Vec<TreeItem> = Vec::new();
            let mut excel_gable_content: Option<TreeData> = None;
            let sheets_len: usize = sheets.len();

            for (full_path, sheet_name) in sheets {
                // 读取每个sheet文件的内容
                let gable_content: Option<GableData> =
                    file_contents.get(&full_path).cloned().unwrap_or(None);
                // 确定文件类型
                let sheet_type: ESheetType = utils::determine_sheet_type(Path::new(&full_path));
                let tree_data: Option<TreeData> = gable_content.map(|content| TreeData {
                    gable_type: sheet_type,
                    content,
                });

                if sheets_len == 1 && sheet_name.is_empty() {
                    excel_gable_content = tree_data.clone();
                }

                if !sheet_name.is_empty() {
                    children.push(TreeItem {
                        item_type: EItemType::Sheet,
                        display_name: sheet_name,
                        is_open: false,
                        fullpath: full_path.clone(),
                        parent: Some(excel_fullpath.clone()),
                        children: vec![],
                        data: tree_data,
                    });
                } else {
                    // 没有 sheet 部分的文件作为默认 sheet
                    children.push(TreeItem {
                        item_type: EItemType::Sheet,
                        display_name: "默认".to_string(),
                        is_open: false,
                        fullpath: full_path.clone(),
                        parent: Some(excel_fullpath.clone()),
                        children: vec![],
                        data: tree_data,
                    });
                }
            }

            // 对子项进行排序
            children.sort_by(|a, b| a.display_name.cmp(&b.display_name));

            items.push(TreeItem {
                item_type: EItemType::Excel,
                display_name: excel_name,
                is_open: false,
                fullpath: excel_fullpath,
                parent: Some(path.to_string_lossy().to_string()),
                children,
                data: excel_gable_content, // Excel节点本身的内容（如果有默认sheet）
            });
        }
    }

    // 对所有项进行排序，文件夹在前
    items.sort_by(|a, b| match (&a.item_type, &b.item_type) {
        (EItemType::Folder, EItemType::Folder) => a.display_name.cmp(&b.display_name),
        (EItemType::Folder, _) => Ordering::Less,
        (_, EItemType::Folder) => Ordering::Greater,
        _ => a.display_name.cmp(&b.display_name),
    });

    items
}

// 定义内部函数来递归查找指定路径的项
fn get_item_by_path(items: &[TreeItem], path: &str) -> Option<TreeItem> {
    for item in items.iter() {
        if item.fullpath == path {
            return Some(item.clone());
        }

        if let Some(result) = get_item_by_path(&item.children, path) {
            return Some(result);
        }
    }
    None
}

fn find_parent_for_item(item: TreeItem, item_type: EItemType) -> Option<TreeItem> {
    if item.item_type == item_type {
        return Some(item);
    }
    let parent_path: String = item.parent?;
    match TREE_ITEMS.try_lock() {
        Ok(tree_items) => {
            let tree_items_copy: Vec<TreeItem> = tree_items.clone();
            // 释放锁
            drop(tree_items);

            for root_item in tree_items_copy.iter() {
                if let Some(parent_item) = get_item_by_path(&[root_item.clone()], &parent_path) {
                    return find_parent_for_item(parent_item, item_type);
                }
            }
            None
        }
        Err(_) => {
            // 获取锁失败，说明锁已被占用，返回None避免死锁
            log::warn!("无法获取TREE_ITEMS锁，跳过查找父项");
            None
        }
    }
}

/// 根据路径直接获取TreeItem，保证返回的是item_type类型
pub fn find_tree_item_by_path(path: &str, item_type: EItemType) -> Option<TreeItem> {
    let tree_items_copy: Vec<TreeItem> = TREE_ITEMS.lock().unwrap().clone();
    let mut found_item: Option<TreeItem> = None;
    for root_item in tree_items_copy.iter() {
        if let Some(item) = get_item_by_path(&[root_item.clone()], path) {
            found_item = Some(item);
            break;
        }
    }
    if let Some(item) = found_item {
        find_parent_for_item(item, item_type)
    } else {
        None
    }
}

/// 编辑gable文件
pub fn edit_gable(item: TreeItem) {
    if item.item_type == EItemType::Folder {
        log::error!("文件夹不能进行编辑");
        return;
    }

    let excel_name: String = if item.item_type == EItemType::Excel {
        item.display_name.clone()
    } else {
        let file_name: String = {
            let path: &Path = Path::new(&item.fullpath);
            if let Some(file_name) = path.file_name() {
                file_name.to_string_lossy().to_string()
            } else {
                item.fullpath.clone()
            }
        };

        if let Some(at_pos) = file_name.find('@') {
            file_name[..at_pos].to_string()
        } else if let Some(dot_pos) = file_name.rfind('.') {
            file_name[..dot_pos].to_string()
        } else {
            item.display_name.clone()
        }
    };
    let parent_path: String = {
        let path: &Path = Path::new(&item.fullpath);
        if let Some(parent) = path.parent() {
            parent.to_string_lossy().to_string()
        } else {
            ".".to_string()
        }
    };
    let mut related_files: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&parent_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_name: String = entry.file_name().to_string_lossy().to_string();

            // 检查是否为.gable文件且excel名称匹配
            if let Some((parsed_excel_name, _)) = parse_gable_filename(&entry_name) {
                if parsed_excel_name == excel_name {
                    related_files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    match utils::write_excel(&excel_name, related_files) {
        Ok(excel_file_path) => {
            log::info!("编辑文件 {}:", excel_name);
            // 使用系统命令打开Excel文件
            #[cfg(target_os = "windows")]
            {
                if let Err(e) = std::process::Command::new("cmd")
                    .args(&["/C", "start", "", &excel_file_path])
                    .spawn()
                {
                    log::error!("无法打开Excel文件: {}", e);
                }
            }

            #[cfg(target_os = "macos")]
            {
                if let Err(e) = std::process::Command::new("open")
                    .arg(&excel_file_path)
                    .spawn()
                {
                    log::error!("无法打开Excel文件: {}", e);
                }
            }

            #[cfg(target_os = "linux")]
            {
                if let Err(e) = std::process::Command::new("xdg-open")
                    .arg(&excel_file_path)
                    .spawn()
                {
                    log::error!("无法打开Excel文件: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("写入Excel文件时出错: {}", e);
        }
    }
}

/// 项目目录调整好重置数据
pub fn refresh_gables() {
    let workspace: MutexGuard<'_, Option<String>> = setting::WORKSPACE.lock().unwrap();
    let root_path: &Path = if let Some(path) = workspace.as_ref() {
        Path::new(path)
    } else {
        Path::new(".")
    };

    let mut tree_items: Vec<TreeItem> = Vec::new();

    if root_path.exists() && root_path.is_dir() {
        // 直接读取工作空间下的所有子项作为根节点，而不是将工作空间本身作为根节点
        let children = build_tree_from_path(root_path);
        tree_items.extend(children);
    }
    *TREE_ITEMS.lock().unwrap() = tree_items;
}
