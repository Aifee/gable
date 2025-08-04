use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
// 添加时间统计相关引入
use std::time::Instant;

use crate::common::global;
use crate::common::setting;
// 添加 rayon 的引入
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Folder,
    Excel,
    Sheet,
}

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub item_type: ItemType,
    pub display_name: String,
    pub is_open: bool,
    pub fullpath: String,
    // file_name: String,
    // parent: TreeItem,
    pub children: Vec<TreeItem>,
    /// 存储Sheet类型节点的gable文件内容
    pub gable_content: Option<Value>,
}

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

    let name_without_ext = &filename[..filename.len() - global::GABLE_FILE_TYPE.len()];

    if let Some(pos) = name_without_ext.find('@') {
        // 格式为 excelname@sheetname
        let excel_name = name_without_ext[..pos].to_string();
        let sheet_name = name_without_ext[pos + 1..].to_string();
        Some((excel_name, Some(sheet_name)))
    } else {
        // 格式为 excelname
        Some((name_without_ext.to_string(), None))
    }
}

/// 读取并解析gable文件
fn read_gable_file(file_path: &str) -> Option<Value> {
    match fs::read_to_string(file_path) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json_value) => Some(json_value),
            Err(e) => {
                eprintln!("解析JSON文件失败 '{}': {}", file_path, e);
                None
            }
        },
        Err(e) => {
            eprintln!("读取文件失败 '{}': {}", file_path, e);
            None
        }
    }
}

/// 并行读取所有gable文件
fn read_all_gable_files_parallel(
    gable_files: &HashMap<String, Vec<(String, String)>>,
) -> HashMap<String, Option<Value>> {
    // 收集所有文件路径
    let file_paths: Vec<String> = gable_files
        .values()
        .flat_map(|sheets| sheets.iter().map(|(path, _)| path.clone()))
        .collect();

    // 使用rayon并行处理所有文件读取
    file_paths
        .into_par_iter()
        .map(|file_path| {
            let content = read_gable_file(&file_path);
            (file_path, content)
        })
        .collect()
}

/// 递归构建目录树
fn build_tree_from_path(path: &Path) -> Vec<TreeItem> {
    let mut items = Vec::new();

    if !path.exists() || !path.is_dir() {
        return items;
    }

    // 收集目录项和文件项
    let mut directories = Vec::new();
    let mut gable_files: HashMap<String, Vec<(String, String)>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let entry_name = entry.file_name().to_string_lossy().to_string();

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
    let file_contents = read_all_gable_files_parallel(&gable_files);

    // 处理目录
    for (dir_path, dir_name) in directories {
        let children = build_tree_from_path(&dir_path);

        // 检查此路径是否应该展开
        let should_be_expanded = {
            let expanded_folders = EXPANDED_FOLDERS.lock().unwrap();
            expanded_folders.contains(&dir_path.to_string_lossy().to_string())
        };

        // 创建目录项
        items.push(TreeItem {
            item_type: ItemType::Folder,
            display_name: dir_name,
            is_open: should_be_expanded,
            fullpath: dir_path.to_string_lossy().to_string(),
            children,
            gable_content: None, // 目录节点没有内容
        });
    }

    // 处理 .gable 文件
    for (excel_name, sheets) in gable_files {
        if sheets.len() == 1 && sheets[0].1.is_empty() {
            // 读取文件内容
            let gable_content = file_contents.get(&sheets[0].0).cloned().unwrap_or(None);

            items.push(TreeItem {
                item_type: ItemType::Excel,
                display_name: excel_name,
                is_open: false,
                fullpath: sheets[0].0.clone(),
                children: vec![],
                gable_content, // 存储文件内容
            });
        } else {
            // 有多个 sheet 或有 sheet 部分
            let excel_fullpath = format!("{}/{}", path.to_string_lossy(), excel_name);

            // 创建子项
            let mut children = Vec::new();
            for (full_path, sheet_name) in sheets {
                // 读取每个sheet文件的内容
                let gable_content = file_contents.get(&full_path).cloned().unwrap_or(None);

                if !sheet_name.is_empty() {
                    children.push(TreeItem {
                        item_type: ItemType::Sheet,
                        display_name: sheet_name,
                        is_open: false,
                        fullpath: full_path.clone(),
                        children: vec![],
                        gable_content, // 存储文件内容
                    });
                } else {
                    // 没有 sheet 部分的文件作为默认 sheet
                    children.push(TreeItem {
                        item_type: ItemType::Sheet,
                        display_name: "默认".to_string(), // 或者使用其他默认名称
                        is_open: false,
                        fullpath: full_path.clone(),
                        children: vec![],
                        gable_content, // 存储文件内容
                    });
                }
            }

            // 对子项进行排序
            children.sort_by(|a, b| a.display_name.cmp(&b.display_name));

            items.push(TreeItem {
                item_type: ItemType::Excel,
                display_name: excel_name,
                is_open: false,
                fullpath: excel_fullpath,
                children,
                gable_content: None, // Excel节点本身没有内容，内容在子节点中
            });
        }
    }

    // 对所有项进行排序，文件夹在前
    items.sort_by(|a, b| match (&a.item_type, &b.item_type) {
        (ItemType::Folder, ItemType::Folder) => a.display_name.cmp(&b.display_name),
        (ItemType::Folder, _) => std::cmp::Ordering::Less,
        (_, ItemType::Folder) => std::cmp::Ordering::Greater,
        _ => a.display_name.cmp(&b.display_name),
    });

    items
}

/// 项目目录调整好重置数据
pub fn refresh_gables() {
    let workspace = setting::WORKSPACE.lock().unwrap();
    let root_path = if let Some(path) = workspace.as_ref() {
        Path::new(path)
    } else {
        // 如果没有设置工作空间，使用当前目录
        Path::new(".")
    };

    let mut tree_items = Vec::new();

    if root_path.exists() && root_path.is_dir() {
        let root_name = root_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "项目".to_string());

        let children = build_tree_from_path(root_path);

        // 检查根路径是否应该展开
        let should_be_expanded = {
            let expanded_folders = EXPANDED_FOLDERS.lock().unwrap();
            expanded_folders.contains(&root_path.to_string_lossy().to_string())
        };

        // 创建根节点
        let root_item = TreeItem {
            item_type: ItemType::Folder,
            display_name: root_name,
            is_open: should_be_expanded || true, // 根节点始终展开或根据记录展开
            fullpath: root_path.to_string_lossy().to_string(),
            children,
            gable_content: None, // 根节点没有内容
        };

        tree_items.push(root_item);
    }

    // 使用 lock 安全更新 TREE_ITEMS
    *TREE_ITEMS.lock().unwrap() = tree_items;
}
