extern crate log;
extern crate chrono;

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
            eprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[cfg(feature = "type_name")]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }
        let name = type_name_of(f);
        &name[6..name.len() - 4]
    }};
}

#[cfg(not(feature = "type_name"))]
macro_rules! function {
    // () => {{ "<fn>" }}
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            extern crate core;
            core::intrinsics::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[6..name.len() - 4]
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! pj_warn {
    ($fmt:expr) => {
        eprintln!("[⚠️PJWarn⚠️] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        warn!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("[⚠️PJWarn⚠️] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        warn!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! pj_debug {
    ($fmt:expr) => {
        eprintln!("[🏃PJDebug🏃] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        debug!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("[🏃PJDebug🏃] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        debug!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! pj_error {
    ($fmt:expr) => {
        eprintln!("[❌PJError❌] -> {} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        error!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("[❌PJError❌] -> {} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        error!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! pj_info {
    ($fmt:expr) => {
        eprintln!("[ℹ️PJInfoℹ️] -> {} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        info!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("[ℹ️PJInfoℹ️] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        info!($fmt, $($arg)*);
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! pj_trace {
    ($fmt:expr) => {
        eprintln!("[🐒PJInfo🐒] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        trace!(stringify!($fmt));
        };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("[🐒PJInfo🐒] ->{} [module_path: {}, file: {}, function: {}, line: {}, column: {}]---: ", chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S"), module_path!(), file!(), function!(), line!(), column!());
        trace!($fmt, $($arg)*);
    };
}

impl PJLogger {
    /// init log before use log. And the log just need to init for once.
    pub fn pj_init_logger() -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
    }
}
