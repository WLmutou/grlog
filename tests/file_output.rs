use std::path::PathBuf;

/// 验证文件输出
#[test]
fn test_file_output() {
    let log_path = PathBuf::from("/tmp/grlog_test_file_output.log");
    let _ = std::fs::remove_file(&log_path);

    // 使用 init 而非 try_init 以检查初始化是否成功
    let result = grlog::builder()
        .target(grlog::Target::File(log_path.clone()))
        .filter_level(grlog::LevelFilter::Debug)
        .init();

    assert!(result.is_ok(), "logger init should succeed: {:?}", result);

    grlog::debug!("file debug message");
    grlog::info!("file info message");
    grlog::error!("file error message");

    // 等待异步写入完成
    std::thread::sleep(std::time::Duration::from_millis(1000));

    // 验证文件存在且有内容
    assert!(log_path.exists(), "log file should exist at {:?}", log_path);
    let metadata = std::fs::metadata(&log_path).unwrap();
    eprintln!("file size: {} bytes", metadata.len());

    let content = std::fs::read_to_string(&log_path).unwrap_or_default();
    eprintln!("file content: {:?}", content);
    assert!(!content.is_empty(), "log file should not be empty, size={}", metadata.len());
    assert!(content.contains("file debug message"), "content: {}", content);
    assert!(content.contains("file info message"), "content: {}", content);
    assert!(content.contains("file error message"), "content: {}", content);

    // 清理
    let _ = std::fs::remove_file(&log_path);
}