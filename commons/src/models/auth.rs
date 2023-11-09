use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub is_admin: bool,
    pub is_trusted: bool,
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
pub enum AuditLogAction {
    Login,
    // This will require sessions
    // Logout,
    Register {
        username: String,
        email: String,
    },
    ChangePassword,
    ChangeAccountDetails {/* ??? */},
    // Actions with an admin override
    AdminChangeUsername {
        for_user_id: i32,
        new_username: String,
    },
    AdminChangePassword {
        for_user_id: i32,
    },
}
