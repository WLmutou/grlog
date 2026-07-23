use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;

/// 日志写入器 trait
pub trait LogWriter: Send + Sync {
    fn write(&self, msg: &str);
    fn flush(&self);
}

/// 标准输出写入器
pub struct StdoutWriter;

impl LogWriter for StdoutWriter {
    fn write(&self, msg: &str) {
        let _ = io::stdout().lock().write_all(msg.as_bytes());
    }
    fn flush(&self) {
        let _ = io::stdout().flush();
    }
}

/// 标准错误写入器
pub struct StderrWriter;

impl LogWriter for StderrWriter {
    fn write(&self, msg: &str) {
        let _ = io::stderr().lock().write_all(msg.as_bytes());
    }
    fn flush(&self) {
        let _ = io::stderr().flush();
    }
}

/// 文件写入器
pub struct FileWriter {
    file: std::sync::Mutex<std::fs::File>,
}

impl FileWriter {
    pub fn new(path: &PathBuf) -> io::Result<Self> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self {
            file: std::sync::Mutex::new(file),
        })
    }
}

impl LogWriter for FileWriter {
    fn write(&self, msg: &str) {
        if let Ok(mut f) = self.file.lock() {
            let _ = f.write_all(msg.as_bytes());
            let _ = f.flush();
        }
    }
    fn flush(&self) {
        if let Ok(mut f) = self.file.lock() {
            let _ = f.flush();
        }
    }
}

/// 多目标写入器：同时写入多个 LogWriter
pub struct MultiWriter {
    writers: Vec<Box<dyn LogWriter>>,
}

impl MultiWriter {
    pub fn new(writers: Vec<Box<dyn LogWriter>>) -> Self {
        Self { writers }
    }
}

impl LogWriter for MultiWriter {
    fn write(&self, msg: &str) {
        for writer in &self.writers {
            writer.write(msg);
        }
    }
    fn flush(&self) {
        for writer in &self.writers {
            writer.flush();
        }
    }
}

/// 根据 Target 创建对应的 LogWriter
pub fn create_writer(target: &crate::Target) -> Box<dyn LogWriter> {
    match target {
        crate::Target::Stdout => Box::new(StdoutWriter),
        crate::Target::Stderr => Box::new(StderrWriter),
        crate::Target::File(path) => {
            Box::new(FileWriter::new(path).expect("Failed to open log file"))
        }
        crate::Target::Multi(targets) => {
            let writers: Vec<Box<dyn LogWriter>> = targets
                .iter()
                .map(create_writer)
                .collect();
            Box::new(MultiWriter::new(writers))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdout_writer() {
        let writer = StdoutWriter;
        writer.write("test message\n");
        writer.flush();
    }

    #[test]
    fn test_stderr_writer() {
        let writer = StderrWriter;
        writer.write("test message\n");
        writer.flush();
    }

    #[test]
    fn test_file_writer() {
        let temp_dir = std::env::temp_dir();
        let log_path = temp_dir.join("grlog_test.log");
        
        let writer = FileWriter::new(&log_path).expect("Failed to create file writer");
        writer.write("test message\n");
        writer.flush();
        
        // 验证文件存在
        assert!(log_path.exists());
        
        // 清理测试文件
        let _ = std::fs::remove_file(&log_path);
    }

    #[test]
    fn test_create_writer() {
        let stdout_target = crate::Target::Stdout;
        let stderr_target = crate::Target::Stderr;
        
        let stdout_writer = create_writer(&stdout_target);
        let stderr_writer = create_writer(&stderr_target);
        
        stdout_writer.write("test\n");
        stderr_writer.write("test\n");
    }
}
