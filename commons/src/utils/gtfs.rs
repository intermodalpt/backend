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

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use itertools::Itertools;

use crate::errors::Error;
use crate::models::gtfs;

const GTFS_FILES: [&str; 13] = [
    "agency.txt",
    "calendar_dates.txt",
    "facilities.txt",
    "fare_attributes.txt",
    "fare_rules.txt",
    "feed_info.txt",
    "helpdesks.txt",
    "municipalities.txt",
    "routes.txt",
    "shapes.txt",
    "stops.txt",
    "stop_times.txt",
    "trips.txt",
];

// Calculate the GTFS stop id sequence vec for each trip_id when presented with
// a vector of GTFSStopTimes.
#[must_use]
pub fn calculate_gtfs_stop_sequence(
    gtfs_stop_times: &[gtfs::StopTime],
) -> HashMap<String, Vec<String>> {
    gtfs_stop_times
        .iter()
        .into_group_map_by(|x| &x.trip_id)
        .into_iter()
        .map(|(trip_id, stop_times)| {
            let stop_ids = stop_times
                .into_iter()
                .sorted_by_key(|stop_time| stop_time.stop_sequence)
                .map(|stop_time| stop_time.stop_id.clone())
                .collect::<Vec<_>>();

            (trip_id.clone(), stop_ids)
        })
        .collect::<HashMap<_, _>>()
}

#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn calculate_stop_sliding_windows(
    gtfs_stop_sequence: &HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    let mut stop_sequences = vec![];
    for gtfs_stop_sequence in gtfs_stop_sequence.values() {
        for window in gtfs_stop_sequence.windows(3) {
            stop_sequences.push(window);
        }
    }

    stop_sequences
        .into_iter()
        .unique()
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<_>>()
}

pub fn extract(
    zip_file: &str,
    output_dir: &str,
) -> Result<chrono::DateTime<chrono::Utc>, Error> {
    let file = fs::File::open(zip_file)
        .map_err(|e| Error::Filesystem(e.to_string()))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| Error::Extraction(e.to_string()))?;

    let mut last_modification_date = chrono::DateTime::default();

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| Error::Extraction(e.to_string()))?;

        let file_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        println!("{file_path:?}");

        if !GTFS_FILES.contains(&file.name()) {
            continue;
        }

        let modification_date = zip_datetime_to_chrono(file.last_modified())?;
        if modification_date > last_modification_date {
            last_modification_date = modification_date;
        }

        let mut output_dir = PathBuf::from(output_dir);
        output_dir.push(file_path);
        println!("{output_dir:?}");

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output_dir.display());
            fs::create_dir_all(&output_dir)
                .map_err(|e| Error::Filesystem(e.to_string()))?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output_dir.display(),
                file.size()
            );
            if let Some(p) = output_dir.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| Error::Filesystem(e.to_string()))?;
                }
            }
            let mut outfile = fs::File::create(&output_dir)
                .map_err(|e| Error::Filesystem(e.to_string()))?;
            io::copy(&mut file, &mut outfile)
                .map_err(|e| Error::Filesystem(e.to_string()))?;
        }
    }
    Ok(last_modification_date)
}

fn zip_datetime_to_chrono(
    datetime: zip::DateTime,
) -> Result<chrono::DateTime<chrono::Utc>, Error> {
    let date = chrono::NaiveDate::from_ymd_opt(
        i32::from(datetime.year()),
        u32::from(datetime.month()),
        u32::from(datetime.day()),
    );
    let time = chrono::NaiveTime::from_hms_opt(
        u32::from(datetime.hour()),
        u32::from(datetime.minute()),
        u32::from(datetime.second()),
    );

    let date = date.ok_or(Error::Extraction("Bad date".to_string()))?;
    let time = time.ok_or(Error::Extraction("Bad time".to_string()))?;

    Ok(chrono::NaiveDateTime::new(date, time).and_utc())
}
