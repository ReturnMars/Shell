// 终端命令 - 待实现
use tauri::command;

#[command]
pub async fn execute_command() -> Result<String, String> {
    Ok("终端功能待实现".to_string())
}
