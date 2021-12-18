//! サービスプロバイダ(Server)からコールバックで戻ってきたときのクエリパラメータ
use serde::Deserialize;

/// サービスプロバイダ(Server)からコールバックで戻ってきたときのクエリパラメータを表す構造体。
#[derive(Debug, Deserialize)]
pub struct Info {
    pub oauth_token: Option<String>,
    pub oauth_verifier: Option<String>,
    pub denied: Option<String>,
}
