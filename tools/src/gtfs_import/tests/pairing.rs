use itertools::Itertools;
use std::collections::HashSet;

use crate::matcher::{
    pair_patterns_with_subroutes, PatternSummary, RouteSummary, SubrouteSummary,
};

#[test]
fn single_lone_pattern() {
    let code = "123".to_string();
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5];

    let gtfs_route_id_1 = "0000_0".to_string();
    let gtfs_patterns_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0".to_string());
        set
    };
    let gtfs_trips_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0_Mon".to_string());
        set.insert("0000_0_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_1 = vec![
        "01".to_string(),
        "02".to_string(),
        "03".to_string(),
        "04".to_string(),
        "05".to_string(),
    ];

    let summary = RouteSummary {
        iml_route_id: 0,
        route_code: &Some(code),
        subroutes: vec![],
        patterns: vec![PatternSummary {
            gtfs_stop_ids: &gtfs_stop_ids_1,
            iml_stop_ids: iml_stop_ids_1
                .iter()
                .cloned()
                .map(Some)
                .collect_vec(),
            route_id: &gtfs_route_id_1,
            patterns: &gtfs_patterns_1,
            trips: &gtfs_trips_1,
        }],
    };

    let res = pair_patterns_with_subroutes(summary);
    assert_eq!(res.pairings.len(), 0);
    assert_eq!(res.unpaired_gtfs.len(), 1);
    assert_eq!(res.unpaired_iml.len(), 0);
}

#[test]
fn single_lone_subroute() {
    let code = "123".to_string();
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5];

    let summary = RouteSummary {
        iml_route_id: 0,
        route_code: &Some(code),
        subroutes: vec![SubrouteSummary {
            subroute_id: 1,
            prematched_gtfs_pattern: None,
            stop_ids: &iml_stop_ids_1,
            stop_ids_as_option: iml_stop_ids_1
                .iter()
                .cloned()
                .map(Some)
                .collect_vec(),
        }],
        patterns: vec![],
    };

    let res = pair_patterns_with_subroutes(summary);
    assert_eq!(res.pairings.len(), 0);
    assert_eq!(res.unpaired_gtfs.len(), 0);
    assert_eq!(res.unpaired_iml.len(), 1);
}

#[test]
fn single_match() {
    let code = "123".to_string();
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5];

    let gtfs_route_id_1 = "0000_0".to_string();
    let gtfs_patterns_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0".to_string());
        set
    };
    let gtfs_trips_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0_Mon".to_string());
        set.insert("0000_0_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_1 = vec![
        "01".to_string(),
        "02".to_string(),
        "03".to_string(),
        "04".to_string(),
        "05".to_string(),
    ];

    let summary = RouteSummary {
        iml_route_id: 0,
        route_code: &Some(code),
        subroutes: vec![SubrouteSummary {
            subroute_id: 1,
            prematched_gtfs_pattern: None,
            stop_ids: &iml_stop_ids_1,
            stop_ids_as_option: iml_stop_ids_1
                .iter()
                .cloned()
                .map(Some)
                .collect_vec(),
        }],
        patterns: vec![PatternSummary {
            gtfs_stop_ids: &gtfs_stop_ids_1,
            iml_stop_ids: iml_stop_ids_1
                .iter()
                .cloned()
                .map(Some)
                .collect_vec(),
            route_id: &gtfs_route_id_1,
            patterns: &gtfs_patterns_1,
            trips: &gtfs_trips_1,
        }],
    };

    let res = pair_patterns_with_subroutes(summary);
    assert_eq!(res.pairings.len(), 1);
    assert_eq!(res.unpaired_gtfs.len(), 0);
    assert_eq!(res.unpaired_iml.len(), 0);

    let pairing = &res.pairings[0];
    assert_eq!(pairing.stop_matches, 5);
    assert_eq!(pairing.stop_mismatches, 0);
}

#[test]
fn two_equal_matches() {
    let code = "123".to_string();
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5];

    // GTFS route 1
    let gtfs_route_id_1 = "0000_0".to_string();
    let gtfs_patterns_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0".to_string());
        set
    };
    let gtfs_trips_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0_Mon".to_string());
        set.insert("0000_0_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_1 = vec![
        "01".to_string(),
        "02".to_string(),
        "03".to_string(),
        "04".to_string(),
        "05".to_string(),
    ];
    // GTFS route 2
    let gtfs_route_id_2 = "0000_0".to_string();
    let gtfs_patterns_2 = {
        let mut set = HashSet::new();
        set.insert("0000_1_0".to_string());
        set
    };
    let gtfs_trips_2 = {
        let mut set = HashSet::new();
        set.insert("0000_1_0_Mon".to_string());
        set.insert("0000_1_0_Tue".to_string());
        set
    };

    let summary = RouteSummary {
        iml_route_id: 0,
        route_code: &Some(code),
        subroutes: vec![
            SubrouteSummary {
                subroute_id: 1,
                prematched_gtfs_pattern: None,
                stop_ids: &iml_stop_ids_1,
                stop_ids_as_option: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
            },
            SubrouteSummary {
                subroute_id: 2,
                prematched_gtfs_pattern: None,
                stop_ids: &iml_stop_ids_1,
                stop_ids_as_option: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
            },
        ],
        patterns: vec![
            PatternSummary {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
            },
            PatternSummary {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
            },
        ],
    };

    let res = pair_patterns_with_subroutes(summary);
    assert_eq!(res.pairings.len(), 0);
    assert_eq!(res.unpaired_gtfs.len(), 2);
    assert_eq!(res.unpaired_iml.len(), 2);
}

#[test]
fn two_perfect_matches() {
    let code = "123".to_string();
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5];
    let iml_stop_ids_2 = vec![10, 20, 30, 40, 50];

    // GTFS route 1
    let gtfs_route_id_1 = "0000_0".to_string();
    let gtfs_patterns_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0".to_string());
        set
    };
    let gtfs_trips_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0_Mon".to_string());
        set.insert("0000_0_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_1 = vec![
        "01".to_string(),
        "02".to_string(),
        "03".to_string(),
        "04".to_string(),
        "05".to_string(),
    ];
    // GTFS route 2
    let gtfs_route_id_2 = "0000_1".to_string();
    let gtfs_patterns_2 = {
        let mut set = HashSet::new();
        set.insert("0000_1_0".to_string());
        set
    };
    let gtfs_trips_2 = {
        let mut set = HashSet::new();
        set.insert("0000_1_0_Mon".to_string());
        set.insert("0000_1_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_2 = vec![
        "010".to_string(),
        "020".to_string(),
        "030".to_string(),
        "040".to_string(),
        "050".to_string(),
    ];

    let summary = RouteSummary {
        iml_route_id: 0,
        route_code: &Some(code),
        subroutes: vec![
            SubrouteSummary {
                subroute_id: 1,
                prematched_gtfs_pattern: None,
                stop_ids: &iml_stop_ids_1,
                stop_ids_as_option: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
            },
            SubrouteSummary {
                subroute_id: 2,
                prematched_gtfs_pattern: None,
                stop_ids: &iml_stop_ids_2,
                stop_ids_as_option: iml_stop_ids_2
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
            },
        ],
        patterns: vec![
            PatternSummary {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
            },
            PatternSummary {
                gtfs_stop_ids: &gtfs_stop_ids_2,
                iml_stop_ids: iml_stop_ids_2
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
            },
        ],
    };

    let res = pair_patterns_with_subroutes(summary);
    assert_eq!(res.pairings.len(), 2);
    assert_eq!(res.unpaired_gtfs.len(), 0);
    assert_eq!(res.unpaired_iml.len(), 0);

    let pairing = &res.pairings[0];
    assert_eq!(pairing.stop_matches, 5);
    assert_eq!(pairing.stop_mismatches, 0);
    let pairing = &res.pairings[1];
    assert_eq!(pairing.stop_matches, 5);
    assert_eq!(pairing.stop_mismatches, 0);
}

#[test]
fn imperfect_matches() {
    // ID 5 disappears from the GTFS
    let code = "123".to_string();
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iml_stop_ids_2 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

    // GTFS route 1
    let gtfs_route_id_1 = "0000_0".to_string();
    let gtfs_patterns_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0".to_string());
        set
    };
    let gtfs_trips_1 = {
        let mut set = HashSet::new();
        set.insert("0000_0_0_Mon".to_string());
        set.insert("0000_0_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_1 = vec![
        "01".to_string(),
        "02".to_string(),
        "03".to_string(),
        "04".to_string(),
        "06".to_string(),
        "07".to_string(),
        "08".to_string(),
        "09".to_string(),
    ];
    let gtfs_iml_stop_ids_1 = vec![1, 2, 3, 4, 6, 7, 8, 9];
    // GTFS route 2
    let gtfs_route_id_2 = "0000_1".to_string();
    let gtfs_patterns_2 = {
        let mut set = HashSet::new();
        set.insert("0000_1_0".to_string());
        set
    };
    let gtfs_trips_2 = {
        let mut set = HashSet::new();
        set.insert("0000_1_0_Mon".to_string());
        set.insert("0000_1_0_Tue".to_string());
        set
    };
    let gtfs_stop_ids_2 = vec![
        "010".to_string(),
        "020".to_string(),
        "030".to_string(),
        "040".to_string(),
        "060".to_string(),
        "070".to_string(),
        "080".to_string(),
        "090".to_string(),
    ];
    let gtfs_iml_stop_ids_2 = vec![10, 20, 30, 40, 60, 70, 80, 90];

    let summary = RouteSummary {
        iml_route_id: 0,
        route_code: &Some(code),
        subroutes: vec![
            SubrouteSummary {
                subroute_id: 1,
                prematched_gtfs_pattern: None,
                stop_ids: &iml_stop_ids_1,
                stop_ids_as_option: iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
            },
            SubrouteSummary {
                subroute_id: 2,
                prematched_gtfs_pattern: None,
                stop_ids: &iml_stop_ids_2,
                stop_ids_as_option: iml_stop_ids_2
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
            },
        ],
        patterns: vec![
            PatternSummary {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: gtfs_iml_stop_ids_1
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
            },
            PatternSummary {
                gtfs_stop_ids: &gtfs_stop_ids_2,
                iml_stop_ids: gtfs_iml_stop_ids_2
                    .iter()
                    .cloned()
                    .map(Some)
                    .collect_vec(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
            },
        ],
    };

    let res = pair_patterns_with_subroutes(summary);
    assert_eq!(res.pairings.len(), 2);
    assert_eq!(res.unpaired_gtfs.len(), 0);
    assert_eq!(res.unpaired_iml.len(), 0);

    let pairing = &res.pairings[0];
    assert_eq!(pairing.stop_matches, 8);
    assert_eq!(pairing.stop_mismatches, 1);
    let pairing = &res.pairings[1];
    assert_eq!(pairing.stop_matches, 8);
    assert_eq!(pairing.stop_mismatches, 1);
}
