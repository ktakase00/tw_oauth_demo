//! 設定情報
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

/// 設定情報を扱う構造体。
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub callback_url: String,
}

impl AppConfig {
    /// 環境変数から設定情報を生成する。
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        let env_params = Environment::new();
        s.merge(env_params)?;
        s.try_into()
    }
}
