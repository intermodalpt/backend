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
use commons::utils::{gtfs as gtfs_utils, http};

use super::models;
use crate::errors::Error;
use crate::settings::SETTINGS;

fn get_data_root() -> PathBuf {
    let root = SETTINGS.get().unwrap().storage.root.as_str();
    PathBuf::from(root)
}

fn get_operators_data_root() -> PathBuf {
    let mut path = get_data_root();
    path.push("operators");
    path
}

fn get_operator_data_root(operator_id: i32) -> PathBuf {
    let mut path = get_operators_data_root();
    path.push(operator_id.to_string());
    path
}

fn get_operator_storage_meta_path(operator_id: i32) -> PathBuf {
    let mut meta_path = get_operator_data_root(operator_id);
    meta_path.push("meta.json");
    meta_path
}

pub(crate) trait OperatorData {
    fn get_id(&self) -> i32;
    fn get_data_root(&self) -> PathBuf {
        get_operator_data_root(self.get_id())
    }
    fn get_gtfs_root(&self) -> PathBuf {
        let mut path = self.get_data_root();
        path.push("gtfs");
        path
    }
    fn get_storage_meta(&self) -> Result<OperatorStorageMeta, Error> {
        get_operator_storage_meta(self.get_id())
    }
}

impl OperatorData for models::Operator {
    fn get_id(&self) -> i32 {
        self.id
    }
}

impl OperatorData for operators::Operator {
    fn get_id(&self) -> i32 {
        self.id
    }
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct OperatorStorageMeta {
    pub(crate) last_update: Option<DateTime<Utc>>,
    pub(crate) last_gtfs: Option<DateTime<Utc>>,
}

pub(crate) fn get_operator_storage_meta(
    operator_id: i32,
) -> Result<OperatorStorageMeta, Error> {
    let meta_path = get_operator_storage_meta_path(operator_id);

    let meta = if meta_path.exists() {
        let meta: OperatorStorageMeta = serde_json::from_reader(
            fs::File::open(&meta_path).map_err(|err| {
                tracing::error!(
                    msg = "Filesystem error",
                    err=?err,
                    meta_path=?meta_path
                );
                Error::Filesystem
            })?,
        )
        .unwrap_or_default();
        meta
    } else {
        let meta = OperatorStorageMeta::default();

        if let Some(p) = meta_path.parent() {
            if !p.exists() {
                fs::create_dir_all(p).map_err(|err| {
                    tracing::error!(msg = "Filesystem error", err=?err);
                    Error::Filesystem
                })?;
            }
        }
        serde_json::to_writer(
            fs::File::create(&meta_path).map_err(|err| {
                tracing::error!(msg = "Filesystem error", err=?err);
                Error::Filesystem
            })?,
            &meta,
        )
        .map_err(|err| {
            tracing::error!(msg="Unable to serialize meta to file", err=?err);
            Error::Serialization
        })?;
        meta
    };
    Ok(meta)
}

pub(crate) fn set_operator_storage_meta(
    operator_id: i32,
    meta: &OperatorStorageMeta,
) -> Result<(), Error> {
    let meta_path = get_operator_storage_meta_path(operator_id);

    if let Some(p) = meta_path.parent() {
        if !p.exists() {
            fs::create_dir_all(p).unwrap();
        }
    }
    serde_json::to_writer(
        fs::File::create(&meta_path).map_err(|err| {
            tracing::error!(
                msg="Unable to create file",
                err=?err,
                meta_path=?meta_path
            );
            Error::Filesystem
        })?,
        &meta,
    )
    .map_err(|err| {
        tracing::error!(
            msg="Unable to serialize data to file",
            err=?err,
            meta=?meta,
            meta_path=?meta_path
        );
        Error::Serialization
    })?;
    Ok(())
}

pub(crate) async fn update_operator_gtfs(
    operator_id: i32,
    operator_tag: &str,
) -> Result<(), Error> {
    let mut meta = get_operator_storage_meta(operator_id)?;
    match operator_tag {
        "cmet" => {
            let mut zip_path = get_operator_data_root(operator_id);
            zip_path.push("gtfs.zip");
            let mut gtfs_path = get_operator_data_root(operator_id);
            gtfs_path.push("gtfs");

            let url = "https://api.carrismetropolitana.pt/gtfs*";

            http::download_file(url, &zip_path, None)
                .await
                .inspect_err(|err| {
                    tracing::error!(
                        msg="Failed to download file",
                        err=?err,
                        url,
                        zip_path = ?zip_path
                    );
                })?;

            let newest_file = gtfs_utils::extract(&zip_path, &gtfs_path)?;
            meta.last_gtfs = Some(newest_file);
        }
        "carris" => {
            let mut zip_path = get_operator_data_root(operator_id);
            zip_path.push("gtfs.zip");
            let mut gtfs_path = get_operator_data_root(operator_id);
            gtfs_path.push("gtfs");
            let url = "https://gateway.carris.pt/gateway/gtfs/api/v2.11/GTFS";

            http::download_file(url, &zip_path, None)
                .await
                .inspect_err(|err| {
                    tracing::error!(
                        msg="Failed to download file",
                        err=?err,
                        url,
                        zip_path=?zip_path
                    );
                })?;

            let newest_file = gtfs_utils::extract(&zip_path, &gtfs_path)?;
            meta.last_gtfs = Some(newest_file);
        }
        "tcb" => {
            fetch_transporlis_feed(&mut meta, operator_id, 41).await?;
        }
        "ttsl" => {
            fetch_transporlis_feed(&mut meta, operator_id, 4).await?;
        }
        "ml" => {
            fetch_transporlis_feed(&mut meta, operator_id, 2).await?;
        }
        "cp" => {
            fetch_transporlis_feed(&mut meta, operator_id, 3).await?;
        }
        "fert" => {
            fetch_transporlis_feed(&mut meta, operator_id, 13).await?;
        }
        "mobic" => {
            let mut zip_path = get_operator_data_root(operator_id);
            zip_path.push("gtfs.zip");
            let mut gtfs_path = get_operator_data_root(operator_id);
            gtfs_path.push("gtfs");
            let url = "https://dadosabertos.cascais.pt/\
                dataset/ddef8977-0ad0-4d23-99d3-ae269a21b589/\
                resource/819dac57-8843-43a3-a630-9cc7987325c0/\
                download/gtfs-mobicascais.zip";

            http::download_file(url, &zip_path, None)
                .await
                .inspect_err(|err| {
                    tracing::error!(
                        msg="Failed to download file",
                        err=?err,
                        url,
                        zip_path=?zip_path
                    );
                })?;

            let newest_file = gtfs_utils::extract(&zip_path, &gtfs_path)
                .inspect_err(|err| {
                    tracing::error!(
                        msg="Failure extracting GTFS",
                        operator_id,
                        err=?err
                    );
                })?;
            meta.last_gtfs = Some(newest_file);
        }
        _ => {
            tracing::warn!("Unknown operator tag: '{operator_tag}'");
        }
    }

    meta.last_update = Some(Utc::now());
    set_operator_storage_meta(operator_id, &meta)
}

async fn fetch_transporlis_feed(
    meta: &mut OperatorStorageMeta,
    operator_id: i32,
    transporlis_id: i32,
) -> Result<(), Error> {
    let mut zip_path = get_operator_data_root(operator_id);
    zip_path.push("gtfs.zip");
    let mut gtfs_path = get_operator_data_root(operator_id);
    gtfs_path.push("gtfs");
    let url = format!(
        "https://www.transporlis.pt/desktopmodules/\
            trp_opendata/ajax/downloadFile.ashx?op={transporlis_id}&u=web"
    );

    http::download_file(&url, &zip_path, None)
        .await
        .inspect_err(|err| {
            tracing::error!(
                msg="Failed to download file",
                err=?err,
                url,
                zip_path=?zip_path
            );
        })?;

    let newest_file = gtfs_utils::extract(
        &zip_path,
        &gtfs_path,
    )
    .inspect_err(|err| {
        tracing::error!(msg="Failure extracting GTFS", operator_id, err=?err);
    })?;
    meta.last_gtfs = Some(newest_file);
    Ok(())
}
