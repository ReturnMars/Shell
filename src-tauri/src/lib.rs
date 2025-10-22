// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 模块声明
pub mod models;
pub mod services;
pub mod commands;
pub mod utils;

// 重新导出常用类型
pub use models::*;
pub use commands::connection_commands::*;
pub use commands::storage_commands::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            connect_ssh,
            disconnect_ssh,
            disconnect_all_ssh,
            get_connection_status,
            get_connections,
            execute_ssh_command,
            test_connection,
            generate_uuid,
            save_connection,
            load_connection,
            update_connection,
            delete_connection,
            get_saved_connections,
            export_connections,
            import_connections,
            delete_all_connections
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
