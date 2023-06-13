/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022  Cláudio Pereira

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use chrono::{DateTime, Local};
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use sqlx::types::JsonValue;
use utoipa::ToSchema;

use crate::calendar::Calendar;

#[derive(Debug, Serialize, ToSchema)]
pub struct Operator {
    pub id: i32,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OperatorCalendar {
    pub id: i32,
    pub operator: i32,
    pub name: String,
    pub calendar: Calendar,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OperatorVehicle {
    pub id: i32,
    pub name: String,
    pub service_year: u16,
    pub quantity: u16,
    pub bench_seats: u16,
    pub foot_seats: u16,
    pub has_ac: bool,
    pub has_usb_outlets: bool,
    pub has_wifi: bool,
    pub has_bicycle_rack: bool,
    pub has_wheelchair_ramp: bool,
    // TODO complete
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Reseller {
    pub id: i32,
    pub name: String,
    pub service_year: u16,
    pub quantity: u16,
    pub bench_seats: u16,
    pub foot_seats: u16,
    pub has_ac: bool,
    pub has_wifi: bool,
    // TODO complete
}

// Abnormalities are temporary changes to the network
// such as temporary detours
pub struct Abnormally {
    pub id: i32,
    pub summary: String,
    pub message: String,
    pub creation: DateTime<Local>,
    pub from_datetime: Option<DateTime<Local>>,
    pub to_datetime: Option<DateTime<Local>>,
    pub geojson: Option<JsonValue>,
    pub mark_resolved: bool,
}

pub struct AbnormallyOperator {
    pub abnormally_id: i32,
    pub operator_id: i32,
}

pub struct AbnormallyRoute {
    pub abnormally_id: i32,
    pub route_id: i32,
}

pub struct AbnormallyStop {
    pub abnormally_id: i32,
    pub stop_id: i32,
}

// Issues are problems raised by the community in a
// moderated fashion, that ensures issue quality and deduplication.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub creation: DateTime<Local>,
    pub category: IssueCategory,
    pub impact: i32,
    pub geojson: Option<JsonValue>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub state: IssueState,
    pub state_justification: Option<String>,
    pub operator_ids: Vec<i32>,
    pub route_ids: Vec<i32>,
    pub stop_ids: Vec<i32>,
    pub pic_ids: Vec<i32>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum IssueCategory {
    StopIssue,
    StopImprovement,
    RouteImprovement,
    ScheduleIssue,
    ScheduleImprovement,
    ServiceImprovement,
    GTFS,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
    Unanswered,
    Wontfix,
    FixInProgress,
    FixDone,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NewsItemType {
    New,
    Campaign,
    Information,
    Detour,
    Change,
}

impl fmt::Display for NewsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NewsItemType::New => "Novidade",
            NewsItemType::Campaign => "Campanha",
            NewsItemType::Information => "Informação",
            NewsItemType::Detour => "Desvio",
            NewsItemType::Change => "Alteração",
        })
    }
}

// Tickets are user submitted questions that might not meet quality standards
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub creation: DateTime<Local>,
    pub operator: Option<i32>,
    pub user: Option<i32>,
    pub public: bool,
    pub status: TicketStatus,
}

#[repr(u8)]
#[derive(Serialize_repr)]
pub enum TicketReason {
    Suggestion = 0,
    Complaint = 1,
    Other = 10,
}

#[repr(u8)]
pub enum TicketStatus {
    New = 0,
    Unanswered = 1,
    Answered = 2,
}

pub struct TicketComment {
    pub id: i32,
    pub ticket_id: i32,
    pub message: String,
    pub datetime: DateTime<Local>,
    pub user_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NewsItem {
    pub id: i32,
    pub operator_id: Option<i32>,
    pub summary: String,
    pub content: String,
    pub datetime: DateTime<Local>,
    pub geojson: Option<JsonValue>,
    pub visible: bool,
}

pub(crate) mod responses {
    use chrono::{DateTime, Local};
    use serde::Serialize;
    use sqlx::types::JsonValue;
    use utoipa::ToSchema;

    use super::{Calendar, IssueCategory, IssueState};

    #[derive(Debug, Serialize, ToSchema)]
    pub struct OperatorCalendar {
        pub id: i32,
        pub name: String,
        pub calendar: Calendar,
        pub operator_id: i32,
    }

    #[derive(Debug, Serialize, ToSchema)]
    pub struct OperatorNewsItem {
        pub id: i32,
        pub summary: String,
        pub content: String,
        pub datetime: DateTime<Local>,
        pub geojson: Option<JsonValue>,
        pub visible: bool,
    }

    #[derive(Debug, Serialize, ToSchema)]
    pub struct Issue {
        pub id: i32,
        pub title: String,
        pub message: String,
        pub category: IssueCategory,
        pub impact: i32,
        pub creation: DateTime<Local>,
        pub geojson: Option<JsonValue>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub state: IssueState,
        pub state_justification: Option<String>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }

    #[derive(Debug, Serialize, ToSchema)]
    pub struct Abnormally {
        pub id: i32,
        pub summary: String,
        pub message: String,
        pub from_datetime: Option<DateTime<Local>>,
        pub to_datetime: Option<DateTime<Local>>,
        pub geojson: Option<JsonValue>,
        pub mark_resolved: bool,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }
}

pub(crate) mod requests {
    use crate::contrib::models::IssuePatch;
    use crate::operators::models::Issue;
    use chrono::{DateTime, Local};
    use serde::Deserialize;
    use sqlx::types::JsonValue;
    use utoipa::ToSchema;

    use super::{Calendar, IssueCategory, IssueState};

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct NewOperatorCalendar {
        pub name: String,
        pub calendar: Calendar,
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct NewIssue {
        pub title: String,
        pub message: String,
        pub category: IssueCategory,
        pub impact: i32,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub geojson: Option<JsonValue>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
        pub pic_ids: Vec<i32>,
    }

    impl From<NewIssue> for Issue {
        fn from(value: NewIssue) -> Self {
            Issue {
                id: -1,
                title: value.title,
                message: value.message,
                impact: value.impact,
                category: value.category,
                creation: Local::now(),
                state: IssueState::Unanswered,
                state_justification: None,
                lat: value.lat,
                lon: value.lon,
                geojson: value.geojson,
                operator_ids: value.operator_ids,
                route_ids: value.route_ids,
                stop_ids: value.stop_ids,
                pic_ids: value.pic_ids,
            }
        }
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct ChangeIssue {
        pub title: String,
        pub message: String,
        pub geojson: Option<JsonValue>,
        pub category: IssueCategory,
        pub state: IssueState,
        pub state_justification: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
        pub pic_ids: Vec<i32>,
    }

    impl ChangeIssue {
        pub fn derive_patch(&self, issue: &Issue) -> IssuePatch {
            let mut patch = IssuePatch::default();

            if self.title != issue.title {
                patch.title = Some(self.title.clone());
            }
            if self.message != issue.message {
                patch.message = Some(self.message.clone());
            }
            if self.geojson != issue.geojson {
                patch.geojson = Some(self.geojson.clone());
            }
            if self.category != issue.category {
                patch.category = Some(self.category);
            }
            if self.state != issue.state {
                patch.state = Some(self.state);
            }
            if self.state_justification != issue.state_justification {
                patch.state_justification =
                    Some(self.state_justification.clone());
            }
            if self.lat != issue.lat {
                patch.lat = Some(self.lat);
            }
            if self.lon != issue.lon {
                patch.lon = Some(self.lon);
            }
            if self.operator_ids != issue.operator_ids {
                patch.operator_ids = Some(self.operator_ids.clone());
            }
            if self.route_ids != issue.route_ids {
                patch.route_ids = Some(self.route_ids.clone());
            }
            if self.stop_ids != issue.stop_ids {
                patch.stop_ids = Some(self.stop_ids.clone());
            }
            if self.pic_ids != issue.pic_ids {
                patch.pic_ids = Some(self.pic_ids.clone());
            }

            patch
        }
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct NewAbnormally {
        pub summary: String,
        pub message: String,
        pub from_datetime: Option<DateTime<Local>>,
        pub to_datetime: Option<DateTime<Local>>,
        pub geojson: Option<JsonValue>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct ChangeAbnormally {
        pub summary: String,
        pub message: String,
        pub from_datetime: Option<DateTime<Local>>,
        pub to_datetime: Option<DateTime<Local>>,
        pub geojson: Option<JsonValue>,
        pub mark_resolved: bool,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }
}
