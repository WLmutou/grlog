use grlog::{info, debug, warn, Target, LevelFilter};

fn main() {
    // 自定义配置：详细配置日志系统
    let mut builder = grlog::builder();
    builder
        // 设置全局日志级别为 Debug
        .filter_level(LevelFilter::Debug)
        // 为特定模块设置不同级别
        .filter_module("hyper", LevelFilter::Info)
        .filter_module("tokio_reactor", LevelFilter::Warn)
        // 输出到 stdout
        .target(Target::Stdout)
        // 增大异步缓冲区
        .buffer_size(2048);

    builder.init().expect("Failed to initialize logger");

    debug!("Debug message - visible with current config");
    info!("Info message - also visible");
    warn!("Warning message");

    // 使用环境变量也是支持的
    // 运行: RUST_LOG=debug,my_module=trace cargo run --example custom_config
    // 或使用自定义环境变量:
    // std::env::set_var("MY_APP_LOG", "debug");
    // let mut builder = grlog::builder().from_env("MY_APP_LOG");
    // builder.init().unwrap();
}