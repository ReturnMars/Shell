// 错误处理 - 待实现
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("SSH连接错误: {0}")]
    SshError(String),
    
    #[error("文件操作错误: {0}")]
    FileError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
