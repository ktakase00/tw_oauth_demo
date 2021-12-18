//! テンポラリクレデンシャル(リクエストトークン)
use super::error::OauthError;
use url::Url;

/// テンポラリクレデンシャル(リクエストトークン)を表す構造体。
#[derive(Debug)]
pub struct TemporaryCredential {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: bool,
}

impl TemporaryCredential {
    /// クエリ文字列から生成する。
    pub fn from_string(credential_string: &str) -> Result<Self, OauthError> {
        // クエリ文字列の分解方法がわからないのでurlにまかせてみる
        let temp_url_string = format!("http://localhost/?{}", credential_string);
        let temp_url = Url::parse(&temp_url_string)?;

        let mut oauth_token = None;
        let mut oauth_token_secret = None;
        let mut oauth_callback_confirmed = None;

        for (key, value) in temp_url.query_pairs() {
            match key.to_string().as_str() {
                "oauth_token" => {
                    oauth_token = Some(value.to_string());
                }
                "oauth_token_secret" => {
                    oauth_token_secret = Some(value.to_string());
                }
                "oauth_callback_confirmed" => {
                    oauth_callback_confirmed = Some(value.to_string().as_str() == "true");
                }
                _ => {}
            };
        }
        let temporary_credential = Self {
            oauth_token: oauth_token.unwrap_or("".to_owned()),
            oauth_token_secret: oauth_token_secret.unwrap_or("".to_owned()),
            oauth_callback_confirmed: oauth_callback_confirmed.unwrap_or(false),
        };
        Ok(temporary_credential)
    }
}
