use crate::level::Record;
use chrono::Local;

/// 格式化日志消息
pub fn format_log_message(record: &Record) -> String {
    let now = Local::now();
    format!(
        "{} {:<5} [{}] {}\n",
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        record.level(),
        record.target(),
        record.args()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::{Level, Metadata, Record};

    #[test]
    fn test_format_log_message() {
        let metadata = Metadata::builder()
            .level(Level::Info)
            .target("test_module")
            .build();
        
        let record = Record::builder()
            .metadata(metadata)
            .args(format_args!("Test message"))
            .build();
        
        let formatted = format_log_message(&record);
        
        // 验证格式包含必要的部分
        assert!(formatted.contains("INFO"));
        assert!(formatted.contains("test_module"));
        assert!(formatted.contains("Test message"));
        assert!(formatted.contains('\n'));
    }

    #[test]
    fn test_format_different_levels() {
        let levels = [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error];
        
        for level in levels {
            let metadata = Metadata::builder()
                .level(level)
                .target("test")
                .build();
            
            let record = Record::builder()
                .metadata(metadata)
                .args(format_args!("Message"))
                .build();
            
            let formatted = format_log_message(&record);
            assert!(formatted.contains(&format!("{:<5}", level)));
        }
    }
}
