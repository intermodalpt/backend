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

pub(crate) mod requests {
    use serde::Deserialize;
    use std::collections::HashMap;

    use commons::models::{history, routes};

    #[derive(Debug, Deserialize)]
    pub struct ChangeRoute {
        pub code: Option<String>,
        pub name: String,
        pub main_subroute: Option<i32>,
        pub operator_id: i32,
        pub active: bool,
        pub type_id: i32,
        pub badge_text_color: Option<String>,
        pub badge_bg_color: Option<String>,
        // FIXME this default is temporary while we have change logs without it
        #[serde(default)]
        pub circular: bool,
    }

    impl From<routes::Route> for ChangeRoute {
        fn from(route: routes::Route) -> Self {
            Self {
                code: route.code,
                name: route.name,
                main_subroute: route.main_subroute,
                operator_id: route.operator_id,
                active: route.active,
                type_id: route.type_id,
                badge_text_color: route.badge_text_color,
                badge_bg_color: route.badge_bg_color,
                circular: route.circular,
            }
        }
    }

    impl ChangeRoute {
        pub fn derive_patch(
            &self,
            route: &routes::Route,
        ) -> history::routes::RoutePatch {
            let mut patch = history::routes::RoutePatch::default();
            if self.type_id != route.type_id {
                patch.type_id = Some(self.type_id);
            }
            if self.code != route.code {
                patch.code = Some(self.code.clone());
            }
            if self.name != route.name {
                patch.name = Some(self.name.clone());
            }
            if self.main_subroute != route.main_subroute {
                patch.main_subroute = Some(self.main_subroute);
            }
            if self.operator_id != route.operator_id {
                patch.operator_id = Some(self.operator_id);
            }
            if self.active != route.active {
                patch.active = Some(self.active);
            }
            if self.circular != route.circular {
                patch.circular = Some(self.circular);
            }
            patch
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeSubroute {
        pub group: i32,
        pub origin: String,
        pub destination: String,
        pub headsign: String,
        pub circular: bool,
        pub via: Vec<routes::SubrouteVia>,
        // TODO consider deprecating
        pub flag: String,
    }

    impl From<routes::Subroute> for ChangeSubroute {
        fn from(subroute: routes::Subroute) -> Self {
            Self {
                flag: subroute.flag,
                circular: subroute.circular,
                origin: subroute.origin,
                destination: subroute.destination,
                headsign: subroute.headsign,
                group: subroute.group,
                via: subroute.via,
            }
        }
    }

    impl ChangeSubroute {
        pub fn derive_patch(
            &self,
            subroute: &routes::Subroute,
        ) -> history::routes::SubroutePatch {
            let mut patch = history::routes::SubroutePatch::default();
            if self.group != subroute.group {
                patch.group = Some(self.group);
            }
            if self.flag != subroute.flag {
                patch.flag = Some(self.flag.clone());
            }
            if self.headsign != subroute.headsign {
                patch.headsign = Some(self.headsign.clone());
            }
            if self.origin != subroute.origin {
                patch.origin = Some(self.origin.clone());
            }
            if self.destination != subroute.destination {
                patch.destination = Some(self.destination.clone());
            }
            if self.via != subroute.via {
                patch.via = Some(history::vec_into_vec(self.via.clone()));
            }
            if self.circular != subroute.circular {
                patch.circular = Some(self.circular);
            }

            patch
        }
    }

    #[derive(Deserialize)]
    pub struct SubrouteStops {
        pub stops: Vec<i32>,
    }

    #[derive(Deserialize)]
    pub struct ChangeSubrouteStops {
        pub from: SubrouteStops,
        pub to: SubrouteStops,
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeDeparture {
        pub time: i16,
        pub calendar_id: i32,
    }

    impl ChangeDeparture {
        pub fn derive_patch(
            &self,
            departure: &routes::Departure,
        ) -> history::routes::DeparturePatch {
            let mut patch = history::routes::DeparturePatch::default();
            if self.time != departure.time {
                patch.time = Some(self.time);
            }
            if self.calendar_id != departure.calendar_id {
                patch.calendar_id = Some(self.calendar_id);
            }
            patch
        }
    }

    #[derive(Deserialize)]
    pub struct ValidationData {
        pub root: commons::models::gtfs::RouteValidation,
        pub subroutes: HashMap<i32, commons::models::gtfs::SubrouteValidation>,
    }
}

pub(crate) mod responses {
    use serde::Serialize;

    use commons::models::routes;

    /// This is an extension of `commons::Route` that includes parishes
    #[derive(Serialize)]
    pub struct Route {
        pub(crate) id: i32,
        pub(crate) type_id: i32,
        pub(crate) operator: i32,
        pub(crate) code: Option<String>,
        pub(crate) name: String,
        pub(crate) circular: bool,
        pub(crate) badge_text: String,
        pub(crate) badge_bg: String,
        pub(crate) active: bool,
        pub(crate) parishes: Vec<i32>,
        pub(crate) subroutes: Vec<Subroute>,
        // TODO drop
        pub(crate) main_subroute: Option<i32>,
    }

    #[derive(Debug, Serialize)]
    pub struct Subroute {
        pub(crate) id: i32,
        pub(crate) group: i32,
        pub(crate) headsign: String,
        pub(crate) origin: String,
        pub(crate) destination: String,
        pub(crate) via: sqlx::types::Json<Vec<routes::SubrouteVia>>,
        pub(crate) circular: bool,
        pub(crate) polyline: Option<String>,

        // TODO remove flag after transitioned
        pub(crate) flag: String,
    }

    /// Same as a `Route` + validation data
    #[derive(Serialize)]
    pub struct FullRoute {
        pub(crate) id: i32,
        pub(crate) type_id: i32,
        pub(crate) operator: i32,
        pub(crate) code: Option<String>,
        pub(crate) name: String,
        pub(crate) circular: bool,
        pub(crate) badge_text: String,
        pub(crate) badge_bg: String,
        pub(crate) active: bool,
        pub(crate) parishes: Vec<i32>,
        pub(crate) regions: Vec<i32>,
        pub(crate) subroutes: Vec<FullSubroute>,

        pub(crate) validation: sqlx::types::JsonValue,

        //  TODO drop
        pub(crate) main_subroute: Option<i32>,
    }

    /// This is an extension of Subroute that includes validation data
    // The order of these fields CANNOT BE CHANGED because of the COALESCE's in SQL
    #[derive(Debug, Serialize)]
    pub struct FullSubroute {
        pub(crate) id: i32,
        pub(crate) group: i32,

        // TODO remove flag after transitioned
        pub(crate) flag: String,

        pub(crate) headsign: String,
        pub(crate) origin: String,
        pub(crate) destination: String,
        pub(crate) via: sqlx::types::Json<Vec<routes::SubrouteVia>>,
        pub(crate) circular: bool,
        pub(crate) polyline: Option<String>,
        pub(crate) validation: Option<sqlx::types::JsonValue>,
    }

    #[derive(Serialize)]
    pub struct Departure {
        pub id: i32,
        pub subroute: i32,
        // Departure time in minutes starting at midnight
        pub time: i16,
        pub calendar_id: i32,
    }

    #[derive(Serialize)]
    pub struct DateDeparture {
        pub subroute: i32,
        pub time: i16,
    }

    #[derive(Serialize)]
    pub struct SubrouteStops {
        pub subroute: i32,
        pub stops: Vec<i32>,
    }

    // Manual implementations of sqlx::Type due to
    // https://github.com/rust-lang/rust/issues/82219
    impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for Subroute {
        fn decode(
            value: sqlx::postgres::PgValueRef<'r>,
        ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>>
        {
            let mut decoder =
                sqlx::postgres::types::PgRecordDecoder::new(value)?;
            let id = decoder.try_decode::<i32>()?;
            let group = decoder.try_decode::<i32>()?;
            let flag = decoder.try_decode::<String>()?;
            let headsign = decoder.try_decode::<String>()?;
            let origin = decoder.try_decode::<String>()?;
            let destination = decoder.try_decode::<String>()?;
            let via = decoder
                .try_decode::<sqlx::types::Json<Vec<routes::SubrouteVia>>>()?;
            let circular = decoder.try_decode::<bool>()?;
            let polyline = decoder.try_decode::<Option<String>>()?;
            Ok(Subroute {
                id,
                group,
                flag,
                headsign,
                origin,
                destination,
                via,
                circular,
                polyline,
            })
        }
    }

    impl sqlx::Type<sqlx::Postgres> for Subroute {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("Subroute")
        }
    }

    impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for FullSubroute {
        fn decode(
            value: sqlx::postgres::PgValueRef<'r>,
        ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>>
        {
            let mut decoder =
                sqlx::postgres::types::PgRecordDecoder::new(value)?;
            let id = decoder.try_decode::<i32>()?;
            let group = decoder.try_decode::<i32>()?;
            let flag = decoder.try_decode::<String>()?;
            let headsign = decoder.try_decode::<String>()?;
            let origin = decoder.try_decode::<String>()?;
            let destination = decoder.try_decode::<String>()?;
            let via = decoder
                .try_decode::<sqlx::types::Json<Vec<routes::SubrouteVia>>>()?;
            let circular = decoder.try_decode::<bool>()?;
            let polyline = decoder.try_decode::<Option<String>>()?;
            let validation =
                decoder.try_decode::<Option<sqlx::types::JsonValue>>()?;
            Ok(FullSubroute {
                id,
                group,
                flag,
                headsign,
                origin,
                destination,
                via,
                circular,
                polyline,
                validation,
            })
        }
    }

    impl sqlx::Type<sqlx::Postgres> for FullSubroute {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("FullSubroute")
        }
    }
}
