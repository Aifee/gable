use crate::common::{constant, excel_util, setting, utils};
use crate::gui::datas::{
    eitem_type::EItemType, esheet_type::ESheetType, gable_data::GableData, tree_data::TreeData,
    tree_item::TreeItem, watcher_data::WatcherData,
};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::sync::RwLock;
use std::{
    cmp::Ordering, collections::HashMap, fs, io::Error, path::Path, path::PathBuf, sync::Arc,
};

lazy_static! {
    /// 全局存储当前的目录树
    pub static ref TREE_ITEMS: Arc<RwLock<Vec<TreeItem>>> = Arc::new(RwLock::new(Vec::new()));
    /// 正在编辑的文件列表
    pub static ref EDITION_FILES: Arc<RwLock<HashMap<String, WatcherData>>> = Arc::new(RwLock::new(HashMap::new()));
}

/// 添加编辑文件到编辑列表
fn add_editor_file(file_path: String, targe_path: String, sheet_type: ESheetType) {
    let mut editor_files = EDITION_FILES.write().unwrap();
    editor_files.insert(
        file_path,
        WatcherData {
            target_path: targe_path,
            sheet_type,
        },
    );
}

// 移除编辑文件
pub fn remove_editor_file(file_path: &str) {
    let mut editor_files = EDITION_FILES.write().unwrap();
    editor_files.remove(file_path);
}

// 判断是否是编辑文件
fn has_eidtor_file(file_path: &str) -> (bool, Option<WatcherData>) {
    let files = EDITION_FILES.read().unwrap();
    match files.get(file_path) {
        Some(data) => (true, Some(data.clone())),
        None => (false, None),
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
            let content: Option<GableData> = excel_util::read_gable_file(&file_path);
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
                if !constant::IGNORED_DIRS.contains(&entry_name.as_str()) {
                    directories.push((entry_path, entry_name));
                }
            } else if let Some((excel_name, sheet_name)) = utils::parse_gable_filename(&entry_name)
            {
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
        // 创建目录项
        items.push(TreeItem {
            item_type: EItemType::Folder,
            display_name: dir_name,
            link_name: None,
            is_open: false,
            fullpath: dir_path.to_string_lossy().to_string(),
            parent: Some(path.to_string_lossy().to_string()),
            children,
            data: None,
        });
    }

    // 处理 .gable 文件
    for (excel_name, sheets) in gable_files {
        if sheets.len() == 1 && sheets[0].1.is_empty() {
            // let gable_content: Option<GableData> =
            //     file_contents.get(&sheets[0].0).cloned().unwrap_or(None);
            // let sheet_type: ESheetType = utils::determine_sheet_type(Path::new(&sheets[0].0));
            // let tree_data: Option<TreeData> = gable_content.map(|content| TreeData {
            //     gable_type: sheet_type,
            //     content,
            // });
            // items.push(TreeItem {
            //     item_type: EItemType::Excel,
            //     display_name: excel_name.clone(),
            //     link_name: Some(excel_name),
            //     is_open: false,
            //     fullpath: sheets[0].0.clone(),
            //     parent: Some(path.to_string_lossy().to_string()),
            //     children: vec![],
            //     data: tree_data,
            // });
            log::warn!("不应该存在空sheet的excel{}", excel_name);
        } else {
            let excel_fullpath: String = format!("{}/{}", path.to_string_lossy(), excel_name);
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
                    file_name: sheet_name.clone(),
                    content,
                });

                if sheets_len == 1 && sheet_name.is_empty() {
                    excel_gable_content = tree_data.clone();
                }

                if !sheet_name.is_empty() {
                    children.push(TreeItem {
                        item_type: EItemType::Sheet,
                        display_name: sheet_name.clone(),
                        link_name: Some(format!("{}@{}", excel_name, &sheet_name)),
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
                display_name: excel_name.clone(),
                link_name: Some(excel_name),
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

pub fn find_item_by_path<'a>(items: &'a [TreeItem], path: &str) -> Option<&'a TreeItem> {
    for item in items {
        if item.fullpath == path {
            return Some(item);
        }

        if let Some(found) = find_item_by_path(&item.children, path) {
            return Some(found);
        }
    }
    None
}

// 根据路径查找树节点，当节点和item_type不匹配时，往父节点查找
pub fn find_item_clone(path: &str, item_type: EItemType) -> Option<TreeItem> {
    fn find_parent_item(path: &str, target_type: EItemType) -> Option<TreeItem> {
        // 先找到当前项
        let tree_items = TREE_ITEMS.read().unwrap();
        let item: &TreeItem = find_item_by_path(&tree_items, path)?;
        if item.item_type == target_type {
            return Some(item.clone());
        }

        // 如果类型不匹配，需要查找父节点
        let parent_path: String = item.parent.as_ref()?.clone();
        drop(tree_items); // 释放锁

        // 递归查找父节点
        find_parent_item(&parent_path, target_type)
    }

    let tree_items = TREE_ITEMS.read().unwrap();
    let item: &TreeItem = find_item_by_path(&tree_items, path)?;

    if item.item_type == item_type {
        Some(item.clone())
    } else {
        let parent_path: String = item.parent.as_ref()?.clone();
        drop(tree_items); // 释放锁
        find_parent_item(&parent_path, item_type)
    }
}

// 根据路径查找树节点
pub fn get_item_clone(path: &str) -> Option<TreeItem> {
    let tree_items = TREE_ITEMS.read().unwrap();
    fn get_item_by_path<'a>(items: &'a [TreeItem], path: &str) -> Option<&'a TreeItem> {
        for item in items {
            if item.fullpath == path {
                return Some(item);
            }

            if let Some(found) = get_item_by_path(&item.children, path) {
                return Some(found);
            }
        }
        None
    }

    get_item_by_path(&tree_items, path).cloned()
}

// 获取枚举数据
pub fn get_enum_cells<F, R>(link_name: &str, f: F) -> Option<R>
where
    F: FnOnce(&GableData) -> R,
{
    fn get_enum_cells_item<'a>(item: &'a TreeItem, link_name: &str) -> Option<&'a GableData> {
        // 检查当前项是否匹配link_name且类型为ENUM
        if let Some(ref item_link_name) = item.link_name {
            if *item_link_name == link_name {
                if let Some(ref tree_data) = item.data {
                    if tree_data.gable_type == ESheetType::Enum {
                        return Some(&tree_data.content);
                    }
                }
            }
        }

        // 递归检查子项
        for child in &item.children {
            if let Some(cells) = get_enum_cells_item(child, link_name) {
                return Some(cells);
            }
        }

        None
    }

    let tree_items = TREE_ITEMS.read().unwrap();
    for root_item in tree_items.iter() {
        if let Some(cells) = get_enum_cells_item(root_item, link_name) {
            return Some(f(cells));
        }
    }
    None
}

pub fn get_loc_cells<F, R>(link_name: &str, f: F) -> Option<R>
where
    F: FnOnce(&GableData) -> R,
{
    fn get_loc_cells_item<'a>(item: &'a TreeItem, link_name: &str) -> Option<&'a GableData> {
        if let Some(ref item_link_name) = item.link_name {
            if *item_link_name == link_name {
                if let Some(ref tree_data) = item.data {
                    if tree_data.gable_type == ESheetType::Localize {
                        return Some(&tree_data.content);
                    }
                }
            }
        }

        for child in &item.children {
            if let Some(cells) = get_loc_cells_item(child, link_name) {
                return Some(cells);
            }
        }

        None
    }

    let tree_items = TREE_ITEMS.read().unwrap();
    for root_item in tree_items.iter() {
        if let Some(cells) = get_loc_cells_item(root_item, link_name) {
            return Some(f(cells));
        }
    }
    None
}

/// 项目目录调整好重置数据
pub fn refresh_gables() {
    let root_path: &Path = &setting::get_workspace();
    let mut tree_items: Vec<TreeItem> = Vec::new();
    if root_path.exists() && root_path.is_dir() {
        let children: Vec<TreeItem> = build_tree_from_path(root_path);
        tree_items.extend(children);
    }
    *TREE_ITEMS.write().unwrap() = tree_items;
}
pub fn add_new_item(new_path: &Path, new_item: EItemType) -> bool {
    let mut tree_items = TREE_ITEMS.write().unwrap();
    if let Some(file_name) = new_path.file_name() {
        let file_name: String = file_name.to_string_lossy().to_string();
        let parent_path: String = match new_item {
            EItemType::Excel | EItemType::Folder => match new_path.parent() {
                Some(parent) => parent.to_string_lossy().to_string(),
                None => return false,
            },
            EItemType::Sheet => {
                if let Some(parent) = new_path.parent() {
                    let parent_dir: String = parent.to_string_lossy().to_string();
                    if let Some((excel_name, _)) = utils::parse_gable_filename(&file_name) {
                        format!("{}/{}", parent_dir, excel_name)
                    } else {
                        parent_dir
                    }
                } else {
                    return false;
                }
            }
        };
        let mut display_name: String = file_name.clone();
        let mut fullpath: String = new_path.to_string_lossy().to_string();
        let mut tree_data: Option<TreeData> = None;
        match new_item {
            EItemType::Excel => {
                if let Some((e_n, _)) = utils::parse_gable_filename(&file_name) {
                    display_name = e_n;
                    // 修改Excel项的fullpath，使其与build_tree_from_path中的一致
                    fullpath = format!("{}/{}", parent_path, display_name);
                };
            }
            EItemType::Sheet => {
                if let Some((_, s_n)) = utils::parse_gable_filename(&file_name) {
                    if let Some(s_n) = s_n {
                        display_name = s_n;
                    }
                };
                if let Some(gable_data) = excel_util::read_gable_file(&new_path.to_string_lossy()) {
                    let sheet_type: ESheetType = utils::determine_sheet_type(Path::new(&new_path));
                    tree_data = Some(TreeData {
                        gable_type: sheet_type,
                        file_name: display_name.clone(),
                        content: gable_data,
                    });
                }
            }
            _ => {}
        }
        let new_item: TreeItem = TreeItem {
            item_type: new_item,
            display_name,
            link_name: Some(file_name),
            is_open: true,
            fullpath,
            parent: Some(parent_path.clone()),
            children: vec![],
            data: tree_data,
        };
        if parent_path == setting::get_workspace().to_string_lossy() {
            tree_items.push(new_item);
            tree_items.sort_by(|a, b| match (&a.item_type, &b.item_type) {
                (EItemType::Folder, EItemType::Folder) => a.display_name.cmp(&b.display_name),
                (EItemType::Folder, _) => Ordering::Less,
                (_, EItemType::Folder) => Ordering::Greater,
                _ => a.display_name.cmp(&b.display_name),
            });
            true
        } else {
            if add_item_to_parent(&mut tree_items, new_item, &parent_path) {
                true
            } else {
                log::warn!("无法将新项添加到父项中: {}", parent_path);
                false
            }
        }
    } else {
        false
    }
}

fn add_item_to_parent(items: &mut [TreeItem], new_item: TreeItem, parent_path: &str) -> bool {
    for item in items.iter_mut() {
        if item.fullpath == parent_path {
            item.children.push(new_item);
            item.children
                .sort_by(|a, b| match (&a.item_type, &b.item_type) {
                    (EItemType::Folder, EItemType::Folder) => a.display_name.cmp(&b.display_name),
                    (EItemType::Folder, _) => Ordering::Less,
                    (_, EItemType::Folder) => Ordering::Greater,
                    _ => a.display_name.cmp(&b.display_name),
                });
            item.is_open = true;
            return true;
        }
        if add_item_to_parent(&mut item.children, new_item.clone(), parent_path) {
            item.is_open = true;
            return true;
        }
    }
    false
}

// 文件编辑完成时触发
pub fn editor_complete(excel_path: &str) -> bool {
    let (has, data) = has_eidtor_file(excel_path);
    if !has {
        return false;
    }
    let (result, gable_file_paths) = if let Some(data) = data {
        match excel_util::write_gable(excel_path, data.target_path, data.sheet_type) {
            Ok(gable_file_paths) => (true, Some(gable_file_paths)),
            Err(_) => (false, None),
        }
    } else {
        log::error!("无法获取文件 '{}' 的 sheet 类型", excel_path);
        (false, None)
    };
    if !result {
        return false;
    }

    let result = reload_gable(gable_file_paths);
    if !result {
        return false;
    }
    return true;
}

// 重新加载gable文件
fn reload_gable(gable_file_paths: Option<Vec<String>>) -> bool {
    let file_paths = match gable_file_paths {
        Some(paths) => paths,
        None => return true,
    };

    for file_path in file_paths {
        let new_data: Option<GableData> = excel_util::read_gable_file(&file_path);
        let mut tree_items = TREE_ITEMS.write().unwrap();
        fn update_child_data(
            items: &mut [TreeItem],
            file_path: &str,
            new_data: Option<GableData>,
        ) -> bool {
            for item in items.iter_mut() {
                if item.fullpath == file_path {
                    let path: &Path = Path::new(&file_path);
                    let gable_type = utils::determine_sheet_type(path);
                    let file_name = if let Some(file_name) = path.file_name() {
                        file_name.to_string_lossy().to_string()
                    } else {
                        log::error!("无法解析Excel文件名: {}", file_path);
                        return false;
                    };
                    let sheet_name: String;
                    if let Some((_, s_n)) = utils::parse_gable_filename(&file_name) {
                        if let Some(s_n) = s_n {
                            sheet_name = s_n;
                        } else {
                            log::error!("无法解析Sheet文件名: {}", file_name);
                            return false;
                        }
                    } else {
                        log::error!("无法解析Sheet文件名: {}", file_name);
                        return false;
                    };
                    item.data = new_data.map(|data: GableData| TreeData {
                        gable_type,
                        file_name: sheet_name,
                        content: data,
                    });
                    return true;
                }

                if update_child_data(&mut item.children, file_path, new_data.clone()) {
                    return true;
                }
            }
            false
        }

        update_child_data(&mut tree_items, &file_path, new_data);
    }

    true
}

pub fn update_item_display_name(fullpath: String, new_path: String, new_name: String) {
    let mut tree_items = TREE_ITEMS.write().unwrap();
    update_item_display_name_recursive(&mut tree_items, &fullpath, new_path, new_name);
}
fn update_item_display_name_recursive(
    items: &mut [TreeItem],
    target_fullpath: &str,
    new_path: String,
    new_name: String,
) -> bool {
    for item in items.iter_mut() {
        if item.fullpath == target_fullpath {
            item.fullpath = new_path;
            item.display_name = new_name.clone();
            return true;
        }
        if update_item_display_name_recursive(
            &mut item.children,
            target_fullpath,
            new_path.clone(),
            new_name.clone(),
        ) {
            return true;
        }
    }
    false
}

pub fn remove_item(fullpath: &str, item_type: &EItemType) -> bool {
    log::info!("删除条目: {:?} {:?}", item_type, fullpath);

    let result = match item_type {
        // 文件夹不需要处理
        EItemType::Folder => {
            log::info!("文件夹项目不需要删除操作: {}", fullpath);
            true
        }
        // 删除sheet文件
        EItemType::Sheet => {
            // 删除文件
            if let Err(e) = std::fs::remove_file(fullpath) {
                log::error!("删除sheet文件失败: {}", e);
                false
            } else {
                log::info!("成功删除sheet文件: {}", fullpath);
                true
            }
        }
        // 删除Excel及其相关文件
        EItemType::Excel => {
            let path = Path::new(fullpath);
            let parent_path = match path.parent() {
                Some(parent) => parent,
                None => {
                    log::error!("无法获取Excel文件的父目录: {}", fullpath);
                    return false;
                }
            };
            let excel_name = if let Some(file_name) = path.file_name() {
                file_name.to_string_lossy().to_string()
            } else {
                log::error!("无法解析Excel文件名: {}", fullpath);
                return false;
            };

            // 查找并删除所有相关文件
            let mut delete_success = true;
            if let Ok(entries) = std::fs::read_dir(parent_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let entry_name = entry.file_name().to_string_lossy().to_string();
                    // 检查是否为.gable文件且excel名称匹配
                    if let Some((parsed_excel_name, _)) = utils::parse_gable_filename(&entry_name) {
                        if parsed_excel_name == excel_name {
                            let entry_path = entry.path();
                            if let Err(e) = std::fs::remove_file(&entry_path) {
                                log::error!(
                                    "删除Excel相关文件失败: {} - {}",
                                    entry_path.display(),
                                    e
                                );
                                delete_success = false;
                            } else {
                                log::info!("成功删除Excel相关文件: {}", entry_path.display());
                            }
                        }
                    }
                }
            } else {
                log::error!("无法读取目录: {}", parent_path.display());
                return false;
            }

            delete_success
        }
    };

    result
}

/// 使用通道或其他机制在下一帧更新，避免锁冲突
pub fn request_remove_item_from_tree(fullpath: String) {
    // 使用线程来延迟执行删除操作，避免当前上下文中的锁冲突
    std::thread::spawn(move || {
        // 等待一小段时间，让当前操作完成
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 然后执行删除
        let mut tree_items = TREE_ITEMS.write().unwrap();
        remove_item_from_tree_recursive(&mut tree_items, &fullpath);
    });
}

/// 递归地从树结构中移除条目
fn remove_item_from_tree_recursive(items: &mut Vec<TreeItem>, fullpath: &str) -> bool {
    // 先尝试直接在当前层级找到并移除
    if let Some(pos) = items.iter().position(|item| item.fullpath == fullpath) {
        items.remove(pos);
        return true;
    }

    // 否则在子项中递归查找
    for item in items.iter_mut() {
        if remove_item_from_tree_recursive(&mut item.children, fullpath) {
            return true;
        }
    }

    false
}

/// 编辑gable文件
pub fn command_edit_gable(item: &TreeItem) {
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
            if let Some((parsed_excel_name, _)) = utils::parse_gable_filename(&entry_name) {
                if parsed_excel_name == excel_name {
                    related_files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    let sheet_type: ESheetType = {
        // 首先尝试从 item.data 获取
        if let Some(ref data) = item.data {
            data.gable_type.clone()
        } else {
            let mut found_type: Option<ESheetType> = None;
            for child in &item.children {
                if let Some(ref child_data) = child.data {
                    found_type = Some(child_data.gable_type.clone());
                    break;
                }
            }

            // 如果仍然没有找到，则使用默认值
            found_type.unwrap_or_else(|| {
                log::warn!(
                    "无法从 {} 或其子项中获取 sheet 类型，使用默认类型 DATA",
                    item.fullpath
                );
                ESheetType::Normal
            })
        }
    };
    match excel_util::write_excel(&excel_name, &sheet_type, related_files) {
        Ok(excel_file_path) => {
            add_editor_file(excel_file_path.clone(), parent_path, sheet_type);
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
