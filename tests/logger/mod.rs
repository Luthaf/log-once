use log::{LogRecord, LogLevelFilter, LogMetadata};
use std::sync::{Mutex, Once, ONCE_INIT};
use std::fmt::Write;

lazy_static!{
    static ref LOGGED_DATA: Mutex<String> = Mutex::new(String::new());
}

/// A simple in memory logger, using
pub struct MemoryLogger;

impl ::log::Log for MemoryLogger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        let mut buffer = LOGGED_DATA.lock().expect("Mutex has been poisonned");
        writeln!(*buffer, "{}", record.args()).expect("Error while writing");
    }
}

pub fn init() {
    static START: Once = ONCE_INIT;
    START.call_once(|| {
        ::log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Trace);
            Box::new(MemoryLogger)
        }).expect("Could not set the logger");
    });
}

pub fn logged_data() -> String {
    LOGGED_DATA.lock().expect("Mutex has been poisonned").clone()
}
