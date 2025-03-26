use std::fs::File;
use std::io::Write;
use std::path::Path;
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
    log_file: Option<File>
}

impl Logger {
    pub const fn create(level: Level) -> Logger {
        Logger { current_level: level, log_file: None }
    }

    pub fn create_with_logfile(level: Level, filename: &str) -> Logger {
        let mut logger = Logger { current_level: level, log_file: None };
        logger.create_log_file(filename);
        logger
    }

    pub fn create_log_file(&mut self, filename: &str) {
        self.log_file = match File::create(Path::new(filename)) {
            Ok(file) => Some(file),
            Err(err) => {
                error!(&self, "Could not create logfile: {} -> {}", filename, err);
                None
            },
        };
    }

    pub fn set_level(&mut self, level: Level) {
        self.current_level = level;
    }

    pub fn log(&self, message: &str, level: Level) {
        if level.idx() >= self.current_level.idx() {
            let log_out = format!("[{}] [{}] {}", chrono::offset::Local::now().format("%d.%m.%Y %H:%M").to_string(), level.name(), message);
            if self.log_file.is_some() {
                let res = self.log_file.as_ref().unwrap().write_all((log_out.clone() + "\n").as_bytes());
                if res.is_err() {
                    error!(&self, "Could not write to logfile");
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
        assert!(logger.log_file.is_some());
        info!(&logger, "Test");
    }
}
