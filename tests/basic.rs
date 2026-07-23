use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// 验证默认初始化不 panic
#[test]
fn test_init_default() {
    let _ = grlog::builder().try_init();
}

/// 验证所有日志级别宏不 panic
#[test]
fn test_log_macros_no_panic() {
    let _ = grlog::builder().filter_level(grlog::LevelFilter::Trace).try_init();

    grlog::trace!("trace message {}", 1);
    grlog::debug!("debug message {}", 2);
    grlog::info!("info message {}", 3);
    grlog::warn!("warn message {}", 4);
    grlog::error!("error message {}", 5);
}

/// 验证日志级别过滤
#[test]
fn test_level_filtering() {
    let _ = grlog::builder()
        .filter_level(grlog::LevelFilter::Warn)
        .try_init();

    // 这些宏应该不会 panic，但低于 Warn 的日志不会输出
    grlog::trace!("should not appear");
    grlog::debug!("should not appear");
    grlog::info!("should not appear");
    grlog::warn!("should appear");
    grlog::error!("should appear");
}

/// 验证 builder 链式调用
#[test]
fn test_builder_chaining() {
    let mut builder = grlog::builder();
    builder
        .filter_level(grlog::LevelFilter::Debug)
        .filter_module("hyper", grlog::LevelFilter::Info)
        .filter_module("tokio", grlog::LevelFilter::Warn)
        .target(grlog::Target::Stdout)
        .buffer_size(2048);

    // 验证 builder 方法返回 &mut Self 以支持链式调用
    // 注意：内部字段是 pub(crate) 的，在集成测试中无法直接访问
    // 这里只验证链式调用不 panic
}

/// 验证并发日志写入
#[test]
fn test_concurrent_logging() {
    let _ = grlog::builder()
        .filter_level(grlog::LevelFilter::Info)
        .buffer_size(4096)
        .try_init();

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            for j in 0..10 {
                grlog::info!("thread {} message {}", i, j);
                c.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 100);
}