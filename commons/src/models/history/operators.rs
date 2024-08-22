/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023  Cláudio Pereira

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
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use super::calendar::Calendar;
use crate::errors::Error;
use crate::models::content::RichContent;
use crate::models::operators as current;

#[derive(Debug, Serialize)]
pub struct Operator {
    pub id: i32,
    pub name: String,
    pub tag: String,
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

#[derive(Debug, Serialize)]
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
    pub content: RichContent,
    pub mark_resolved: bool,
}

impl From<current::Abnormally> for Abnormally {
    fn from(abnormally: current::Abnormally) -> Self {
        Self {
            id: abnormally.id,
            summary: abnormally.summary,
            message: abnormally.message,
            creation: abnormally.creation,
            from_datetime: abnormally.from_datetime,
            to_datetime: abnormally.to_datetime,
            content: abnormally.content,
            mark_resolved: abnormally.mark_resolved,
        }
    }
}

impl TryFrom<Abnormally> for current::Abnormally {
    type Error = Error;

    fn try_from(abnormally: Abnormally) -> Result<Self, Self::Error> {
        Ok(Self {
            id: abnormally.id,
            summary: abnormally.summary,
            message: abnormally.message,
            creation: abnormally.creation,
            from_datetime: abnormally.from_datetime,
            to_datetime: abnormally.to_datetime,
            content: abnormally.content,
            mark_resolved: abnormally.mark_resolved,
        })
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<current::Issue> for Issue {
    fn from(issue: current::Issue) -> Self {
        Self {
            id: issue.id,
            title: issue.title,
            creation: issue.creation,
            category: issue.category.into(),
            impact: issue.impact,
            lat: issue.lat,
            lon: issue.lon,
            content: issue.content,
            state: issue.state.into(),
            state_justification: issue.state_justification,
            region_ids: issue.region_ids,
            operator_ids: issue.operator_ids,
            route_ids: issue.route_ids,
            stop_ids: issue.stop_ids,
        }
    }
}

impl TryFrom<Issue> for current::Issue {
    type Error = Error;

    fn try_from(issue: Issue) -> Result<Self, Self::Error> {
        Ok(Self {
            id: issue.id,
            title: issue.title,
            creation: issue.creation,
            category: issue.category.try_into()?,
            impact: issue.impact,
            lat: issue.lat,
            lon: issue.lon,
            content: issue.content,
            state: issue.state.try_into()?,
            state_justification: issue.state_justification,
            region_ids: issue.region_ids,
            operator_ids: issue.operator_ids,
            route_ids: issue.route_ids,
            stop_ids: issue.stop_ids,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

impl From<current::IssueCategory> for IssueCategory {
    fn from(category: current::IssueCategory) -> Self {
        match category {
            current::IssueCategory::StopIssue => IssueCategory::StopIssue,
            current::IssueCategory::StopImprovement => {
                IssueCategory::StopImprovement
            }
            current::IssueCategory::RouteImprovement => {
                IssueCategory::RouteImprovement
            }
            current::IssueCategory::ScheduleIssue => {
                IssueCategory::ScheduleIssue
            }
            current::IssueCategory::ScheduleImprovement => {
                IssueCategory::ScheduleImprovement
            }
            current::IssueCategory::ServiceImprovement => {
                IssueCategory::ServiceImprovement
            }
            current::IssueCategory::GTFS => IssueCategory::GTFS,
        }
    }
}

impl TryFrom<IssueCategory> for current::IssueCategory {
    type Error = Error;

    fn try_from(category: IssueCategory) -> Result<Self, Self::Error> {
        match category {
            IssueCategory::StopIssue => Ok(current::IssueCategory::StopIssue),
            IssueCategory::StopImprovement => {
                Ok(current::IssueCategory::StopImprovement)
            }
            IssueCategory::RouteImprovement => {
                Ok(current::IssueCategory::RouteImprovement)
            }
            IssueCategory::ScheduleIssue => {
                Ok(current::IssueCategory::ScheduleIssue)
            }
            IssueCategory::ScheduleImprovement => {
                Ok(current::IssueCategory::ScheduleImprovement)
            }
            IssueCategory::ServiceImprovement => {
                Ok(current::IssueCategory::ServiceImprovement)
            }
            IssueCategory::GTFS => Ok(current::IssueCategory::GTFS),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
    Unanswered,
    Wontfix,
    FixInProgress,
    FixDone,
}

impl From<current::IssueState> for IssueState {
    fn from(state: current::IssueState) -> Self {
        match state {
            current::IssueState::Unanswered => IssueState::Unanswered,
            current::IssueState::Wontfix => IssueState::Wontfix,
            current::IssueState::FixInProgress => IssueState::FixInProgress,
            current::IssueState::FixDone => IssueState::FixDone,
        }
    }
}

impl TryFrom<IssueState> for current::IssueState {
    type Error = Error;

    fn try_from(state: IssueState) -> Result<Self, Self::Error> {
        match state {
            IssueState::Unanswered => Ok(current::IssueState::Unanswered),
            IssueState::Wontfix => Ok(current::IssueState::Wontfix),
            IssueState::FixInProgress => Ok(current::IssueState::FixInProgress),
            IssueState::FixDone => Ok(current::IssueState::FixDone),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NewsItemType {
    New,
    Campaign,
    Information,
    Detour,
    Change,
}

impl From<current::NewsItemType> for NewsItemType {
    fn from(item_type: current::NewsItemType) -> Self {
        match item_type {
            current::NewsItemType::New => NewsItemType::New,
            current::NewsItemType::Campaign => NewsItemType::Campaign,
            current::NewsItemType::Information => NewsItemType::Information,
            current::NewsItemType::Detour => NewsItemType::Detour,
            current::NewsItemType::Change => NewsItemType::Change,
        }
    }
}

impl From<NewsItemType> for current::NewsItemType {
    fn from(item_type: NewsItemType) -> Self {
        match item_type {
            NewsItemType::New => current::NewsItemType::New,
            NewsItemType::Campaign => current::NewsItemType::Campaign,
            NewsItemType::Information => current::NewsItemType::Information,
            NewsItemType::Detour => current::NewsItemType::Detour,
            NewsItemType::Change => current::NewsItemType::Change,
        }
    }
}

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

impl From<current::TicketReason> for TicketReason {
    fn from(reason: current::TicketReason) -> Self {
        match reason {
            current::TicketReason::Suggestion => TicketReason::Suggestion,
            current::TicketReason::Complaint => TicketReason::Complaint,
            current::TicketReason::Other => TicketReason::Other,
        }
    }
}

impl TryFrom<TicketReason> for current::TicketReason {
    type Error = Error;

    fn try_from(reason: TicketReason) -> Result<Self, Self::Error> {
        match reason {
            TicketReason::Suggestion => Ok(current::TicketReason::Suggestion),
            TicketReason::Complaint => Ok(current::TicketReason::Complaint),
            TicketReason::Other => Ok(current::TicketReason::Other),
        }
    }
}

#[repr(u8)]
pub enum TicketStatus {
    New = 0,
    Unanswered = 1,
    Answered = 2,
}

impl From<current::TicketStatus> for TicketStatus {
    fn from(status: current::TicketStatus) -> Self {
        match status {
            current::TicketStatus::New => TicketStatus::New,
            current::TicketStatus::Unanswered => TicketStatus::Unanswered,
            current::TicketStatus::Answered => TicketStatus::Answered,
        }
    }
}

impl TryFrom<TicketStatus> for current::TicketStatus {
    type Error = Error;

    fn try_from(status: TicketStatus) -> Result<Self, Self::Error> {
        match status {
            TicketStatus::New => Ok(current::TicketStatus::New),
            TicketStatus::Unanswered => Ok(current::TicketStatus::Unanswered),
            TicketStatus::Answered => Ok(current::TicketStatus::Answered),
        }
    }
}

pub struct TicketComment {
    pub id: i32,
    pub ticket_id: i32,
    pub message: String,
    pub datetime: DateTime<Local>,
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct NewsItem {
    pub id: i32,
    pub operator_id: Option<i32>,
    pub summary: String,
    pub content: String,
    pub datetime: DateTime<Local>,
    pub geojson: Option<serde_json::Value>,
    pub visible: bool,
}

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

impl From<MatchSource> for current::MatchSource {
    fn from(source: MatchSource) -> Self {
        match source {
            MatchSource::Unknown => current::MatchSource::Unknown,
            MatchSource::Tml => current::MatchSource::Tml,
            MatchSource::Manual => current::MatchSource::Manual,
            MatchSource::Osm => current::MatchSource::Osm,
            MatchSource::Flags => current::MatchSource::Flags,
            MatchSource::H1 => current::MatchSource::H1,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IssuePatch {
    pub title: Option<String>,
    pub message: Option<String>,
    pub creation: Option<DateTime<Local>>,
    pub category: Option<IssueCategory>,
    pub impact: Option<i32>,
    pub state: Option<IssueState>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub state_justification: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub lat: Option<Option<f64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub lon: Option<Option<f64>>,
    pub content: Option<RichContent>,
    pub operator_ids: Option<Vec<i32>>,
    pub route_ids: Option<Vec<i32>>,
    pub stop_ids: Option<Vec<i32>>,
    pub pic_ids: Option<Vec<i32>>,
}

impl IssuePatch {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.creation.is_none()
            && self.impact.is_none()
            && self.category.is_none()
            && self.state.is_none()
            && self.state_justification.is_none()
            && self.lat.is_none()
            && self.lon.is_none()
            && self.content.is_none()
            && self.operator_ids.is_none()
            && self.route_ids.is_none()
            && self.stop_ids.is_none()
            && self.pic_ids.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, issue: &mut current::Issue) -> Result<(), Error> {
        if let Some(title) = self.title {
            issue.title = title;
        }
        if let Some(creation) = self.creation {
            issue.creation = creation;
        }
        if let Some(category) = self.category {
            issue.category = category.try_into()?;
        }
        if let Some(impact) = self.impact {
            issue.impact = impact;
        }
        if let Some(state) = self.state {
            issue.state = state.try_into()?;
        }
        if let Some(state_justification) = self.state_justification {
            issue.state_justification = state_justification;
        }
        if let Some(lat) = self.lat {
            issue.lat = lat;
        }
        if let Some(lon) = self.lon {
            issue.lon = lon;
        }
        if let Some(content) = self.content {
            issue.content = content;
        }
        if let Some(operator_ids) = self.operator_ids {
            issue.operator_ids = operator_ids;
        }
        if let Some(route_ids) = self.route_ids {
            issue.route_ids = route_ids;
        }
        if let Some(stop_ids) = self.stop_ids {
            issue.stop_ids = stop_ids;
        }
        Ok(())
    }
}
