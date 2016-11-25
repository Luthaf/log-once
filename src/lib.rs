//! Helper macros for logging some events only once.
//!
//! # Examples
//!
//! ```rust, no_run
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate log_once;
//!
//! # #[derive(Debug)] pub struct Yak(String);
//! # impl Yak { fn shave(&self, _: u32) {} }
//! # fn find_a_razor() -> Result<u32, u32> { Ok(1) }
//! pub fn shave_the_yak(yaks: &[Yak]) {
//!     info!(target: "yak_events", "Commencing yak shaving for {:?}", yak);
//!
//!     for yak in yaks {
//!         loop {
//!             match find_a_razor() {
//!                 Ok(razor) => {
//!                     // This will only appear once in the logger output for each razor
//!                     info_once!("Razor located: {}", razor);
//!                     yak.shave(razor);
//!                     break;
//!                 }
//!                 Err(err) => {
//!                     // This will only appear once in the logger output for each error
//!                     warn_once!("Unable to locate a razor: {}, retrying", err);
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! # fn main() {}
//! ```

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub use log::LogLevel;

/// Standard logging macro, logging events once for each arguments.
///
/// The log event will only be emmited once for each combinaison of target/arguments.
///
/// This macro will generically log with the specified `LogLevel` and `format!`
/// based argument list.
///
/// The `max_level_*` features can be used to statically disable logging at
/// various levels.
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

/// Logs a message once at the error level.
///
/// The log event will only be emmited once for each combinaison of target/arguments.
///
/// Logging at this level is disabled if the `max_level_off` feature is present.
#[macro_export]
macro_rules! error_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Error, $($arg)*);
    )
}

/// Logs a message once at the warn level.
///
/// The log event will only be emmited once for each combinaison of target/arguments.
///
/// Logging at this level is disabled if any of the following features are
/// present: `max_level_off` or `max_level_error`.
///
/// When building in release mode (i.e., without the `debug_assertions` option),
/// logging at this level is also disabled if any of the following features are
/// present: `release_max_level_off` or `max_level_error`.
#[macro_export]
macro_rules! warn_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Warn, $($arg)*);
    )
}

/// Logs a message once at the info level.
///
/// The log event will only be emmited once for each combinaison of target/arguments.
///
/// Logging at this level is disabled if any of the following features are
/// present: `max_level_off`, `max_level_error`, or `max_level_warn`.
///
/// When building in release mode (i.e., without the `debug_assertions` option),
/// logging at this level is also disabled if any of the following features are
/// present: `release_max_level_off`, `release_max_level_error`, or
/// `release_max_level_warn`.
#[macro_export]
macro_rules! info_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Info, $($arg)*);
    )
}

/// Logs a message once at the debug level.
///
/// The log event will only be emmited once for each combinaison of target/arguments.
///
/// Logging at this level is disabled if any of the following features are
/// present: `max_level_off`, `max_level_error`, `max_level_warn`, or
/// `max_level_info`.
///
/// When building in release mode (i.e., without the `debug_assertions` option),
/// logging at this level is also disabled if any of the following features are
/// present: `release_max_level_off`, `release_max_level_error`,
/// `release_max_level_warn`, or `release_max_level_info`.
#[macro_export]
macro_rules! debug_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Debug, $($arg)*);
    )
}

/// Logs a message once at the trace level.
///
/// The log event will only be emmited once for each combinaison of target/arguments.
///
/// Logging at this level is disabled if any of the following features are
/// present: `max_level_off`, `max_level_error`, `max_level_warn`,
/// `max_level_info`, or `max_level_debug`.
///
/// When building in release mode (i.e., without the `debug_assertions` option),
/// logging at this level is also disabled if any of the following features are
/// present: `release_max_level_off`, `release_max_level_error`,
/// `release_max_level_warn`, `release_max_level_info`, or
/// `release_max_level_debug`.
#[macro_export]
macro_rules! trace_once {
    (target: $target:expr, $($arg:tt)*) => (
        log_once!(target: $target, $crate::LogLevel::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        log_once!($crate::LogLevel::Trace, $($arg)*);
    )
}
