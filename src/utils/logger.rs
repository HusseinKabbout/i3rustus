// Inspired by from https://docs.rs/log/latest/log/
use log::{Record, Level, Metadata, Log};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl Logger {

    pub fn info(&self, log_text: &str, filename: &str) {
        self.log(&Record::builder()
                .args(format_args!("{}", log_text))
                .level(Level::Info)
                .target("i3rustus")
                .file(Some(filename))
                .line(Some(144))
                .build());
    }

    pub fn warning(&self, log_text: &str, filename: &str) {
        self.log(&Record::builder()
                .args(format_args!("{}", log_text))
                .level(Level::Warn)
                .target("i3rustus")
                .file(Some(filename))
                .line(Some(144))
                .build());
    }

    pub fn error(&self, log_text: &str, filename: &str) {
        self.log(&Record::builder()
                .args(format_args!("{}", log_text))
                .level(Level::Error)
                .target("i3rustus")
                .file(Some(filename))
                .line(Some(144))
                .build());
    }
}
