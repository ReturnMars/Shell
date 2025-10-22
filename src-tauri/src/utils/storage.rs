// 存储工具 - 待实现
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf, String> {
    dirs::config_dir()
        .ok_or_else(|| "无法获取配置目录".to_string())
        .map(|mut path| {
            path.push("ShellMars");
            path
        })
}

pub fn get_data_dir() -> Result<PathBuf, String> {
    dirs::data_dir()
        .ok_or_else(|| "无法获取数据目录".to_string())
        .map(|mut path| {
            path.push("ShellMars");
            path
        })
}
