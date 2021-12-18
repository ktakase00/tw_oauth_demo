//! エラー定義
use thiserror::Error;

/// エラーを表す列挙子。
#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),

    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Oauth(#[from] crate::oauth::error::OauthError),

    #[error(transparent)]
    Template(#[from] handlebars::TemplateError),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}
