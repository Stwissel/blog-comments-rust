/*
 * System configuration, from environment
 * loads localenv.toml if available
 * (C) 2020 @notessensei, Apache 2.0 license
 */

// Variable names pulled from environment
const CLIENT_SECRET: &str = "ClientSecret";
const CLIENT_TOKEN: &str = "ClientToken";
const PORT: &str = "PORT";
const CAPTCHA_SECRET: &str = "CaptchaSecret";
const REPOSITORY_URL: &str = "RepositoryURL";
const OAUTH_URL: &str = "OauthURL";
const PUSH_USER: &str = "PushUser";
const PUSH_TOKEN: &str = "PushToken";

//#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsConfig {
    pub client_secret: String,
    pub client_token: String,
    pub port: u16,
    pub captcha_secret: String,
    pub repository_url: String,
    pub oauth_url: String,
    pub push_user: String,
    pub push_token: String,
}

impl CommentsConfig {
    pub fn new() -> Self {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name("config/default"))
            .unwrap()
            .merge(config::File::with_name("config/localenv").required(false))
            .unwrap()
            .merge(config::Environment::default())
            .unwrap();

        CommentsConfig {
            client_secret: settings.get_str(CLIENT_SECRET).unwrap(),
            client_token: settings.get_str(CLIENT_TOKEN).unwrap(),
            port: settings.get_int(PORT).unwrap() as u16,
            captcha_secret: settings.get_str(CAPTCHA_SECRET).unwrap(),
            repository_url: settings.get_str(REPOSITORY_URL).unwrap(),
            oauth_url: settings.get_str(OAUTH_URL).unwrap(),
            push_user: settings.get_str(PUSH_USER).unwrap(),
            push_token: settings.get_str(PUSH_TOKEN).unwrap(),
        }
    }
}
