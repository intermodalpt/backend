/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023  Cláudio Pereira

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

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use base64ct::{Base64, Encoding};
use captcha_rs::CaptchaBuilder;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::Error;
use crate::gtfs;

const CAPTCHA_LIMIT: i64 = 5;
const CAPTCHA_STORE_CLEANUP_TIME: i64 = 5;

#[allow(clippy::module_name_repetitions)]
pub type AppState = Arc<State>;

pub struct State {
    pub bucket: s3::Bucket,
    pub pool: PgPool,
    pub cached: Cached,
    pub captchas: CaptchaStorage,
}

impl State {
    pub fn new(bucket: s3::Bucket, pool: PgPool) -> Self {
        State {
            bucket,
            pool,
            cached: Cached {
                gtfs_stops: RwLock::new(HashMap::new()),
                tml_routes: RwLock::new(HashMap::new()),
            },
            captchas: CaptchaStorage::new(),
        }
    }

    // For integration tests
    #[allow(unused)]
    pub fn test_state(pool: PgPool) -> State {
        let credentials =
            s3::creds::Credentials::new(Some(""), Some(""), None, None, None)
                .unwrap();
        let bucket = s3::Bucket::new(
            "",
            s3::Region::R2 {
                account_id: String::new(),
            },
            credentials,
        )
        .unwrap()
        .with_path_style();

        State::new(bucket, pool)
    }
}

pub struct Cached {
    pub gtfs_stops: RwLock<HashMap<i32, Arc<Vec<commons::models::gtfs::Stop>>>>,
    pub tml_routes: RwLock<HashMap<i32, Arc<Vec<gtfs::models::TMLRoute>>>>,
}

pub struct CaptchaStorage {
    pub captchas: RwLock<HashMap<Uuid, Captcha>>,
    pub last_cleaned: RwLock<DateTime<Utc>>,
}

pub struct Captcha {
    pub iat: DateTime<Utc>,
    pub answer: String,
    pub used: AtomicBool,
}

impl CaptchaStorage {
    pub fn new() -> Self {
        CaptchaStorage {
            captchas: RwLock::new(HashMap::new()),
            last_cleaned: RwLock::new(Utc::now()),
        }
    }

    pub fn gen_captcha(&self) -> Result<(Uuid, String), Error> {
        let captcha = CaptchaBuilder::new()
            .length(5)
            .width(130)
            .height(40)
            .dark_mode(false)
            .complexity(1)
            .compression(40)
            .build();

        let captcha_digest = Captcha {
            iat: Utc::now(),
            answer: captcha.text,
            used: AtomicBool::new(false),
        };
        let uuid = Uuid::new_v4();

        let res = if let Ok(mut captchas) = self.captchas.write() {
            captchas.insert(uuid, captcha_digest);

            let img_b64 = Base64::encode_string(captcha.image.as_bytes());
            (uuid, img_b64)
        } else {
            return Err(Error::IllegalState);
        };

        self.cleanup();

        Ok(res)
    }

    pub(crate) fn attempt_captcha(
        &self,
        uuid: Uuid,
        answer: &str,
    ) -> Result<bool, Error> {
        let now = Utc::now();
        if let Ok(captchas) = self.captchas.read() {
            if let Some(captcha) = captchas.get(&uuid) {
                let used = captcha.used.swap(true, Ordering::Relaxed);
                let elapsed = now - captcha.iat;
                let max_elapsed =
                    chrono::Duration::try_minutes(CAPTCHA_LIMIT).unwrap();
                // Captcha already used
                // or captcha expired
                // or solution invalid
                if used || elapsed >= max_elapsed || captcha.answer != answer {
                    return Ok(false);
                }
            } else {
                // Captcha not found
                return Ok(false);
            }
        } else {
            return Err(Error::IllegalState);
        }

        self.cleanup();

        Ok(true)
    }

    pub fn cleanup(&self) {
        // Check if the last cleaning was more than 5 minutes ago
        let last_cleaned = *self.last_cleaned.read().unwrap();
        let now = Utc::now();
        if now - last_cleaned
            <= chrono::Duration::try_minutes(CAPTCHA_STORE_CLEANUP_TIME)
                .unwrap()
        {
            return;
        }

        if let Ok(mut last_clean) = self.last_cleaned.write() {
            let mut captchas = self.captchas.write().unwrap();
            captchas.retain(|_, captcha| {
                now - captcha.iat
                    <= chrono::Duration::try_minutes(CAPTCHA_LIMIT).unwrap()
                    || !captcha.used.load(Ordering::Relaxed)
            });
            *last_clean = Utc::now();
        }
    }
}
