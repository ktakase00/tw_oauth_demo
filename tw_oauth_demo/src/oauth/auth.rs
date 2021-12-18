//! コンシューマ(Client)
use super::error::OauthError;
use super::request::OauthClient;

/// コンシューマ(Client)を表す構造体。
#[derive(Debug, Clone)]
pub struct Consumer {
    consumer_key: String,
    consumer_secret: String,
}

impl Consumer {
    /// 初期化する。
    pub fn new(consumer_key: &str, consumer_secret: &str) -> Self {
        Self {
            consumer_key: consumer_key.to_owned(),
            consumer_secret: consumer_secret.to_owned(),
        }
    }

    /// POSTメソッドでリクエストを送信する。
    pub async fn post(&self, url_string: &str) -> Result<String, OauthError> {
        let oauth_client = OauthClient::new(&self.consumer_key, &self.consumer_secret, None, None);
        let body_string = oauth_client.post(url_string).await?;
        Ok(body_string)
    }

    /// GETメソッドでリクエストを送信する。
    pub async fn get(&self, url_string: &str) -> Result<String, OauthError> {
        let oauth_client = OauthClient::new(&self.consumer_key, &self.consumer_secret, None, None);
        let body_string = oauth_client.get(url_string).await?;
        Ok(body_string)
    }
}

/// アクセストークンを表す構造体。
#[derive(Debug)]
pub struct AccessToken {
    consumer: Consumer,
    access_token: String,
    access_token_secret: String,
}

impl AccessToken {
    /// 初期化する。
    pub fn new(consumer: &Consumer, access_token: &str, access_token_secret: &str) -> Self {
        Self {
            consumer: consumer.clone(),
            access_token: access_token.to_owned(),
            access_token_secret: access_token_secret.to_owned(),
        }
    }

    /// POSTメソッドでリクエストを送信する。
    pub async fn post(&self, url_string: &str) -> Result<String, OauthError> {
        let oauth_client = OauthClient::new(
            &self.consumer.consumer_key,
            &self.consumer.consumer_secret,
            Some(&self.access_token),
            Some(&self.access_token_secret),
        );
        let body_string = oauth_client.post(url_string).await?;
        Ok(body_string)
    }

    /// GETメソッドでリクエストを送信する。
    pub async fn get(&self, url_string: &str) -> Result<String, OauthError> {
        let oauth_client = OauthClient::new(
            &self.consumer.consumer_key,
            &self.consumer.consumer_secret,
            Some(&self.access_token),
            Some(&self.access_token_secret),
        );
        let body_string = oauth_client.get(url_string).await?;
        Ok(body_string)
    }
}
