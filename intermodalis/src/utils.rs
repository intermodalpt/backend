/*
    Intermodalis, transportation information aggregator
    Copyright (C) 2022  Cláudio Pereira

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

use crate::Error;
use chrono::NaiveDateTime;

pub fn within_dates(date: (u8, u8), start: (u8, u8), end: (u8, u8)) -> bool {
    let (from_month, from_day) = start;
    let (to_month, to_day) = end;
    let (month, day) = date;

    if month < from_month || (month == from_month && day < from_day) {
        false
    } else {
        !(month > to_month || (month == to_month && day > to_day))
    }
}

pub fn extract_f64_gps_coord(repr: &[exif::Rational]) -> Result<f64, Error> {
    if repr.len() != 3 {
        return Err(Error::Processing("Invalid EXIF coord".to_string()));
    }
    let deg = repr[0];
    let min = repr[1];
    let sec = repr[2];

    let res = deg.to_f64() + min.to_f64() / 60.0 + sec.to_f64() / 3600.0;
    Ok(res)
}

pub fn string_from_exif_ascii(repr: &[Vec<u8>]) -> Result<String, Error> {
    if repr.len() != 1 {
        return Err(Error::Processing("Invalid EXIF String".to_string()));
    }

    let string = std::str::from_utf8(&repr[0])
        .map_err(|_e| {
            Error::Processing("Invalid EXIF String encoding".to_string())
        })?
        .to_string();
    Ok(string)
}

pub fn datetime_from_exif_ascii(
    repr: &[Vec<u8>],
) -> Result<NaiveDateTime, Error> {
    if repr.len() != 1 {
        return Err(Error::Processing("Invalid EXIF String".to_string()));
    }

    let string = std::str::from_utf8(&repr[0]).map_err(|_err| {
        Error::Processing("Invalid EXIF String encoding".to_string())
    })?;
    let datetime =
        NaiveDateTime::parse_from_str(string, "%Y:%m:%d %H:%M:%S")
            .map_err(|_| Error::Processing("Invalid EXIF date".to_string()))?;
    Ok(datetime)
}

#[derive(Default, Debug)]
pub(crate) struct Exif {
    pub(crate) lat: Option<f64>,
    pub(crate) lon: Option<f64>,
    pub(crate) capture: Option<NaiveDateTime>,
    pub(crate) camera: Option<String>,
}

impl From<exif::Exif> for Exif {
    fn from(data: exif::Exif) -> Self {
        let mut result = Exif::default();

        if let Some(field) =
            data.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY)
        {
            if let exif::Value::Rational(val) = &field.value {
                if let Ok(coord) = extract_f64_gps_coord(&val) {
                    result.lat = Some(coord);
                } else {
                    println!("Invalid value for GPS Lat");
                }
            } else {
                println!("Invalid value for GPS Lat");
            }
        }

        if let Some(field) =
            data.get_field(exif::Tag::GPSLongitude, exif::In::PRIMARY)
        {
            if let exif::Value::Rational(val) = &field.value {
                if let Ok(coord) = extract_f64_gps_coord(&val) {
                    result.lon = Some(coord);
                } else {
                    println!("Invalid value for GPS Lon");
                }
            } else {
                println!("Invalid value for GPS Lon");
            }
        }

        if let Some(field) =
            data.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        {
            if let exif::Value::Ascii(val) = &field.value {
                if let Ok(datetime) = datetime_from_exif_ascii(val) {
                    result.capture = Some(datetime);
                }
            } else {
                println!("Invalid value for Original Datetime");
            }
        }

        if result.capture.is_none() {
            if let Some(field) =
                data.get_field(exif::Tag::DateTimeDigitized, exif::In::PRIMARY)
            {
                if let exif::Value::Ascii(val) = &field.value {
                    if let Ok(datetime) = datetime_from_exif_ascii(val) {
                        result.capture = Some(datetime);
                    }
                } else {
                    println!("Invalid value for Digitized Datetime");
                }
            }
        }

        if let Some(field) = data.get_field(exif::Tag::Model, exif::In::PRIMARY)
        {
            if let exif::Value::Ascii(val) = &field.value {
                if let Ok(datetime) = string_from_exif_ascii(val) {
                    result.camera = Some(datetime);
                }
            } else {
                println!("Invalid value for Camera Model");
            }
        }

        result
    }
}
