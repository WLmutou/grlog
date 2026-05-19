use crate::level::LevelFilter;
use std::collections::HashMap;
use std::sync::Arc;
use crate::Target;
use crate::writer::create_writer;
use crate::backend::AsyncLogBackend;
use crate::logger::GrLogger;

/// LoggerBuilder - 类似 env_logger 的构建器 API
pub struct LoggerBuilder {
    target: Target,
    level: LevelFilter,
    module_levels: HashMap<String, LevelFilter>,
    buffer_size: usize,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self {
            target: Target::default(),
            level: LevelFilter::Info,
            module_levels: HashMap::new(),
            buffer_size: 1024,
        }
    }

    /// 从环境变量初始化构建器
    /// 格式: RUST_LOG=info 或 RUST_LOG=module1=debug,module2=warn
    pub fn from_env(env_var: &str) -> Self {
        let mut builder = Self::new();
        if let Ok(spec) = std::env::var(env_var) {
            builder.parse_spec(&spec);
        }
        builder
    }

    /// 设置全局日志级别
    pub fn filter_level(&mut self, level: LevelFilter) -> &mut Self {
        self.level = level;
        self
    }

    /// 设置特定模块的日志级别
    pub fn filter_module(&mut self, module: &str, level: LevelFilter) -> &mut Self {
        self.module_levels.insert(module.to_string(), level);
        self
    }

    /// 设置日志输出目标
    pub fn target(&mut self, target: Target) -> &mut Self {
        self.target = target;
        self
    }

    /// 设置异步缓冲区大小
    pub fn buffer_size(&mut self, size: usize) -> &mut Self {
        self.buffer_size = size;
        self
    }

    /// 解析日志级别规范字符串
    fn parse_spec(&mut self, spec: &str) {
        for part in spec.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((module, level)) = part.split_once('=') {
                if let Some(level) = parse_level(level) {
                    self.filter_module(module, level);
                }
            } else if let Some(level) = parse_level(part) {
                self.filter_level(level);
            }
        }
    }

    /// 初始化日志系统
    pub fn init(&mut self) -> Result<(), SetLoggerError> {
        let writer = create_writer(&self.target);
        let backend = Arc::new(AsyncLogBackend::new(writer, self.buffer_size));

        let logger = GrLogger::new(
            self.level,
            self.module_levels.clone(),
            backend,
        );

        set_boxed_logger(Box::new(logger))?;
        set_max_level(self.level);

        Ok(())
    }

    /// 尝试初始化，失败时静默忽略
    pub fn try_init(&mut self) {
        let _ = self.init();
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 解析日志级别字符串
fn parse_level(level: &str) -> Option<LevelFilter> {
    match level.to_lowercase().as_str() {
        "trace" => Some(LevelFilter::Trace),
        "debug" => Some(LevelFilter::Debug),
        "info" => Some(LevelFilter::Info),
        "warn" => Some(LevelFilter::Warn),
        "error" => Some(LevelFilter::Error),
        "off" => Some(LevelFilter::Off),
        _ => None,
    }
}

/// 设置全局日志器错误类型
#[derive(Debug)]
pub struct SetLoggerError;

impl std::fmt::Display for SetLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attempted to set a logger after the logging system was already initialized")
    }
}

impl std::error::Error for SetLoggerError {}

use std::sync::OnceLock;

pub(crate) static GLOBAL_LOGGER: OnceLock<Box<dyn crate::level::Log>> = OnceLock::new();
pub(crate) static MAX_LEVEL: OnceLock<LevelFilter> = OnceLock::new();

fn set_boxed_logger(logger: Box<dyn crate::level::Log>) -> Result<(), SetLoggerError> {
    if GLOBAL_LOGGER.set(logger).is_err() {
        return Err(SetLoggerError);
    }
    Ok(())
}

fn set_max_level(level: LevelFilter) {
    let _ = MAX_LEVEL.set(level);
}

/// 使用默认配置初始化日志（输出到 stderr，级别 info）
pub fn init() {
    LoggerBuilder::new().try_init();
}

/// 从环境变量 RUST_LOG 初始化日志
pub fn init_from_env() {
    LoggerBuilder::from_env("RUST_LOG").try_init();
}

/// 创建一个新的 LoggerBuilder
pub fn builder() -> LoggerBuilder {
    LoggerBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level() {
        assert_eq!(parse_level("trace"), Some(LevelFilter::Trace));
        assert_eq!(parse_level("debug"), Some(LevelFilter::Debug));
        assert_eq!(parse_level("info"), Some(LevelFilter::Info));
        assert_eq!(parse_level("warn"), Some(LevelFilter::Warn));
        assert_eq!(parse_level("error"), Some(LevelFilter::Error));
        assert_eq!(parse_level("off"), Some(LevelFilter::Off));
        assert_eq!(parse_level("invalid"), None);
        assert_eq!(parse_level("INFO"), Some(LevelFilter::Info)); // 大小写不敏感
    }

    #[test]
    fn test_builder_default() {
        let builder = LoggerBuilder::new();
        assert_eq!(builder.level, LevelFilter::Info);
        assert_eq!(builder.buffer_size, 1024);
        assert!(builder.module_levels.is_empty());
    }

    #[test]
    fn test_builder_filter_level() {
        let mut builder = LoggerBuilder::new();
        builder.filter_level(LevelFilter::Debug);
        assert_eq!(builder.level, LevelFilter::Debug);
    }

    #[test]
    fn test_builder_filter_module() {
        let mut builder = LoggerBuilder::new();
        builder.filter_module("my_module", LevelFilter::Debug);
        assert_eq!(builder.module_levels.get("my_module"), Some(&LevelFilter::Debug));
    }

    #[test]
    fn test_builder_parse_spec() {
        let mut builder = LoggerBuilder::new();
        builder.parse_spec("info,my_module=debug,other=warn");
        
        assert_eq!(builder.level, LevelFilter::Info);
        assert_eq!(builder.module_levels.get("my_module"), Some(&LevelFilter::Debug));
        assert_eq!(builder.module_levels.get("other"), Some(&LevelFilter::Warn));
    }

    #[test]
    fn test_builder_from_env() {
        std::env::set_var("GRLOG_TEST_LOG", "debug,my_module=trace");
        let builder = LoggerBuilder::from_env("GRLOG_TEST_LOG");
        
        assert_eq!(builder.level, LevelFilter::Debug);
        assert_eq!(builder.module_levels.get("my_module"), Some(&LevelFilter::Trace));
        
        std::env::remove_var("GRLOG_TEST_LOG");
    }

    #[test]
    fn test_builder_functions() {
        // 测试便捷函数
        init(); // 应该不会 panic
        init_from_env(); // 应该不会 panic
        let _builder = builder();
    }
}
