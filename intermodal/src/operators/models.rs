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

use serde::Serialize;
use serde_repr::Serialize_repr;
use utoipa::Component;

use crate::calendar::Calendar;

#[derive(Debug, Serialize, Component)]
pub struct Operator {
    pub id: i32,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Component)]
pub struct OperatorCalendar {
    pub id: i32,
    pub operator: i32,
    pub name: String,
    pub calendar: Calendar,
}

#[derive(Debug, Serialize, Component)]
pub struct OperatorVehicle {
    pub id: i32,
    pub name: String,
    pub sevice_year: u16,
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

#[derive(Debug, Serialize, Component)]
pub struct Reseller {
    pub id: i32,
    pub name: String,
    pub sevice_year: u16,
    pub quantity: u16,
    pub bench_seats: u16,
    pub foot_seats: u16,
    pub has_ac: bool,
    pub has_wifi: bool,
    // TODO complete
}

pub struct Anormaly {
    pub id: i32,
    pub summary: String,
    pub message: String,
    pub from_datetime: Option<DateTime<Local>>,
    pub to_datetime: Option<DateTime<Local>>,
    pub geojson: Option<String>,
    pub mark_resolved: bool,
}

pub struct AnormalyOperator {
    pub anormaly_id: i32,
    pub operator_id: i32,
}

pub struct AnormalyRoute {
    pub anormaly_id: i32,
    pub route_id: i32,
}

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

#[derive(Debug, Serialize, Component)]
pub struct NewsItem {
    pub id: i32,
    pub operator_id: Option<i32>,
    pub summary: String,
    pub content: String,
    pub datetime: DateTime<Local>,
    pub geojson: Option<String>,
    pub visible: bool,
}

#[repr(u8)]
#[derive(Serialize_repr)]
pub enum TicketReason {
    Suggestion = 0,
    Complaint = 1,
    Other = 10,
}

pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub datetime: DateTime<Local>,
    pub operator: Option<i32>,
    pub user: Option<i32>,
    pub status: TicketStatus,
}

pub struct TicketComment {
    pub id: i32,
    pub ticket_id: i32,
    pub message: String,
    pub datetime: DateTime<Local>,
    pub user_id: i32,
}

#[repr(u8)]
pub enum TicketStatus {
    New = 0,
    Unanswered = 1,
    Answered = 2,
}

pub(crate) mod responses {
    use chrono::{DateTime, Local};
    use serde::Serialize;
    use utoipa::Component;

    use super::Calendar;

    #[derive(Debug, Serialize, Component)]
    pub struct OperatorCalendar {
        pub id: i32,
        pub name: String,
        pub calendar: Calendar,
        pub operator_id: i32,
    }

    #[derive(Debug, Serialize, Component)]
    pub struct OperatorNewsItem {
        pub id: i32,
        pub summary: String,
        pub content: String,
        pub datetime: DateTime<Local>,
        pub geojson: Option<String>,
        pub visible: bool,
    }
}

pub(crate) mod requests {
    use serde::{Deserialize};
    use utoipa::Component;

    use super::Calendar;

    #[derive(Debug, Deserialize, Component)]
    pub struct NewOperatorCalendar {
        pub name: String,
        pub calendar: Calendar,
    }
}