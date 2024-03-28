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

use std::io::Cursor;

use crate::errors::Error;

pub async fn download_file(
    url: &str,
    output: &str,
    max_content_len: Option<usize>,
) -> Result<(), Error> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| Error::Download(e.to_string()))?;

    // TODO This implementation is fragile at best. Improve it.

    if resp.status().is_success() {
        if let Some(max_content_len) = max_content_len {
            if let Some(content_len) = resp.content_length() {
                if content_len > max_content_len as u64 {
                    return Err(Error::Download(
                        "Max content len exceeded".to_string(),
                    ));
                }
            }
        }

        let mut content = Cursor::new(
            resp.bytes()
                .await
                .map_err(|e| Error::Download(e.to_string()))?,
        );

        let mut file = std::fs::File::create(output)
            .map_err(|e| Error::Filesystem(e.to_string()))?;
        std::io::copy(&mut content, &mut file)
            .map_err(|e| Error::Filesystem(e.to_string()))?;

        Ok(())
    } else {
        Err(Error::Download(format!(
            "Unexpected return code: {}",
            resp.status()
        )))
    }
}
