use std::path::PathBuf;

/// 验证 Target::Multi 嵌套
#[test]
fn test_target_multi_nested() {
    let log_path1 = PathBuf::from("/tmp/grlog_test_nested1.log");
    let log_path2 = PathBuf::from("/tmp/grlog_test_nested2.log");
    let _ = std::fs::remove_file(&log_path1);
    let _ = std::fs::remove_file(&log_path2);

    let _ = grlog::builder()
        .target(grlog::Target::Multi(vec![
            grlog::Target::Stdout,
            grlog::Target::File(log_path1.clone()),
            grlog::Target::File(log_path2.clone()),
        ]))
        .filter_level(grlog::LevelFilter::Info)
        .try_init();

    grlog::info!("nested multi target message");

    std::thread::sleep(std::time::Duration::from_millis(500));

    // 两个文件都应有内容
    let content1 = std::fs::read_to_string(&log_path1).unwrap_or_default();
    let content2 = std::fs::read_to_string(&log_path2).unwrap_or_default();
    assert!(!content1.is_empty(), "nested1 should not be empty");
    assert!(!content2.is_empty(), "nested2 should not be empty");
    assert!(content1.contains("nested multi target message"), "content1: {}", content1);
    assert!(content2.contains("nested multi target message"), "content2: {}", content2);

    let _ = std::fs::remove_file(&log_path1);
    let _ = std::fs::remove_file(&log_path2);
}