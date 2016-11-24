#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub use log::LogLevel;

#[macro_export]
macro_rules! log_once {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        use ::std::collections::BTreeSet;
        use ::std::sync::Mutex;
        lazy_static!{
            static ref __SEEN_MESSAGES: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
        }
        let mut seen_messages = __SEEN_MESSAGES.lock().expect("Mutex was poisonned");
        let message = String::from(stringify!($target)) + stringify!($lvl) + &format!($($arg)+);
        if !seen_messages.contains(&message) {
            seen_messages.insert(message);
            log!(target: $target, $lvl, $($arg)+);
        }
    });
    ($lvl:expr, $($arg:tt)+) => (log_once!(target: module_path!(), $lvl, $($arg)+))
}

#[macro_export]
macro_rules! error_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Error, $($arg)*);
    )
}

#[macro_export]
macro_rules! warn_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Warn, $($arg)*);
    )
}

#[macro_export]
macro_rules! info_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Info, $($arg)*);
    )
}

#[macro_export]
macro_rules! debug_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Debug, $($arg)*);
    )
}

#[macro_export]
macro_rules! trace_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Trace, $($arg)*);
    )
}
