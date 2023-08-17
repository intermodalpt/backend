/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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

use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use commons::models::operators;

use crate::errors::Error;

pub(crate) trait OperatorData {
    fn get_data_root(&self) -> PathBuf;
    fn get_gtfs_root(&self) -> PathBuf;
    fn get_storage_meta(&self) -> Result<OperatorStorageMeta, Error>;
}
impl OperatorData for operators::Operator {
    fn get_data_root(&self) -> PathBuf {
        PathBuf::from(format!("./data/operators/{}/", self.id))
    }
    fn get_gtfs_root(&self) -> PathBuf {
        PathBuf::from(format!("./data/operators/{}/gtfs/", self.id))
    }

    fn get_storage_meta(&self) -> Result<OperatorStorageMeta, Error> {
        Ok(get_operator_storage_meta(self.id)?)
    }
}
#[derive(Deserialize, Serialize)]
pub struct OperatorStorageMeta {
    pub(crate) last_update: Option<DateTime<Utc>>,
    pub(crate) last_gtfs: Option<DateTime<Utc>>,
}

impl Default for OperatorStorageMeta {
    fn default() -> Self {
        Self {
            last_update: None,
            last_gtfs: None,
        }
    }
}

pub(crate) fn get_operator_storage_meta(
    operator_id: i32,
) -> Result<OperatorStorageMeta, Error> {
    let meta_path =
        PathBuf::from(format!("./data/operators/{}/meta.json", operator_id));

    let meta = if meta_path.exists() {
        let meta: OperatorStorageMeta = serde_json::from_reader(
            fs::File::open(meta_path)
                .map_err(|e| Error::Processing(e.to_string()))?,
        )
        .unwrap_or_default();
        meta
    } else {
        let meta = OperatorStorageMeta::default();

        if let Some(p) = meta_path.parent() {
            if !p.exists() {
                fs::create_dir_all(p).unwrap();
            }
        }
        serde_json::to_writer(
            fs::File::create(meta_path)
                .map_err(|e| Error::Processing(e.to_string()))?,
            &meta,
        )
        .map_err(|e| Error::Processing(e.to_string()))?;
        meta
    };
    Ok(meta)
}

pub(crate) fn set_operator_storage_meta(
    operator_id: i32,
    meta: OperatorStorageMeta,
) -> Result<(), Error> {
    let meta_path =
        PathBuf::from(format!("./data/operators/{}/meta.json", operator_id));

    if let Some(p) = meta_path.parent() {
        if !p.exists() {
            fs::create_dir_all(p).unwrap();
        }
    }
    serde_json::to_writer(
        fs::File::create(meta_path)
            .map_err(|e| Error::Processing(e.to_string()))?,
        &meta,
    )
    .map_err(|e| Error::Processing(e.to_string()))?;
    Ok(())
}

pub(crate) fn update_operator_meta<U>(
    operator_id: i32,
    updater: U,
) -> Result<(), Error>
where
    U: FnOnce(&mut OperatorStorageMeta) -> Result<bool, Error>,
{
    let mut meta = get_operator_storage_meta(operator_id)?;

    updater(&mut meta)?;

    meta.last_update = Some(Utc::now());
    set_operator_storage_meta(operator_id, meta)
}
