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

use chrono::NaiveDateTime;

pub fn decode_gps_coord(repr: &[exif::Rational]) -> Result<f64, &'static str> {
    if repr.len() != 3 {
        return Err("Invalid EXIF coord");
    }
    let deg = repr[0];
    let min = repr[1];
    let sec = repr[2];

    let res = deg.to_f64() + min.to_f64() / 60.0 + sec.to_f64() / 3600.0;
    Ok(res)
}

pub fn decode_ascii(repr: &[Vec<u8>]) -> Result<String, &'static str> {
    if repr.len() != 1 {
        return Err("Invalid EXIF String");
    }

    let string = std::str::from_utf8(&repr[0])
        .map_err(|_e| "Invalid EXIF String encoding")?
        .to_string();
    Ok(string)
}

pub fn decode_datetime(
    repr: &[Vec<u8>],
) -> Result<NaiveDateTime, &'static str> {
    if repr.len() != 1 {
        return Err("Invalid EXIF String");
    }

    let string = std::str::from_utf8(&repr[0])
        .map_err(|_err| "Invalid EXIF String encoding")?;
    let datetime = NaiveDateTime::parse_from_str(string, "%Y:%m:%d %H:%M:%S")
        .map_err(|_| "Invalid EXIF date")?;
    Ok(datetime)
}

pub fn decode_u16(repr: &[u16]) -> Result<u16, &'static str> {
    if repr.len() != 1 {
        return Err("Invalid EXIF u16");
    }

    Ok(repr[0])
}

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Horizontal = 1,
    Mirror = 2,
    Rotate180 = 3,
    MirrorVertical = 4,
    MirrorHorizontalRotate270 = 5,
    Rotate90 = 6,
    MirrorHorizontalRotate90 = 7,
    Rotate270 = 8,
}

fn orientation_from_u16(orientation: u16) -> Orientation {
    match orientation {
        1 => Orientation::Horizontal,
        2 => Orientation::Mirror,
        3 => Orientation::Rotate180,
        4 => Orientation::MirrorVertical,
        5 => Orientation::MirrorHorizontalRotate270,
        6 => Orientation::Rotate90,
        7 => Orientation::MirrorHorizontalRotate90,
        8 => Orientation::Rotate270,
        _ => {
            eprintln!(
                "Invalid EXIF orientation: {orientation}. Proceeding with horizontal."
            );
            Orientation::Horizontal
        }
    }
}

#[derive(Default, Debug)]
pub struct Exif {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub capture: Option<NaiveDateTime>,
    pub camera: Option<String>,
    pub orientation: Option<Orientation>,
}

impl From<exif::Exif> for Exif {
    fn from(data: exif::Exif) -> Self {
        let mut result = Exif::default();

        if let Some(field) =
            data.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        {
            if let exif::Value::Short(val) = &field.value {
                if let Ok(orientation) = decode_u16(val) {
                    result.orientation =
                        Some(orientation_from_u16(orientation));
                }
            } else {
                println!("Possibly bad orientation EXIF");
            }
        }

        if let Some(field) =
            data.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY)
        {
            if let exif::Value::Rational(val) = &field.value {
                if let Ok(coord) = decode_gps_coord(val) {
                    if !coord.is_nan() {
                        result.lat = Some(coord);
                    }
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
                if let Ok(coord) = decode_gps_coord(val) {
                    if !coord.is_nan() {
                        result.lon = Some(-coord);
                    }
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
                if let Ok(datetime) = decode_datetime(val) {
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
                    if let Ok(datetime) = decode_datetime(val) {
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
                if let Ok(datetime) = decode_ascii(val) {
                    result.camera = Some(datetime);
                }
            } else {
                println!("Invalid value for Camera Model");
            }
        }

        result
    }
}
