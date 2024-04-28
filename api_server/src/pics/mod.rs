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

pub(crate) mod handlers;
pub(crate) mod logic;
pub(crate) mod models;
pub(crate) mod sql;

use once_cell::sync::OnceCell;

pub(crate) static IMG_ROOT: OnceCell<&'static str> = OnceCell::new();

pub(crate) fn get_stop_pic_ori_named_path(sha: &str, filename: &str) -> String {
    format!("{}/ori/{sha}/{filename}", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_stop_pic_ori_path(sha: &str) -> String {
    format!("{}/ori/{sha}/stop", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_stop_pic_medium_path(sha: &str) -> String {
    format!("{}/medium/{sha}/preview", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_stop_pic_thumb_path(sha: &str) -> String {
    format!("{}/thumb/{sha}/preview", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_logo_path(operator_id: i32, sha: &str) -> String {
    format!(
        "{}/operators/{operator_id}/{sha}/logo",
        IMG_ROOT.get().unwrap()
    )
}

pub(crate) fn get_news_pic_full_path(sha: &str) -> String {
    format!("{}/news/ori/{sha}/stop", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_news_pic_medium_path(sha: &str) -> String {
    format!("{}/news/medium/{sha}/preview", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_news_pic_thumb_path(sha: &str) -> String {
    format!("{}/news/thumb/{sha}/preview", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_external_news_pic_path(sha: &str) -> String {
    format!("{}/enews/{sha}/img", IMG_ROOT.get().unwrap())
}

pub(crate) fn get_external_news_ss_path(sha: &str) -> String {
    format!("{}/enews_ss/{sha}/img", IMG_ROOT.get().unwrap())
}
