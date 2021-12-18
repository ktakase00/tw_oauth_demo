use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tw_oauth_demo::app_config::AppConfig;
use tw_oauth_demo::handler::auth;
use tw_oauth_demo::handler::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{:?}", &err);
            panic!();
        }
    };
    // println!("{:?}", &app_config);
    HttpServer::new(move || {
        App::new()
            .data(app_config.clone())
            .wrap(
                CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false),
            )
            .service(home)
            .service(auth)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
