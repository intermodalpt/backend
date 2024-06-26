use chrono::NaiveDate;
use once_cell::sync::Lazy;

use commons::models::history::stops::StopPatch;
use commons::models::stops;

use crate::contrib::logic;
use crate::errors::Error;

static STOP1: Lazy<stops::Stop> = Lazy::new(|| stops::Stop {
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
    a11y: stops::A11yMeta {
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
        illumination_strength: Some(stops::IlluminationStrength::None),
        illumination_position: Some(stops::IlluminationPos::Indirect),
        has_illuminated_path: Some(true),
        has_visibility_from_within: Some(true),
        has_visibility_from_area: Some(true),
        is_visible_from_outside: Some(true),
        parking_visibility_impairment: Some(
            stops::ParkingVisualLimitation::Little,
        ),
        parking_local_access_impairment: Some(
            stops::LocalParkingLimitation::Low,
        ),
        parking_area_access_impairment: Some(stops::AreaParkingLimitation::Low),
        advertisement_qty: Some(stops::AdvertisementQuantification::None),
        tmp_issues: vec![],
    },
    verification_level: 0,
    service_check_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
    infrastructure_check_date: Some(
        NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
    ),
    osm_id: None,
    license: "IML".to_string(),
    tags: vec!["tags".to_string()],
    notes: Some("notes".to_string()),
    is_ghost: false,
});

#[test]
fn ok_keep_verification() {
    let mut patch = StopPatch {
        has_shelter: Some(Some(false)),
        ..Default::default()
    };

    let mut current = STOP1.clone();
    current.verification_level = stops::StopVerification::verified().into();

    assert_eq!(current.a11y.has_shelter, Some(true));

    let resulting_stop =
        logic::accept_stop_contribution(current, &mut patch, true, &None)
            .unwrap();

    assert_eq!(resulting_stop.a11y.has_shelter, Some(false));
    assert_eq!(
        resulting_stop.verification(),
        stops::StopVerification {
            position: stops::Verification::Verified,
            service: stops::Verification::Verified,
            infrastructure: stops::Verification::Verified,
        }
    );
}

#[test]
fn ok_drop_verification() {
    let mut patch = StopPatch {
        has_shelter: Some(Some(false)),
        ..Default::default()
    };

    let mut current = STOP1.clone();
    current.verification_level = stops::StopVerification::verified().into();

    assert_eq!(current.a11y.has_shelter, Some(true));

    let resulting_stop =
        logic::accept_stop_contribution(current, &mut patch, false, &None)
            .unwrap();

    assert_eq!(resulting_stop.a11y.has_shelter, Some(false));
    assert_eq!(
        resulting_stop.verification(),
        stops::StopVerification {
            position: stops::Verification::Verified,
            service: stops::Verification::Verified,
            infrastructure: stops::Verification::NotVerified,
        }
    );
}

#[test]
fn err_only_patches_ignored_fields() {
    let mut patch = StopPatch {
        has_shelter: Some(Some(false)),
        ..Default::default()
    };

    let current = STOP1.clone();

    assert_eq!(current.a11y.has_shelter, Some(true));

    let error = logic::accept_stop_contribution(
        current,
        &mut patch,
        true,
        &Some("has_shelter".to_string()),
    )
    .unwrap_err();

    assert_eq!(
        error,
        Error::ValidationFailure("Patch no longer does anything".to_string())
    );
}

#[test]
fn err_repeated_patch_application() {
    let mut patch = StopPatch {
        name: Some("changed".to_string()),
        ..Default::default()
    };

    let current = STOP1.clone();

    assert_eq!(current.name, "name".to_string());
    assert_eq!(current.verification_level, 0);

    let resulting_stop =
        logic::accept_stop_contribution(current, &mut patch, true, &None)
            .unwrap();
    assert_eq!(resulting_stop.name, "changed".to_string());
    assert_eq!(resulting_stop.verification_level, 0);

    let error = logic::accept_stop_contribution(
        resulting_stop,
        &mut patch,
        true,
        &None,
    )
    .unwrap_err();
    assert_eq!(
        error,
        Error::ValidationFailure("Patch no longer does anything".to_string())
    );
}
