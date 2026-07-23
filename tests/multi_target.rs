use std::path::PathBuf;

/// 验证多目标输出（控制台 + 文件）
#[test]
fn test_multi_target_console_and_file() {
    let log_path = PathBuf::from("/tmp/grlog_test_multi_target.log");
    let _ = std::fs::remove_file(&log_path);

    let _ = grlog::builder()
        .targets(vec![
            grlog::Target::Stdout,
            grlog::Target::File(log_path.clone()),
        ])
        .filter_level(grlog::LevelFilter::Info)
        .try_init();

    grlog::info!("multi target message");

    // 等待异步写入完成
    std::thread::sleep(std::time::Duration::from_millis(500));

    // 验证文件输出
    assert!(log_path.exists(), "log file should exist");
    let content = std::fs::read_to_string(&log_path).unwrap_or_default();
    assert!(!content.is_empty(), "log file should not be empty");
    assert!(content.contains("multi target message"), "content: {}", content);

    let _ = std::fs::remove_file(&log_path);
}