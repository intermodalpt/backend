use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use perms::Permissions;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_superuser: bool,
    pub works_for: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: i64,
    pub user_id: i32,
    pub datetime: DateTime<Utc>,
    pub action: AuditLogAction,
    pub addr: IpNetwork,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "action", content = "data")]
pub enum AuditLogAction {
    Login,
    RefreshToken,
    ManagementTokenIssued { session_id: Uuid },
    SessionRevoked { session_id: Uuid, was_logout: bool },
    Register { username: String, email: String },
    ChangePassword,
    ChangeAccountDetails {/* ??? */},
    // Actions with an admin override
    AdminChangeUsername { user_id: i32, new_username: String },
    AdminChangePassword { user_id: i32 },
    QueryManagementTokens,
}

impl AuditLogAction {
    pub fn action_type_name(&self) -> &'static str {
        match self {
            AuditLogAction::Login => "login",
            AuditLogAction::RefreshToken => "refreshToken",
            AuditLogAction::ManagementTokenIssued { .. } => {
                "managementTokenIssued"
            }
            AuditLogAction::SessionRevoked { .. } => "sessionRevoked",
            AuditLogAction::Register { .. } => "register",
            AuditLogAction::ChangePassword => "changePassword",
            AuditLogAction::ChangeAccountDetails { .. } => {
                "changeAccountDetails"
            }
            AuditLogAction::AdminChangeUsername { .. } => "adminChangeUsername",
            AuditLogAction::AdminChangePassword { .. } => "adminChangePassword",
            AuditLogAction::QueryManagementTokens => "queryManagementTokens",
        }
    }
}

mod perms {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Permissions {
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
        pub fn merge(&mut self, perms: Self) {
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

        pub fn everything() -> Self {
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

        pub fn new_user_default() -> Self {
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

    pub mod subperm {
        use serde::{Deserialize, Serialize};

        use super::is_false;

        #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
        pub struct Regions {
            #[serde(default, skip_serializing_if = "is_false")]
            pub create: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub modify: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub delete: bool,
        }

        impl Regions {
            pub fn merge(&mut self, perms: &Self) {
                self.create = self.create || perms.create;
                self.modify = self.modify || perms.modify;
                self.delete = self.delete || perms.delete;
            }

            pub fn everything() -> Self {
                Self {
                    create: true,
                    modify: true,
                    delete: true,
                }
            }
        }

        #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
        pub struct Operators {
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
            pub fn merge(&mut self, perms: &Self) {
                self.create = self.create || perms.create;
                self.modify_base = self.modify_base || perms.modify_base;
                self.modify_stops = self.modify_stops || perms.modify_stops;
                self.modify_calendars =
                    self.modify_calendars || perms.modify_calendars;
                self.delete = self.delete || perms.delete;
            }

            pub fn everything() -> Self {
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
        pub struct Routes {
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
            pub fn merge(&mut self, perms: &Self) {
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

            pub fn everything() -> Self {
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
        pub struct Stops {
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
            pub fn merge(&mut self, perms: &Self) {
                self.create = self.create || perms.create;
                self.modify_pos = self.modify_pos || perms.modify_pos;
                self.modify_attrs = self.modify_attrs || perms.modify_attrs;
                self.modify_map_features =
                    self.modify_map_features || perms.modify_map_features;
                self.delete = self.delete || perms.delete;
                self.contrib_modify_attrs =
                    self.contrib_modify_attrs || perms.contrib_modify_attrs;
            }

            pub fn everything() -> Self {
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
        pub struct OsmStops {
            #[serde(default, skip_serializing_if = "is_false")]
            pub update: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub delete: bool,
        }

        impl OsmStops {
            pub fn merge(&mut self, perms: &Self) {
                self.update = self.update || perms.update;
                self.delete = self.delete || perms.delete;
            }

            pub fn everything() -> Self {
                Self {
                    update: true,
                    delete: true,
                }
            }
        }

        #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
        pub struct StopPics {
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
            pub fn merge(&mut self, perms: &Self) {
                self.upload = self.upload || perms.upload;
                self.view_untagged = self.view_untagged || perms.view_untagged;
                self.view_sensitive =
                    self.view_sensitive || perms.view_sensitive;
                self.modify_own = self.modify_own || perms.modify_own;
                self.modify_others = self.modify_others || perms.modify_others;
                self.delete = self.delete || perms.delete;
                self.contrib_upload =
                    self.contrib_upload || perms.contrib_upload;
                self.contrib_modify =
                    self.contrib_modify || perms.contrib_modify;
            }

            pub fn everything() -> Self {
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
        pub struct News {
            #[serde(default, skip_serializing_if = "is_false")]
            pub create: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub modify: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub delete: bool,
        }

        impl News {
            pub fn merge(&mut self, perms: &Self) {
                self.create = self.create || perms.create;
                self.modify = self.modify || perms.modify;
                self.delete = self.delete || perms.delete;
            }

            pub fn everything() -> Self {
                Self {
                    create: true,
                    modify: true,
                    delete: true,
                }
            }
        }

        #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
        pub struct ExternalNews {
            #[serde(default, skip_serializing_if = "is_false")]
            pub read_private: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub modify: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub delete: bool,
        }

        impl ExternalNews {
            pub fn merge(&mut self, perms: &Self) {
                self.read_private = self.read_private || perms.read_private;
                self.modify = self.modify || perms.modify;
                self.delete = self.delete || perms.delete;
            }

            pub fn everything() -> Self {
                Self {
                    read_private: true,
                    modify: true,
                    delete: true,
                }
            }
        }

        #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
        pub struct Admin {
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
            pub fn merge(&mut self, perms: &Self) {
                self.read_audit_log =
                    self.read_audit_log || perms.read_audit_log;
                self.manage_user_sessions =
                    self.manage_user_sessions || perms.manage_user_sessions;
                self.change_passwords =
                    self.change_passwords || perms.change_passwords;
                self.change_permissions =
                    self.change_permissions || perms.change_permissions;
                self.suspend_users = self.suspend_users || perms.suspend_users;
            }

            pub fn everything() -> Self {
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
        pub struct Misc {
            #[serde(default, skip_serializing_if = "is_false")]
            pub modify_issues: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub contrib_evaluator: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub expensive_calls: bool,
            #[serde(default, skip_serializing_if = "is_false")]
            pub patch_gtfs: bool,
        }

        impl Misc {
            pub fn merge(&mut self, perms: &Self) {
                self.modify_issues = self.modify_issues || perms.modify_issues;
                self.contrib_evaluator =
                    self.contrib_evaluator || perms.contrib_evaluator;
                self.expensive_calls =
                    self.expensive_calls || perms.expensive_calls;
                self.patch_gtfs = self.patch_gtfs || perms.patch_gtfs;
            }

            pub fn everything() -> Self {
                Self {
                    modify_issues: true,
                    contrib_evaluator: true,
                    expensive_calls: true,
                    patch_gtfs: true,
                }
            }
        }
    }

    pub(crate) fn is_false(val: &bool) -> bool {
        !val
    }
}
