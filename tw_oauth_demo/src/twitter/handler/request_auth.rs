use crate::app_config::AppConfig;
use crate::app_error::AppError;
use crate::oauth::auth::Consumer;
use crate::oauth::temporary_credential::TemporaryCredential;
use crate::twitter::ApiRequestToken;
use actix_session::Session;
use actix_web::web;

/// リクエストトークンの取得を行う。
#[derive(Debug)]
pub struct RequestAuth {}

impl RequestAuth {
    /// 初期化する。
    pub fn new() -> Self {
        Self {}
    }

    // リクエストトークンを取得してセッションに保存する
    pub async fn execute(
        &self,
        data: &web::Data<AppConfig>,
        session: &Session,
    ) -> Result<String, AppError> {
        let consumer = Consumer::new(&data.consumer_key, &data.consumer_secret);
        let callback_url = urlencoding::encode(&data.callback_url);

        // リクエストトークンの取得を要求する
        let api_request_token = ApiRequestToken::new();
        let body_string = api_request_token.execute(&consumer, &callback_url).await?;

        // 返却値からリクエストトークンを取り出す
        let temporary_credential = TemporaryCredential::from_string(&body_string)?;

        // トークンをセッションに保存する
        let _ = session.set("oauth_token", &temporary_credential.oauth_token)?;

        // 認証用ページのURLを生成する
        let redirect_url = format!(
            "https://api.twitter.com/oauth/authorize?oauth_token={}",
            &temporary_credential.oauth_token
        );
        Ok(redirect_url)
    }
}
