//! OAuthリクエストを送信する
use super::error::OauthError;
use super::string_pair::StringPairList;
use awc::ClientRequest;
use chrono::{DateTime, Utc};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use url::Url;
use uuid::Uuid;

/// OAuthリクエストを送信するための構造体。
#[derive(Debug)]
pub struct OauthClient {
    consumer_key: String,
    consumer_secret: String,
    oauth_token: Option<String>,
    oauth_token_secret: Option<String>,
}

impl OauthClient {
    /// 初期化する。
    pub fn new(
        consumer_key: &str,
        consumer_secret: &str,
        oauth_token: Option<&str>,
        oauth_token_secret: Option<&str>,
    ) -> Self {
        Self {
            consumer_key: consumer_key.to_owned(),
            consumer_secret: consumer_secret.to_owned(),
            oauth_token: oauth_token.map(|x| x.to_owned()),
            oauth_token_secret: oauth_token_secret.map(|x| x.to_owned()),
        }
    }

    /// POST で送信する。
    pub async fn post(&self, url_string: &str) -> Result<String, OauthError> {
        let authorization_content = self.make_authorization("POST", url_string)?;
        let client = awc::Client::default();
        let client_request = client.post(url_string);
        let body_string = self
            .send_request(client_request, &authorization_content)
            .await?;
        Ok(body_string)
    }

    /// GET で送信する。
    pub async fn get(&self, url_string: &str) -> Result<String, OauthError> {
        let authorization_content = self.make_authorization("GET", url_string)?;
        let client = awc::Client::default();
        let client_request = client.get(url_string);
        let body_string = self
            .send_request(client_request, &authorization_content)
            .await?;
        Ok(body_string)
    }

    /// HTTPリクエストを送信する。
    async fn send_request(
        &self,
        client_request: ClientRequest,
        authorization_content: &str,
    ) -> Result<String, OauthError> {
        let mut response = client_request
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded".to_owned(),
            )
            .header("Authorization", authorization_content.to_owned())
            .send()
            .await?;
        // println!("Response: {:?}", response);
        let body = response.body().await?;
        let body_string = String::from_utf8(body.to_vec())?;
        // println!("{}", &body_string);
        Ok(body_string)
    }

    /// Authorization HTTPヘッダーフィールドの値を生成する。
    fn make_authorization(&self, method: &str, url_string: &str) -> Result<String, OauthError> {
        let url = Url::parse(url_string)?;

        // ベースストリング URI
        let base_string_uri = self.make_base_uri(&url);

        // タイムスタンプとノンス
        let current_at: DateTime<Utc> = Utc::now();
        let oauth_timestamp = format!("{}", &current_at.timestamp());
        let oauth_nonce = Uuid::new_v4().to_simple().to_string();

        // シグニチャベースストリングに含めるパラメータ
        let mut signature_params = StringPairList::new();
        signature_params
            .add("oauth_consumer_key", &self.consumer_key)
            .add("oauth_signature_method", "HMAC-SHA1")
            .add("oauth_nonce", &oauth_nonce)
            .add("oauth_timestamp", &oauth_timestamp)
            .add("oauth_version", "1.0");
        // oauth_token があれば追加する
        if let Some(oauth_token) = &self.oauth_token {
            signature_params.add("oauth_token", oauth_token);
        }
        // クエリパラメータを追加する
        let pairs = url.query_pairs();
        for (key, value) in pairs {
            signature_params.add(&key, &value);
        }
        // println!("{}", url_string);
        // println!("{:?}", &signature_params);
        // println!("{}", &base_string_uri);

        // シグニチャベースストリング
        let signature_base_string =
            self.make_signature_base_string(method, &base_string_uri, &signature_params);
        // println!("{}", &signature_base_string);

        // oauth_signature
        let oauth_token_secret = self.oauth_token_secret.as_ref().map(|x| x.clone());
        let oauth_signature = self.make_oauth_signature(
            &self.consumer_secret,
            &signature_base_string,
            oauth_token_secret,
        );
        // println!("{}", &oauth_signature);

        // パラメータに oauth_signature を追加
        signature_params.add("oauth_signature", &oauth_signature);

        // Authorization ヘッダーフィールドの値を生成
        let authorization_content = signature_params.make_authorization();
        // println!("{}", &authorization_content);
        Ok(authorization_content)
    }

    // ベースストリング URI を組み立てる。
    fn make_base_uri(&self, url: &Url) -> String {
        format!(
            "{}://{}{}",
            url.scheme(),
            url.host_str().unwrap_or(""),
            url.path()
        )
    }

    // シグニチャベースストリングを組み立てる。
    fn make_signature_base_string(
        &self,
        request_method: &str,
        base_uri: &str,
        signature_params: &StringPairList,
    ) -> String {
        let method = request_method.to_uppercase();

        let src_uri_string = &base_uri.to_lowercase();
        let uri = urlencoding::encode(&src_uri_string);

        let query = signature_params.make_query();
        // println!("{}", &query);
        let encoded_query = urlencoding::encode(&query);

        format!("{}&{}&{}", &method, &uri, &encoded_query)
    }

    // oauth_signature を生成する。
    fn make_oauth_signature(
        &self,
        consumer_secret: &str,
        signature_base_string: &str,
        oauth_token_secret: Option<String>,
    ) -> String {
        let key_string = format!(
            "{}&{}",
            consumer_secret,
            oauth_token_secret.unwrap_or("".to_owned())
        );

        let hasher = Sha1::new();
        let mut hmac = Hmac::new(hasher, &key_string.into_bytes());
        hmac.input(&signature_base_string.to_owned().into_bytes());
        base64::encode(hmac.result().code())
    }
}
