#![warn(clippy, clippy_pedantic)]
#![allow(unknown_lints)]
#![allow(
    new_without_default, new_without_default_derive, useless_attribute,
    missing_docs_in_private_items
)]

//! Collection of helper macros for logging some events only once.
//!
//! This crate provide macro in the `log_once` family (`warn_once!`,
//! `trace_once!`, ...); that only send a logging event once for every message.
//! It rely and uses the logging infrastructure in the [log][log] crate; and
//! is fully compatible with any logger implementation.
//!
//! These macro will store the already seen messages in a `BTreeSet`, and check
//! if a message is in the set before sending the log event.
//!
//! [log]: https://crates.io/crates/log
//!
//! # Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate log_once;
//!
//! # #[derive(Debug)] pub struct Yak(String);
//! # impl Yak { fn shave(&self, _: u32) {} }
//! # fn find_a_razor() -> Result<u32, u32> { Ok(1) }
//! pub fn shave_the_yak(yaks: &[Yak]) {
//!     for yak in yaks {
//!         info!(target: "yak_events", "Commencing yak shaving for {:?}", yak);
//!
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

#[allow(unused_imports)]
#[macro_use]
extern crate log;
pub use log::LogLevel;

use std::collections::BTreeSet;
use std::sync::{Mutex, MutexGuard, PoisonError};

#[doc(hidden)]
pub struct __MessagesSet {
    inner: Mutex<BTreeSet<String>>
}

impl __MessagesSet {
    pub fn new() -> __MessagesSet {
        __MessagesSet {
            inner: Mutex::new(BTreeSet::new())
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<BTreeSet<String>>, PoisonError<MutexGuard<BTreeSet<String>>>> {
        self.inner.lock()
    }
}

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
    (@CREATE STATIC) => ({
        use ::std::sync::{Once, ONCE_INIT};
        static mut __SEEN_MESSAGES: *const $crate::__MessagesSet = 0 as *const _;
        static ONCE: Once = ONCE_INIT;
        unsafe {
            ONCE.call_once(|| {
                let singleton = $crate::__MessagesSet::new();
                __SEEN_MESSAGES = ::std::mem::transmute(Box::new(singleton));
            });
            &(*__SEEN_MESSAGES)
        }
    });
    (target: $target:expr, $lvl:expr, $message:expr) => ({
        #[allow(non_snake_case)]
        let __SEEN_MESSAGES = log_once!(@CREATE STATIC);
        let mut seen_messages = __SEEN_MESSAGES.lock().expect("Mutex was poisonned");
        let event = String::from(stringify!($target)) + stringify!($lvl) + $message.as_ref();
        if !seen_messages.contains(&event) {
            seen_messages.insert(event);
            log!(target: $target, $lvl, "{}", $message);
        }
    });
    (target: $target:expr, $lvl:expr, $format:expr, $($arg:tt)+) => ({
        let message = format!($format, $($arg)+);
        log_once!(target: $target, $lvl, message);
    });
    ($lvl:expr, $message:expr) => (log_once!(target: module_path!(), $lvl, $message));
    ($lvl:expr, $format:expr, $($arg:tt)+) => (log_once!(target: module_path!(), $lvl, $format, $($arg)+));
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

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::sync::{Once, ONCE_INIT};
    use log::{Log, LogRecord, LogMetadata, LogLevelFilter};

    struct SimpleLogger;
    impl Log for SimpleLogger {
        fn enabled(&self, _: &LogMetadata) -> bool {true}
        fn log(&self, record: &LogRecord) {
            println!("{}", record.args());
        }
    }

    #[test]
    fn called_once() {
        static START: Once = ONCE_INIT;
        START.call_once(|| {
            ::log::set_logger(|max_log_level| {
                max_log_level.set(LogLevelFilter::Trace);
                Box::new(SimpleLogger)
            }).expect("Could not set the logger");
        });

        let counter = Cell::new(0);
        let function = || {
            counter.set(counter.get() + 1);
            counter.get()
        };

        info_once!("Counter is: {}", function());
        assert_eq!(counter.get(), 1);
    }
}
