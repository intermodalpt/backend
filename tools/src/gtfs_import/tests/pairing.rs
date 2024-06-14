use once_cell::sync::Lazy;
use std::collections::HashSet;

use crate::iml;
use crate::matcher::{
    pair_route_intersection, ImlGtfsRouteIntersection, PatternCluster,
    SubrouteSummary,
};

static DUMMY_REFERENCE_ROUTE: Lazy<iml::Route> = Lazy::new(|| iml::Route {
    id: 0,
    name: "".to_string(),
    code: None,
    operator: 0,
    circular: false,
    badge_text: "".to_string(),
    badge_bg: "".to_string(),
    type_id: 0,
    active: false,
    subroutes: vec![],
});
static DUMMY_REFERENCE_SUBROUTE: Lazy<iml::Subroute> =
    Lazy::new(|| iml::Subroute {
        id: 0,
        flag: "".to_string(),
        circular: false,
        headsign: None,
        destination: None,
        stops: vec![],
        validation: iml::SubrouteValidation {
            current: vec![],
            gtfs: None,
        },
    });

static EMPTY_SET_OF_STRINGS: Lazy<HashSet<String>> =
    Lazy::new(|| HashSet::new());

#[test]
fn single_lone_pattern() {
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

    let route_intersection = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![],
        patterns_cluster: vec![PatternCluster {
            gtfs_stop_ids: &gtfs_stop_ids_1,
            iml_stop_ids: iml_stop_ids_1.clone(),
            route_id: &gtfs_route_id_1,
            patterns: &gtfs_patterns_1,
            trips: &gtfs_trips_1,
            headsigns: &EMPTY_SET_OF_STRINGS,
        }],
    };

    let route_pairing = pair_route_intersection(route_intersection);
    assert_eq!(route_pairing.subroute_pairings.len(), 0);
    assert_eq!(route_pairing.unpaired_gtfs.len(), 1);
    assert_eq!(route_pairing.unpaired_iml.len(), 0);
}

#[test]
fn single_lone_subroute() {
    let iml_stop_ids_1 = vec![1, 2, 3, 4, 5];

    let route_intersection = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![SubrouteSummary {
            subroute: &DUMMY_REFERENCE_SUBROUTE,
            subroute_id: 1,
            gtfs_patterns: HashSet::default(),
            stop_ids: &iml_stop_ids_1,
        }],
        patterns_cluster: vec![],
    };

    let pairing = pair_route_intersection(route_intersection);
    assert_eq!(pairing.subroute_pairings.len(), 0);
    assert_eq!(pairing.unpaired_gtfs.len(), 0);
    assert_eq!(pairing.unpaired_iml.len(), 1);
}

#[test]
fn single_match() {
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

    let route_intersection = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![SubrouteSummary {
            subroute: &DUMMY_REFERENCE_SUBROUTE,
            subroute_id: 1,
            gtfs_patterns: HashSet::default(),
            stop_ids: &iml_stop_ids_1,
        }],
        patterns_cluster: vec![PatternCluster {
            gtfs_stop_ids: &gtfs_stop_ids_1,
            iml_stop_ids: iml_stop_ids_1.clone(),
            route_id: &gtfs_route_id_1,
            patterns: &gtfs_patterns_1,
            trips: &gtfs_trips_1,
            headsigns: &EMPTY_SET_OF_STRINGS,
        }],
    };

    let route_pairing = pair_route_intersection(route_intersection);
    assert_eq!(route_pairing.subroute_pairings.len(), 1);
    assert_eq!(route_pairing.unpaired_gtfs.len(), 0);
    assert_eq!(route_pairing.unpaired_iml.len(), 0);

    let pairing = &route_pairing.subroute_pairings[0];
    assert_eq!(pairing.stop_matches, 5);
    assert_eq!(pairing.stop_mismatches, 0);
}

#[test]
fn two_equal_matches() {
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

    let route_intersection = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![
            SubrouteSummary {
                subroute: &DUMMY_REFERENCE_SUBROUTE,
                subroute_id: 1,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_1,
            },
            SubrouteSummary {
                subroute: &DUMMY_REFERENCE_SUBROUTE,
                subroute_id: 2,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_1,
            },
        ],
        patterns_cluster: vec![
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: iml_stop_ids_1.clone(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
                headsigns: &EMPTY_SET_OF_STRINGS,
            },
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: iml_stop_ids_1.clone(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
                headsigns: &EMPTY_SET_OF_STRINGS,
            },
        ],
    };

    let route_pairing = pair_route_intersection(route_intersection);
    assert_eq!(route_pairing.subroute_pairings.len(), 0);
    assert_eq!(route_pairing.unpaired_gtfs.len(), 2);
    assert_eq!(route_pairing.unpaired_iml.len(), 2);
}

#[test]
fn two_perfect_matches() {
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

    let route_intersection = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![
            SubrouteSummary {
                subroute: &DUMMY_REFERENCE_SUBROUTE,
                subroute_id: 1,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_1,
            },
            SubrouteSummary {
                subroute: &DUMMY_REFERENCE_SUBROUTE,
                subroute_id: 2,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_2,
            },
        ],
        patterns_cluster: vec![
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: iml_stop_ids_1.clone(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
                headsigns: &EMPTY_SET_OF_STRINGS,
            },
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_2,
                iml_stop_ids: iml_stop_ids_2.clone(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
                headsigns: &EMPTY_SET_OF_STRINGS,
            },
        ],
    };

    let route_pairing = pair_route_intersection(route_intersection);
    assert_eq!(route_pairing.subroute_pairings.len(), 2);
    assert_eq!(route_pairing.unpaired_gtfs.len(), 0);
    assert_eq!(route_pairing.unpaired_iml.len(), 0);

    let pairing = &route_pairing.subroute_pairings[0];
    assert_eq!(pairing.stop_matches, 5);
    assert_eq!(pairing.stop_mismatches, 0);
    let pairing = &route_pairing.subroute_pairings[1];
    assert_eq!(pairing.stop_matches, 5);
    assert_eq!(pairing.stop_mismatches, 0);
}

#[test]
fn imperfect_matches() {
    // ID 5 disappears from the GTFS
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

    let summary = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![
            SubrouteSummary {
                subroute: &DUMMY_REFERENCE_SUBROUTE,
                subroute_id: 1,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_1,
            },
            SubrouteSummary {
                subroute: &DUMMY_REFERENCE_SUBROUTE,
                subroute_id: 2,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_2,
            },
        ],
        patterns_cluster: vec![
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: gtfs_iml_stop_ids_1.clone(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
                headsigns: &EMPTY_SET_OF_STRINGS,
            },
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_2,
                iml_stop_ids: gtfs_iml_stop_ids_2.clone(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
                headsigns: &EMPTY_SET_OF_STRINGS,
            },
        ],
    };

    let route_pairing = pair_route_intersection(summary);
    assert_eq!(route_pairing.subroute_pairings.len(), 2);
    assert_eq!(route_pairing.unpaired_gtfs.len(), 0);
    assert_eq!(route_pairing.unpaired_iml.len(), 0);

    let pairing = &route_pairing.subroute_pairings[0];
    assert_eq!(pairing.stop_matches, 8);
    assert_eq!(pairing.stop_mismatches, 1);
    let pairing = &route_pairing.subroute_pairings[1];
    assert_eq!(pairing.stop_matches, 8);
    assert_eq!(pairing.stop_mismatches, 1);
}

#[test]
fn match_through_headsign() {
    let iml_stop_ids_1 = vec![];
    let iml_subroute_1 = iml::Subroute {
        headsign: Some("Quinta do queijo".to_string()),
        ..DUMMY_REFERENCE_SUBROUTE.clone()
    };
    let iml_stop_ids_2 = vec![];
    let iml_subroute_2 = iml::Subroute {
        headsign: Some("Quinta do fiambre".to_string()),
        ..DUMMY_REFERENCE_SUBROUTE.clone()
    };

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
    let gtfs_headsigns_1 = {
        let mut set = HashSet::new();
        set.insert("Quinta do fiambre".to_string());
        set
    };
    let gtfs_stop_ids_1 = vec![];
    let gtfs_iml_stop_ids_1 = vec![];
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
    let gtfs_headsigns_2 = {
        let mut set = HashSet::new();
        set.insert("Quinta do queijo".to_string());
        set
    };
    let gtfs_stop_ids_2 = vec![];
    let gtfs_iml_stop_ids_2 = vec![];

    let route_intersection = ImlGtfsRouteIntersection {
        iml_route: &DUMMY_REFERENCE_ROUTE,
        iml_route_id: 0,
        subroutes: vec![
            SubrouteSummary {
                subroute: &iml_subroute_1,
                subroute_id: 1,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_1,
            },
            SubrouteSummary {
                subroute: &iml_subroute_2,
                subroute_id: 2,
                gtfs_patterns: HashSet::default(),
                stop_ids: &iml_stop_ids_2,
            },
        ],
        patterns_cluster: vec![
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_1,
                iml_stop_ids: gtfs_iml_stop_ids_1.clone(),
                route_id: &gtfs_route_id_1,
                patterns: &gtfs_patterns_1,
                trips: &gtfs_trips_1,
                headsigns: &gtfs_headsigns_1,
            },
            PatternCluster {
                gtfs_stop_ids: &gtfs_stop_ids_2,
                iml_stop_ids: gtfs_iml_stop_ids_2.clone(),
                route_id: &gtfs_route_id_2,
                patterns: &gtfs_patterns_2,
                trips: &gtfs_trips_2,
                headsigns: &gtfs_headsigns_2,
            },
        ],
    };

    let route_pairing = pair_route_intersection(route_intersection);
    assert_eq!(route_pairing.subroute_pairings.len(), 2);
    assert_eq!(route_pairing.unpaired_gtfs.len(), 0);
    assert_eq!(route_pairing.unpaired_iml.len(), 0);
    let pairing = &route_pairing.subroute_pairings[0];
    assert_eq!(pairing.iml.subroute_id, 1);
    assert_eq!(pairing.gtfs.route_id, "0000_1");
    let pairing = &route_pairing.subroute_pairings[1];
    assert_eq!(pairing.iml.subroute_id, 2);
    assert_eq!(pairing.gtfs.route_id, "0000_0");
}
