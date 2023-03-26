use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::models;
use crate::stops;

pub(crate) fn summarize_stop_meta_contributions(
    contributions: Vec<models::Contribution>,
) -> Vec<stops::models::Stop> {
    let mut modified_stops = HashMap::new();

    for contribution in contributions {
        match contribution.change {
            models::Change::StopUpdate {
                mut original,
                patch,
            } => match modified_stops.entry(original.id) {
                Entry::Occupied(mut entry) => patch.apply(entry.get_mut()),
                Entry::Vacant(entry) => {
                    patch.apply(&mut original);
                    entry.insert(original);
                }
            },
            _ => {
                unreachable!()
            }
        }
    }

    modified_stops.into_values().collect()
}
