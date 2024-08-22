/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2024  Cláudio Pereira

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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Operator {
    pub id: i32,
    pub name: String,
    pub tag: String,
    pub logo_sha1: Option<String>,
}

pub(crate) mod responses {
    use chrono::{DateTime, Local};
    use serde::Serialize;
    use sqlx::types::JsonValue;

    use commons::models::calendar::Calendar;
    use commons::models::content::RichContent;
    use commons::models::operators;

    use crate::geo::models::responses::SimpleRegion;
    use crate::routes::models::responses::SimpleRoute;
    use crate::stops::models::responses::SimpleStop;

    #[derive(Debug, Clone, Serialize)]
    pub struct SimpleOperator {
        pub id: i32,
        pub name: String,
        pub tag: String,
    }

    #[derive(Serialize)]
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

    #[derive(Serialize)]
    pub struct OperatorWithRegions {
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
        pub regions: Vec<i32>,
    }

    #[derive(Serialize)]
    pub struct OperatorStopRel {
        pub id: i32,
        pub official_name: Option<String>,
        pub stop_ref: Option<String>,
        pub source: String,
        pub lat: f64,
        pub lon: f64,
    }

    #[derive(Debug, Serialize)]
    pub struct OperatorRouteType {
        pub id: i32,
        pub name: Option<String>,
        pub zapping_cost: i32,
        pub board_cost: i32,
        pub multi_trip: bool,
        pub badge_text_color: String,
        pub badge_bg_color: String,
    }

    #[derive(Debug, Serialize)]
    pub struct OperatorCalendar {
        pub id: i32,
        pub name: String,
        pub calendar: Calendar,
        pub operator_id: i32,
    }

    #[derive(Debug, Serialize)]
    pub struct OperatorNewsItem {
        pub id: i32,
        pub summary: String,
        pub content: String,
        pub datetime: DateTime<Local>,
        pub geojson: Option<JsonValue>,
        pub visible: bool,
    }

    #[derive(Debug, Serialize)]
    pub struct Issue {
        pub id: i32,
        pub title: String,
        pub message: String,
        pub category: operators::IssueCategory,
        pub impact: i32,
        pub creation: DateTime<Local>,
        pub geojson: Option<JsonValue>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub state: operators::IssueState,
        pub state_justification: Option<String>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }

    #[derive(Debug, Serialize)]
    pub struct FullIssue {
        pub id: i32,
        pub title: String,
        pub category: operators::IssueCategory,
        pub impact: i32,
        pub creation: DateTime<Local>,
        pub content: RichContent,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub state: operators::IssueState,
        pub state_justification: Option<String>,
        pub operators: Vec<SimpleOperator>,
        pub routes: Vec<SimpleRoute>,
        pub stops: Vec<SimpleStop>,
        pub regions: Vec<SimpleRegion>,
    }

    #[derive(Debug, Serialize)]
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
        pub region_ids: Vec<i32>,
    }
}

pub(crate) mod requests {
    use chrono::{DateTime, Local};
    use serde::Deserialize;
    use sqlx::types::JsonValue;

    use commons::models::calendar::Calendar;
    use commons::models::content::RichContent;
    use commons::models::history;
    use commons::models::operators;

    use crate::utils::canonicalize_optional_string;
    use crate::Error;

    #[derive(Debug, Deserialize)]
    pub struct ChangeOperator {
        pub name: String,
        pub tag: String,
        pub description: Option<String>,
        pub is_complete: bool,
        pub website_url: Option<String>,
        pub forum_url: Option<String>,
        pub library_url: Option<String>,
        pub contact_uris: Vec<String>,
    }

    impl ChangeOperator {
        pub fn tidy(&mut self) {
            canonicalize_optional_string(&mut self.description);
            canonicalize_optional_string(&mut self.website_url);
            canonicalize_optional_string(&mut self.forum_url);
            canonicalize_optional_string(&mut self.library_url);

            self.contact_uris.retain(|uri| !uri.is_empty());
        }
    }

    fn default_stop_operator_source() -> String {
        "unknown".to_string()
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeOperatorStop {
        pub official_name: Option<String>,
        pub stop_ref: Option<String>,
        #[serde(default = "default_stop_operator_source")]
        pub source: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeOperatorRouteType {
        pub name: Option<String>,
        pub zapping_cost: i32,
        pub board_cost: i32,
        pub multi_trip: bool,
        pub badge_text_color: String,
        pub badge_bg_color: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct NewOperatorCalendar {
        pub name: String,
        pub calendar: Calendar,
    }

    #[derive(Debug, Deserialize)]
    pub struct NewIssue {
        pub title: String,
        pub category: operators::IssueCategory,
        pub impact: i32,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub content: RichContent,
        pub region_ids: Vec<i32>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }

    impl NewIssue {
        pub(crate) fn validate(&self) -> Result<(), Error> {
            if self.title.trim().is_empty() {
                return Err(Error::ValidationFailure(
                    "Empty title".to_string(),
                ));
            }

            if self.lat.is_some() && self.lon.is_none() {
                return Err(Error::ValidationFailure(
                    "Latitude without longitude".to_string(),
                ));
            }

            if self.lat.is_none() && self.lon.is_some() {
                return Err(Error::ValidationFailure(
                    "Longitude without latitude".to_string(),
                ));
            }

            if let Some(lon) = self.lon {
                if !(-180.0..=180.0).contains(&lon) {
                    return Err(Error::ValidationFailure(
                        "Longitude out of bounds".to_string(),
                    ));
                }
            }

            if let Some(lat) = self.lat {
                if !(-90.0..=90.0).contains(&lat) {
                    return Err(Error::ValidationFailure(
                        "Latitude out of bounds".to_string(),
                    ));
                }
            }

            Ok(())
        }
    }

    impl From<NewIssue> for operators::Issue {
        fn from(value: NewIssue) -> Self {
            operators::Issue {
                id: -1,
                title: value.title,
                impact: value.impact,
                category: value.category,
                creation: Local::now(),
                state: operators::IssueState::Unanswered,
                state_justification: None,
                lat: value.lat,
                lon: value.lon,
                content: value.content,
                region_ids: value.region_ids,
                operator_ids: value.operator_ids,
                route_ids: value.route_ids,
                stop_ids: value.stop_ids,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeIssue {
        pub title: String,
        pub category: operators::IssueCategory,
        pub impact: i32,
        pub state: operators::IssueState,
        pub state_justification: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub content: RichContent,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
    }

    impl ChangeIssue {
        pub(crate) fn validate(&self) -> Result<(), Error> {
            if self.title.trim().is_empty() {
                return Err(Error::ValidationFailure(
                    "Empty title".to_string(),
                ));
            }

            if self.lat.is_some() && self.lon.is_none() {
                return Err(Error::ValidationFailure(
                    "Latitude without longitude".to_string(),
                ));
            }

            if self.lat.is_none() && self.lon.is_some() {
                return Err(Error::ValidationFailure(
                    "Longitude without latitude".to_string(),
                ));
            }

            if let Some(lon) = self.lon {
                if !(-180.0..=180.0).contains(&lon) {
                    return Err(Error::ValidationFailure(
                        "Longitude out of bounds".to_string(),
                    ));
                }
            }

            if let Some(lat) = self.lat {
                if !(-90.0..=90.0).contains(&lat) {
                    return Err(Error::ValidationFailure(
                        "Latitude out of bounds".to_string(),
                    ));
                }
            }

            Ok(())
        }
        pub fn derive_patch(
            &self,
            issue: &operators::Issue,
        ) -> history::operators::IssuePatch {
            let mut patch = history::operators::IssuePatch::default();

            if self.title != issue.title {
                patch.title = Some(self.title.clone());
            }
            if self.category != issue.category {
                patch.category = Some(self.category.into());
            }
            if self.impact != issue.impact {
                patch.impact = Some(self.impact);
            }
            if self.state != issue.state {
                patch.state = Some(self.state.into());
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
            if self.content != issue.content {
                patch.content = Some(self.content.clone());
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

            patch
        }
    }

    #[derive(Debug, Deserialize)]
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

    #[derive(Debug, Deserialize)]
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
