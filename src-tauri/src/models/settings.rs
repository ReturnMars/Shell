use serde::{Deserialize, Serialize};

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: Theme,
    pub language: String,
    pub auto_save: bool,
    pub connection_timeout: u64,
    pub terminal_settings: TerminalSettings,
    pub file_manager_settings: FileManagerSettings,
}

/// 主题设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// 终端设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub font_family: String,
    pub font_size: u16,
    pub background_color: String,
    pub foreground_color: String,
    pub cursor_color: String,
    pub scrollback_lines: usize,
}

/// 文件管理器设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManagerSettings {
    pub show_hidden_files: bool,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub default_view: ViewMode,
}

/// 排序方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortBy {
    Name,
    Size,
    Modified,
    Type,
}

/// 排序顺序
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// 视图模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewMode {
    List,
    Grid,
    Tree,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Auto,
            language: "zh-CN".to_string(),
            auto_save: true,
            connection_timeout: 30,
            terminal_settings: TerminalSettings::default(),
            file_manager_settings: FileManagerSettings::default(),
        }
    }
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            font_family: "FiraCode".to_string(),
            font_size: 14,
            background_color: "#1e1e1e".to_string(),
            foreground_color: "#ffffff".to_string(),
            cursor_color: "#ffffff".to_string(),
            scrollback_lines: 1000,
        }
    }
}

impl Default for FileManagerSettings {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            sort_by: SortBy::Name,
            sort_order: SortOrder::Ascending,
            default_view: ViewMode::List,
        }
    }
}
