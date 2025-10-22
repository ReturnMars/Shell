### ShellMars 项目上下文（精简导出）

- 项目: ShellMars（Tauri + Vue3 + Naive UI + Rust）
- 平台: Windows/macOS/Linux
- 当前阶段: 基础框架开发 + 核心功能完善
- 进度:
  - T001 环境搭建: 完成 ✅
  - T002 基础UI框架: 完成 ✅（MainLayout/Sidebar/TabBar/ConnectionStatus）
  - T003.1-4: 完成 ✅（SSH依赖/模型/服务/Tauri命令）
  - T003.5: 完成 ✅（连接配置持久化：保存/加载/导入/导出/按名称去重/删除全部）
  - T003.6: 完成 ✅（前端连接管理组件：ConnectionForm + 现代化UI）
  - T003.7: 完成 ✅（集成测试 + Bug修复 + 功能优化）

#### 关键能力与实现

- SSH 服务（Rust）
  - 文件: `src-tauri/src/services/ssh_service.rs`
  - 功能: 连接建立（密码/密钥/混合）、命令执行、主动断开/批量断开、状态与连接池管理（`RwLock<HashMap>`）

- 持久化存储（Rust）
  - 文件: `src-tauri/src/services/connection_storage.rs`
  - 目录: 系统配置目录下 `ShellMars`
    - Windows: `C:\Users\<User>\AppData\Roaming\ShellMars`
    - macOS: `/Users/<User>/Library/Application Support/ShellMars`
    - Linux: `/home/<User>/.config/ShellMars`
  - 文件: `connections.json`（连接配置，密码加密），`master.key`（AES-256-GCM 主密钥）
  - 策略: 按连接名称自动重命名（name(1), name(2)...）；支持删除全部；UUID生成优化

- 加密
  - 文件: `src-tauri/src/utils/crypto.rs`
  - 算法: AES-256-GCM（随机 nonce），PBKDF2；Base64 仅用于序列化
  - 说明: 已抑制第三方内部的 `generic-array` 警告

- Tauri 命令
  - 连接命令: `src-tauri/src/commands/connection_commands.rs`
    - `connect_ssh(config) -> String`
    - `disconnect_ssh(connectionId) -> ()`
    - `disconnect_all_ssh() -> ()`
    - `get_connection_status(connectionId) -> ConnectionStatus`
    - `get_connections() -> Vec<ConnectionConfig>`
    - `execute_ssh_command(connection_id, command) -> String`（参数名已修复）
    - `test_connection(config) -> String`（测试后自动断开）
    - `generate_uuid() -> String`（UUID生成）
  - 存储命令: `src-tauri/src/commands/storage_commands.rs`
    - `save_connection(config) -> ()`（自动重命名重复名称）
    - `load_connection(connectionId) -> ConnectionConfig`
    - `update_connection(config) -> ()`（更新现有连接）
    - `delete_connection(connectionId) -> ()`
    - `get_saved_connections() -> Vec<ConnectionConfig>`
    - `export_connections() -> String`
    - `import_connections(jsonData) -> ()`（自动重命名重复名称）
    - `delete_all_connections() -> ()`
  - 注册: `src-tauri/src/lib.rs` 中 `invoke_handler([...])`

- 前端（Vue3 + Naive UI + Pinia）
  - 状态管理: `src/stores/connection/`
    - `index.ts`: Pinia store，管理连接状态和CRUD操作
    - `type.d.ts`: 连接相关类型定义
    - 功能: 连接池管理、选中连接、CRUD操作、状态同步
  - 文件: `src/components/core/MainLayout.vue`
  - 已接入按钮与逻辑:
    - 测试SSH连接 → `test_connection`（打开ConnectionForm）
    - 测试命令 → `execute_ssh_command`（参数名已修复）
    - 断开所有连接 → `disconnect_all_ssh`
    - 保存当前连接 → `save_connection`
    - 加载保存的连接 → `get_saved_connections`
    - 清理所有连接 → `delete_all_connections`（设置菜单中）
    - 底部状态栏显示连接数（使用Pinia store）
  - 文件: `src/components/connection/ConnectionForm.vue`
  - 现代化连接表单组件:
    - 完整的连接配置表单（名称/主机/端口/用户名/认证方式/密码/私钥）
    - 动态测试按钮（成功/失败/测试中状态显示）
    - 自动保存功能（连接成功后自动保存）
    - Vue 3.4+ 现代化写法（defineModel、Composition API）
    - 完整的表单验证和错误处理
  - 文件: `src/components/connection/ConnectionList.vue`
  - 连接列表组件:
    - 使用Pinia store管理连接数据
    - hover显示操作按钮（编辑/删除）
    - 选中状态高亮显示
    - 响应式连接状态更新

#### 关键数据结构（Rust 概要）

- `ConnectionConfig`: `id,name,host,port,username,password?,private_key_path?,auth_method,created_at,updated_at`
  - 验证: 必填校验、认证方式约束
- `AuthMethod`: `Password | PrivateKey | Both`
- `ConnectionStatus`: `Disconnected | Connecting | Connected | Error(String)`
- `Session`: `id, connection_id, title, status, created_at, last_activity`

#### 成功测试记录（样例）

- 连接: `47.109.195.0:22` 用户 `root` → 连接成功，返回 `connectionId`
- 命令: `echo 'Hello from SSH!'` → 正常返回（参数名已修复）
- 断开: `disconnect_all_ssh` → 连接清空
- 持久化: `save_connection` 后 `get_saved_connections` 可读出，自动重命名策略生效，按时间倒序排序
- 测试连接: `test_connection` → 测试后自动断开，不保留在连接池
- UUID生成: `generate_uuid` → 返回标准UUID格式
- 清理功能: `delete_all_connections` → 一键清理所有连接和配置

#### 依赖（Cargo.toml 关键）

- SSH: `ssh2 = "0.9"`, `tokio = "1"`
- 安全: `aes-gcm = "0.10.3"`, `pbkdf2 = "0.12"`, `sha2 = "0.10"`, `rand = "0.8"`, `base64 = "0.21"`
- 工具: `serde/serde_json`, `lazy_static`, `log/env_logger`, `anyhow`, `uuid`, `chrono`, `dirs`

#### 下一步建议（T003.6）

- Sidebar：添加“保存的连接”列表与一键连接
- 连接表单：新增/编辑/删除连接（含密码/密钥）
- 导入导出：前端入口与文件选择
- 错误提示与状态反馈（Naive UI `n-message`）


