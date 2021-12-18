//! Actix Webに対するリクエストのハンドラー
mod auth_handler;
mod home_handler;

pub use auth_handler::auth;
pub use home_handler::home;
