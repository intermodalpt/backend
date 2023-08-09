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

use itertools::Itertools;

use commons::models::gtfs;

// Calculate the GTFS stop id sequence vec for each trip_id when presented with
// a vector of GTFSStopTimes.
pub fn calculate_gtfs_stop_sequence(
    gtfs_stop_times: &Vec<gtfs::GTFSStopTimes>,
) -> HashMap<String, Vec<u32>> {
    gtfs_stop_times
        .into_iter()
        .into_group_map_by(|x| &x.trip_id)
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
