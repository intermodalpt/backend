use chrono::NaiveDate;
use once_cell::sync::Lazy;

use commons::models::history::stops::StopPatch;
use commons::models::stops::{
    A11yMeta, AdvertisementQuantification, AreaParkingLimitation,
    IlluminationPos, IlluminationStrength, LocalParkingLimitation,
    ParkingVisualLimitation, Stop,
};

static STOP1: Lazy<Stop> = Lazy::new(|| Stop {
    id: 1,
    name: "name".to_string(),
    short_name: Some("short_name".to_string()),

    // TODO, continue from here
    locality: Some("locality".to_string()),
    street: Some("street".to_string()),
    door: Some("door".to_string()),
    parish: None,
    lat: 1.0,
    lon: 2.0,
    a11y: A11yMeta {
        schedules: Some(vec![]),
        flags: Some(vec![]),
        has_crossing: Some(true),
        has_wide_access: Some(true),
        has_flat_access: Some(true),
        has_tactile_access: Some(true),
        has_sidewalk: Some(true),
        has_sidewalked_path: Some(true),
        has_shelter: Some(true),
        has_cover: Some(true),
        has_bench: Some(true),
        has_trash_can: Some(true),
        has_waiting_times: Some(true),
        has_ticket_seller: Some(true),
        has_costumer_support: Some(true),
        illumination_strength: Some(IlluminationStrength::None),
        illumination_position: Some(IlluminationPos::Indirect),
        has_illuminated_path: Some(true),
        has_visibility_from_within: Some(true),
        has_visibility_from_area: Some(true),
        is_visible_from_outside: Some(true),
        parking_visibility_impairment: Some(ParkingVisualLimitation::Little),
        parking_local_access_impairment: Some(LocalParkingLimitation::Low),
        parking_area_access_impairment: Some(AreaParkingLimitation::Low),
        advertisement_qty: Some(AdvertisementQuantification::None),
        tmp_issues: vec![],
    },
    verification_level: 0,
    service_check_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
    infrastructure_check_date: Some(
        NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
    ),
    tags: vec!["tags".to_string()],
    notes: Some("notes".to_string()),
});

#[test]
fn drops_name() {
    let mut patch = StopPatch {
        name: Some("name".to_string()),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_short_name() {
    let mut patch = StopPatch {
        short_name: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_short_name() {
    let mut patch = StopPatch {
        short_name: Some(Some("short_name".to_string())),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_locality() {
    let mut patch = StopPatch {
        locality: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_locality() {
    let mut patch = StopPatch {
        locality: Some(Some("locality".to_string())),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_street() {
    let mut patch = StopPatch {
        street: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_street() {
    let mut patch = StopPatch {
        street: Some(Some("street".to_string())),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_door() {
    let mut patch = StopPatch {
        door: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_door() {
    let mut patch = StopPatch {
        door: Some(Some("door".to_string())),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_schedules() {
    let mut patch = StopPatch {
        schedules: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_schedules() {
    let mut patch = StopPatch {
        schedules: Some(Some(vec![])),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}
#[test]
fn keeps_flags() {
    let mut patch = StopPatch {
        flags: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_flags() {
    let mut patch = StopPatch {
        flags: Some(Some(vec![])),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_has_sidewalk() {
    let mut patch = StopPatch {
        has_sidewalk: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_has_sidewalk() {
    let mut patch = StopPatch {
        has_sidewalk: Some(Some(true)),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_has_sidewalked_path() {
    let mut patch = StopPatch {
        has_sidewalked_path: Some(None),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_has_sidewalked_path() {
    let mut patch = StopPatch {
        has_sidewalked_path: Some(Some(true)),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}

#[test]
fn keeps_verification_level() {
    let mut patch = StopPatch {
        verification_level: Some(255),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(!patch.is_empty());
}

#[test]
fn drops_verification_level() {
    let mut patch = StopPatch {
        verification_level: Some(0),
        ..StopPatch::default()
    };

    assert!(patch.drop_noops(&STOP1).is_ok());
    assert!(patch.is_empty());
}
