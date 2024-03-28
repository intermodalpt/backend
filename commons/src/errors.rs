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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to download: `{0}`")]
    Download(String),
    #[error("Filesystem: `{0}`")]
    Filesystem(String),
    #[error("Unable to extract content: `{0}`")]
    Extraction(String),
    #[error("Patching failure: field `{field}` does not accept value `{value}`")]
    Patching { field: &'static str, value: String },
    #[error("Unable to convert from old models`")]
    Conversion,
}
