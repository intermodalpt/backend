/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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

pub(crate) mod responses {
    use chrono::{DateTime, Local};
    use serde::Serialize;
    use sqlx::types::JsonValue;
    use utoipa::ToSchema;

    use commons::models::calendar::Calendar;
    use commons::models::operators;

    #[derive(Serialize, ToSchema)]
    pub struct OperatorStop {
        pub id: i32,
        pub official_name: Option<String>,
        pub stop_ref: Option<String>,
        pub source: String,
        // TODO Why the option?
        pub lat: Option<f64>,
        pub lon: Option<f64>,
    }

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
    use chrono::{DateTime, Local};
    use serde::Deserialize;
    use sqlx::types::JsonValue;
    use utoipa::ToSchema;

    use commons::models::calendar::Calendar;
    use commons::models::history;
    use commons::models::operators;

    fn default_stop_operator_source() -> String {
        "unknown".to_string()
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct ChangeOperatorStop {
        pub official_name: Option<String>,
        pub stop_ref: Option<String>,
        #[serde(default = "default_stop_operator_source")]
        pub source: String,
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct NewOperatorCalendar {
        pub name: String,
        pub calendar: Calendar,
    }

    #[derive(Debug, Deserialize, ToSchema)]
    pub struct NewIssue {
        pub title: String,
        pub message: String,
        pub category: operators::IssueCategory,
        pub impact: i32,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub geojson: Option<JsonValue>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
        pub pic_ids: Vec<i32>,
    }

    impl From<NewIssue> for operators::Issue {
        fn from(value: NewIssue) -> Self {
            operators::Issue {
                id: -1,
                title: value.title,
                message: value.message,
                impact: value.impact,
                category: value.category,
                creation: Local::now(),
                state: operators::IssueState::Unanswered,
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
        pub category: operators::IssueCategory,
        pub impact: i32,
        pub state: operators::IssueState,
        pub state_justification: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub geojson: Option<JsonValue>,
        pub operator_ids: Vec<i32>,
        pub route_ids: Vec<i32>,
        pub stop_ids: Vec<i32>,
        pub pic_ids: Vec<i32>,
    }

    impl ChangeIssue {
        pub fn derive_patch(
            &self,
            issue: &operators::Issue,
        ) -> history::IssuePatch {
            let mut patch = history::IssuePatch::default();

            if self.title != issue.title {
                patch.title = Some(self.title.clone());
            }
            if self.message != issue.message {
                patch.message = Some(self.message.clone());
            }
            if self.category != issue.category {
                patch.category = Some(self.category);
            }
            if self.impact != issue.impact {
                patch.impact = Some(self.impact);
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
            if self.geojson != issue.geojson {
                patch.geojson = Some(self.geojson.clone());
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
