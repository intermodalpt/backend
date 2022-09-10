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

use std::fmt;

use crate::utils::within_dates;
use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use utoipa::Component;

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

pub static SUMMER: [(u8, u8); 2] = [(6, 23), (9, 23)];

// Made up FIXME
pub static SCHOOL_PERIODS: [[(u8, u8); 2]; 3] = [
    // From   To  [month, day]
    [(1, 5), (3, 20)],
    [(3, 27), (6, 10)],
    [(9, 20), (12, 15)],
];

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    Component,
)]
#[repr(u8)]
#[component(example = 4)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Weekday::Monday => "Segunda",
            Weekday::Tuesday => "Terça",
            Weekday::Wednesday => "Quarta",
            Weekday::Thursday => "Quinta",
            Weekday::Friday => "Sexta",
            Weekday::Saturday => "Sábado",
            Weekday::Sunday => "Domingo",
        })
    }
}

impl From<u8> for Weekday {
    fn from(val: u8) -> Self {
        match val {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            6 => Weekday::Sunday,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Calendar {
    pub weekdays: Vec<Weekday>,
    pub only_if: Vec<Condition>,
    pub also_if: Vec<Condition>,
    pub except_if: Vec<Condition>,
}

impl Calendar {
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn includes(&self, date: NaiveDate) -> bool {
        let month = date.month() as u8;
        let day = date.day() as u8;
        let weekday = date.weekday().num_days_from_monday() as u8;

        let date = (month, day);
        let is_holiday = HOLIDAYS.contains(&date);
        let is_summer = within_dates(date, SUMMER[0], SUMMER[1]);
        let is_school = SCHOOL_PERIODS
            .into_iter()
            .any(|period| within_dates(date, period[0], period[1]));

        let condition_matches = |condition: &Condition| match condition {
            Condition::Holiday => is_holiday,
            Condition::Summer => is_summer,
            Condition::School => is_school,
            Condition::Nth { nth } => *nth == (day % 7) + 1,
            Condition::Range { start, end } => within_dates(date, *start, *end),
        };

        if !self
            .only_if
            .iter()
            .all(|condition| condition_matches(condition))
        {
            return false;
        }

        if self
            .except_if
            .iter()
            .any(|condition| condition_matches(condition))
        {
            return false;
        }

        if self
            .also_if
            .iter()
            .any(|condition| condition_matches(condition))
        {
            return true;
        }

        self.weekdays.contains(&(weekday.into()))
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Take some notable cases out
        match &self {
            Calendar {
                weekdays,
                except_if,
                ..
            } if weekdays == &WEEKDAYS
                && except_if == &[Condition::Holiday] =>
            {
                return f.write_str("Dias úteis");
            }
            Calendar {
                weekdays, only_if, ..
            } if weekdays == &WEEKDAYS && only_if == &[Condition::School] => {
                return f.write_str("Dias úteis de período escolar");
            }
            _ => (),
        }

        let named_weekdays = match &self.weekdays {
            weekdays if weekdays == &EVERY_DAY => "Todos os dias".to_string(),
            weekdays if weekdays == &WEEKDAYS => "Dias de semana".to_string(),
            weekdays if weekdays == &WEEKEND => "Fins de semana".to_string(),
            _ => {
                let mut named_weekdays = vec![];
                let mut weekdays = self.weekdays.clone();
                if WEEKDAYS.iter().all(|item| weekdays.contains(item)) {
                    weekdays.retain(|weekday| !WEEKDAYS.contains(weekday));
                    named_weekdays.push("Dias úteis".to_string());
                }

                weekdays
                    .into_iter()
                    .map(|weekday| weekday.to_string())
                    .for_each(|name| named_weekdays.push(name));

                let named_weekday_count = named_weekdays.len();

                match named_weekday_count {
                    0 => "Indefinido".to_string(),
                    1 => named_weekdays[0].to_string(),
                    len => {
                        format!(
                            "{} e {}",
                            named_weekdays[0..len - 1].iter().join(", "),
                            named_weekdays[len - 1]
                        )
                    }
                }
            }
        };

        let mut named_conditions = vec![];

        let condition_gen =
            |variant: &Vec<Condition>, connector| match variant.len() {
                0 => None,
                len => Some(if len == 1 {
                    format!("{connector} {}", variant[0])
                } else {
                    let conditions = variant
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>();
                    format!(
                        "{} e {}",
                        conditions[0..len - 1].iter().join(", "),
                        conditions[len - 1]
                    )
                }),
            };

        if let Some(sentence) = condition_gen(&self.only_if, "que sejam") {
            named_conditions.push(sentence);
        }

        if let Some(sentence) = condition_gen(&self.except_if, "exceto") {
            named_conditions.push(sentence);
        }

        if let Some(sentence) = condition_gen(&self.also_if, "ou") {
            named_conditions.push(sentence);
        }

        if named_conditions.is_empty() {
            return f.write_str(&named_weekdays);
        }

        f.write_fmt(format_args!(
            "{} {}",
            &named_weekdays,
            named_conditions.into_iter().join(" ")
        ))
    }
}

// https://github.com/juhaku/utoipa/issues/176
// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Component)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "condition")]
pub enum Condition {
    Holiday,
    Summer,
    School,
    Range { start: (u8, u8), end: (u8, u8) },
    Nth { nth: u8 },
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Holiday => f.write_str("feriados"),
            Condition::Summer => f.write_str("verão"),
            Condition::School => f.write_str("período escolar"),
            Condition::Nth { nth } => {
                f.write_fmt(format_args!("{}º do mês", nth))
            }
            Condition::Range {
                start: (start_month, start_day),
                end: (end_month, end_day),
            } => f.write_fmt(format_args!(
                "entre {start_day}/{start_month} e {end_day}/{end_month}"
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::calendar::{Calendar, Condition, EVERY_DAY, WEEKDAYS, WEEKEND};
    use crate::Weekday;
    use chrono::NaiveDate;

    #[test]
    fn yes_holiday() {
        let date = NaiveDate::from_ymd(2022, 12, 25);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::Holiday],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn no_holiday() {
        let date = NaiveDate::from_ymd(2022, 12, 24);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::Holiday],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn no_pre_summer_start() {
        let date = NaiveDate::from_ymd(2022, 06, 22);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::Summer],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn yes_summer_start() {
        let date = NaiveDate::from_ymd(2022, 06, 23);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::Summer],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn yes_summer_end() {
        let date = NaiveDate::from_ymd(2022, 09, 23);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::Summer],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn no_summer_post_end() {
        let date = NaiveDate::from_ymd(2022, 09, 24);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::Summer],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn no_pre_school_start() {
        let date = NaiveDate::from_ymd(2022, 01, 04);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::School],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn yes_school_start() {
        let date = NaiveDate::from_ymd(2022, 01, 05);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::School],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn yes_school_end() {
        let date = NaiveDate::from_ymd(2022, 12, 15);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::School],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn no_school_post_end() {
        let date = NaiveDate::from_ymd(2022, 12, 16);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![Condition::School],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn allow_exception() {
        let date = NaiveDate::from_ymd(2022, 12, 25);
        let cal = Calendar {
            weekdays: vec![],
            only_if: vec![],
            also_if: vec![Condition::Holiday],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn deny_exemption() {
        let date = NaiveDate::from_ymd(2022, 12, 25);
        let cal = Calendar {
            weekdays: EVERY_DAY.to_vec(),
            only_if: vec![],
            also_if: vec![Condition::Holiday],
            except_if: vec![Condition::Holiday],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn yes_single_weekday() {
        let date = NaiveDate::from_ymd(2022, 12, 15);
        let cal = Calendar {
            weekdays: vec![Weekday::Sunday],
            only_if: vec![],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn yes_multiple_weekdays() {
        let date = NaiveDate::from_ymd(2022, 12, 15);
        let cal = Calendar {
            weekdays: WEEKEND.to_vec(),
            only_if: vec![],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(cal.includes(date));
    }

    #[test]
    fn no_single_weekday() {
        let date = NaiveDate::from_ymd(2022, 12, 15);
        let cal = Calendar {
            weekdays: vec![],
            only_if: vec![],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }

    #[test]
    fn no_multiple_weekdays() {
        let date = NaiveDate::from_ymd(2022, 12, 15);
        let cal = Calendar {
            weekdays: WEEKDAYS.to_vec(),
            only_if: vec![],
            also_if: vec![],
            except_if: vec![],
        };
        assert!(!cal.includes(date));
    }
}
