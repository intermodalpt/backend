use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
