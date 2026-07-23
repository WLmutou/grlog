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
    /// 同时输出到多个目标
    Multi(Vec<Target>),
}

impl Default for Target {
    fn default() -> Self {
        Target::Stdout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_target() {
        let target = Target::default();
        assert!(matches!(target, Target::Stdout));
    }

    #[test]
    fn test_target_variants() {
        let stdout = Target::Stdout;
        let stderr = Target::Stderr;
        let file = Target::File(PathBuf::from("/tmp/test.log"));
        let multi = Target::Multi(vec![Target::Stdout, Target::File(PathBuf::from("/tmp/test.log"))]);

        assert!(matches!(stdout, Target::Stdout));
        assert!(matches!(stderr, Target::Stderr));
        assert!(matches!(file, Target::File(_)));
        assert!(matches!(multi, Target::Multi(_)));
    }

    #[test]
    fn test_target_clone() {
        let target = Target::File(PathBuf::from("/tmp/test.log"));
        let cloned = target.clone();
        assert!(matches!(cloned, Target::File(_)));

        let multi = Target::Multi(vec![Target::Stdout, Target::Stderr]);
        let cloned_multi = multi.clone();
        assert!(matches!(cloned_multi, Target::Multi(_)));
    }
}
