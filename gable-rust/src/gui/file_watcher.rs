use eframe::egui::TextBuffer;
use notify::{Config, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, channel},
    },
    thread,
    time::Duration,
};

use crate::{common::utils, gui::datas::gables};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: Arc<Mutex<Receiver<Result<Event>>>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();

        // 创建推荐的文件监控器
        let watcher = RecommendedWatcher::new(tx, Config::default())?;
        Ok(FileWatcher {
            watcher,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    pub fn watch_temp_directory(&mut self, path: PathBuf) -> Result<()> {
        log::info!(
            "Start monitoring directory: {}",
            &path.to_string_lossy().to_string()
        );
        // 监控临时目录，非递归模式
        self.watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(())
    }

    pub fn start_watching(&self) {
        let rx: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>> = self.rx.clone();
        thread::spawn(move || {
            // 持有rx的克隆，确保在线程运行期间通道不会被关闭
            let _rx_holder: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>> = rx.clone();

            loop {
                // 使用简单的接收方式
                let event_result: Option<std::result::Result<Event, Error>> = {
                    match rx.lock() {
                        Ok(receiver) => match receiver.recv() {
                            Ok(event) => Some(event),
                            Err(e) => {
                                log::error!("Receiving event error: {:?}", e);
                                None
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to acquire the receiver lock: {:?}", e);
                            None
                        }
                    }
                };

                match event_result {
                    Some(event_result) => {
                        match event_result {
                            Ok(event) => {
                                let mut excel_files: Vec<&std::path::PathBuf> = Vec::new();
                                for path in &event.paths {
                                    if let Some(file_name) =
                                        path.file_name().and_then(|f| f.to_str())
                                    {
                                        if file_name.ends_with(".xlsx") {
                                            excel_files.push(path);
                                        }
                                    }
                                }

                                // 只有当有.xlsx文件时才处理事件
                                if !excel_files.is_empty() {
                                    match event.kind {
                                        EventKind::Modify(_) => {
                                            for file_path in &excel_files {
                                                if let Some(file_name) =
                                                    file_path.file_name().and_then(|f| f.to_str())
                                                {
                                                    if !utils::is_temp_file(file_name) {
                                                        gables::editor_complete(file_path);
                                                    }
                                                }
                                            }
                                        }
                                        EventKind::Remove(_) => {
                                            for file_path in &excel_files {
                                                if let Some(file_name) =
                                                    file_path.file_name().and_then(|f| f.to_str())
                                                {
                                                    if utils::is_temp_file(file_name) {
                                                        let normalized_path =
                                                            file_path.to_string_lossy();
                                                        let original_file_name: String =
                                                            utils::temp_to_formal(file_name);
                                                        // 构造原始文件的路径
                                                        if let Some(parent_path) =
                                                            Path::new(normalized_path.as_str())
                                                                .parent()
                                                        {
                                                            let original_file_path =
                                                                PathBuf::from(parent_path)
                                                                    .join(original_file_name);
                                                            // 检查原始文件是否存在
                                                            if original_file_path.exists() {
                                                                let path_str = original_file_path
                                                                    .to_string_lossy();
                                                                gables::editor_complete(
                                                                    &original_file_path,
                                                                );
                                                                gables::remove_editor_file(
                                                                    path_str.as_str(),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("File monitoring error: {:?}", e);
                            }
                        }
                    }
                    None => {
                        // 接收失败，退出循环
                        break;
                    }
                }

                // 添加小延迟以避免过度占用CPU
                thread::sleep(Duration::from_millis(10));
            }

            log::info!("The file monitoring thread has exited.");
        });
    }
}
