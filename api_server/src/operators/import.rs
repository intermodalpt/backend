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
use commons::utils::{git, gtfs as gtfs_utils, http};

use super::models;
use crate::errors::Error;

pub(crate) trait OperatorData {
    fn get_id(&self) -> i32;
    fn get_data_root(&self) -> PathBuf {
        PathBuf::from(format!("./data/operators/{}/", self.get_id()))
    }
    fn get_gtfs_root(&self) -> PathBuf {
        PathBuf::from(format!("./data/operators/{}/gtfs/", self.get_id()))
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
    let meta_path =
        PathBuf::from(format!("./data/operators/{operator_id}/meta.json"));

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
                fs::create_dir_all(p).unwrap();
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
    let meta_path =
        PathBuf::from(format!("./data/operators/{operator_id}/meta.json"));

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
            let path = format!("./data/operators/{operator_id}/gtfsrepo");
            let url = "https://github.com/carrismetropolitana/gtfs";
            let remote_name = "origin";
            let remote_branch = "live";

            let version_date =
                git::update_repo(url, &path, remote_name, remote_branch)
                    .map_err(|err| {
                        tracing::error!(err=?err);
                        Error::Processing
                    })?;

            if meta.last_gtfs != Some(version_date) {
                meta.last_gtfs = Some(version_date);
                gtfs_utils::extract(
                    &format!(
                        "./data/operators/{operator_id}/gtfsrepo/CarrisMetropolitana.zip"
                    ),
                    &format!("./data/operators/{operator_id}/gtfs"),
                ).inspect_err(|err| {
                    tracing::error!(msg="Failure extracting GTFS", operator_id, err=?err)
                })?;
            }
        }
        "carris" => {
            let path = format!("./data/operators/{operator_id}/gtfs.zip");
            let url = "https://gateway.carris.pt/gateway/gtfs/api/v2.11/GTFS";

            http::download_file(url, &path, None)
                .await
                .inspect_err(|err| {
                    tracing::error!(
                        msg="Failed to download file",
                        err=?err,
                        url,
                        path
                    );
                })?;

            let newest_file = gtfs_utils::extract(
                &format!("./data/operators/{operator_id}/gtfs.zip"),
                &format!("./data/operators/{operator_id}/gtfs"),
            )?;
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
    let path = format!("./data/operators/{operator_id}/gtfs.zip");
    let url = format!(
        "https://www.transporlis.pt/desktopmodules/\
            trp_opendata/ajax/downloadFile.ashx?op={transporlis_id}&u=web"
    );

    http::download_file(&url, &path, None)
        .await
        .inspect_err(|err| {
            tracing::error!(
                msg="Failed to download file",
                err=?err,
                url,
                path
            );
        })?;

    let newest_file = gtfs_utils::extract(
        &format!("./data/operators/{operator_id}/gtfs.zip"),
        &format!("./data/operators/{operator_id}/gtfs"),
    ).inspect_err(|err| {
        tracing::error!(msg="Failure extracting GTFS", operator_id, err=?err)
    })?;
    meta.last_gtfs = Some(newest_file);
    Ok(())
}
