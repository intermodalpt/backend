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

#[must_use]
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
