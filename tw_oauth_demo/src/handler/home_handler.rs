use crate::app_config::AppConfig;
use crate::oauth::info::Info;
use crate::twitter::AuthAndVerify;
use crate::view::HomeView;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};

/// アクセストークンを取得してユーザーの情報を取得する。
#[get("/home")]
pub async fn home(
    info: web::Query<Info>,
    data: web::Data<AppConfig>,
    session: Session,
) -> impl Responder {
    let auth_and_verify = AuthAndVerify::new();

    // アクセストークンを取得してユーザーの情報を取得する。
    let twitter_user = match auth_and_verify.execute(&info, &data, &session).await {
        Ok(twitter_user) => twitter_user,
        Err(err) => {
            eprintln!("{:?}", &err);
            None
        }
    };

    // HTMLを生成して返却する。
    let home_view = HomeView::new();
    let content = match home_view.generate(twitter_user.as_ref()) {
        Ok(content) => content,
        Err(err) => format!("{:?}", &err),
    };
    HttpResponse::Ok().body(content)
}
