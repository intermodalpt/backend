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

use std::fs;
use std::io;
use std::path::PathBuf;

const GTFS_FILES: [&'static str; 13] = [
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

pub fn extract_gtfs(zip_file: &str, output_dir: &str) {
    let file = fs::File::open(zip_file).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        println!("{:?}", file_path);

        if !GTFS_FILES.contains(&file.name()) {
            continue;
        }

        let mut output_dir = PathBuf::from(output_dir);
        output_dir.push(file_path);
        println!("{:?}", output_dir);

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output_dir.display());
            fs::create_dir_all(&output_dir).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output_dir.display(),
                file.size()
            );
            if let Some(p) = output_dir.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&output_dir).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
