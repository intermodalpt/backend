use serde::{Deserialize, Serialize};

use crate::auth::Claims;

pub(crate) trait ClaimPermission {
    fn is_valid(permissions: &[Permission]) -> bool;
}

pub(crate) struct ScopedClaim<P: ClaimPermission>(
    pub(crate) Claims,
    pub(crate) std::marker::PhantomData<P>,
);

fn is_false(val: &bool) -> bool {
    !val
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "perm")]
pub enum Permission {
    // TODO find a way to implement these two
    Region {
        region_id: i32,
        permissions: Vec<Permission>,
    },
    Operator {
        operator_id: i32,
        permissions: Vec<Permission>,
    },
    Regions {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    Operators {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_base: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_stops: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_calendars: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    Routes {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_base: bool,
        // This refers to the direct data, not the derivatives
        // such as the stops and departures
        #[serde(default, skip_serializing_if = "is_false")]
        modify_subroutes: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_stops: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_departures: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        validate_gtfs: bool,
        // Delete as a separate perm makes sense because it
        // allows the modification of everything else
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    Stops {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_pos: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_attrs: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_map_features: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        contrib_modify_attrs: bool,
    },
    OsmStops {
        #[serde(default, skip_serializing_if = "is_false")]
        update: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    StopPics {
        #[serde(default, skip_serializing_if = "is_false")]
        upload: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        view_untagged: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        view_sensitive: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_own: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify_others: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        contrib_upload: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        contrib_modify: bool,
    },
    News {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    ExternalNews {
        #[serde(default, skip_serializing_if = "is_false")]
        read_private: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    Admin {
        #[serde(default, skip_serializing_if = "is_false")]
        read_audit_log: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        manage_user_sessions: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        change_passwords: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        change_permissions: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        suspend_users: bool,
    },
    Misc {
        #[serde(default, skip_serializing_if = "is_false")]
        modify_issues: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        handle_contrib: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        expensive_calls: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        patch_gtfs: bool,
    },
}

impl Permission {
    fn every_operators_perm() -> Permission {
        Permission::Operators {
            create: true,
            modify_base: true,
            modify_stops: true,
            modify_calendars: true,
            delete: true,
        }
    }

    fn every_routes_perm() -> Permission {
        Permission::Routes {
            create: true,
            modify_base: true,
            modify_subroutes: false,
            modify_stops: true,
            modify_departures: true,
            validate_gtfs: true,
            delete: true,
        }
    }

    fn every_stops_perm() -> Permission {
        Permission::Stops {
            create: true,
            modify_pos: true,
            modify_attrs: true,
            modify_map_features: true,
            delete: true,
            contrib_modify_attrs: false,
        }
    }

    fn every_osm_stops_perm() -> Permission {
        Permission::OsmStops {
            update: true,
            delete: true,
        }
    }

    fn every_stop_pics_perm() -> Permission {
        Permission::StopPics {
            upload: true,
            view_untagged: true,
            view_sensitive: true,
            modify_own: true,
            modify_others: true,
            delete: true,
            contrib_upload: false,
            contrib_modify: false,
        }
    }

    fn every_news_perm() -> Permission {
        Permission::News {
            create: true,
            modify: true,
            delete: true,
        }
    }

    fn every_external_news_perm() -> Permission {
        Permission::ExternalNews {
            read_private: true,
            modify: true,
            delete: true,
        }
    }

    fn every_admin_perm() -> Permission {
        Permission::Admin {
            read_audit_log: true,
            manage_user_sessions: true,
            change_passwords: true,
            change_permissions: true,
            suspend_users: true,
        }
    }

    fn every_misc_perm() -> Permission {
        Permission::Misc {
            modify_issues: true,
            handle_contrib: true,
            expensive_calls: true,
            patch_gtfs: true,
        }
    }

    pub(crate) fn admin_default() -> Vec<Permission> {
        vec![
            Self::every_operators_perm(),
            Self::every_routes_perm(),
            Self::every_stops_perm(),
            Self::every_osm_stops_perm(),
            Self::every_stop_pics_perm(),
            Self::every_news_perm(),
            Self::every_external_news_perm(),
            Self::every_admin_perm(),
            Self::every_misc_perm(),
        ]
    }

    pub(crate) fn trusted_default() -> Vec<Permission> {
        vec![
            Self::every_routes_perm(),
            Self::every_stops_perm(),
            Self::every_stop_pics_perm(),
        ]
    }

    pub(crate) fn operator_default(operator_id: i32) -> Vec<Permission> {
        vec![Permission::Operator {
            operator_id,
            permissions: vec![
                Permission::Routes {
                    create: true,
                    modify_base: true,
                    modify_subroutes: true,
                    modify_stops: true,
                    modify_departures: true,
                    validate_gtfs: true,
                    delete: true,
                },
                Permission::Stops {
                    create: true,
                    modify_pos: true,
                    modify_attrs: true,
                    modify_map_features: true,
                    delete: true,
                    contrib_modify_attrs: false,
                },
                Permission::StopPics {
                    upload: true,
                    view_untagged: true,
                    view_sensitive: true,
                    modify_own: true,
                    modify_others: true,
                    delete: true,
                    contrib_upload: false,
                    contrib_modify: false,
                },
            ],
        }]
    }

    pub(crate) fn user_default() -> Vec<Permission> {
        vec![
            Permission::Stops {
                create: false,
                modify_pos: false,
                modify_attrs: false,
                modify_map_features: false,
                delete: false,
                contrib_modify_attrs: true,
            },
            Permission::StopPics {
                upload: false,
                view_untagged: false,
                view_sensitive: false,
                modify_own: false,
                modify_others: false,
                delete: false,
                contrib_upload: true,
                contrib_modify: true,
            },
        ]
    }
}

pub struct CreateRegion;

impl ClaimPermission for CreateRegion {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Regions { create: true, .. }))
    }
}

pub struct ModifyRegion;

impl ClaimPermission for ModifyRegion {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Regions { modify: true, .. }))
    }
}

pub struct DeleteRegion;

impl ClaimPermission for DeleteRegion {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Regions { delete: true, .. }))
    }
}

pub struct CreateOperator;

impl ClaimPermission for CreateOperator {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Operators { create: true, .. }))
    }
}

pub struct ModifyOperatorMeta;

impl ClaimPermission for ModifyOperatorMeta {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Operators {
                    modify_base: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyOperatorStops;

impl ClaimPermission for ModifyOperatorStops {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Operators {
                    modify_stops: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyOperatorCalendars;

impl ClaimPermission for ModifyOperatorCalendars {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Operators {
                    modify_calendars: true,
                    ..
                }
            )
        })
    }
}

pub struct DeleteOperator;

impl ClaimPermission for DeleteOperator {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Operators { delete: true, .. }))
    }
}

pub struct CreateRoute;

impl ClaimPermission for CreateRoute {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Routes { create: true, .. }))
    }
}
pub struct ModifyRouteBase;

impl ClaimPermission for ModifyRouteBase {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Routes {
                    modify_base: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyRouteSubroutes;

impl ClaimPermission for ModifyRouteSubroutes {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Routes {
                    modify_subroutes: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyRouteStops;

impl ClaimPermission for ModifyRouteStops {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Routes {
                    modify_stops: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyRouteDepartures;

impl ClaimPermission for ModifyRouteDepartures {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Routes {
                    modify_departures: true,
                    ..
                }
            )
        })
    }
}

pub struct ValidateRouteGtfs;

impl ClaimPermission for ValidateRouteGtfs {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Routes {
                    validate_gtfs: true,
                    ..
                }
            )
        })
    }
}

pub struct DeleteRoute;

impl ClaimPermission for DeleteRoute {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Routes { delete: true, .. }))
    }
}

pub struct CreateStop;

impl ClaimPermission for CreateStop {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Stops { create: true, .. }))
    }
}

pub struct ModifyStopPos;

impl ClaimPermission for ModifyStopPos {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Stops {
                    modify_pos: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyStopAttrs;

impl ClaimPermission for ModifyStopAttrs {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Stops {
                    modify_attrs: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyStopMapFeatures;

impl ClaimPermission for ModifyStopMapFeatures {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Stops {
                    modify_map_features: true,
                    ..
                }
            )
        })
    }
}
pub struct DeleteStop;

impl ClaimPermission for DeleteStop {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Stops { delete: true, .. }))
    }
}

pub struct ContribModifyStopAttrs;

impl ClaimPermission for ContribModifyStopAttrs {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Stops {
                    contrib_modify_attrs: true,
                    ..
                }
            )
        })
    }
}

pub struct UpdateOsmStops;

impl ClaimPermission for UpdateOsmStops {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::OsmStops { update: true, .. }))
    }
}

pub struct DeleteOsmStop;

impl ClaimPermission for DeleteOsmStop {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::OsmStops { delete: true, .. }))
    }
}

pub struct UploadStopPic;

impl ClaimPermission for UploadStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::StopPics { upload: true, .. }))
    }
}

pub struct ViewUntaggedStopPic;

impl ClaimPermission for ViewUntaggedStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::StopPics {
                    view_untagged: true,
                    ..
                }
            )
        })
    }
}

pub struct ViewSensitiveStopPic;

impl ClaimPermission for ViewSensitiveStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::StopPics {
                    view_sensitive: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyOwnStopPic;
impl ClaimPermission for ModifyOwnStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::StopPics {
                    modify_own: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyOthersStopPic;

impl ClaimPermission for ModifyOthersStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::StopPics {
                    modify_others: true,
                    ..
                }
            )
        })
    }
}

pub struct DeleteStopPic;

impl ClaimPermission for DeleteStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::StopPics { delete: true, .. }))
    }
}

pub struct ContribUploadStopPic;

impl ClaimPermission for ContribUploadStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::StopPics {
                    contrib_upload: true,
                    ..
                }
            )
        })
    }
}

pub struct ContribModifyStopPic;

impl ClaimPermission for ContribModifyStopPic {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::StopPics {
                    contrib_modify: true,
                    ..
                }
            )
        })
    }
}

pub struct CreateNews;

impl ClaimPermission for CreateNews {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::News { create: true, .. }))
    }
}

pub struct ModifyNews;

impl ClaimPermission for ModifyNews {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::News { modify: true, .. }))
    }
}

pub struct DeleteNews;

impl ClaimPermission for DeleteNews {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::News { delete: true, .. }))
    }
}

pub struct ReadPrivateExternalNews;

impl ClaimPermission for ReadPrivateExternalNews {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::ExternalNews {
                    read_private: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyExternalNews;

impl ClaimPermission for ModifyExternalNews {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::ExternalNews { modify: true, .. }))
    }
}

pub struct DeleteExternalNews;

impl ClaimPermission for DeleteExternalNews {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .any(|p| matches!(p, Permission::ExternalNews { delete: true, .. }))
    }
}

pub struct ReadAuditLog;

impl ClaimPermission for ReadAuditLog {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Admin {
                    read_audit_log: true,
                    ..
                }
            )
        })
    }
}

pub struct ManageUserSessions;

impl ClaimPermission for ManageUserSessions {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Admin {
                    manage_user_sessions: true,
                    ..
                }
            )
        })
    }
}

pub struct ChangePasswords;

impl ClaimPermission for ChangePasswords {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Admin {
                    change_passwords: true,
                    ..
                }
            )
        })
    }
}

pub struct ChangePermissions;

impl ClaimPermission for ChangePermissions {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Admin {
                    change_permissions: true,
                    ..
                }
            )
        })
    }
}

pub struct SuspendUsers;

impl ClaimPermission for SuspendUsers {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Admin {
                    suspend_users: true,
                    ..
                }
            )
        })
    }
}

pub struct ModifyIssues;

impl ClaimPermission for ModifyIssues {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Misc {
                    modify_issues: true,
                    ..
                }
            )
        })
    }
}

pub struct HandleContrib;

impl ClaimPermission for HandleContrib {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Misc {
                    handle_contrib: true,
                    ..
                }
            )
        })
    }
}

pub struct ExpensiveCalls;

impl ClaimPermission for ExpensiveCalls {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Misc {
                    expensive_calls: true,
                    ..
                }
            )
        })
    }
}

pub struct PatchGtfs;

impl ClaimPermission for PatchGtfs {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| {
            matches!(
                p,
                Permission::Misc {
                    patch_gtfs: true,
                    ..
                }
            )
        })
    }
}
