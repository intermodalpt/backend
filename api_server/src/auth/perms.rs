use serde::{Deserialize, Serialize};

use crate::auth::Claims;

pub(crate) trait ClaimPermission {
    fn is_valid(permissions: &Permissions) -> bool;
}

pub(crate) struct ScopedClaim<P: ClaimPermission>(
    pub(crate) Claims,
    pub(crate) std::marker::PhantomData<P>,
);

pub(crate) fn is_false(val: &bool) -> bool {
    !val
}

pub(crate) mod subperm {
    use serde::{Deserialize, Serialize};

    use super::is_false;

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct Regions {
        #[serde(default, skip_serializing_if = "is_false")]
        pub create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
    }

    impl Regions {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.create = self.create || perms.create;
            self.modify = self.modify || perms.modify;
            self.delete = self.delete || perms.delete;
        }

        pub(crate) fn everything() -> Self {
            Self {
                create: true,
                modify: true,
                delete: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct Operators {
        #[serde(default, skip_serializing_if = "is_false")]
        pub create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_base: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_stops: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_calendars: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
    }

    impl Operators {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.create = self.create || perms.create;
            self.modify_base = self.modify_base || perms.modify_base;
            self.modify_stops = self.modify_stops || perms.modify_stops;
            self.modify_calendars =
                self.modify_calendars || perms.modify_calendars;
            self.delete = self.delete || perms.delete;
        }

        pub(crate) fn everything() -> Self {
            Self {
                create: true,
                modify_base: true,
                modify_stops: true,
                modify_calendars: true,
                delete: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct Routes {
        #[serde(default, skip_serializing_if = "is_false")]
        pub create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_base: bool,
        // This refers to the direct data, not the derivatives
        // such as the stops and departures
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_subroutes: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_stops: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_departures: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub validate_gtfs: bool,
        // Delete as a separate perm makes sense because it
        // allows the modification of everything else
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
    }

    impl Routes {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.create = self.create || perms.create;
            self.modify_base = self.modify_base || perms.modify_base;
            self.modify_subroutes =
                self.modify_subroutes || perms.modify_subroutes;
            self.modify_stops = self.modify_stops || perms.modify_stops;
            self.modify_departures =
                self.modify_departures || perms.modify_departures;
            self.validate_gtfs = self.validate_gtfs || perms.validate_gtfs;
            self.delete = self.delete || perms.delete;
        }

        pub(crate) fn everything() -> Self {
            Self {
                create: true,
                modify_base: true,
                modify_subroutes: true,
                modify_stops: true,
                modify_departures: true,
                validate_gtfs: true,
                delete: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct Stops {
        #[serde(default, skip_serializing_if = "is_false")]
        pub create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_pos: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_attrs: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_map_features: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub contrib_modify_attrs: bool,
    }

    impl Stops {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.create = self.create || perms.create;
            self.modify_pos = self.modify_pos || perms.modify_pos;
            self.modify_attrs = self.modify_attrs || perms.modify_attrs;
            self.modify_map_features =
                self.modify_map_features || perms.modify_map_features;
            self.delete = self.delete || perms.delete;
            self.contrib_modify_attrs =
                self.contrib_modify_attrs || perms.contrib_modify_attrs;
        }

        pub(crate) fn everything() -> Self {
            Self {
                create: true,
                modify_pos: true,
                modify_attrs: true,
                modify_map_features: true,
                delete: true,
                contrib_modify_attrs: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]

    pub(crate) struct OsmStops {
        #[serde(default, skip_serializing_if = "is_false")]
        pub update: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
    }

    impl OsmStops {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.update = self.update || perms.update;
            self.delete = self.delete || perms.delete;
        }

        pub(crate) fn everything() -> Self {
            Self {
                update: true,
                delete: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct StopPics {
        #[serde(default, skip_serializing_if = "is_false")]
        pub upload: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub view_untagged: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub view_sensitive: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_own: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_others: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub contrib_upload: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub contrib_modify: bool,
    }

    impl StopPics {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.upload = self.upload || perms.upload;
            self.view_untagged = self.view_untagged || perms.view_untagged;
            self.view_sensitive = self.view_sensitive || perms.view_sensitive;
            self.modify_own = self.modify_own || perms.modify_own;
            self.modify_others = self.modify_others || perms.modify_others;
            self.delete = self.delete || perms.delete;
            self.contrib_upload = self.contrib_upload || perms.contrib_upload;
            self.contrib_modify = self.contrib_modify || perms.contrib_modify;
        }

        pub(crate) fn everything() -> Self {
            Self {
                upload: true,
                view_untagged: true,
                view_sensitive: true,
                modify_own: true,
                modify_others: true,
                delete: true,
                contrib_upload: true,
                contrib_modify: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct News {
        #[serde(default, skip_serializing_if = "is_false")]
        pub create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
    }

    impl News {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.create = self.create || perms.create;
            self.modify = self.modify || perms.modify;
            self.delete = self.delete || perms.delete;
        }

        pub(crate) fn everything() -> Self {
            Self {
                create: true,
                modify: true,
                delete: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct ExternalNews {
        #[serde(default, skip_serializing_if = "is_false")]
        pub read_private: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub delete: bool,
    }

    impl ExternalNews {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.read_private = self.read_private || perms.read_private;
            self.modify = self.modify || perms.modify;
            self.delete = self.delete || perms.delete;
        }

        pub(crate) fn everything() -> Self {
            Self {
                read_private: true,
                modify: true,
                delete: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct Admin {
        #[serde(default, skip_serializing_if = "is_false")]
        pub read_audit_log: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub manage_user_sessions: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub change_passwords: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub change_permissions: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub suspend_users: bool,
    }

    impl Admin {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.read_audit_log = self.read_audit_log || perms.read_audit_log;
            self.manage_user_sessions =
                self.manage_user_sessions || perms.manage_user_sessions;
            self.change_passwords =
                self.change_passwords || perms.change_passwords;
            self.change_permissions =
                self.change_permissions || perms.change_permissions;
            self.suspend_users = self.suspend_users || perms.suspend_users;
        }

        pub(crate) fn everything() -> Self {
            Self {
                read_audit_log: true,
                manage_user_sessions: true,
                change_passwords: true,
                change_permissions: true,
                suspend_users: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    pub(crate) struct Misc {
        #[serde(default, skip_serializing_if = "is_false")]
        pub modify_issues: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub handle_contrib: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub expensive_calls: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        pub patch_gtfs: bool,
    }

    impl Misc {
        pub(crate) fn merge(&mut self, perms: &Self) {
            self.modify_issues = self.modify_issues || perms.modify_issues;
            self.handle_contrib = self.handle_contrib || perms.handle_contrib;
            self.expensive_calls =
                self.expensive_calls || perms.expensive_calls;
            self.patch_gtfs = self.patch_gtfs || perms.patch_gtfs;
        }

        pub(crate) fn everything() -> Self {
            Self {
                modify_issues: true,
                handle_contrib: true,
                expensive_calls: true,
                patch_gtfs: true,
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "perm")]
pub(crate) struct Permissions {
    // TODO find a way to implement these two
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // region: Option<RegionPerm>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // operator: Option<OperatorPerm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<subperm::Regions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operators: Option<subperm::Operators>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<subperm::Routes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stops: Option<subperm::Stops>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub osm_stops: Option<subperm::OsmStops>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pics: Option<subperm::StopPics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub news: Option<subperm::News>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_news: Option<subperm::ExternalNews>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<subperm::Admin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc: Option<subperm::Misc>,
}

// TODO find a way to implement these two
// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", tag = "perm")]
// pub enum ExtraPermission {
//
//     Region {
//         region_id: i32,
//         permissions: Vec<Permission>,
//     },
//     Operator {
//         operator_id: i32,
//         permissions: Vec<Permission>,
//     },
// }

impl Permissions {
    pub(crate) fn merge(&mut self, perms: Self) {
        if let Some(regions) = perms.regions {
            if let Some(ref mut r) = self.regions {
                r.merge(&regions);
            } else {
                self.regions = Some(regions);
            }
        }
        if let Some(operators) = perms.operators {
            if let Some(ref mut o) = self.operators {
                o.merge(&operators);
            } else {
                self.operators = Some(operators);
            }
        }
        if let Some(routes) = perms.routes {
            if let Some(ref mut r) = self.routes {
                r.merge(&routes);
            } else {
                self.routes = Some(routes);
            }
        }
        if let Some(stops) = perms.stops {
            if let Some(ref mut s) = self.stops {
                s.merge(&stops);
            } else {
                self.stops = Some(stops);
            }
        }
        if let Some(osm_stops) = perms.osm_stops {
            if let Some(ref mut o) = self.osm_stops {
                o.merge(&osm_stops);
            } else {
                self.osm_stops = Some(osm_stops);
            }
        }
        if let Some(stop_pics) = perms.stop_pics {
            if let Some(ref mut s) = self.stop_pics {
                s.merge(&stop_pics);
            } else {
                self.stop_pics = Some(stop_pics);
            }
        }
        if let Some(news) = perms.news {
            if let Some(ref mut n) = self.news {
                n.merge(&news);
            } else {
                self.news = Some(news);
            }
        }
        if let Some(external_news) = perms.external_news {
            if let Some(ref mut e) = self.external_news {
                e.merge(&external_news);
            } else {
                self.external_news = Some(external_news);
            }
        }
        if let Some(admin) = perms.admin {
            if let Some(ref mut a) = self.admin {
                a.merge(&admin);
            } else {
                self.admin = Some(admin);
            }
        }
        if let Some(misc) = perms.misc {
            if let Some(ref mut m) = self.misc {
                m.merge(&misc);
            } else {
                self.misc = Some(misc);
            }
        }
    }

    pub(crate) fn everything() -> Self {
        Self {
            regions: Some(subperm::Regions::everything()),
            operators: Some(subperm::Operators::everything()),
            routes: Some(subperm::Routes::everything()),
            stops: Some(subperm::Stops::everything()),
            osm_stops: Some(subperm::OsmStops::everything()),
            stop_pics: Some(subperm::StopPics::everything()),
            news: Some(subperm::News::everything()),
            external_news: Some(subperm::ExternalNews::everything()),
            admin: Some(subperm::Admin::everything()),
            misc: Some(subperm::Misc::everything()),
        }
    }

    pub(crate) fn new_user_default() -> Self {
        Self {
            stops: Some(subperm::Stops {
                create: false,
                modify_pos: false,
                modify_attrs: false,
                modify_map_features: false,
                delete: false,
                contrib_modify_attrs: true,
            }),
            stop_pics: Some(subperm::StopPics {
                upload: false,
                view_untagged: false,
                view_sensitive: false,
                modify_own: false,
                modify_others: false,
                delete: false,
                contrib_upload: true,
                contrib_modify: true,
            }),
            ..Self::default()
        }
    }
}

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

pub struct ValidateRouteGtfs;

impl ClaimPermission for ValidateRouteGtfs {
    fn is_valid(permissions: &Permissions) -> bool {
        permissions.routes.as_ref().is_some_and(|p| p.validate_gtfs)
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
        permissions.misc.as_ref().is_some_and(|p| p.handle_contrib)
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
