use crate::app_error::AppError;
use crate::oauth::auth::Consumer;

/// リクエストトークンの取得を要求する。
#[derive(Debug)]
pub struct ApiRequestToken {}

impl ApiRequestToken {
    /// 初期化する。
    pub fn new() -> Self {
        Self {}
    }

    /// APIの呼び出しを実行する。
    pub async fn execute(
        &self,
        consumer: &Consumer,
        callback_url: &str,
    ) -> Result<String, AppError> {
        let url = format!(
            "https://api.twitter.com/oauth/request_token?oauth_callback={}",
            callback_url
        );
        let body_string = consumer.post(&url).await?;
        // println!("{}", &body_string);
        Ok(body_string)
    }
}
