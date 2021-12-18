use crate::app_config::AppConfig;
use crate::app_error::AppError;
use crate::oauth::auth::{AccessToken, Consumer};
use crate::oauth::info::Info;
use crate::oauth::token_credential::TokenCredential;
use crate::twitter::{ApiAccessToken, ApiVerifyCredentials, User};
use actix_session::Session;
use actix_web::web;

/// アクセストークンとユーザー情報の取得を行う。
#[derive(Debug)]
pub struct AuthAndVerify {}

impl AuthAndVerify {
    /// 初期化する。
    pub fn new() -> Self {
        Self {}
    }

    /// 認証に成功しているかを判定し、ユーザー情報の取得を行う。
    pub async fn execute(
        &self,
        info: &web::Query<Info>,
        data: &web::Data<AppConfig>,
        session: &Session,
    ) -> Result<Option<User>, AppError> {
        let twitter_user = match &info.oauth_verifier {
            Some(oauth_verifier) => {
                let twitter_user = self
                    .make_twitter_user(&oauth_verifier, data, session)
                    .await?;
                Some(twitter_user)
            }
            None => None,
        };
        Ok(twitter_user)
    }

    /// 
    async fn make_twitter_user(
        &self,
        oauth_verifier: &str,
        data: &web::Data<AppConfig>,
        session: &Session,
    ) -> Result<User, AppError> {
        let consumer = Consumer::new(&data.consumer_key, &data.consumer_secret);

        // アクセストークンの取得を要求する
        let token_string = self
            .make_token_string(&consumer, oauth_verifier, session)
            .await?;

        // 返却された文字列からアクセストークンを取得する
        let token_credential = TokenCredential::from_string(&token_string)?;
        println!("{:?}", &token_credential);

        // ユーザー情報の取得を行う
        let user_json = self.make_user_json(&consumer, &token_credential).await?;

        // 返却されたJSON文字列からユーザーの情報を取り出す。
        let twitter_user = User::from_json(&user_json)?;
        println!("{:?}", &twitter_user);

        Ok(twitter_user)
    }

    /// アクセストークンの取得を要求する。
    async fn make_token_string(
        &self,
        consumer: &Consumer,
        oauth_verifier: &str,
        session: &Session,
    ) -> Result<String, AppError> {
        let oauth_token = session
            .get::<String>("oauth_token")?
            .unwrap_or("".to_owned());

        let api_access_token = ApiAccessToken::new();
        let token_string = api_access_token
            .execute(&consumer, &oauth_token, oauth_verifier)
            .await?;
        Ok(token_string)
    }

    /// ユーザー情報の取得を行う。
    async fn make_user_json(
        &self,
        consumer: &Consumer,
        token_credential: &TokenCredential,
    ) -> Result<String, AppError> {
        let access_token = AccessToken::new(
            &consumer,
            &token_credential.oauth_token,
            &token_credential.oauth_token_secret,
        );

        let api_verify_credentials = ApiVerifyCredentials::new();
        let body_string = api_verify_credentials.execute(&access_token).await?;
        Ok(body_string)
    }
}
