//! トークンクレデンシャル
use super::error::OauthError;
use url::Url;

/// トークンクレデンシャルを表す構造体。
#[derive(Debug)]
pub struct TokenCredential {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub user_id: Option<String>,
    pub screen_name: Option<String>,
}

impl TokenCredential {
    /// クエリ文字列から生成する。
    pub fn from_string(credential_string: &str) -> Result<Self, OauthError> {
        // クエリ文字列の分解方法がわからないのでurlにまかせてみる
        let temp_url_string = format!("http://localhost/?{}", credential_string);
        let temp_url = Url::parse(&temp_url_string)?;

        let mut oauth_token = None;
        let mut oauth_token_secret = None;
        let mut user_id = None;
        let mut screen_name = None;

        for (key, value) in temp_url.query_pairs() {
            match key.to_string().as_str() {
                "oauth_token" => {
                    oauth_token = Some(value.to_string());
                }
                "oauth_token_secret" => {
                    oauth_token_secret = Some(value.to_string());
                }
                "user_id" => {
                    user_id = Some(value.to_string());
                }
                "screen_name" => {
                    screen_name = Some(value.to_string());
                }
                _ => {}
            };
        }
        let token_credential = Self {
            oauth_token: oauth_token.unwrap_or("".to_owned()),
            oauth_token_secret: oauth_token_secret.unwrap_or("".to_owned()),
            user_id,
            screen_name,
        };
        Ok(token_credential)
    }
}
