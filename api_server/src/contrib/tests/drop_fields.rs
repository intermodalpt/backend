use chrono::NaiveDate;

use commons::models::history::{self, stops::StopPatch};
#[test]
fn stop_patch_drop_name() {
    let mut patch = StopPatch {
        name: Some("foo".to_string()),
        ..Default::default()
    };

    let fields = vec!["name"].into_iter().collect();

    assert!(patch.name.is_some());
    patch.drop_fields(&fields);
    assert!(patch.name.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_short_name() {
    let mut patch = StopPatch {
        short_name: Some(Some("foo".to_string())),
        ..Default::default()
    };

    let fields = vec!["short_name"].into_iter().collect();

    assert!(patch.short_name.is_some());
    patch.drop_fields(&fields);
    assert!(patch.short_name.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_locality() {
    let mut patch = StopPatch {
        locality: Some(Some("foo".to_string())),
        ..Default::default()
    };

    let fields = vec!["locality"].into_iter().collect();

    assert!(patch.locality.is_some());
    patch.drop_fields(&fields);
    assert!(patch.locality.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_street() {
    let mut patch = StopPatch {
        street: Some(Some("foo".to_string())),
        ..Default::default()
    };

    let fields = vec!["street"].into_iter().collect();

    assert!(patch.street.is_some());
    patch.drop_fields(&fields);
    assert!(patch.street.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_door() {
    let mut patch = StopPatch {
        door: Some(Some("foo".to_string())),
        ..Default::default()
    };

    let fields = vec!["door"].into_iter().collect();

    assert!(patch.door.is_some());
    patch.drop_fields(&fields);
    assert!(patch.door.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_schedules() {
    let mut patch = StopPatch {
        schedules: Some(None),
        ..Default::default()
    };

    let fields = vec!["schedules"].into_iter().collect();

    assert!(patch.schedules.is_some());
    patch.drop_fields(&fields);
    assert!(patch.schedules.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_flags() {
    let mut patch = StopPatch {
        flags: Some(None),
        ..Default::default()
    };

    let fields = vec!["flags"].into_iter().collect();

    assert!(patch.flags.is_some());
    patch.drop_fields(&fields);
    assert!(patch.flags.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_sidewalk() {
    let mut patch = StopPatch {
        has_sidewalk: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_sidewalk"].into_iter().collect();

    assert!(patch.has_sidewalk.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_sidewalk.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_sidewalked_path() {
    let mut patch = StopPatch {
        has_sidewalked_path: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_sidewalked_path"].into_iter().collect();

    assert!(patch.has_sidewalked_path.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_sidewalked_path.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_shelter() {
    let mut patch = StopPatch {
        has_shelter: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_shelter"].into_iter().collect();

    assert!(patch.has_shelter.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_shelter.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_cover() {
    let mut patch = StopPatch {
        has_cover: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_cover"].into_iter().collect();

    assert!(patch.has_cover.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_cover.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_bench() {
    let mut patch = StopPatch {
        has_bench: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_bench"].into_iter().collect();

    assert!(patch.has_bench.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_bench.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_trash_can() {
    let mut patch = StopPatch {
        has_trash_can: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_trash_can"].into_iter().collect();

    assert!(patch.has_trash_can.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_trash_can.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_waiting_times() {
    let mut patch = StopPatch {
        has_waiting_times: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_waiting_times"].into_iter().collect();

    assert!(patch.has_waiting_times.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_waiting_times.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_ticket_seller() {
    let mut patch = StopPatch {
        has_ticket_seller: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_ticket_seller"].into_iter().collect();

    assert!(patch.has_ticket_seller.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_ticket_seller.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_costumer_support() {
    let mut patch = StopPatch {
        has_costumer_support: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_costumer_support"].into_iter().collect();

    assert!(patch.has_costumer_support.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_costumer_support.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_advertisement_qty() {
    let mut patch = StopPatch {
        advertisement_qty: Some(Some(
            history::stops::AdvertisementQuantification::None,
        )),
        ..Default::default()
    };

    let fields = vec!["advertisement_qty"].into_iter().collect();

    assert!(patch.advertisement_qty.is_some());
    patch.drop_fields(&fields);
    assert!(patch.advertisement_qty.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_crossing() {
    let mut patch = StopPatch {
        has_crossing: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_crossing"].into_iter().collect();

    assert!(patch.has_crossing.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_crossing.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_wide_access() {
    let mut patch = StopPatch {
        has_wide_access: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_wide_access"].into_iter().collect();

    assert!(patch.has_wide_access.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_wide_access.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_flat_access() {
    let mut patch = StopPatch {
        has_flat_access: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_flat_access"].into_iter().collect();

    assert!(patch.has_flat_access.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_flat_access.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_tactile_access() {
    let mut patch = StopPatch {
        has_tactile_access: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_tactile_access"].into_iter().collect();

    assert!(patch.has_tactile_access.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_tactile_access.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_illumination_strength() {
    let mut patch = StopPatch {
        illumination_strength: Some(Some(
            history::stops::IlluminationStrength::None,
        )),
        ..Default::default()
    };

    let fields = vec!["illumination_strength"].into_iter().collect();

    assert!(patch.illumination_strength.is_some());
    patch.drop_fields(&fields);
    assert!(patch.illumination_strength.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_illumination_position() {
    let mut patch = StopPatch {
        illumination_position: Some(Some(
            history::stops::IlluminationPos::Indirect,
        )),
        ..Default::default()
    };

    let fields = vec!["illumination_position"].into_iter().collect();

    assert!(patch.illumination_position.is_some());
    patch.drop_fields(&fields);
    assert!(patch.illumination_position.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_illuminated_path() {
    let mut patch = StopPatch {
        has_illuminated_path: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_illuminated_path"].into_iter().collect();

    assert!(patch.has_illuminated_path.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_illuminated_path.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_visibility_from_within() {
    let mut patch = StopPatch {
        has_visibility_from_within: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_visibility_from_within"].into_iter().collect();

    assert!(patch.has_visibility_from_within.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_visibility_from_within.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_has_visibility_from_area() {
    let mut patch = StopPatch {
        has_visibility_from_area: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["has_visibility_from_area"].into_iter().collect();

    assert!(patch.has_visibility_from_area.is_some());
    patch.drop_fields(&fields);
    assert!(patch.has_visibility_from_area.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_is_visible_from_outside() {
    let mut patch = StopPatch {
        is_visible_from_outside: Some(Some(true)),
        ..Default::default()
    };

    let fields = vec!["is_visible_from_outside"].into_iter().collect();

    assert!(patch.is_visible_from_outside.is_some());
    patch.drop_fields(&fields);
    assert!(patch.is_visible_from_outside.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_parking_visibility_impairment() {
    let mut patch = StopPatch {
        parking_visibility_impairment: Some(Some(
            history::stops::ParkingVisualLimitation::None,
        )),
        ..Default::default()
    };

    let fields = vec!["parking_visibility_impairment"].into_iter().collect();

    assert!(patch.parking_visibility_impairment.is_some());
    patch.drop_fields(&fields);
    assert!(patch.parking_visibility_impairment.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_parking_local_access_impairment() {
    let mut patch = StopPatch {
        parking_local_access_impairment: Some(Some(
            history::stops::LocalParkingLimitation::None,
        )),
        ..Default::default()
    };

    let fields = vec!["parking_local_access_impairment"]
        .into_iter()
        .collect();

    assert!(patch.parking_local_access_impairment.is_some());
    patch.drop_fields(&fields);
    assert!(patch.parking_local_access_impairment.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_parking_area_access_impairment() {
    let mut patch = StopPatch {
        parking_area_access_impairment: Some(Some(
            history::stops::AreaParkingLimitation::None,
        )),
        ..Default::default()
    };

    let fields = vec!["parking_area_access_impairment"].into_iter().collect();

    assert!(patch.parking_area_access_impairment.is_some());
    patch.drop_fields(&fields);
    assert!(patch.parking_area_access_impairment.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_tmp_issues() {
    let mut patch = StopPatch {
        tmp_issues: Some(vec![]),
        ..Default::default()
    };

    let fields = vec!["tmp_issues"].into_iter().collect();

    assert!(patch.tmp_issues.is_some());
    patch.drop_fields(&fields);
    assert!(patch.tmp_issues.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_tags() {
    let mut patch = StopPatch {
        tags: Some(vec![]),
        ..Default::default()
    };

    let fields = vec!["tags"].into_iter().collect();

    assert!(patch.tags.is_some());
    patch.drop_fields(&fields);
    assert!(patch.tags.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_notes() {
    let mut patch = StopPatch {
        notes: Some(Some("notes".to_string())),
        ..Default::default()
    };

    let fields = vec!["notes"].into_iter().collect();

    assert!(patch.notes.is_some());
    patch.drop_fields(&fields);
    assert!(patch.notes.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_service_check_date() {
    let mut patch = StopPatch {
        service_check_date: Some(Some(NaiveDate::default())),
        ..Default::default()
    };

    let fields = vec!["service_check_date"].into_iter().collect();

    assert!(patch.service_check_date.is_some());
    patch.drop_fields(&fields);
    assert!(patch.service_check_date.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_drop_infrastructure_check_date() {
    let mut patch = StopPatch {
        infrastructure_check_date: Some(Some(NaiveDate::default())),
        ..Default::default()
    };

    let fields = vec!["infrastructure_check_date"].into_iter().collect();

    assert!(patch.infrastructure_check_date.is_some());
    patch.drop_fields(&fields);
    assert!(patch.infrastructure_check_date.is_none());
    assert!(patch.is_empty());
}
