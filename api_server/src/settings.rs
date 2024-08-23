/*
    Intermodal, transportation information aggregator
    Copyright (C) 2024  Cláudio Pereira

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use once_cell::sync::OnceCell;
use serde::Deserialize;

pub(crate) static SETTINGS: OnceCell<Settings> = OnceCell::new();

#[derive(Deserialize, Debug)]
pub(crate) struct Settings {
    pub(crate) http: Http,
    #[serde(default)]
    pub(crate) storage: Storage,
    pub(crate) db: Database,
    pub(crate) s3: S3Api,
    pub(crate) jwt: Jwt,
    pub(crate) cookies: Cookies,
    pub(crate) images: Images,
}

fn default_data_root() -> String {
    "./data".to_string()
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Storage {
    #[serde(default = "default_data_root")]
    pub(crate) root: String,
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
    #[serde(default)]
    pub(crate) endpoint: Option<String>,
    #[serde(default)]
    pub(crate) account_id: Option<String>,
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) bucket_name: String,
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
    let file = if std::path::Path::new("./settings.toml").exists() {
        match std::fs::read_to_string("./settings.toml") {
            Ok(file) => file,
            Err(e) => {
                eprint!("Could not read local settings file: {}", e);
                std::process::exit(-1);
            }
        }
    } else if std::path::Path::new("/conf/settings.toml").exists() {
        match std::fs::read_to_string("/conf/settings.toml") {
            Ok(file) => file,
            Err(e) => {
                eprint!("Could not read container settings file: {}", e);
                std::process::exit(-1);
            }
        }
    } else {
        eprint!("No settings file found");
        std::process::exit(-1);
    };

    match toml::from_str(&file) {
        Ok(settings) => SETTINGS.set(settings).expect("Unable to set settings"),
        Err(e) => {
            eprint!("Invalid settings file: {}", e);
            std::process::exit(-1);
        }
    }
}
