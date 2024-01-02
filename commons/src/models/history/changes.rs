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

use super::{operators, pics, routes, stops};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contribution {
    pub id: i64,
    pub author_id: i32,
    pub change: Change,
    pub submission_date: DateTime<Local>,
    pub accepted: Option<bool>,
    pub evaluator_id: Option<i32>,
    pub evaluation_date: Option<DateTime<Local>>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Changeset {
    pub id: i64,
    pub author_id: i32,
    pub changes: Vec<Change>,
    pub datetime: DateTime<Local>,
    pub contribution_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Change {
    // Hint: Do not to change these names without a corresponding migration
    // as they'll be stored as strings in the database
    StopCreation {
        data: stops::Stop,
    },
    StopUpdate {
        original: stops::Stop,
        patch: stops::StopPatch,
    },
    StopDeletion {
        data: stops::Stop,
    },
    RouteCreation {
        data: routes::Route,
    },
    RouteUpdate {
        original: routes::Route,
        patch: routes::RoutePatch,
    },
    RouteDeletion {
        data: routes::Route,
    },
    SubrouteCreation {
        data: routes::Subroute,
    },
    SubrouteUpdate {
        original: routes::Subroute,
        patch: routes::SubroutePatch,
    },
    SubrouteDeletion {
        #[serde(alias = "data")]
        subroute: routes::Subroute,
        // TODO drop the Option after history is rebuilt
        stops: Option<Vec<i32>>,
        // TODO drop the Option after history is rebuilt
        departures: Option<Vec<routes::Departure>>,
    },
    DepartureCreation {
        data: routes::Departure,
    },
    DepartureUpdate {
        original: routes::Departure,
        patch: routes::DeparturePatch,
    },
    DepartureDeletion {
        data: routes::Departure,
    },
    StopPicUpload {
        pic: pics::StopPic,
        stops: Vec<pics::StopAttrs>,
    },
    StopPicMetaUpdate {
        // TODO drop the Option when the mess of unlinked updates gets sorted
        pic_id: Option<i32>,
        original_meta: pics::StopPicDynMeta,
        original_stops: Vec<pics::StopAttrs>,
        meta_patch: pics::StopPicturePatch,
        stops: Vec<pics::StopAttrs>,
    },
    StopPicDeletion {
        pic: pics::StopPic,
        stops: Vec<pics::StopAttrs>,
    },
    IssueCreation {
        data: operators::Issue,
    },
    IssueUpdate {
        original: operators::Issue,
        patch: operators::IssuePatch,
    },
}
