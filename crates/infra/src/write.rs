use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum OverwritePolicy {
    /// 如果目标文件已存在（例如 pom.xml），直接返回错误，不做任何修改。
    AbortIfExists,
    /// 如果目标文件已存在直接覆盖。
    OverwriteIfExists,
    /// 如果目标文件已存在，提示用户选择是否覆盖。
    AskIfExists,
    /// 如果目标文件已存在，自动重命名新文件以避免覆盖（例如 pom(count).xml）。
    RenameIfExists,
}

/// 文件夹存在的处理策略
#[derive(Debug, Clone, Copy)]
pub enum DirExistPolicy {
    IgnoreIfExists, // 目录已存在就 OK
    AbortIfExists,  // 已存在报错
    EmptyRequired,  // 必须为空（检查 read_dir 是否空）
}

/// 写入文件的通用接口
pub trait FileWrite {
    fn write<P: AsRef<Path>>(&self, path: P, policy: OverwritePolicy) -> Result<PathBuf, Box<dyn Error>>;
}

/// 创建目录的通用接口
pub trait FolderCreator {
    fn create<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>>;
}