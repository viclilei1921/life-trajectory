pub mod entry;
pub mod vault;

pub use entry::{create_entry, get_entry};
pub use vault::{
  vault_change_password, vault_get_status, vault_lock, vault_setup_password, vault_unlock,
};
