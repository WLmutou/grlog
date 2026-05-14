use log::{LevelFilter, Metadata, Record, Log};
use std::collections::HashMap;
use std::sync::Arc;
use crate::backend::AsyncLogBackend;
use crate::formatter::format_log_message;

/// GrLogger - 实现 log::Log trait
pub struct GrLogger {
    level: LevelFilter,
    module_levels: HashMap<String, LevelFilter>,
    backend: Arc<AsyncLogBackend>,
}

impl GrLogger {
    pub fn new(
        level: LevelFilter,
        module_levels: HashMap<String, LevelFilter>,
        backend: Arc<AsyncLogBackend>,
    ) -> Self {
        Self {
            level,
            module_levels,
            backend,
        }
    }

    fn should_log(&self, metadata: &Metadata) -> bool {
        // 检查模块级别的日志级别
        if let Some(module_level) = self.module_levels.get(metadata.target()) {
            return metadata.level() <= *module_level;
        }
        // 检查全局日志级别
        metadata.level() <= self.level
    }
}

impl Log for GrLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.should_log(metadata)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let formatted = format_log_message(record);
            self.backend.send(formatted);
        }
    }

    fn flush(&self) {
        self.backend.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;
    use std::sync::Mutex;

    struct MockWriter {
        messages: Arc<Mutex<Vec<String>>>,
    }

    impl MockWriter {
        fn new() -> (Self, Arc<Mutex<Vec<String>>>) {
            let messages = Arc::new(Mutex::new(Vec::new()));
            (
                Self {
                    messages: messages.clone(),
                },
                messages,
            )
        }
    }

    impl crate::writer::LogWriter for MockWriter {
        fn write(&self, msg: &str) {
            if let Ok(mut msgs) = self.messages.lock() {
                msgs.push(msg.to_string());
            }
        }
        fn flush(&self) {}
    }

    #[test]
    fn test_logger_enabled() {
        let (writer, _) = MockWriter::new();
        let backend = Arc::new(AsyncLogBackend::new(Box::new(writer), 1024));
        
        let logger = GrLogger::new(LevelFilter::Info, HashMap::new(), backend);
        
        let info_meta = Metadata::builder()
            .level(Level::Info)
            .target("test")
            .build();
        let debug_meta = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();
        
        assert!(logger.enabled(&info_meta));
        assert!(!logger.enabled(&debug_meta));
    }

    #[test]
    fn test_logger_module_level() {
        let (writer, _) = MockWriter::new();
        let backend = Arc::new(AsyncLogBackend::new(Box::new(writer), 1024));
        
        let mut module_levels = HashMap::new();
        module_levels.insert("verbose_module".to_string(), LevelFilter::Debug);
        
        let logger = GrLogger::new(LevelFilter::Info, module_levels, backend);
        
        let verbose_debug = Metadata::builder()
            .level(Level::Debug)
            .target("verbose_module")
            .build();
        let other_debug = Metadata::builder()
            .level(Level::Debug)
            .target("other_module")
            .build();
        
        assert!(logger.enabled(&verbose_debug));
        assert!(!logger.enabled(&other_debug));
    }
}
