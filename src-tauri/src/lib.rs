// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 模块声明
pub mod commands;
pub mod models;
pub mod services;
pub mod utils;

// 重新导出常用类型
pub use commands::connection_commands::*;
pub use commands::hardware_commands::*;
pub use commands::storage_commands::*;
pub use models::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志 - 设置日志级别为 INFO
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            connect_ssh,
            disconnect_ssh,
            disconnect_all_ssh,
            get_connection_status,
            get_connections,
            get_connected_count,
            get_connected_connections,
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
            delete_all_connections,
            // 标签页相关命令
            get_tabs_list,
            add_tab,
            remove_tab,
            set_active_tab,
            get_active_tab,
            close_all_tabs,
            close_other_tabs,
            get_tab_by_connection_id,
            // 连接健康检查
            check_connection_status,
            reconnect_ssh,
            // 硬件信息相关命令
            get_hardware_info,
            get_cpu_info,
            get_memory_info,
            get_storage_info,
            get_network_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
