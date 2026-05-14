use std::path::PathBuf;

/// 日志输出目标
#[derive(Debug, Clone)]
pub enum Target {
    /// 标准输出
    Stdout,
    /// 标准错误
    Stderr,
    /// 文件输出
    File(PathBuf),
}

impl Default for Target {
    fn default() -> Self {
        Target::Stderr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_target() {
        let target = Target::default();
        assert!(matches!(target, Target::Stderr));
    }

    #[test]
    fn test_target_variants() {
        let stdout = Target::Stdout;
        let stderr = Target::Stderr;
        let file = Target::File(PathBuf::from("/tmp/test.log"));

        assert!(matches!(stdout, Target::Stdout));
        assert!(matches!(stderr, Target::Stderr));
        assert!(matches!(file, Target::File(_)));
    }

    #[test]
    fn test_target_clone() {
        let target = Target::File(PathBuf::from("/tmp/test.log"));
        let cloned = target.clone();
        assert!(matches!(cloned, Target::File(_)));
    }
}
