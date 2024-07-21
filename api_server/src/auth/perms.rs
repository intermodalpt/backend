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
        create: bool,
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
        contrib_create: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        contrib_modify: bool,
    },
    Admin,
}

impl Permission {
    pub(crate) fn admin_default() -> Vec<Permission> {
        vec![Permission::Admin]
    }

    pub(crate) fn trusted_default() -> Vec<Permission> {
        vec![
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
                create: true,
                view_untagged: true,
                view_sensitive: true,
                modify_own: true,
                modify_others: true,
                delete: true,
                contrib_create: false,
                contrib_modify: false,
            },
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
                    create: true,
                    view_untagged: true,
                    view_sensitive: true,
                    modify_own: true,
                    modify_others: true,
                    delete: true,
                    contrib_create: false,
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
                create: false,
                view_untagged: false,
                view_sensitive: false,
                modify_own: false,
                modify_others: false,
                delete: false,
                contrib_create: true,
                contrib_modify: true,
            },
        ]
    }
}

pub struct Admin;

impl ClaimPermission for Admin {
    fn is_valid(permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| p == &Permission::Admin)
    }
}

pub struct Trusted;

impl ClaimPermission for Trusted {
    fn is_valid(permissions: &[Permission]) -> bool {
        // TODO get rid of me
        permissions.iter().any(|p| p == &Permission::Admin)
    }
}
