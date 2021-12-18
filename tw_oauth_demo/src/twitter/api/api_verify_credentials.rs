use crate::app_error::AppError;
use crate::oauth::auth::AccessToken;

/// ユーザーの情報を取得する。
#[derive(Debug)]
pub struct ApiVerifyCredentials {}

impl ApiVerifyCredentials {
    /// 初期化する。
    pub fn new() -> Self {
        Self {}
    }

    /// APIの呼び出しを実行する。
    pub async fn execute(&self, access_token: &AccessToken) -> Result<String, AppError> {
        let url = format!("https://api.twitter.com/1.1/account/verify_credentials.json");
        let body_string = access_token.get(&url).await?;
        // println!("{}", &body_string);
        Ok(body_string)
    }
}
