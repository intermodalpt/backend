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

mod sql;

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use sqlx::PgPool;

use config::Config;

use commons::models::{history, stops};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("SETTINGS"))
        .build()
        .unwrap();

    let pool = PgPool::connect(&settings.get_string("db").expect("db not set"))
        .await
        .expect("Unable to connect to the database");

    let dry_run = std::env::args().any(|arg| arg == "--dry-run");

    fix_duplicated_schedules(&pool, dry_run).await.unwrap();
}

pub async fn fix_duplicated_schedules(
    pool: &PgPool,
    dry_run: bool,
) -> Result<()> {
    let undecided =
        sql::fetch_undecided_contributions(pool, None, 0, i64::max_value())
            .await?;

    // Regex for time
    let time_regex = Regex::new(r"(\d{2})[:,._](\d{2})").unwrap();

    for mut contribution in undecided.into_iter() {
        match &mut contribution.change {
            history::Change::StopUpdate { original: _, patch } => {
                if let Some(Some(schedules)) = &patch.schedules {
                    let mut seen_routes: HashMap<
                        &Option<String>,
                        Vec<stops::Schedule>,
                    > = HashMap::new();
                    for schedule in schedules.iter() {
                        let current_schedule: stops::Schedule =
                            schedule.clone().try_into().unwrap();
                        seen_routes
                            .entry(&schedule.code)
                            .and_modify(|e| {
                                e.push(current_schedule.clone());
                            })
                            .or_insert(vec![current_schedule]);
                    }

                    let mut merged = false;

                    let schedules = seen_routes.into_iter()
                        .flat_map(|(code, schedules)| {
                            let time_based = schedules.iter().all(|schedule| {
                                if let Some(discriminator) =
                                    &schedule.discriminator
                                {
                                    time_regex.is_match(discriminator)
                                } else {
                                    false
                                }
                            });


                            let same_type = schedules
                                .iter()
                                //.map(|schedule| schedule.schedule_type)
                                .map(|schedule|
                                    if schedule.schedule_type == stops::ScheduleType::Frequency {
                                        stops::ScheduleType::Prediction
                                    } else {
                                        schedule.schedule_type
                                    })
                                .unique()
                                .count()
                                == 1;

                            if time_based && same_type {
                                merged = true;

                                let first = stops::Schedule {
                                    discriminator: Some("?".to_string()),
                                    ..schedules.first().unwrap().clone()
                                };

                                println!(
                                    "Merged schedules for {:?} \n\tfrom [{:?}] \n\tto {:?}",
                                    code,
                                    schedules
                                        .iter()
                                        .map(|s|
                                            format!("({:?}: {})", s.schedule_type, s.discriminator.as_ref().unwrap()))
                                        .join(","),
                                    first
                                );

                                vec![first]
                            } else {
                                schedules
                            }
                        })
                        .collect::<Vec<stops::Schedule>>();

                    if merged {
                        println!("Merged result {:?}", schedules);
                        patch.schedules =
                            Some(Some(history::vec_into_vec(schedules)));

                        if !dry_run {
                            sql::update_contribution(
                                pool,
                                contribution.id,
                                &contribution.change,
                                &contribution.comment,
                            )
                            .await?;
                        }
                    }
                };
            }
            _ => {}
        }
    }

    Ok(())
}
