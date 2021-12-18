use crate::app_error::AppError;
use crate::oauth::auth::Consumer;

/// アクセストークンの取得を要求する。
#[derive(Debug)]
pub struct ApiAccessToken {}

impl ApiAccessToken {
    /// 初期化する。
    pub fn new() -> Self {
        Self {}
    }

    /// APIの呼び出しを実行する。
    pub async fn execute(
        &self,
        consumer: &Consumer,
        oauth_token: &str,
        oauth_verifier: &str,
    ) -> Result<String, AppError> {
        let url = format!(
            "https://api.twitter.com/oauth/access_token?oauth_token={}&oauth_verifier={}",
            &oauth_token, oauth_verifier
        );
        let body_string = consumer.get(&url).await?;
        // println!("{}", &body_string);
        Ok(body_string)
    }
}
