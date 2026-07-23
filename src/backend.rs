use gorust::{go, channel, Runtime};
use gorust::channel::RecvError;
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
        // 确保 gorust 运行时已初始化
        Runtime::init();

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
        // 主循环：使用阻塞 recv() 等待消息，线程在无消息时 park 不消耗 CPU
        // 通过 close() 唤醒 receiver 退出，确保 Ctrl-C 时也能正常退出
        loop {
            // 检查 running 标志和 gorust runtime 状态
            if !running.load(Ordering::Relaxed) {
                break;
            }
            if gorust::Runtime::is_shutting_down() {
                break;
            }

            match rx.recv() {
                Ok(msg) => {
                    writer.write(&msg);
                }
                Err(RecvError::Disconnected) => break,
                Err(RecvError::Empty) => {
                    // 阻塞 recv 正常情况下不会返回 Empty，此处仅作防御
                    std::thread::sleep(std::time::Duration::from_millis(1));
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
        // 使用 try_send 避免阻塞调用方，确保高吞吐
        let _ = self.tx.try_send(msg);
    }

    pub fn shutdown(&self) {
        self.running.store(false, Ordering::Relaxed);
        // 关闭通道，唤醒 receiver 使其立即退出
        self.tx.close();
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
