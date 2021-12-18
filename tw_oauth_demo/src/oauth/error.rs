//! OAuthリクエスト処理内で発生するエラー
use thiserror::Error;

/// OAuthリクエスト処理内で発生するエラーを表す列挙子。
#[derive(Error, Debug)]
pub enum OauthError {
    #[error(transparent)]
    AwcPayload(#[from] awc::error::PayloadError),

    #[error(transparent)]
    AwcSendRequest(#[from] awc::error::SendRequestError),

    #[error(transparent)]
    StringFromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}
