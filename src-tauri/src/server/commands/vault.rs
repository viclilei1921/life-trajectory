use tauri::State;

use crate::server::dto::VaultStatusDto;
use crate::server::handlers;
use crate::server::state::AppState;
use crate::server::transport::to_command_result;

/// 获取 Vault 状态
/// 
/// @return  Vault 状态
#[tauri::command]
pub async fn vault_get_status(state: State<'_, AppState>) -> Result<VaultStatusDto, String> {
  let vault = state.vault.lock().await;
  to_command_result(handlers::vault::get_status(&state.db, &vault).await)
}

/// 解锁 Vault
/// 
/// @param password - 密码
/// 
/// @return  Vault 状态
#[tauri::command]
pub async fn vault_unlock(state: State<'_, AppState>, password: String) -> Result<VaultStatusDto, String> {
  let mut vault = state.vault.lock().await;
  to_command_result(handlers::vault::unlock(&state.db, &mut vault, password).await)
}

/// 锁定 Vault
/// 
/// @return  Vault 状态
#[tauri::command]
pub async fn vault_lock(state: State<'_, AppState>) -> Result<VaultStatusDto, String> {
  let mut vault = state.vault.lock().await;
  to_command_result(handlers::vault::lock(&state.db, &mut vault).await)
}

/// 设置 Vault 密码
///
/// @param password - 密码
/// 
/// @return  Vault 状态
#[tauri::command]
pub async fn vault_setup_password(state: State<'_, AppState>, password: String) -> Result<VaultStatusDto, String> {
  let mut vault = state.vault.lock().await;
  to_command_result(handlers::vault::setup_password(&state.db, &mut vault, password).await)
}

/// 修改 Vault 密码
/// 
/// @param state - 应用状态
/// @param old - 旧密码
/// @param new - 新密码
/// 
/// @return 结果
#[tauri::command]
pub async fn vault_change_password(state: State<'_, AppState>, old: String, new: String) -> Result<(), String> {
  let vault = state.vault.lock().await;
  to_command_result(handlers::vault::change_password(&state.db, &vault, old, new).await)
}
