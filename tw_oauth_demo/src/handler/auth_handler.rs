use crate::app_config::AppConfig;
use crate::twitter::RequestAuth;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};

/// リクエストトークンを取得して認証ページにリダイレクトする。
#[get("/auth")]
pub async fn auth(data: web::Data<AppConfig>, session: Session) -> impl Responder {
    let request_auth = RequestAuth::new();

    match request_auth.execute(&data, &session).await {
        Ok(redirect_url) => HttpResponse::Found()
            .header("Location", redirect_url)
            .finish(),
        Err(err) => {
            let content = format!("{:?}", &err);
            HttpResponse::Ok().body(content)
        }
    }
}
