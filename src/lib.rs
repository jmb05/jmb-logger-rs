use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use ansi_term::{Color, Style};
use crate::Level::{*};
use chrono;

#[macro_export]
macro_rules! trace {
    ($logger:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), Level::TRACE);
    }};
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), Level::DEBUG);
    }};
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), Level::INFO);
    }};
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), Level::WARN);
    }};
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), Level::ERROR);
    }};
}

#[macro_export]
macro_rules! fatal {
    ($logger:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), Level::FATAL);
    }};
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $level:expr, $($arg:tt)*) => {{
        ($logger as &Logger).log(&format!($($arg)*), ($level as Level));
    }};
}

#[derive(Clone, Copy, Debug)]
pub enum Level {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL
}

impl Level {
    pub fn idx(&self) -> i32 {
        match self {
            TRACE => 0,
            DEBUG => 1,
            INFO => 2,
            WARN => 3,
            ERROR => 4,
            FATAL => 5
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TRACE => "TRACE",
            DEBUG => "DEBUG",
            INFO => "INFO",
            WARN => "WARN",
            ERROR => "ERROR",
            FATAL => "FATAL"
        }
    }

    pub fn color_code(&self) -> Style {
        match self {
            TRACE => Color::Blue.normal(),
            DEBUG => Color::Green.normal(),
            INFO => Style::new().bold(),
            WARN => Color::Yellow.normal(),
            ERROR => Color::Red.normal(),
            FATAL => Color::Red.bold()
        }
    }
}

pub struct Logger {
    current_level: Level,
    log_file: OnceLock<Mutex<File>>,
    prefix: Option<String>,
}

impl Logger {
    pub const fn create(level: Level) -> Logger {
        Logger { current_level: level, log_file: OnceLock::new(), prefix: None }
    }
    pub const fn create_with_custom_format(level: Level, fmt: String) -> Logger {
        Logger { current_level: level, log_file: OnceLock::new(), prefix: Some(fmt) }
    }

    pub fn create_with_logfile(level: Level, filename: &str) -> Logger {
        let mut logger = Logger { current_level: level, log_file: OnceLock::new(), prefix: None };
        logger.create_log_file(filename);
        logger
    }

    pub fn create_timestamped_logfile(&self, name: &str) {
        self.create_log_file(&format!("{name}-{}.log", chrono::offset::Local::now().format("%Y-%m-%d_%H-%M").to_string()));
    }

    pub fn create_log_file(&self, filename: &str) {
        self.log_file.get_or_init(|| {
            let Ok(file) = File::create(Path::new(filename)) else {
                error!(&self, "Error creating logfile: {filename}");
                panic!("Error creating logfile: {filename}")
            };
            Mutex::new(file)
        });
    }

    pub fn set_level(&mut self, level: Level) {
        self.current_level = level;
    }

    pub fn log(&self, message: &str, level: Level) {
        if level.idx() >= self.current_level.idx() {
            let prefix_format = if self.prefix.is_none() {
                "[{time}] [{level}]"
            } else {
                self.prefix.as_ref().unwrap()
            };
            let log_prefix = prefix_format
                .replace("{time}", chrono::offset::Local::now().format("%d.%m.%Y %H:%M").to_string().as_str())
                .replace("{level}", level.name());
            let log_out = format!("{} {}", log_prefix, message);
            if let Some(file) = self.log_file.get() {
                let file_lock = file.lock();
                if file_lock.is_err() {
                    error!(&self, "Couldn't get lock on log file");
                } else {
                    let res = file_lock.unwrap().write_all((log_out.clone() + "\n").as_bytes());
                    if res.is_err() {
                        error!(&self, "Could not write to logfile");
                    }
                }
            }
            println!("{}", level.color_code().paint(log_out));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
        let mut logger = Logger::create(TRACE);
        logger.create_log_file("log/test.log");
        trace!(&logger, "Hello {}", "world");
        debug!(&logger, "Hello {}", "world");
        info!(&logger, "Hello {}", "world");
        warn!(&logger, "Hello {}", "world");
        error!(&logger, "Hello {}", "world");
        fatal!(&logger, "Hello {}", "world");
    }

    #[test]
    fn test_log_file() {
        let logger = Logger::create_with_logfile(TRACE, "test.log");
        assert!(logger.log_file.get().is_some());
        info!(&logger, "Test");
    }

    #[test]
    fn test_timestamped_log_file() {
        let logger = Logger::create(TRACE);
        logger.create_timestamped_logfile("log/test");
        assert!(logger.log_file.get().is_some());
        info!(&logger, "Test");
    }

    #[test]
    fn test_custom_prefix() {
        let logger = Logger::create_with_custom_format(TRACE, "[{level}]".to_string());
        info!(&logger, "Hello logger my old friend");
    }
}
