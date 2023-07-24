use std::{collections::HashMap, sync::Arc, thread,};

use crate::{
    api::{ApiError, ApiResponse},
    db::Entity,
};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveDateTime, Timelike};
use log::warn;
use macros::Entity;
use mongodb::{options::FindOneOptions, options::FindOptions, Client, Cursor, Database};
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    futures::{future::try_join_all, StreamExt, TryStreamExt},
    post,
    serde::json::Json,
    State, Orbit, Rocket
};
use serde::{Deserialize, Serialize};
use tokio::{runtime::Runtime, sync::RwLock};
use utoipa::ToSchema;
pub struct StageUpdater;

// Rocket endpoints
#[utoipa::path(post, tag = "Map Data", path = "/api/fetchMapData", request_body = MapDataRequest)]
#[post("/fetchMapData", format = "application/json", data = "<request>")]
pub async fn fetch_map_data<'a>(
    db: &State<Option<Client>>,
    loadshedding_stage: &State<Option<Arc<RwLock<LoadSheddingStage>>>>,
    request: Json<MapDataRequest>,
) -> ApiResponse<'a, MapDataDefaultResponse> {
    let connection = &db.inner().as_ref().unwrap().database("production");
    let south_west: Vec<f64> = request.bottom_left.iter().cloned().map(|x| x).collect();
    let north_east: Vec<f64> = request.top_right.iter().cloned().map(|x| x).collect();
    let query = doc! {
        "geometry.bounds" : {
            "$geoWithin" : {
                "$box" : [south_west, north_east]
            }
        }
    };

    let options = FindOptions::default();
    let cursor: Cursor<MunicipalityEntity> = match connection
        .collection("municipality")
        .find(query, options)
        .await
    {
        Ok(cursor) => cursor,
        Err(err) => {
            log::error!("Database error occured when handling geo query: {err}");
            return ApiError::ServerError(
                "Database error occured when handling request. Check logs.",
            )
            .into();
        }
    };
    let stage = &loadshedding_stage
        .inner()
        .as_ref()
        .clone()
        .unwrap()
        .read()
        .await
        .stage;
    let municipalities: Vec<MunicipalityEntity> = match cursor.try_collect().await {
        Ok(item) => item,
        Err(err) => {
            log::error!("Unable to Collect suburbs from cursor {err}");
            return ApiError::ServerError("Error occured on the server, sorry :<").into();
        }
    };
    let future_data = municipalities.iter().map(|municipality| {
        municipality.get_regions_at_time(stage.to_owned(), request.time, connection)
    });
    let response = try_join_all(future_data).await;
    if let Ok(data) = response {
        return ApiResponse::Ok(data.into_iter().fold(
            MapDataDefaultResponse {
                map_polygons: vec![],
                on: vec![],
                off: vec![],
            },
            |acc, obj| acc + obj,
        ));
    } else {
        log::error!("Unable to fold MapDataResponse");
        return ApiError::ServerError("Error occured on the server, sorry :<").into();
    }
}


#[utoipa::path(post, tag = "Suburb Statistics", path = "/api/fetchSuburbStats", request_body = Stats)]
#[post("/fetchSuburbStats", format = "application/json", data = "<request>")]
pub async fn fetch_suburb_stats<'a>(
    db: &State<Option<Client>>,
    request: Json<SuburbStatsRequest>,
) -> ApiResponse<'a, SuburbStatsResponse> {
    if let Some(data) = request.suburb_id.as_ref() {
        let oid = ObjectId::parse_str(data);
        if let Err(_err) = oid {
            return ApiError::ServerError("Invalid Object ID").into()
        }
        let connection = db.as_ref().unwrap().database("production");
        let query = doc! {"_id" : oid.unwrap()};
        let suburb: SuburbEntity = match connection.collection("suburbs").find_one(query, None).await.unwrap() {
            Some(result) => result,
            None => {
                return ApiError::ServerError("Document not found").into()
            }
        };
        match suburb.get_stats(&connection).await {
            Ok(data) => return ApiResponse::Ok(data),
            Err(_) => {
                return ApiError::ServerError("server side error :<").into()
            }
        }
    }
    todo!()
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "stage_log"]
pub struct LoadSheddingStage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub stage: i32,
    pub time: i64,
    #[serde(skip_serializing, skip_deserializing)]
    db: Option<Client>,
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
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub number: i32,
    pub suburbs: Vec<ObjectId>,
    // consider a small refactor to add groups associated municipalities for better efficiency
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "suburbs"]
pub struct SuburbEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub municipality: ObjectId,
    pub name: String,
    pub geometry: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "timeschedule"]
pub struct TimeScheduleEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub start_hour: i32,
    pub start_minute: i32,
    pub stop_hour: i32,
    pub stop_minute: i32,
    pub stages: Vec<StageTimes>,
    pub municipality: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "municipality"]
pub struct MunicipalityEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub geometry: GeoJson,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StageTimes {
    pub stage: i32,
    pub groups: Vec<ObjectId>,
}

// Requests
#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json! {
    MapDataRequest {
        bottom_left: [-90.0, 90.0],
        top_right: [90.0, -90.0],
        time: None
    }
})]
pub struct MapDataRequest {
    pub bottom_left: [f64; 2],
    pub top_right: [f64; 2],
    pub time: Option<i64>,
}
// Responses
#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapDataDefaultResponse {
    pub map_polygons: Vec<GeoJson>,
    pub on: Vec<SuburbEntity>,
    pub off: Vec<SuburbEntity>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json! {
    SuburbStatsRequest {
        suburb_id : Some("64b522a848b2645f6f627c1e".to_string()),
        suburb_object : None
    }
})]
pub struct SuburbStatsRequest {
    pub suburb_id: Option<String>,
    pub suburb_object: Option<SuburbEntity>
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuburbStatsResponse {
    pub total_time: TotalTimeForWeek,
    pub per_day_times: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalTimeForWeek {
    pub on: i32,
    pub off: i32,
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

// entity implimentations:
fn get_date_time(time: Option<i64>) -> DateTime<FixedOffset> {
    // South African Standard Time Offset
    let sast = FixedOffset::east_opt(2 * 3600).unwrap();
    // get search time
    match time {
        Some(time) => DateTime::from_utc(NaiveDateTime::from_timestamp_opt(time, 0).unwrap(), sast),
        None => Local::now().with_timezone(&sast),
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
        let time_to_search: DateTime<FixedOffset> = get_date_time(time);

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
        let mut suburbs: HashMap<ObjectId, SuburbEntity> = suburbs
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
            let groups: Vec<ObjectId> = times
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

impl SuburbEntity {
    pub async fn get_stats(
        self,
        connection: &Database,
    ) -> Result<SuburbStatsResponse, ApiError> {
        // queries
        // get the relevant group
        let query = doc! {
            "suburbs" : {
                "$in" : [self.id.unwrap()]
            }
        };
        let group: GroupEntity = match connection
            .collection("groups")
            .find_one(query, None)
            .await
            .unwrap()
        {
            Some(group) => group,
            None => {
                warn!("Error, a suburb is not associated with a group: {:?}", self);
                return Err(ApiError::ServerError(
                    "Group cannot be identified for specified suburb",
                ));
            }
        };

        // get all the stage changes from the past week
        let time_now = Local::now();
        let one_week_ago = (Local::now() - chrono::Duration::weeks(1)).timestamp();
        let query = doc! {
            "time": {
                "$gte": one_week_ago
            }
        };
        let find_options = FindOptions::builder().sort(doc! { "timestamp": 1 }).build();
        let stage_change_cursor: Cursor<LoadSheddingStage> = match connection
            .collection("stage_log")
            .find(query, find_options)
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
        let mut all_stages: Vec<LoadSheddingStage> = match stage_change_cursor.try_collect().await {
            Ok(item) => item,
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        all_stages.reverse();

        // find first timestamp after one week ago
        let query = doc! {
            "time": {
                "$lte": one_week_ago
            }
        };
        let find_options = FindOneOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();
        let first_stage_change: Option<LoadSheddingStage> = match connection
            .collection("stage_log")
            .find_one(query, find_options)
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
        match first_stage_change {
            Some(item) => all_stages.push(item),
            None => (),
        };

        // get the timeschedules
        let query = doc! {
            "municipality" : self.municipality,
        };
        let timeschedule_cursor: Cursor<TimeScheduleEntity> = match connection
            .collection("timeschedule")
            .find(query, None)
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
        let schedule: Vec<TimeScheduleEntity> = match timeschedule_cursor.try_collect().await {
            Ok(item) => item,
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };

        // Time
        let mut time_to_search: DateTime<FixedOffset> = get_date_time(Some(one_week_ago));
        time_to_search = time_to_search.with_minute(0).unwrap();
        let mut down_time = 0;
        while time_to_search <= time_now {
            let hour = time_to_search.hour() as i32;
            let minute = time_to_search.minute() as i32;
            let day = time_to_search.day() as i32;
            // get the timeslots for the current time interval
            let time_slots: Vec<TimeScheduleEntity> = schedule
                .clone()
                .into_iter()
                .filter(|time| {
                    // check what time it falls under
                    if time.stop_hour >= hour
                        && time.stop_minute >= minute
                        && time.start_hour <= hour
                        && time.start_minute <= minute
                    {
                        true
                    } else {
                        false
                    }
                })
                .collect();
            // check next to see if its less than the current TTS
            if all_stages.len() >= 2 {
                if all_stages[1].time <= time_to_search.timestamp() {
                    all_stages.remove(0);
                }
            }

            let mut add_time = false;
            for time_slot in time_slots {
                let mut count: usize = 0;
                let stage = &all_stages[0];
                while (count as i32) < stage.stage {
                    if time_slot.stages.get(count).unwrap().groups[(day - 1) as usize]
                        == group.id.unwrap()
                    {
                        add_time = true;
                        break;
                    }
                    count = count + 1;
                }
                if add_time {
                    break;
                }
            }
            if add_time {
                down_time += 30;
            }
            // update times
            time_to_search = time_to_search
                .checked_add_signed(Duration::minutes(30))
                .unwrap();
        }
        let total_time = 10080;
        let uptime = total_time - down_time;
        Ok(SuburbStatsResponse {
            total_time: TotalTimeForWeek {
                on: uptime,
                off: down_time,
            },
            per_day_times : 0
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
                        self.time = Local::now().timestamp();
                        self.log_stage_data().await;
                    }
                }
                Err(_) => warn!("Eskom API did not return a integer when we queried it."),
            }
        } else {
            warn!("Connection to Eskom Dropped before any operations could take place");
        }
        Ok(self.stage)
    }

    pub async fn log_stage_data(&self) {
        if let Some(client) = &self.db {
            let db_con = &client.database("production");
            let query = doc! {
                "time" : 1
            };
            let find_options = FindOneOptions::builder().sort(query).build();

            // Execute the query to find the latest item
            let result: LoadSheddingStage = match db_con
                .collection("stage_log")
                .find_one(None, find_options)
                .await
                .unwrap()
            {
                Some(data) => data,
                None => LoadSheddingStage {
                    id: None,
                    stage: -1,
                    time: 0,
                    db: None,
                },
            };
            if result.stage != self.stage {
                let _ = self.insert(db_con).await;
            }
        } else {
            return ();
        }
    }
    pub fn set_db(&mut self, db: &Client) {
        self.db = Some(db.to_owned());
    }
}

#[async_trait]
impl Fairing for StageUpdater {
    fn info(&self) -> Info {
        Info {
            name: "Stage Updater",
            kind: Kind::Ignite | Kind::Liftoff,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<rocket::Build>) -> fairing::Result {
        let stage_info = Arc::new(RwLock::new(LoadSheddingStage {
            id: None,
            stage: 0,
            time: Local::now().timestamp(),
            db: None,
        }));
        let stage_info_ref = stage_info.clone();
        thread::spawn(move || {
            loop {
                {
                    let stage_info = stage_info_ref.write();
                    let runtime = Runtime::new().unwrap();
                    let mut info = runtime.block_on(stage_info);
                    let stage = info.fetch_stage();
                    let _ = runtime.block_on(stage);
                }
                // Perform any other necessary processing on stage info
                thread::sleep(std::time::Duration::from_secs(600)); // Sleep for 10 minutes
            }
        });
        let rocket = rocket.manage(Some(stage_info));
        Ok(rocket)
    }
    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let db = rocket.state::<Option<Client>>().unwrap();
        let stage_updater = rocket
            .state::<Option<Arc<RwLock<LoadSheddingStage>>>>()
            .unwrap();
        if let Some(stage) = stage_updater {
            let mut stage_ref = stage.as_ref().clone().write().await;
            if let Some(db) = db {
                stage_ref.set_db(&db.clone());
            }
        }
    }
}
