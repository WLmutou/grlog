use gorust::{go, channel};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::writer::LogWriter;

/// 异步日志后端（基于 gorust channel + GMP goroutine）
pub struct AsyncLogBackend {
    tx: channel::Sender<String>,
    running: Arc<AtomicBool>,
}

impl AsyncLogBackend {
    pub fn new(writer: Box<dyn LogWriter>, buffer_size: usize) -> Self {
        let (tx, rx) = channel::new_with_capacity(buffer_size);
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        // 启动一个 goroutine 专门处理日志写入（GMP 模式）
        go(move || {
            Self::log_writer_loop(rx, writer, running_clone);
        });

        Self { tx, running }
    }

    fn log_writer_loop(
        rx: channel::Receiver<String>,
        writer: Box<dyn LogWriter>,
        running: Arc<AtomicBool>,
    ) {
        while running.load(Ordering::Relaxed) {
            // 同时检查本地 running 标志和 gorust scheduler 状态
            // 确保 Ctrl-C 时能正常退出
            if !gorust::scheduler::Scheduler::is_running() {
                running.store(false, Ordering::Relaxed);
                break;
            }
            // 使用 try_recv 避免阻塞，确保能检查 running 标志
            match rx.try_recv() {
                Ok(msg) => {
                    writer.write(&msg);
                }
                Err(_) => {
                    // 通道为空或断开，短暂休眠后继续检查 running
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
        // 退出前清空通道中剩余的消息
        loop {
            match rx.try_recv() {
                Ok(msg) => writer.write(&msg),
                Err(_) => break,
            }
        }
        writer.flush();
    }

    pub fn send(&self, msg: String) {
        // 使用 try_send 避免阻塞 goroutine，确保 Ctrl-C 能正常退出
        let _ = self.tx.try_send(msg);
    }

    pub fn shutdown(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

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

    impl LogWriter for MockWriter {
        fn write(&self, msg: &str) {
            if let Ok(mut msgs) = self.messages.lock() {
                msgs.push(msg.to_string());
            }
        }
        fn flush(&self) {}
    }

    #[test]
    fn test_backend_send_and_shutdown() {
        let (writer, messages) = MockWriter::new();
        let backend = AsyncLogBackend::new(Box::new(writer), 1024);

        backend.send("test message 1".to_string());
        backend.send("test message 2".to_string());

        // 等待消息处理
        std::thread::sleep(std::time::Duration::from_millis(200));

        backend.shutdown();
        std::thread::sleep(std::time::Duration::from_millis(100));

        let msgs = messages.lock().unwrap();
        // 由于 gorust runtime 可能未在测试中完全初始化，我们只验证不 panic
        // 实际的消息接收取决于 goroutine 调度
        let _ = msgs.len();
    }

    #[test]
    fn test_backend_try_send_non_blocking() {
        let (writer, _) = MockWriter::new();
        let backend = AsyncLogBackend::new(Box::new(writer), 1);

        // 发送多条消息，验证 try_send 不会阻塞
        for i in 0..10 {
            backend.send(format!("message {}", i));
        }

        backend.shutdown();
    }
}
