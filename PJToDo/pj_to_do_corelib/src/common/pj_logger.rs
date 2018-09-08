extern crate log;

use log::{Record, Level, Metadata};
use log::{SetLoggerError, LevelFilter};

static LOGGER: PJLogger = PJLogger;

pub struct PJLogger;

impl log::Log for PJLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[allow(unused_macros)]
macro_rules! pj_warn {
    ($fmt:expr) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        warn!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        warn!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
macro_rules! pj_debug {
    ($fmt:expr) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        debug!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        debug!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
macro_rules! pj_error {
    ($fmt:expr) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        error!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        error!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
macro_rules! pj_info {
    ($fmt:expr) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        info!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        info!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
macro_rules! pj_trace {
    ($fmt:expr) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        trace!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        print!("module_path: {}, file: {}, line: {}, column: {} : ", module_path!(), file!(), line!(), column!());
        trace!($fmt, $($arg)*);
    };
}

impl PJLogger {
    /// init log before use log. And the log just need to init for once.
    pub fn pj_init_logger() -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
    }

    // pub fn pj_warn(warn: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     warn!("{}", warn);
    // }

    // pub fn pj_warn_target(target:&str, warn: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     warn!(target: target, "{}", warn);
    // }

    // pub fn pj_debug(debug: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     debug!("{}", debug);
    // }

    // pub fn pj_debug_target(target:&str, debug: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     debug!(target: target, "{}", debug);
    // }

    // pub fn pj_error(error: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     error!("{}", error);
    // }

    // pub fn pj_error_target(target:&str, error: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     error!(target: target, "{}", error);
    // }

    // pub fn pj_info(info: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     info!("{}", info);
    // }

    // pub fn pj_info_target(target:&str, info: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     info!(target: target, "{}", info);
    // }

    // pub fn pj_trace(trace: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     trace!("{}", trace);
    // }

    // pub fn pj_trace_target(target:&str, trace: &str) {
    //     println!("module_path: {}, file: {}, line: {}, column: {}", module_path!(), file!(), line!(), column!());
    //     trace!(target: target, "{}", trace);
    // }
}