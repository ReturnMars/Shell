// 文件命令 - 待实现
use tauri::command;

#[command]
pub async fn upload_file() -> Result<String, String> {
    Ok("文件功能待实现".to_string())
}
