use std::path::{Path, PathBuf};

/// 应用运行时配置（路径等）
#[derive(Debug, Clone)]
pub struct AppConfig {
  /// 应用数据目录
  pub data_dir: PathBuf,
  /// DB 路径
  /// 通常是 data_dir/life.db
  pub db_path: PathBuf,
}

/// 从应用数据目录创建配置
impl AppConfig {
  /// 从应用数据目录创建配置
  pub fn from_app_data_dir(app_data_dir: &Path) -> Self {
    let data_dir = app_data_dir.to_path_buf();
    let db_path = data_dir.join("life.db");
    Self { data_dir, db_path }
  }
}
