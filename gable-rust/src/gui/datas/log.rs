use chrono::Local;
use log::LevelFilter;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::Mutex;

pub struct GableLog {
    file: Mutex<Option<File>>,
    level: LevelFilter,
}

impl GableLog {
    pub fn new(log_dir_path: Option<&str>) -> Result<GableLog, io::Error> {
        let file = match log_dir_path {
            Some(dir_path) => {
                std::fs::create_dir_all(dir_path)?;
                let day_file_name = Local::now().format("%Y-%m-%d");
                let file_path =
                    std::path::Path::new(dir_path).join(format!("{}.log", day_file_name));
                let f = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)?;
                Some(f)
            }
            None => None,
        };

        Ok(GableLog {
            file: Mutex::new(file),
            level: LevelFilter::Trace,
        })
    }

    pub fn init(log_dir_path: Option<&str>, level: LevelFilter) -> Result<(), log::SetLoggerError> {
        match GableLog::new(log_dir_path) {
            Ok(logger) => {
                log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level))
            }
            Err(_e) => {
                let logger = GableLog {
                    file: Mutex::new(None),
                    level: level,
                };
                log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level))
            }
        }
    }
}

impl log::Log for GableLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let level = record.level();
            let target = record.target();
            let args = record.args();

            let log_message = format!("{} [{}] {} - {}\n", timestamp, level, target, args);

            print!("{}", log_message);

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
        let _ = std::io::stdout().flush();
    }
}
