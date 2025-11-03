use crate::{common::utils, gui::datas::gables};
use eframe::egui::TextBuffer;
use notify::{
    Config, Error, Event, EventKind, ReadDirectoryChangesWatcher, RecommendedWatcher,
    RecursiveMode, Result, Watcher,
};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread,
    time::Duration,
};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: Arc<Mutex<Receiver<Result<Event>>>>,
    watched_path: Option<PathBuf>,
    shutdown_sender: Option<Sender<()>>,
    worker_thread: Option<thread::JoinHandle<()>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();
        let watcher: ReadDirectoryChangesWatcher = RecommendedWatcher::new(tx, Config::default())?;
        Ok(FileWatcher {
            watcher,
            rx: Arc::new(Mutex::new(rx)),
            watched_path: None,
            shutdown_sender: None,
            worker_thread: None,
        })
    }

    pub fn watch_temp_directory(&mut self, path: PathBuf) -> Result<()> {
        if let Some(old_path) = &self.watched_path {
            log::info!(
                "Stop monitoring old directory: {}",
                &old_path.to_string_lossy().to_string()
            );
            self.watcher.unwatch(old_path)?;
        }

        log::info!(
            "Start monitoring directory: {}",
            &path.to_string_lossy().to_string()
        );
        self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
        self.watched_path = Some(path);
        Ok(())
    }

    pub fn start_watching(&mut self) {
        let rx: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>> = self.rx.clone();
        let (shutdown_tx, shutdown_rx) = channel::<()>();
        self.shutdown_sender = Some(shutdown_tx);

        let handle = thread::spawn(move || {
            let _rx_holder: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>> = rx.clone();

            loop {
                if shutdown_rx.try_recv().is_ok() {
                    log::info!("Received shutdown signal, exiting file watcher thread");
                    break;
                }

                // 使用简单的接收方式，带有超时以允许定期检查关闭信号
                let event_result: Option<std::result::Result<Event, Error>> = {
                    match rx.lock() {
                        Ok(receiver) => match receiver.recv_timeout(Duration::from_millis(100)) {
                            Ok(event) => Some(event),
                            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
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
                        break;
                    }
                }
            }

            log::info!("The file monitoring thread has exited.");
        });

        self.worker_thread = Some(handle);
    }

    pub fn end_watching(&mut self) {
        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
        }

        if let Some(handle) = self.worker_thread.take() {
            let _ = handle.join();
        }

        if let Some(path) = &self.watched_path {
            log::info!(
                "Stop monitoring directory: {}",
                &path.to_string_lossy().to_string()
            );
            let _ = self.watcher.unwatch(path);
        }
        self.watched_path = None;
    }
}
