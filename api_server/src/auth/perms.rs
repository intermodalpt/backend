use commons::models::auth::Permissions;

use crate::auth::Claims;

pub(crate) trait ClaimPermission {
    fn is_valid(permissions: &Permissions) -> bool;
}

pub(crate) struct ScopedClaim<P: ClaimPermission>(
    pub(crate) Claims,
    pub(crate) std::marker::PhantomData<P>,
);

pub struct CreateRegion;

impl ClaimPermission for CreateRegion {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.regions.as_ref().is_some_and(|p| p.create)
    }
}

pub struct ModifyRegion;

impl ClaimPermission for ModifyRegion {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.regions.as_ref().is_some_and(|p| p.modify)
    }
}

pub struct DeleteRegion;

impl ClaimPermission for DeleteRegion {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.regions.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct CreateOperator;

impl ClaimPermission for CreateOperator {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.operators.as_ref().is_some_and(|p| p.create)
    }
}

pub struct ModifyOperatorMeta;

impl ClaimPermission for ModifyOperatorMeta {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .operators
            .as_ref()
            .is_some_and(|p| p.modify_base)
    }
}

pub struct ModifyOperatorStops;

impl ClaimPermission for ModifyOperatorStops {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .operators
            .as_ref()
            .is_some_and(|p| p.modify_stops)
    }
}

pub struct ModifyOperatorCalendars;

impl ClaimPermission for ModifyOperatorCalendars {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .operators
            .as_ref()
            .is_some_and(|p| p.modify_calendars)
    }
}

pub struct DeleteOperator;

impl ClaimPermission for DeleteOperator {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.operators.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct CreateRoute;

impl ClaimPermission for CreateRoute {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.routes.as_ref().is_some_and(|p| p.create)
    }
}
pub struct ModifyRouteBase;

impl ClaimPermission for ModifyRouteBase {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.routes.as_ref().is_some_and(|p| p.modify_base)
    }
}

pub struct ModifyRouteSubroutes;

impl ClaimPermission for ModifyRouteSubroutes {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .routes
            .as_ref()
            .is_some_and(|p| p.modify_subroutes)
    }
}

pub struct ModifyRouteStops;

impl ClaimPermission for ModifyRouteStops {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.routes.as_ref().is_some_and(|p| p.modify_stops)
    }
}

pub struct ModifyRouteDepartures;

impl ClaimPermission for ModifyRouteDepartures {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .routes
            .as_ref()
            .is_some_and(|p| p.modify_departures)
    }
}

pub struct AuthenticateRoute;

impl ClaimPermission for AuthenticateRoute {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.routes.as_ref().is_some_and(|p| p.authenticate)
    }
}

pub struct DeleteRoute;

impl ClaimPermission for DeleteRoute {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.routes.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct CreateStop;

impl ClaimPermission for CreateStop {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stops.as_ref().is_some_and(|p| p.create)
    }
}

pub struct ModifyStopPos;

impl ClaimPermission for ModifyStopPos {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stops.as_ref().is_some_and(|p| p.modify_pos)
    }
}

pub struct ModifyStopAttrs;

impl ClaimPermission for ModifyStopAttrs {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stops.as_ref().is_some_and(|p| p.modify_attrs)
    }
}

pub struct ModifyStopMapFeatures;

impl ClaimPermission for ModifyStopMapFeatures {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stops
            .as_ref()
            .is_some_and(|p| p.modify_map_features)
    }
}

pub struct AuthenticateStop;

impl ClaimPermission for AuthenticateStop {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stops.as_ref().is_some_and(|p| p.authenticate)
    }
}

pub struct DeleteStop;

impl ClaimPermission for DeleteStop {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stops.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct ContribModifyStopAttrs;

impl ClaimPermission for ContribModifyStopAttrs {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stops
            .as_ref()
            .is_some_and(|p| p.contrib_modify_attrs)
    }
}

pub struct UpdateOsmStops;

impl ClaimPermission for UpdateOsmStops {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.osm_stops.as_ref().is_some_and(|p| p.update)
    }
}

pub struct DeleteOsmStop;

impl ClaimPermission for DeleteOsmStop {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.osm_stops.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct UploadStopPic;

impl ClaimPermission for UploadStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stop_pics.as_ref().is_some_and(|p| p.upload)
    }
}

pub struct ViewUntaggedStopPic;

impl ClaimPermission for ViewUntaggedStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stop_pics
            .as_ref()
            .is_some_and(|p| p.view_untagged)
    }
}

pub struct ViewSensitiveStopPic;

impl ClaimPermission for ViewSensitiveStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stop_pics
            .as_ref()
            .is_some_and(|p| p.view_sensitive)
    }
}

pub struct ModifyOwnStopPic;
impl ClaimPermission for ModifyOwnStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stop_pics.as_ref().is_some_and(|p| p.modify_own)
    }
}

pub struct ModifyOthersStopPic;

impl ClaimPermission for ModifyOthersStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stop_pics
            .as_ref()
            .is_some_and(|p| p.modify_others)
    }
}

pub struct DeleteStopPic;

impl ClaimPermission for DeleteStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.stop_pics.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct ContribUploadStopPic;

impl ClaimPermission for ContribUploadStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stop_pics
            .as_ref()
            .is_some_and(|p| p.contrib_upload)
    }
}

pub struct ContribModifyStopPic;

impl ClaimPermission for ContribModifyStopPic {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .stop_pics
            .as_ref()
            .is_some_and(|p| p.contrib_modify)
    }
}

pub struct CreateNews;

impl ClaimPermission for CreateNews {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.news.as_ref().is_some_and(|p| p.create)
    }
}

pub struct ModifyNews;

impl ClaimPermission for ModifyNews {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.news.as_ref().is_some_and(|p| p.modify)
    }
}

pub struct DeleteNews;

impl ClaimPermission for DeleteNews {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.news.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct ReadPrivateExternalNews;

impl ClaimPermission for ReadPrivateExternalNews {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .external_news
            .as_ref()
            .is_some_and(|p| p.read_private)
    }
}

pub struct ModifyExternalNews;

impl ClaimPermission for ModifyExternalNews {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.external_news.as_ref().is_some_and(|p| p.modify)
    }
}

pub struct DeleteExternalNews;

impl ClaimPermission for DeleteExternalNews {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.external_news.as_ref().is_some_and(|p| p.delete)
    }
}

pub struct ReadAuditLog;

impl ClaimPermission for ReadAuditLog {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.admin.as_ref().is_some_and(|p| p.read_audit_log)
    }
}

pub struct ManageUserSessions;

impl ClaimPermission for ManageUserSessions {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .admin
            .as_ref()
            .is_some_and(|p| p.manage_user_sessions)
    }
}

pub struct ChangePasswords;

impl ClaimPermission for ChangePasswords {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .admin
            .as_ref()
            .is_some_and(|p| p.change_passwords)
    }
}

pub struct ChangePermissions;

impl ClaimPermission for ChangePermissions {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .admin
            .as_ref()
            .is_some_and(|p| p.change_permissions)
    }
}

pub struct SuspendUsers;

impl ClaimPermission for SuspendUsers {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.admin.as_ref().is_some_and(|p| p.suspend_users)
    }
}

pub struct ModifyIssues;

impl ClaimPermission for ModifyIssues {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.misc.as_ref().is_some_and(|p| p.modify_issues)
    }
}

pub struct HandleContrib;

impl ClaimPermission for HandleContrib {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions
            .misc
            .as_ref()
            .is_some_and(|p| p.contrib_evaluator)
    }
}

pub struct ExpensiveCalls;

impl ClaimPermission for ExpensiveCalls {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.misc.as_ref().is_some_and(|p| p.expensive_calls)
    }
}

pub struct PatchGtfs;

impl ClaimPermission for PatchGtfs {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.misc.as_ref().is_some_and(|p| p.patch_gtfs)
    }
}
