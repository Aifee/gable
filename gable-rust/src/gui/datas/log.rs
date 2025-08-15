use chrono::Local;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use log::LevelFilter;
use once_cell::sync::OnceCell;
use std::fmt::Arguments;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

// 添加静态变量来存储全局LogRecord列表
static GLOBAL_LOG_RECORDS: OnceCell<Arc<Mutex<Vec<LogRecord>>>> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct LogRecord {
    pub timestamp: String,
    pub level: log::Level,
    pub target: String,
    pub args: String,
}

pub struct LogTrace {
    file: Mutex<Option<File>>,
    level: LevelFilter,
}
impl Clone for LogTrace {
    fn clone(&self) -> Self {
        LogTrace {
            file: Mutex::new(None), // 不克隆文件句柄
            level: self.level,
        }
    }
}

impl LogTrace {
    pub fn new(log_dir_path: Option<&str>) -> Result<LogTrace, io::Error> {
        let file: Option<File> = match log_dir_path {
            Some(dir_path) => {
                fs::create_dir_all(dir_path)?;
                let day_file_name: DelayedFormat<StrftimeItems<'_>> =
                    Local::now().format("%Y-%m-%d");
                let file_path: PathBuf = Path::new(dir_path).join(format!("{}.log", day_file_name));
                let f = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)?;
                Some(f)
            }
            None => None,
        };
        if GLOBAL_LOG_RECORDS.get().is_none() {
            let _ = GLOBAL_LOG_RECORDS.set(Arc::new(Mutex::new(Vec::new())));
        }
        Ok(LogTrace {
            file: Mutex::new(file),
            level: LevelFilter::Trace,
        })
    }

    pub fn init(log_dir_path: Option<&str>, level: LevelFilter) -> Result<(), log::SetLoggerError> {
        match LogTrace::new(log_dir_path) {
            Ok(logger) => {
                // 设置全局实例
                log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level))
            }
            Err(_e) => {
                // 确保全局日志记录列表已初始化
                if GLOBAL_LOG_RECORDS.get().is_none() {
                    let _ = GLOBAL_LOG_RECORDS.set(Arc::new(Mutex::new(Vec::new())));
                }
                let logger = LogTrace {
                    file: Mutex::new(None),
                    level: level,
                };
                // 设置全局实例
                log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level))
            }
        }
    }

    // 获取全局日志记录列表的方法
    pub fn get_log_records() -> Option<&'static Arc<Mutex<Vec<LogRecord>>>> {
        GLOBAL_LOG_RECORDS.get()
    }
}

impl log::Log for LogTrace {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let timestamp: DelayedFormat<StrftimeItems<'_>> =
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let level: log::Level = record.level();
            let target: &str = record.target();
            let args: &Arguments<'_> = record.args();

            let log_message: String = format!("{} [{}] {} - {}\n", timestamp, level, target, args);

            print!("{}", log_message);

            // 更新全局日志记录列表
            if let Some(global_records) = LogTrace::get_log_records() {
                if let Ok(mut records) = global_records.lock() {
                    records.push(LogRecord {
                        timestamp: timestamp.to_string(),
                        level,
                        target: target.to_string(),
                        args: args.to_string(),
                    });

                    // 可以限制存储的日志数量，避免内存无限增长
                    if records.len() > 1000 {
                        records.drain(0..100);
                    }
                }
            }

            if let Ok(mut file_guard) = self.file.lock() {
                if let Some(ref mut file) = *file_guard {
                    let _ = file.write_all(log_message.as_bytes());
                    let _ = file.flush();
                }
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file_guard) = self.file.lock() {
            if let Some(ref mut file) = *file_guard {
                let _ = file.flush();
            }
        }
        let _ = io::stdout().flush();
    }
}
