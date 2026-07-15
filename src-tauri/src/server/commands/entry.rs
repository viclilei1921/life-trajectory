use tauri::State;

use crate::server::dto::EntryDto;
use crate::server::handlers;
use crate::server::state::AppState;
use crate::server::transport::to_command_result;

/// 创建 Entry
/// 
/// @param content - Entry 内容
/// @param happened_at - Entry 发生时间
/// @param title - Entry 标题
/// 
/// @return  Entry
#[tauri::command]
pub async fn create_entry(
  state: State<'_, AppState>,
  content: String,
  happened_at: i64,
  title: Option<String>,
) -> Result<EntryDto, String> {
  let vault = state.vault.lock().await;
  to_command_result(handlers::entry::create(&state.db, &vault, content, happened_at, title).await)
}

/// 获取 Entry
/// 
/// @param id - Entry ID
/// 
/// @return  Entry
#[tauri::command]
pub async fn get_entry(state: State<'_, AppState>, id: String) -> Result<EntryDto, String> {
  let vault = state.vault.lock().await;
  to_command_result(handlers::entry::get(&state.db, &vault, id).await)
}
