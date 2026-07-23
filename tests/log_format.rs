use std::path::PathBuf;

/// 验证日志格式包含必要字段
#[test]
fn test_log_format() {
    let log_path = PathBuf::from("/tmp/grlog_test_format.log");
    let _ = std::fs::remove_file(&log_path);

    let _ = grlog::builder()
        .target(grlog::Target::File(log_path.clone()))
        .filter_level(grlog::LevelFilter::Info)
        .try_init();

    grlog::info!("format check message");

    std::thread::sleep(std::time::Duration::from_millis(500));

    let content = std::fs::read_to_string(&log_path).unwrap_or_default();
    assert!(!content.is_empty(), "log file should not be empty");
    // 格式: YYYY-MM-DD HH:MM:SS.mmm LEVEL [target] message
    assert!(content.contains("INFO"), "content: {}", content);
    assert!(content.contains("format check message"), "content: {}", content);
    assert!(content.contains('\n'), "content: {}", content);
    // 验证包含时间戳（数字开头）
    assert!(content.chars().next().map_or(false, |c| c.is_ascii_digit()));

    let _ = std::fs::remove_file(&log_path);
}