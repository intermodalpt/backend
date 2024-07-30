use once_cell::sync::OnceCell;
use serde::Deserialize;

pub(crate) static SETTINGS: OnceCell<Settings> = OnceCell::new();

#[derive(Deserialize, Debug)]
pub(crate) struct Settings {
    pub(crate) http: Http,
    pub(crate) db: Database,
    pub(crate) s3: S3Api,
    pub(crate) jwt: Jwt,
    pub(crate) cookies: Cookies,
    pub(crate) images: Images,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Database {
    pub(crate) url: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Http {
    pub(crate) port: u16,
}

#[derive(Deserialize, Debug)]
pub(crate) struct S3Api {
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) bucket_name: String,
    pub(crate) account_id: String,
}
#[derive(Deserialize, Debug)]
pub(crate) struct Jwt {
    pub(crate) refresh_secret: String,
    pub(crate) refresh_days: i64,
    pub(crate) access_secret: String,
    pub(crate) access_minutes: i64,
    pub(crate) management_secret: String,
    pub(crate) management_days: i64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Cookies {
    pub(crate) domain: String,
    pub(crate) secure: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Images {
    pub(crate) root: String,
}

pub(crate) fn load() {
    let file = std::fs::read_to_string("./settings.toml")
        .expect("Could not read settings file");
    let settings: Settings = toml::from_str(&file).unwrap();
    SETTINGS.set(settings).expect("Unable to set settings");
}
