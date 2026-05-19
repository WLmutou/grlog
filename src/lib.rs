pub mod target;
pub mod level;
pub mod writer;
pub mod formatter;
pub mod backend;
pub mod logger;
pub mod builder;

pub use target::Target;
pub use level::{Level, LevelFilter, Metadata, Record, Log};
pub use builder::{LoggerBuilder, init, init_from_env, builder, SetLoggerError};

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        $crate::_log($crate::Level::Trace, module_path!(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        $crate::_log($crate::Level::Debug, module_path!(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::_log($crate::Level::Info, module_path!(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::_log($crate::Level::Warn, module_path!(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::_log($crate::Level::Error, module_path!(), format_args!($($arg)+))
    };
}

pub fn _log(level: Level, target: &str, args: std::fmt::Arguments) {
    use crate::builder::GLOBAL_LOGGER;
    use crate::builder::MAX_LEVEL;
    
    if let Some(max_level) = MAX_LEVEL.get() {
        if max_level.is_enabled(level) {
            if let Some(logger) = GLOBAL_LOGGER.get() {
                let metadata = Metadata::builder()
                    .level(level)
                    .target(target)
                    .build();
                
                if logger.enabled(&metadata) {
                    let record = Record::builder()
                        .metadata(metadata)
                        .args(args)
                        .build();
                    logger.log(&record);
                }
            }
        }
    }
}
