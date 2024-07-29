use serde::{Deserialize, Serialize};

use crate::auth::Claims;

pub(crate) trait ClaimPermission {
    fn is_valid(permissions: &[Permission]) -> bool;
}

pub(crate) struct ScopedClaim<P: ClaimPermission>(
    pub(crate) Claims,
    pub(crate) std::marker::PhantomData<P>,
);

// TODO complete this later
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Permissions {
    pub is_admin: bool,
    pub is_trusted: bool,
}

fn is_false(val: &bool) -> bool {
    !val
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "perm")]
pub enum Permission {
    Operator {
        operator_id: i32,
        permissions: Vec<Permission>,
    },
    Region {
        region_id: i32,
        permissions: Vec<Permission>,
    },
    Operators {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        delete: bool,
    },
    Routes {
        #[serde(default, skip_serializing_if = "is_false")]
        create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        modify: bool,
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
        delete: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        contrib_modify_attrs: bool,
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
}

impl Permission {
    fn every_operators_perm() -> Permission {
        Permission::Operators {
            create: true,
            modify: true,
            delete: true,
        }
    }

    fn every_routes_perm() -> Permission {
        Permission::Routes {
            create: true,
            modify: true,
            delete: true,
        }
    }

    fn every_stops_perm() -> Permission {
        Permission::Stops {
            create: true,
            modify_pos: true,
            modify_attrs: true,
            delete: true,
            contrib_modify_attrs: false,
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

    fn every_admin_perm() -> Permission {
        Permission::Admin {
            read_audit_log: true,
            manage_user_sessions: true,
            change_passwords: true,
            change_permissions: true,
            suspend_users: true,
        }
    }

    pub(crate) fn admin_default() -> Vec<Permission> {
        vec![
            Self::every_operators_perm(),
            Self::every_routes_perm(),
            Self::every_stops_perm(),
            Self::every_stop_pics_perm(),
            Self::every_news_perm(),
            Self::every_admin_perm(),
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
                    modify: true,
                    delete: true,
                },
                Permission::Stops {
                    create: true,
                    modify_pos: true,
                    modify_attrs: true,
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

pub struct Admin;

impl ClaimPermission for Admin {
    fn is_valid(permissions: &[Permission]) -> bool {
        // TODO this is wrong
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Admin { .. }))
    }
}

pub struct Trusted;

impl ClaimPermission for Trusted {
    fn is_valid(permissions: &[Permission]) -> bool {
        // TODO get rid of me
        permissions
            .iter()
            .any(|p| matches!(p, Permission::Admin { .. }))
    }
}
