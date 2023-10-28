use commons::models::history::StopPatch;
use commons::models::stops;

#[test]
fn stop_patch_drop_verification_level() {
    let mut patch = StopPatch {
        verification_level: Some(0),
        ..Default::default()
    };

    let fields = vec!["verification_level"].into_iter().collect();

    assert!(patch.verification_level.is_some());
    patch.drop_fields(&fields);
    assert!(patch.verification_level.is_none());
    assert!(patch.is_empty());
}

#[test]
fn stop_patch_deverify_empty_patch() {
    let mut patch = StopPatch {
        verification_level: Some(stops::StopVerification::verified().into()),
        ..Default::default()
    };

    patch.deverify(stops::StopVerification::verified().into());

    assert_eq!(patch.verification_level, None);
}

#[test]
fn stop_patch_deverify_deverification_patch() {
    let mut patch = StopPatch {
        verification_level: Some(stops::StopVerification::unverified().into()),
        ..Default::default()
    };

    patch.deverify(stops::StopVerification::verified().into());

    assert_eq!(
        patch.verification_level,
        Some(stops::StopVerification::unverified().into())
    );
}

#[test]
fn stop_patch_deverify_non_empty_patch() {
    let mut patch = StopPatch {
        verification_level: Some(stops::StopVerification::verified().into()),
        ..Default::default()
    };

    patch.deverify(stops::StopVerification::unverified().into());

    assert_eq!(patch.verification_level, None);
}

#[test]
fn stop_patch_deverify_patch_with_service_changes() {
    let mut patch = StopPatch {
        flags: Some(None),
        ..Default::default()
    };

    patch.deverify(stops::StopVerification::verified().into());

    let resulting_verification =
        stops::StopVerification::from(patch.verification_level.unwrap());
    assert_eq!(
        resulting_verification.service,
        stops::Verification::NotVerified
    );
    assert_eq!(
        resulting_verification.infrastructure,
        stops::Verification::Verified
    );
    assert_eq!(
        resulting_verification.position,
        stops::Verification::Verified
    );
}

#[test]
fn stop_patch_deverify_patch_with_infra_changes() {
    let mut patch = StopPatch {
        has_sidewalk: Some(None),
        ..Default::default()
    };

    patch.deverify(stops::StopVerification::verified().into());

    let resulting_verification =
        stops::StopVerification::from(patch.verification_level.unwrap());
    assert_eq!(
        resulting_verification.service,
        stops::Verification::Verified
    );
    assert_eq!(
        resulting_verification.infrastructure,
        stops::Verification::NotVerified
    );
    assert_eq!(
        resulting_verification.position,
        stops::Verification::Verified
    );
}

#[test]
fn stop_patch_deverify_full_patch() {
    let mut patch = StopPatch {
        name: Some(None),
        short_name: Some(None),
        locality: Some(None),
        street: Some(None),
        door: Some(None),
        schedules: Some(None),
        flags: Some(None),
        has_sidewalk: Some(None),
        has_sidewalked_path: Some(None),
        has_shelter: Some(None),
        has_cover: Some(None),
        has_bench: Some(None),
        has_trash_can: Some(None),
        has_waiting_times: Some(None),
        has_ticket_seller: Some(None),
        has_costumer_support: Some(None),
        advertisement_qty: Some(None),
        has_crossing: Some(None),
        has_wide_access: Some(None),
        has_flat_access: Some(None),
        has_tactile_access: Some(None),
        illumination_strength: Some(None),
        illumination_position: Some(None),
        has_illuminated_path: Some(None),
        has_visibility_from_within: Some(None),
        has_visibility_from_area: Some(None),
        is_visible_from_outside: Some(None),
        parking_visibility_impairment: Some(None),
        parking_local_access_impairment: Some(None),
        parking_area_access_impairment: Some(None),
        tmp_issues: Some(vec![]),
        tags: Some(vec![]),
        notes: Some(None),
        service_check_date: Some(None),
        infrastructure_check_date: Some(None),
        verification_level: None,
    };

    patch.deverify(stops::StopVerification::verified().into());

    let resulting_verification =
        stops::StopVerification::from(patch.verification_level.unwrap());
    assert_eq!(
        resulting_verification.service,
        stops::Verification::NotVerified
    );
    assert_eq!(
        resulting_verification.infrastructure,
        stops::Verification::NotVerified
    );
    assert_eq!(
        resulting_verification.position,
        stops::Verification::Verified
    );
}
