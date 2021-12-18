use crate::app_error::AppError;
use crate::twitter::User;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

/// ページ表示用のHTMLを生成する。
#[derive(Debug)]
pub struct HomeView {}

impl HomeView {
    /// 初期化する。
    pub fn new() -> Self {
        Self {}
    }

    /// ページ表示用のHTMLを生成する。
    pub fn generate(&self, twitter_user: Option<&User>) -> Result<String, AppError> {
        let file = File::open("template/home.html.hbs")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("t1", contents)?;
        let mut data = BTreeMap::new();
        if let Some(twitter_user) = twitter_user {
            data.insert("screen_name".to_string(), twitter_user.screen_name.clone());
            data.insert(
                "profile_image_url_https".to_string(),
                twitter_user.profile_image_url_https.clone(),
            );
        }
        contents = handlebars.render("t1", &data)?;
        Ok(contents)
    }
}
