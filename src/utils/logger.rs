// Inspired by from https://docs.rs/log/latest/log/
use log::{ Record, Level, Metadata, Log };

pub struct Logger<'a> {
    pub log_file: &'a str
}

impl Log for Logger<'_> {
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

impl Logger<'_> {

    pub fn info(&self, log_text: String) {
        self.log(&Record::builder()
                .args(format_args!("{}", log_text))
                .level(Level::Info)
                .target("i3rustus")
                .file(Some(self.log_file))
                .line(Some(144))
                .build());
    }

    pub fn warning(&self, log_text: String) {
        self.log(&Record::builder()
                .args(format_args!("{}", log_text))
                .level(Level::Warn)
                .target("i3rustus")
                .file(Some(self.log_file))
                .line(Some(144))
                .build());
    }

    pub fn error(&self, log_text: String) {
        self.log(&Record::builder()
                .args(format_args!("{}", log_text))
                .level(Level::Error)
                .target("i3rustus")
                .file(Some(self.log_file))
                .line(Some(144))
                .build());
    }
}
