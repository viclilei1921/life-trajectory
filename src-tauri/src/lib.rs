pub mod crypto;
pub mod server;

use log::LevelFilter;
use server::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(
      tauri_plugin_log::Builder::new()
        .level(LevelFilter::Info)
        .filter(|metadata| metadata.target() != "hyper")
        .build(),
    )
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      server::bootstrap(app)?;
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      vault_get_status,
      vault_unlock,
      vault_lock,
      vault_setup_password,
      vault_change_password,
      create_entry,
      get_entry,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
