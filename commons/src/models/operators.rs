/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023 - 2024 Cláudio Pereira

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

use std::fmt;

use super::calendar::Calendar;
use crate::models::content::RichContent;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize)]
pub struct Operator {
    pub id: i32,
    pub name: String,
    pub tag: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub is_complete: bool,
    pub website_url: Option<String>,
    pub forum_url: Option<String>,
    pub library_url: Option<String>,
    pub contact_uris: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct OperatorCalendar {
    pub id: i32,
    pub operator: i32,
    pub name: String,
    pub calendar: Calendar,
}

#[derive(Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
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

// Abnormalities are temporary changes to the network
// such as temporary detours
pub struct Abnormality {
    pub id: i32,
    pub summary: String,
    pub creation: DateTime<Local>,
    pub from_datetime: Option<DateTime<Local>>,
    pub to_datetime: Option<DateTime<Local>>,
    pub content: RichContent,
    pub mark_resolved: bool,
    pub region_ids: Vec<i32>,
    pub operator_ids: Vec<i32>,
    pub route_ids: Vec<i32>,
    pub stop_ids: Vec<i32>,
}

// Issues are problems raised by the community in a
// moderated fashion, that ensures issue quality and deduplication.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub creation: DateTime<Local>,
    pub category: IssueCategory,
    // TODO Drop default
    #[serde(default)]
    pub impact: i32,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub content: RichContent,
    pub state: IssueState,
    pub state_justification: Option<String>,
    pub region_ids: Vec<i32>,
    pub operator_ids: Vec<i32>,
    pub route_ids: Vec<i32>,
    pub stop_ids: Vec<i32>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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

// TODO use this again in the operator-stop links
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MatchSource {
    Unknown,
    Tml,
    Manual,
    Osm,
    Flags,
    H1,
}

impl fmt::Display for MatchSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchSource::Unknown => write!(f, "unknown"),
            MatchSource::Tml => write!(f, "tml"),
            MatchSource::Manual => write!(f, "manual"),
            MatchSource::Osm => write!(f, "osm"),
            MatchSource::Flags => write!(f, "flags"),
            MatchSource::H1 => write!(f, "h1"),
        }
    }
}
