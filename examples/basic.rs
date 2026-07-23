use grlog::{trace, debug, info, warn, error};

fn main() {
    // 使用默认配置初始化（输出到 stdout，级别 debug）
    grlog::init();

    // 所有日志级别
    trace!("Trace level - finest granularity");
    debug!("Debug level - diagnostic information");
    info!("Info level - general information");
    warn!("Warn level - potentially harmful situations");
    error!("Error level - error events");

    // 带参数的日志
    let user = "Alice";
    let count = 42;
    info!("User {} logged in {} times", user, count);
}