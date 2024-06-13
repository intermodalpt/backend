use chrono::NaiveDate;
use once_cell::sync::Lazy;

use commons::models::history::opt_vec_into_opt_vec;
use commons::models::history::stops::StopPatch;
use commons::models::stops::{
    A11yMeta, AdvertisementQuantification, AreaParkingLimitation, Flag,
    IlluminationPos, IlluminationStrength, LocalParkingLimitation,
    ParkingVisualLimitation, Schedule, ScheduleType, Stop,
};

use crate::stops::models::requests::ChangeStop;

static STOP1: Lazy<Stop> = Lazy::new(|| Stop {
    id: 1,
    name: "Original".to_string(),
    short_name: Some("Original short".to_string()),
    locality: Some("Fooland".to_string()),
    street: Some("Barstreet".to_string()),
    door: Some("123A".to_string()),
    parish: None,
    lat: 1.0,
    lon: 2.0,
    a11y: A11yMeta {
        schedules: Some(vec![Schedule {
            code: Some("123".to_string()),
            discriminator: Some("321".to_string()),
            schedule_type: ScheduleType::Origin,
        }]),
        flags: Some(vec![Flag {
            id: "123".to_string(),
            name: Some("ABC".to_string()),
            route_codes: vec!["1234".to_string(), "4321".to_string()],
        }]),
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
        tmp_issues: vec!["issue1".to_string(), "issue2".to_string()],
    },
    verification_level: 0,
    service_check_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
    infrastructure_check_date: Some(
        NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
    ),
    osm_id: None,
    license: "IML".to_string(),
    tags: vec!["tag1".to_string()],
    notes: Some("foo note".to_string()),
    is_ghost: false,
});

static STOP1_NOOP_CHANGE: Lazy<ChangeStop> = Lazy::new(|| ChangeStop {
    lon: STOP1.lon.clone(),
    lat: STOP1.lat.clone(),
    name: STOP1.name.clone(),
    short_name: STOP1.short_name.clone(),
    locality: STOP1.locality.clone(),
    street: STOP1.street.clone(),
    door: STOP1.door.clone(),
    notes: STOP1.notes.clone(),
    tags: STOP1.tags.clone(),
    a11y: STOP1.a11y.clone(),
    verification_level: STOP1.verification_level.clone(),
    service_check_date: STOP1.service_check_date.clone(),
    infrastructure_check_date: STOP1.infrastructure_check_date.clone(),
    license: STOP1.license.clone(),
    is_ghost: STOP1.is_ghost,
});

#[test]
fn ok_derive_name_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = "New name".to_string();

    change.name = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.name, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.name = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_name_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.name = STOP1.name.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_name_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = "New name".to_string();

    let mut patch = StopPatch::default();
    patch.name = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.name, new_val);

    // Revert, confirm that nothing else changed
    stop.name = STOP1.name.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_short_name_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = "New name".to_string();

    change.short_name = Some(new_val.clone());

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.short_name, Some(Some(new_val)));

    // Ensure that the patch is only about this attribute
    patch.short_name = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_short_name_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.short_name = STOP1.short_name.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_short_name_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some("New name".to_string());

    let mut patch = StopPatch::default();
    patch.short_name = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.short_name, new_val);

    // Revert, confirm that nothing else changed
    stop.short_name = STOP1.short_name.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_locality_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some("New locality".to_string());

    change.locality = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.locality, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.locality = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_locality_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.locality = STOP1.locality.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_locality_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some("New locality".to_string());

    let mut patch = StopPatch::default();
    patch.locality = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.locality, new_val);

    // Revert, confirm that nothing else changed
    stop.locality = STOP1.locality.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_street_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some("New street".to_string());

    change.street = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.street, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.street = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_street_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.street = STOP1.street.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_street_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some("New street".to_string());

    let mut patch = StopPatch::default();
    patch.street = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.street, new_val);

    // Revert, confirm that nothing else changed
    stop.street = STOP1.street.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_door_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some("New door".to_string());

    change.door = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.door, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.door = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_door_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.door = STOP1.door.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_door_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some("New door".to_string());

    let mut patch = StopPatch::default();
    patch.door = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.door, new_val);

    // Revert, confirm that nothing else changed
    stop.door = STOP1.door.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_notes_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some("New note".to_string());

    change.notes = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.notes, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.notes = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_notes_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.notes = STOP1.notes.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_notes_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some("New note".to_string());

    let mut patch = StopPatch::default();
    patch.notes = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.notes, new_val);

    // Revert, confirm that nothing else changed
    stop.notes = STOP1.notes.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_tags_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = vec!["tag1".to_string(), "tag2".to_string()];

    change.tags = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.tags, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.tags = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_tags_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.tags = STOP1.tags.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_tags_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = vec!["tag1".to_string(), "tag2".to_string()];

    let mut patch = StopPatch::default();
    patch.tags = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.tags, new_val);

    // Revert, confirm that nothing else changed
    stop.tags = STOP1.tags.clone();
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_verification_level_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = 8;

    change.verification_level = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.verification_level, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.verification_level = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_verification_level_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.verification_level = STOP1.verification_level;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_verification_level_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = 8;

    let mut patch = StopPatch::default();
    patch.verification_level = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.verification_level, new_val);

    // Revert, confirm that nothing else changed
    stop.verification_level = STOP1.verification_level;
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_service_check_date_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(NaiveDate::from_ymd_opt(2021, 02, 01).unwrap());

    change.service_check_date = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.service_check_date, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.service_check_date = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_service_check_date_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.service_check_date = STOP1.service_check_date;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_service_check_date_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(NaiveDate::from_ymd_opt(2021, 02, 01).unwrap());

    let mut patch = StopPatch::default();
    patch.service_check_date = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.service_check_date, new_val);

    // Revert, confirm that nothing else changed
    stop.service_check_date = STOP1.service_check_date;
    assert_eq!(stop, *STOP1);
}

#[test]
fn ok_derive_infrastructure_check_date_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(NaiveDate::from_ymd_opt(2021, 02, 01).unwrap());

    change.infrastructure_check_date = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.infrastructure_check_date, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.infrastructure_check_date = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_infrastructure_check_date_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.infrastructure_check_date = STOP1.infrastructure_check_date;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_infrastructure_check_date_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(NaiveDate::from_ymd_opt(2021, 02, 01).unwrap());

    let mut patch = StopPatch::default();
    patch.infrastructure_check_date = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.infrastructure_check_date, new_val);

    // Revert, confirm that nothing else changed
    stop.infrastructure_check_date = STOP1.infrastructure_check_date;
    assert_eq!(stop, *STOP1);
}

// ###### A11y fields ########

// Validation for schedules

#[test]
fn ok_derive_schedules_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(vec![Schedule {
        code: Some("987".to_string()),
        discriminator: Some("789".to_string()),
        schedule_type: ScheduleType::Prediction,
    }]);

    change.a11y.schedules = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.schedules, Some(opt_vec_into_opt_vec(new_val)));

    // Ensure that the patch is only about this attribute
    patch.schedules = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_schedules_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.schedules = STOP1.a11y.schedules.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_schedules_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(vec![Schedule {
        code: Some("987".to_string()),
        discriminator: Some("789".to_string()),
        schedule_type: ScheduleType::Prediction,
    }]);

    let mut patch = StopPatch::default();
    patch.schedules = Some(opt_vec_into_opt_vec(new_val.clone()));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.schedules, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.schedules = STOP1.a11y.schedules.clone();
    assert_eq!(stop, *STOP1);
}

// Validation for flags

#[test]
fn ok_derive_flags_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(vec![Flag {
        id: "987".to_string(),
        name: Some("XYZ".to_string()),
        route_codes: vec!["1234".to_string()],
    }]);

    change.a11y.flags = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.flags, Some(opt_vec_into_opt_vec(new_val)));

    // Ensure that the patch is only about this attribute
    patch.flags = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_flags_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.flags = STOP1.a11y.flags.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_flags_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(vec![Flag {
        id: "987".to_string(),
        name: Some("XYZ".to_string()),
        route_codes: vec!["1234".to_string()],
    }]);

    let mut patch = StopPatch::default();
    patch.flags = Some(opt_vec_into_opt_vec(new_val.clone()));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.flags, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.flags = STOP1.a11y.flags.clone();
    assert_eq!(stop, *STOP1);
}

// Validation for has_crossing

#[test]
fn ok_derive_has_crossing_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_crossing = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_crossing, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_crossing = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_crossing_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_crossing = STOP1.a11y.has_crossing;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_crossing_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_crossing = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_crossing, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_crossing = STOP1.a11y.has_crossing;
    assert_eq!(stop, *STOP1);
}

// Validation for has_wide_access

#[test]
fn ok_derive_has_wide_access_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_wide_access = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_wide_access, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_wide_access = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_wide_access_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_wide_access = STOP1.a11y.has_wide_access;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_wide_access_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_wide_access = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_wide_access, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_wide_access = STOP1.a11y.has_wide_access;
    assert_eq!(stop, *STOP1);
}

// Validation for has_flat_access

#[test]
fn ok_derive_has_flat_access_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_flat_access = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_flat_access, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_flat_access = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_flat_access_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_flat_access = STOP1.a11y.has_flat_access;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_flat_access_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_flat_access = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_flat_access, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_flat_access = STOP1.a11y.has_flat_access;
    assert_eq!(stop, *STOP1);
}

// Validation for has_tactile_access

#[test]
fn ok_derive_has_tactile_access_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_tactile_access = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_tactile_access, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_tactile_access = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_tactile_access_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_tactile_access = STOP1.a11y.has_tactile_access;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_tactile_access_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_tactile_access = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_tactile_access, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_tactile_access = STOP1.a11y.has_tactile_access;
    assert_eq!(stop, *STOP1);
}

// Now for has_sidewalk

#[test]
fn ok_derive_has_sidewalk_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_sidewalk = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_sidewalk, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_sidewalk = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_sidewalk_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_sidewalk = STOP1.a11y.has_sidewalk;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_sidewalk_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_sidewalk = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_sidewalk, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_sidewalk = STOP1.a11y.has_sidewalk;
    assert_eq!(stop, *STOP1);
}

// Validation for has_sidewalked_path

#[test]
fn ok_derive_has_sidewalked_path_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_sidewalked_path = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_sidewalked_path, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_sidewalked_path = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_sidewalked_path_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_sidewalked_path = STOP1.a11y.has_sidewalked_path;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_sidewalked_path_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_sidewalked_path = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_sidewalked_path, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_sidewalked_path = STOP1.a11y.has_sidewalked_path;
    assert_eq!(stop, *STOP1);
}

// Validation for has_shelter

#[test]
fn ok_derive_has_shelter_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_shelter = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_shelter, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_shelter = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_shelter_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_shelter = STOP1.a11y.has_shelter;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_shelter_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_shelter = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_shelter, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_shelter = STOP1.a11y.has_shelter;
    assert_eq!(stop, *STOP1);
}

// Validation for has_cover

#[test]
fn ok_derive_has_cover_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_cover = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_cover, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_cover = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_cover_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_cover = STOP1.a11y.has_cover;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_cover_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_cover = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_cover, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_cover = STOP1.a11y.has_cover;
    assert_eq!(stop, *STOP1);
}

// Validation for has_bench

#[test]
fn ok_derive_has_bench_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_bench = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_bench, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_bench = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_bench_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_bench = STOP1.a11y.has_bench;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_bench_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_bench = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_bench, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_bench = STOP1.a11y.has_bench;
    assert_eq!(stop, *STOP1);
}

// Validation for has_trash_can

#[test]
fn ok_derive_has_trash_can_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_trash_can = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_trash_can, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_trash_can = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_trash_can_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_trash_can = STOP1.a11y.has_trash_can;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_trash_can_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_trash_can = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_trash_can, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_trash_can = STOP1.a11y.has_trash_can;
    assert_eq!(stop, *STOP1);
}

// Validation for has_waiting_times

#[test]
fn ok_derive_has_waiting_times_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_waiting_times = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_waiting_times, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_waiting_times = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_waiting_times_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_waiting_times = STOP1.a11y.has_waiting_times;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_waiting_times_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_waiting_times = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_waiting_times, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_waiting_times = STOP1.a11y.has_waiting_times;
    assert_eq!(stop, *STOP1);
}

// Validation for has_ticket_seller

#[test]
fn ok_derive_has_ticket_seller_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_ticket_seller = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_ticket_seller, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_ticket_seller = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_ticket_seller_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_ticket_seller = STOP1.a11y.has_ticket_seller;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_ticket_seller_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_ticket_seller = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_ticket_seller, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_ticket_seller = STOP1.a11y.has_ticket_seller;
    assert_eq!(stop, *STOP1);
}

// Validation for has_costumer_support

#[test]
fn ok_derive_has_costumer_support_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_costumer_support = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_costumer_support, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_costumer_support = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_costumer_support_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_costumer_support = STOP1.a11y.has_costumer_support;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_costumer_support_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_costumer_support = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_costumer_support, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_costumer_support = STOP1.a11y.has_costumer_support;
    assert_eq!(stop, *STOP1);
}

// Validation for illumination_strength

#[test]
fn ok_derive_illumination_strength_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(IlluminationStrength::High);

    change.a11y.illumination_strength = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.illumination_strength, Some(new_val.map(Into::into)));

    // Ensure that the patch is only about this attribute
    patch.illumination_strength = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_illumination_strength_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.illumination_strength = STOP1.a11y.illumination_strength;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_illumination_strength_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(IlluminationStrength::High);

    let mut patch = StopPatch::default();
    patch.illumination_strength = Some(new_val.clone().map(Into::into));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.illumination_strength, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.illumination_strength = STOP1.a11y.illumination_strength;
    assert_eq!(stop, *STOP1);
}

// Validation for illumination_position

#[test]
fn ok_derive_illumination_position_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(IlluminationPos::Own);

    change.a11y.illumination_position = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.illumination_position, Some(new_val.map(Into::into)));

    // Ensure that the patch is only about this attribute
    patch.illumination_position = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_illumination_position_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.illumination_position = STOP1.a11y.illumination_position;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_illumination_position_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(IlluminationPos::Own);

    let mut patch = StopPatch::default();
    patch.illumination_position = Some(new_val.clone().map(Into::into));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.illumination_position, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.illumination_position = STOP1.a11y.illumination_position;
    assert_eq!(stop, *STOP1);
}

// Validation for has_illuminated_path

#[test]
fn ok_derive_has_illuminated_path_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_illuminated_path = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_illuminated_path, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_illuminated_path = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_illuminated_path_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_illuminated_path = STOP1.a11y.has_illuminated_path;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_illuminated_path_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_illuminated_path = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_illuminated_path, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_illuminated_path = STOP1.a11y.has_illuminated_path;
    assert_eq!(stop, *STOP1);
}

// Validation for has_visibility_from_within

#[test]
fn ok_derive_has_visibility_from_within_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_visibility_from_within = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_visibility_from_within, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_visibility_from_within = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_visibility_from_within_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_visibility_from_within =
        STOP1.a11y.has_visibility_from_within;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_visibility_from_within_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_visibility_from_within = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_visibility_from_within, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_visibility_from_within =
        STOP1.a11y.has_visibility_from_within;
    assert_eq!(stop, *STOP1);
}

// Validation for has_visibility_from_area

#[test]
fn ok_derive_has_visibility_from_area_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.has_visibility_from_area = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.has_visibility_from_area, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.has_visibility_from_area = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_has_visibility_from_area_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.has_visibility_from_area = STOP1.a11y.has_visibility_from_area;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_has_visibility_from_area_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.has_visibility_from_area = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.has_visibility_from_area, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.has_visibility_from_area = STOP1.a11y.has_visibility_from_area;
    assert_eq!(stop, *STOP1);
}

// Validation for is_visible_from_outside

#[test]
fn ok_derive_is_visible_from_outside_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(false);

    change.a11y.is_visible_from_outside = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.is_visible_from_outside, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.is_visible_from_outside = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_is_visible_from_outside_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.is_visible_from_outside = STOP1.a11y.is_visible_from_outside;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_is_visible_from_outside_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(false);

    let mut patch = StopPatch::default();
    patch.is_visible_from_outside = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.is_visible_from_outside, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.is_visible_from_outside = STOP1.a11y.is_visible_from_outside;
    assert_eq!(stop, *STOP1);
}

// Validation for parking_visibility_impairment

#[test]
fn ok_derive_parking_visibility_impairment_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(ParkingVisualLimitation::Very);

    change.a11y.parking_visibility_impairment = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(
        patch.parking_visibility_impairment,
        Some(new_val.map(Into::into))
    );

    // Ensure that the patch is only about this attribute
    patch.parking_visibility_impairment = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_parking_visibility_impairment_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.parking_visibility_impairment =
        STOP1.a11y.parking_visibility_impairment;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_parking_visibility_impairment_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(ParkingVisualLimitation::Very);

    let mut patch = StopPatch::default();
    patch.parking_visibility_impairment = Some(new_val.clone().map(Into::into));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.parking_visibility_impairment, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.parking_visibility_impairment =
        STOP1.a11y.parking_visibility_impairment;
    assert_eq!(stop, *STOP1);
}

// Validation for parking_local_access_impairment

#[test]
fn ok_derive_parking_local_access_impairment_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(LocalParkingLimitation::High);

    change.a11y.parking_local_access_impairment = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(
        patch.parking_local_access_impairment,
        Some(new_val.map(Into::into))
    );

    // Ensure that the patch is only about this attribute
    patch.parking_local_access_impairment = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_parking_local_access_impairment_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.parking_local_access_impairment =
        STOP1.a11y.parking_local_access_impairment;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_parking_local_access_impairment_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(LocalParkingLimitation::High);

    let mut patch = StopPatch::default();
    patch.parking_local_access_impairment =
        Some(new_val.clone().map(Into::into));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.parking_local_access_impairment, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.parking_local_access_impairment =
        STOP1.a11y.parking_local_access_impairment;
    assert_eq!(stop, *STOP1);
}

// Validation for parking_area_access_impairment

#[test]
fn ok_derive_parking_area_access_impairment_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(AreaParkingLimitation::High);

    change.a11y.parking_area_access_impairment = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(
        patch.parking_area_access_impairment,
        Some(new_val.map(Into::into))
    );

    // Ensure that the patch is only about this attribute
    patch.parking_area_access_impairment = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_parking_area_access_impairment_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.parking_area_access_impairment =
        STOP1.a11y.parking_area_access_impairment;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_parking_area_access_impairment_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(AreaParkingLimitation::High);

    let mut patch = StopPatch::default();
    patch.parking_area_access_impairment =
        Some(new_val.clone().map(Into::into));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.parking_area_access_impairment, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.parking_area_access_impairment =
        STOP1.a11y.parking_area_access_impairment;
    assert_eq!(stop, *STOP1);
}

// Validation for advertisement_qty

#[test]
fn ok_derive_advertisement_qty_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = Some(AdvertisementQuantification::Many);

    change.a11y.advertisement_qty = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.advertisement_qty, Some(new_val.map(Into::into)));

    // Ensure that the patch is only about this attribute
    patch.advertisement_qty = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_advertisement_qty_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.advertisement_qty = STOP1.a11y.advertisement_qty;

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_advertisement_qty_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = Some(AdvertisementQuantification::Many);

    let mut patch = StopPatch::default();
    patch.advertisement_qty = Some(new_val.clone().map(Into::into));

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.advertisement_qty, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.advertisement_qty = STOP1.a11y.advertisement_qty;
    assert_eq!(stop, *STOP1);
}

// Validation for tmp_issues

#[test]
fn ok_derive_tmp_issues_patch() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    let new_val = vec!["issue3".to_string()];

    change.a11y.tmp_issues = new_val.clone();

    let mut patch = change.derive_patch(&STOP1);
    assert_eq!(patch.tmp_issues, Some(new_val));

    // Ensure that the patch is only about this attribute
    patch.tmp_issues = None;
    assert!(patch.is_empty());
}

#[test]
fn ok_noop_tmp_issues_change() {
    let mut change: ChangeStop = STOP1_NOOP_CHANGE.clone();
    change.a11y.tmp_issues = STOP1.a11y.tmp_issues.clone();

    let patch = change.derive_patch(&STOP1);
    assert!(patch.is_empty());
}

#[test]
fn ok_apply_tmp_issues_patch() {
    let mut stop: Stop = STOP1.clone();
    let new_val = vec!["issue3".to_string()];

    let mut patch = StopPatch::default();
    patch.tmp_issues = Some(new_val.clone());

    assert!(patch.apply(&mut stop).is_ok());
    assert_eq!(stop.a11y.tmp_issues, new_val);

    // Revert, confirm that nothing else changed
    stop.a11y.tmp_issues = STOP1.a11y.tmp_issues.clone();
    assert_eq!(stop, *STOP1);
}
