use crate::app_error::AppError;
use serde::Deserialize;

/// ユーザーの情報を表す構造体。
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub id_str: String,
    pub screen_name: String,
    pub name: String,
    pub description: String,
    pub profile_image_url_https: String,
}

impl User {
    /// APIからの返却値から生成する。
    pub fn from_json(json_string: &str) -> Result<Self, AppError> {
        let twitter_user: Self = serde_json::from_str(json_string)?;
        Ok(twitter_user)
    }
}
