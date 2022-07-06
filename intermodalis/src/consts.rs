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

use crate::models::Weekday;

pub static EVERY_DAY: [Weekday; 7] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
    Weekday::Sunday,
];

pub static WEEKDAYS: [Weekday; 5] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
];

pub static WEEKEND: [Weekday; 2] = [Weekday::Saturday, Weekday::Sunday];

pub static HOLIDAYS: [(u8, u8); 13] = [
    (1, 1),
    (4, 15),
    (4, 17),
    (4, 17),
    (4, 25),
    (6, 10),
    (6, 16),
    (8, 15),
    (10, 5),
    (11, 1),
    (12, 1),
    (12, 8),
    (12, 25),
];

pub static SUMMER: [(u8, u8); 2] = [(23, 6), (23, 9)];

// Made up FIXME
pub static SCHOOL_PERIODS: [[(u8, u8); 2]; 3] = [
    // From   To  [month, day]
    [(1, 5), (3, 20)],
    [(3, 27), (6, 10)],
    [(9, 20), (12, 15)],
];
