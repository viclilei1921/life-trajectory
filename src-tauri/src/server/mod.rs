pub mod commands;
pub mod config;
pub mod db;
pub mod dto;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod state;
pub mod transport;

use log::error;
use tauri::{App, Manager};
use tokio::sync::Mutex;

use crate::server::services::VaultService;
use crate::server::state::AppState;

/// 启动时初始化 DB + Vault 会话并注入 AppState
/// 拿到Tauri的应用数据目录
/// 确定DB路径并初始化连接池
/// 从 DB 读 Vault 配置，初始化/恢复会话
/// 注入AppState
pub fn bootstrap(app: &App) -> Result<(), Box<dyn std::error::Error>> {
  let handle = app.handle().clone();

  // block_on是因为 setup 是同步的，而 DB 初始化是 async，需要在这里阻塞等待完成。
  let result = tauri::async_runtime::block_on(async move {
    let app_data = handle.path().app_data_dir()?;
    let cfg = config::AppConfig::from_app_data_dir(&app_data);
    let pool = db::init_pool(&cfg.db_path).await?;
    let session = VaultService::bootstrap_session(&pool).await?;

    // Tauri 命令需要共享、长生命周期的后端状态
    // 前端每次 invoke 命令。通过 State<'_, AppState> 拿到同一份全局状态
    handle.manage(AppState {
      db: pool,
      vault: Mutex::new(session),
    });

    Ok::<(), Box<dyn std::error::Error>>(())
  });

  if let Err(ref e) = result {
    error!("server bootstrap failed: {e}");
  }

  result
}
