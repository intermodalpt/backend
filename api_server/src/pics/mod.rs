/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2024  Cláudio Pereira

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

pub(crate) mod handlers;
pub(crate) mod logic;
pub(crate) mod models;
pub(crate) mod sql;

use uuid::Uuid;

use crate::settings::SETTINGS;

pub(crate) fn get_stop_pic_ori_named_path(sha: &str, filename: &str) -> String {
    format!(
        "{}/ori/{sha}/{filename}",
        SETTINGS.get().unwrap().images.root
    )
}

pub(crate) fn get_stop_pic_ori_path(sha: &str) -> String {
    format!("{}/ori/{sha}/stop", SETTINGS.get().unwrap().images.root)
}

pub(crate) fn get_stop_pic_medium_path(sha: &str) -> String {
    format!(
        "{}/medium/{sha}/preview",
        SETTINGS.get().unwrap().images.root
    )
}

pub(crate) fn get_stop_pic_thumb_path(sha: &str) -> String {
    format!(
        "{}/thumb/{sha}/preview",
        SETTINGS.get().unwrap().images.root
    )
}

pub(crate) fn get_logo_path(operator_id: i32, sha: &str) -> String {
    format!(
        "{}/operators/{operator_id}/{sha}/logo",
        SETTINGS.get().unwrap().images.root,
    )
}

// ----- Images in rich content blocks -----

pub(crate) fn get_rich_img_full_path(id: Uuid) -> String {
    format!(
        "{}/content/{id}/ori/original",
        SETTINGS.get().unwrap().images.root
    )
}

pub(crate) fn get_rich_img_medium_path(id: Uuid) -> String {
    format!(
        "{}/content/{id}/medium/preview",
        SETTINGS.get().unwrap().images.root
    )
}

pub(crate) fn get_rich_img_thumb_path(id: Uuid) -> String {
    format!(
        "{}/content/{id}/thumb/preview",
        SETTINGS.get().unwrap().images.root
    )
}

// ----- External news images -----

pub(crate) fn get_external_news_pic_path(sha: &str) -> String {
    format!("{}/enews/{sha}/img", SETTINGS.get().unwrap().images.root)
}

pub(crate) fn get_external_news_ss_path(sha: &str) -> String {
    format!("{}/enews_ss/{sha}/img", SETTINGS.get().unwrap().images.root)
}
