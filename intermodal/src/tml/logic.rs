use super::models;
use itertools::Itertools;
use std::collections::HashMap;

// Calculate the GTFS stop id sequence vec for each trip_id when presented with
// a vector of GTFSStopTimes.
pub fn calculate_gtfs_stop_sequence(
    gtfs_stop_times: &Vec<models::GTFSStopTimes>,
) -> HashMap<String, Vec<u32>> {
    gtfs_stop_times
        .into_iter()
        .group_by(|x| &x.trip_id)
        .into_iter()
        .map(|(trip_id, stop_times)| {
            let stop_ids = stop_times
                .into_iter()
                .sorted_by_key(|stop_time| stop_time.stop_sequence)
                .map(|stop_time| stop_time.stop_id)
                .collect::<Vec<_>>();

            (trip_id.clone(), stop_ids)
        })
        .collect::<HashMap<_, _>>()
}

// Calculate the stop sliding windows for every trip
pub fn calculate_stop_sliding_windows(
    gtfs_stop_sequence: &HashMap<String, Vec<u32>>,
) -> Vec<Vec<u32>> {
    let mut stop_sequences = vec![];
    for gtfs_stop_sequence in gtfs_stop_sequence.values() {
        for window in gtfs_stop_sequence.windows(3) {
            stop_sequences.push(window);
        }
    }

    stop_sequences
        .into_iter()
        .unique()
        .map(|seq| seq.to_owned())
        .collect::<Vec<_>>()
}
