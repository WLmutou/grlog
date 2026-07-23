use grlog::{info, warn, error, Target, LevelFilter};
use std::path::PathBuf;

fn main() {
    // 同时输出到控制台和文件
    let log_path = PathBuf::from("/tmp/grlog_example.log");

    let mut builder = grlog::builder();
    builder
        .targets(vec![
            Target::Stdout,
            Target::File(log_path),
        ])
        .filter_level(LevelFilter::Debug);

    builder.init().expect("Failed to initialize logger");

    info!("Application started");
    warn!("This is a warning message");
    error!("This is an error message");

    // 等待异步日志写入完成
    std::thread::sleep(std::time::Duration::from_millis(200));

    println!("\nLog file has been written to /tmp/grlog_example.log");
}