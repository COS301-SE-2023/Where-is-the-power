use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{api::ApiError, db::Entity};
use async_trait::async_trait;
use bson::doc;
use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDateTime, Timelike};
use log::warn;
use macros::Entity;
use mongodb::{Cursor, Database};
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    futures::{StreamExt, TryStreamExt},
    Rocket,
};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

// Rocket Persistent Data Structs
pub struct StageUpdater;

#[derive(Debug)]
pub struct LoadSheddingStage {
    pub stage: i32,
}

// GeoJson Struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GeometryType {
    Polygon,
    MultiPolygon,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Coordinates {
    Polygon(Vec<Vec<Vec<f64>>>),
    MultiPolygon(Vec<Vec<Vec<Vec<f64>>>>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoJson {
    pub name: String,
    pub map_layer_type: String,
    pub bounds: Vec<Vec<f64>>,
    pub center: Vec<f64>,
    pub zoom: u32,
    pub median_zoom: u32,
    pub count: u32,
    pub property_names: Vec<String>,
    pub r#type: String,
    pub features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub r#type: String,
    pub id: u32,
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(rename = "SP_CODE")]
    pub sp_code: f64,

    #[serde(rename = "SP_CODE_st")]
    pub sp_code_st: String,

    #[serde(rename = "SP_NAME")]
    pub sp_name: String,

    #[serde(rename = "MP_CODE")]
    pub mp_code: f64,

    #[serde(rename = "MP_CODE_st")]
    pub mp_code_st: String,

    #[serde(rename = "MP_NAME")]
    pub mp_name: String,

    #[serde(rename = "MN_MDB_C")]
    pub mn_mdb_c: String,

    #[serde(rename = "MN_CODE")]
    pub mn_code: f64,

    #[serde(rename = "MN_CODE_st")]
    pub mn_code_st: String,

    #[serde(rename = "MN_NAME")]
    pub mn_name: String,

    #[serde(rename = "DC_MDB_C")]
    pub dc_mdb_c: String,

    #[serde(rename = "DC_MN_C")]
    pub dc_mn_c: f64,

    #[serde(rename = "DC_MN_C_st")]
    pub dc_mn_c_st: String,

    #[serde(rename = "DC_NAME")]
    pub dc_name: String,

    #[serde(rename = "PR_MDB_C")]
    pub pr_mdb_c: String,

    #[serde(rename = "PR_CODE")]
    pub pr_code: f64,

    #[serde(rename = "PR_CODE_st")]
    pub pr_code_st: String,

    #[serde(rename = "PR_NAME")]
    pub pr_name: String,

    #[serde(rename = "ALBERS_ARE")]
    pub albers_are: f64,

    #[serde(rename = "Shape_Leng")]
    pub shape_leng: f64,

    #[serde(rename = "Shape_Area")]
    pub shape_area: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub r#type: GeometryType,
    pub coordinates: Coordinates,
}

// Loadshedding Data Structures
#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "groups"]
pub struct GroupEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub number: i32,
    pub suburbs: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "suburbs"]
pub struct SuburbEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub municipality: u32,
    pub name: String,
    pub geometry: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "timeschedule"]
pub struct TimeScheduleEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub start_hour: i32,
    pub start_minute: i32,
    pub stop_hour: i32,
    pub stop_minute: i32,
    pub stages: Vec<StageTimes>,
    pub municipality: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "municipality"]
pub struct MunicipalityEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub name: String,
    pub geometry: GeoJson,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StageTimes {
    pub stage: i32,
    pub groups: Vec<u32>,
}

// Requests
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapDataRequest {
    pub bottom_left: [f64; 2],
    pub top_right: [f64; 2],
    pub time: Option<i64>,
}
// Responses
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapDataDefaultResponse {
    pub map_polygons: Vec<GeoJson>,
    pub on: Vec<SuburbEntity>,
    pub off: Vec<SuburbEntity>,
}

impl std::ops::Add for MapDataDefaultResponse {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut merged = self;
        merged.map_polygons.extend(other.map_polygons);
        merged.on.extend(other.on);
        merged.off.extend(other.off);
        merged
    }
}

impl MunicipalityEntity {
    pub async fn get_regions_at_time(
        &self,
        stage: i32,
        time: Option<i64>,
        connection: &Database,
    ) -> Result<MapDataDefaultResponse, ApiError> {
        let mut suburbs_off = Vec::<SuburbEntity>::new();
        let suburbs_on: Vec<SuburbEntity>;

        // get search time
        let sast = FixedOffset::east_opt(2 * 3600).unwrap(); // SAST
        let time_to_search: DateTime<FixedOffset>;
        if let Some(time) = time {
            time_to_search =
                DateTime::from_utc(NaiveDateTime::from_timestamp_opt(time, 0).unwrap(), sast);
        } else {
            time_to_search = Local::now().with_timezone(&sast);
        }

        // schedule query: all that fit the search time
        let query = doc! {
            "$and": [
                {
                    "start_hour": {
                        "$lte": time_to_search.hour()
                    },
                    "start_minute": {
                        "$lte": time_to_search.minute()
                    }
                },
                {
                    "end_hour": {
                        "$gte": time_to_search.hour()
                    },
                    "end_minute": {
                        "$gte": time_to_search.minute()
                    }
                },
                {
                    "municipality": self.id.unwrap()
                }
            ]
        };
        let mut schedule_cursor: Cursor<TimeScheduleEntity> = match connection
            .collection("timeschedule")
            .find(query, mongodb::options::FindOptions::default())
            .await
        {
            Ok(cursor) => cursor,
            Err(err) => {
                log::error!("Database error occured when querying timeschedules: {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        // schedule query end

        // suburbs query: all suburbs
        let query = doc! {
            "municipality" : self.id
        };
        let suburbs_cursor: Cursor<SuburbEntity> = match connection
            .collection("suburbs")
            .find(query, mongodb::options::FindOptions::default())
            .await
        {
            Ok(cursor) => cursor,
            Err(err) => {
                log::error!("Database error occured when querying suburbs: {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        let suburbs: Vec<SuburbEntity> = match suburbs_cursor.try_collect().await {
            Ok(item) => item,
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        // end of suburbs query

        // collect suburbs into a map for quick lookup and moving
        let mut suburbs: HashMap<u32, SuburbEntity> = suburbs
            .into_iter()
            .map(|suburb| (suburb.id.unwrap(), suburb))
            .collect();

        // go through schedules
        while let Some(Ok(doc)) = schedule_cursor.next().await {
            // All the groups that could be affected by the current stage
            let times: Vec<StageTimes> = doc
                .stages
                .iter()
                .filter(|&times| times.stage <= stage)
                .cloned()
                .map(|stage_time| stage_time)
                .collect();
            // All the filtered groups affected on this day at this time
            let groups: Vec<u32> = times
                .iter()
                .map(|schedule| {
                    schedule
                        .groups
                        .get((time_to_search.day() - 1) as usize)
                        .unwrap()
                })
                .cloned()
                .collect();

            // groups query: find all affected groups
            let query = doc! {
                "_id" : {"$in": groups}
            };
            let mut groups_cursor: Cursor<GroupEntity> = match connection
                .collection("groups")
                .find(query, mongodb::options::FindOptions::default())
                .await
            {
                Ok(cursor) => cursor,
                Err(err) => {
                    log::error!("Database error occured when querying timeschedules: {err}");
                    return Err(ApiError::ServerError(
                        "Error occured on the server, sorry :<",
                    ));
                }
            };
            // groups query end

            // go through the relevant groups and place the affected suburbs into
            //  the suburbs_off array
            while let Some(Ok(group)) = groups_cursor.next().await {
                let removed: Vec<SuburbEntity> = group
                    .suburbs
                    .iter()
                    .filter_map(|key| suburbs.remove(key))
                    .collect();
                suburbs_off.extend(removed);
            }
        }
        // place all the remaining suburbs after checking into the on array
        suburbs_on = suburbs.drain().map(|(_, value)| value).collect();

        Ok(MapDataDefaultResponse {
            map_polygons: vec![self.geometry.clone()],
            on: suburbs_on,
            off: suburbs_off,
        })
    }
}

// Rocket State Loop Objects
impl LoadSheddingStage {
    pub async fn fetch_stage(&mut self) -> Result<i32, reqwest::Error> {
        let stage = reqwest::get("https://loadshedding.eskom.co.za/LoadShedding/GetStatus").await?;
        if stage.status().is_success() {
            match stage.text().await?.parse::<i32>() {
                Ok(num) => {
                    if num >= 1 {
                        self.stage = num - 1;
                    }
                }
                Err(_) => warn!("Eskom API did not return a integer when we queried it."),
            }
        } else {
            warn!("Connection to Eskom Dropped before any operations could take place");
        }
        Ok(self.stage)
    }
}

#[async_trait]
impl Fairing for StageUpdater {
    fn info(&self) -> Info {
        Info {
            name: "Stage Updater",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<rocket::Build>) -> fairing::Result {
        let stage_info = Arc::new(Mutex::new(LoadSheddingStage { stage: 0 }));

        let stage_info_ref = stage_info.clone();
        thread::spawn(move || {
            loop {
                let mut stage_info = stage_info_ref.lock().unwrap();
                let runtime = Runtime::new().unwrap();
                let stage = stage_info.fetch_stage();
                let _ = runtime.block_on(stage);
                // Perform any other necessary processing on stage info
                thread::sleep(Duration::from_secs(600)); // Sleep for 10 minutes
            }
        });

        fairing::Result::Ok(rocket.manage(stage_info))
    }
}
