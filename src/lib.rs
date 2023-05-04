use std::fs::File;
use std::io::Write;
use std::path::Path;
use ansi_term::{Color, Style};
use crate::Level::{*};
use chrono;

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
            INFO => Style::new(),
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

    pub fn create_log_file(&mut self, filename: &str) {
        self.log_file = match File::create(Path::new(filename)) {
            Ok(file) => Some(file),
            Err(err) => {
                self.error(&format!("Could not create logfile: {} -> {}", filename, err));
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
                    self.error("Could not write to logfile");
                }
            }
            println!("{}", level.color_code().paint(log_out));
        }
    }

    pub fn trace(&self, message: &str) {
        self.log(message, TRACE);
    }

    pub fn debug(&self, message: &str) {
        self.log(message, DEBUG);
    }

    pub fn info(&self, message: &str) {
        self.log(message, INFO);
    }

    pub fn warn(&self, message: &str) {
        self.log(message, WARN);
    }

    pub fn error(&self, message: &str) {
        self.log(message, ERROR);
    }

    pub fn fatal(&self, message: &str) {
        self.log(message, FATAL);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
        let mut logger = Logger::create(TRACE);
        logger.create_log_file("log/test.log");
        logger.trace("Hi");
        logger.info("Hi");
        logger.warn("Hi");
        logger.error("Hi");
        logger.fatal("Hi");
    }
}
