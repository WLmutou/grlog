use std::fmt;
use std::str::FromStr;

/// 日志级别
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

/// 日志级别过滤器
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LevelFilter {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl LevelFilter {
    pub fn to_level(&self) -> Option<Level> {
        match self {
            LevelFilter::Trace => Some(Level::Trace),
            LevelFilter::Debug => Some(Level::Debug),
            LevelFilter::Info => Some(Level::Info),
            LevelFilter::Warn => Some(Level::Warn),
            LevelFilter::Error => Some(Level::Error),
            LevelFilter::Off => None,
        }
    }

    pub fn from_level(level: Level) -> Self {
        match level {
            Level::Trace => LevelFilter::Trace,
            Level::Debug => LevelFilter::Debug,
            Level::Info => LevelFilter::Info,
            Level::Warn => LevelFilter::Warn,
            Level::Error => LevelFilter::Error,
        }
    }

    pub fn is_enabled(&self, level: Level) -> bool {
        match self {
            LevelFilter::Off => false,
            _ => level >= self.to_level().unwrap_or(Level::Trace),
        }
    }
}

impl FromStr for LevelFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LevelFilter::Trace),
            "debug" => Ok(LevelFilter::Debug),
            "info" => Ok(LevelFilter::Info),
            "warn" => Ok(LevelFilter::Warn),
            "error" => Ok(LevelFilter::Error),
            "off" => Ok(LevelFilter::Off),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LevelFilter::Trace => f.pad("TRACE"),
            LevelFilter::Debug => f.pad("DEBUG"),
            LevelFilter::Info => f.pad("INFO"),
            LevelFilter::Warn => f.pad("WARN"),
            LevelFilter::Error => f.pad("ERROR"),
            LevelFilter::Off => f.pad("OFF"),
        }
    }
}

/// 日志元数据
#[derive(Clone, Debug)]
pub struct Metadata<'a> {
    level: Level,
    target: &'a str,
}

impl<'a> Metadata<'a> {
    pub fn builder() -> MetadataBuilder<'a> {
        MetadataBuilder::new()
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn target(&self) -> &'a str {
        self.target
    }
}

pub struct MetadataBuilder<'a> {
    level: Option<Level>,
    target: Option<&'a str>,
}

impl<'a> MetadataBuilder<'a> {
    pub fn new() -> Self {
        Self {
            level: None,
            target: None,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    pub fn target(mut self, target: &'a str) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Metadata<'a> {
        Metadata {
            level: self.level.expect("level must be set"),
            target: self.target.expect("target must be set"),
        }
    }
}

impl<'a> Default for MetadataBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// 日志记录
pub struct Record<'a> {
    metadata: Metadata<'a>,
    args: fmt::Arguments<'a>,
}

impl<'a> Record<'a> {
    pub fn builder() -> RecordBuilder<'a> {
        RecordBuilder::new()
    }

    pub fn metadata(&self) -> &Metadata<'a> {
        &self.metadata
    }

    pub fn args(&self) -> fmt::Arguments<'a> {
        self.args
    }

    pub fn level(&self) -> Level {
        self.metadata.level
    }

    pub fn target(&self) -> &'a str {
        self.metadata.target
    }
}

pub struct RecordBuilder<'a> {
    metadata: Option<Metadata<'a>>,
    args: Option<fmt::Arguments<'a>>,
}

impl<'a> RecordBuilder<'a> {
    pub fn new() -> Self {
        Self {
            metadata: None,
            args: None,
        }
    }

    pub fn metadata(mut self, metadata: Metadata<'a>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn args(mut self, args: fmt::Arguments<'a>) -> Self {
        self.args = Some(args);
        self
    }

    pub fn build(self) -> Record<'a> {
        Record {
            metadata: self.metadata.expect("metadata must be set"),
            args: self.args.expect("args must be set"),
        }
    }
}

impl<'a> Default for RecordBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// 日志器 trait
pub trait Log: Send + Sync {
    fn enabled(&self, metadata: &Metadata) -> bool;
    fn log(&self, record: &Record);
    fn flush(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_display() {
        assert_eq!(Level::Trace.to_string(), "TRACE");
        assert_eq!(Level::Debug.to_string(), "DEBUG");
        assert_eq!(Level::Info.to_string(), "INFO");
        assert_eq!(Level::Warn.to_string(), "WARN");
        assert_eq!(Level::Error.to_string(), "ERROR");
    }

    #[test]
    fn test_level_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
    }

    #[test]
    fn test_level_filter_from_str() {
        assert_eq!("trace".parse::<LevelFilter>(), Ok(LevelFilter::Trace));
        assert_eq!("debug".parse::<LevelFilter>(), Ok(LevelFilter::Debug));
        assert_eq!("info".parse::<LevelFilter>(), Ok(LevelFilter::Info));
        assert_eq!("warn".parse::<LevelFilter>(), Ok(LevelFilter::Warn));
        assert_eq!("error".parse::<LevelFilter>(), Ok(LevelFilter::Error));
        assert_eq!("off".parse::<LevelFilter>(), Ok(LevelFilter::Off));
        assert_eq!("invalid".parse::<LevelFilter>(), Err(()));
    }

    #[test]
    fn test_level_filter_to_level() {
        assert_eq!(LevelFilter::Trace.to_level(), Some(Level::Trace));
        assert_eq!(LevelFilter::Debug.to_level(), Some(Level::Debug));
        assert_eq!(LevelFilter::Info.to_level(), Some(Level::Info));
        assert_eq!(LevelFilter::Warn.to_level(), Some(Level::Warn));
        assert_eq!(LevelFilter::Error.to_level(), Some(Level::Error));
        assert_eq!(LevelFilter::Off.to_level(), None);
    }

    #[test]
    fn test_level_filter_comparison() {
        assert!(LevelFilter::Debug > LevelFilter::Trace);
        assert!(LevelFilter::Info > LevelFilter::Debug);
        assert!(LevelFilter::Debug.is_enabled(Level::Info));
        assert!(!LevelFilter::Info.is_enabled(Level::Debug));
    }

    #[test]
    fn test_metadata_builder() {
        let metadata = Metadata::builder()
            .level(Level::Info)
            .target("test_module")
            .build();

        assert_eq!(metadata.level(), Level::Info);
        assert_eq!(metadata.target(), "test_module");
    }

    #[test]
    fn test_record_builder() {
        let metadata = Metadata::builder()
            .level(Level::Info)
            .target("test_module")
            .build();

        let record = Record::builder()
            .metadata(metadata)
            .args(format_args!("Test message"))
            .build();

        assert_eq!(record.level(), Level::Info);
        assert_eq!(record.target(), "test_module");
    }
}
