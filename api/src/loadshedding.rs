use std::{collections::HashMap, sync::Arc, thread};

use crate::{
    api::{ApiError, ApiResponse},
    db::Entity,
};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, Document};
use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveDateTime, Timelike, Utc};
use lazy_static::__Deref;
use log::warn;
use macros::Entity;
use mockall::automock;
use mongodb::{options::FindOneOptions, options::FindOptions, Client, Cursor, Database};
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    futures::{future::try_join_all, TryStreamExt},
    post,
    get,
    serde::json::Json,
    Orbit, Rocket, State,
};
use serde::{Deserialize, Deserializer, Serialize};
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
    // query start
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
    let municipalities: Vec<MunicipalityEntity> = match cursor.try_collect().await {
        Ok(item) => item,
        Err(err) => {
            log::error!("Unable to Collect suburbs from cursor {err}");
            return ApiError::ServerError("Error occured on the server, sorry :<").into();
        }
    };
    // query end
    let stage = &loadshedding_stage
        .inner()
        .as_ref()
        .clone()
        .unwrap()
        .read()
        .await
        .stage;
    let db_functions = DBFunctions {};
    let future_data = municipalities.iter().map(|municipality| {
        municipality.get_regions_at_time(
            stage.to_owned(),
            request.time,
            Some(connection),
            &db_functions,
        )
    });
    let response = try_join_all(future_data).await;
    if let Ok(data) = response {
        return ApiResponse::Ok(data.into_iter().fold(
            MapDataDefaultResponse {
                map_polygons: vec![],
            },
            |acc, obj| acc + obj,
        ));
    } else {
        log::error!("Unable to fold MapDataResponse");
        return ApiError::ServerError("Error occured on the server, sorry :<").into();
    }
}

#[utoipa::path(post, tag = "Suburb Statistics", path = "/api/fetchSuburbStats", request_body = SuburbStatsRequest)]
#[post("/fetchSuburbStats", format = "application/json", data = "<request>")]
pub async fn fetch_suburb_stats<'a>(
    db: &State<Option<Client>>,
    request: Json<SuburbStatsRequest>,
) -> ApiResponse<'a, SuburbStatsResponse> {
    let oid = &request.suburb_id;
    let connection = db.as_ref().unwrap().database("production");
    let query = doc! {"geometry" : {"$in" : [oid]}};
    let suburb: SuburbEntity = match connection
        .collection("suburbs")
        .find_one(query, None)
        .await
        .unwrap()
    {
        Some(result) => result,
        None => return ApiError::ServerError("Document not found").into(),
    };
    let db_functions = DBFunctions {};
    match suburb
        .get_total_time_down_stats(Some(&connection), &db_functions, None)
        .await
    {
        Ok(data) => return ApiResponse::Ok(data),
        Err(err) => return err.into(),
    }
}

#[utoipa::path(post, tag = "Schedule Data", path = "/api/fetchScheduleData", request_body = SuburbStatsRequest)]
#[post("/fetchScheduleData", format = "application/json", data = "<request>")]
pub async fn fetch_schedule<'a>(
    db: &State<Option<Client>>,
    request: Json<SuburbStatsRequest>,
) -> ApiResponse<'a, PredictiveSuburbStatsResponse> {
    let oid = &request.suburb_id;
    let connection = db.as_ref().unwrap().database("production");
    let query = doc! {"geometry" : {"$in" : [oid]}};
    let suburb: SuburbEntity = match connection
        .collection("suburbs")
        .find_one(query, None)
        .await
        .unwrap()
    {
        Some(result) => result,
        None => return ApiError::ServerError("Document not found").into(),
    };
    let db_functions = DBFunctions {};
    match suburb.build_schedule(Some(&connection), &db_functions, None).await {
        Ok(data) => return ApiResponse::Ok(data),
        Err(err) => return err.into(),
    }
}

#[utoipa::path(post, tag = "Schedule Data", path = "/api/fetchTimeForPolygon", request_body = SuburbStatsRequest)]
#[post(
    "/fetchTimeForPolygon",
    format = "application/json",
    data = "<request>"
)]
pub async fn fetch_time_for_polygon<'a>(
    db: &State<Option<Client>>,
    request: Json<SuburbStatsRequest>,
) -> ApiResponse<'a, PredictiveSuburbStatsResponse> {
    let oid = &request.suburb_id;
    let connection = db.as_ref().unwrap().database("production");
    let query = doc! {"geometry" : {"$in" : [oid]}};
    let suburb: SuburbEntity = match connection
        .collection("suburbs")
        .find_one(query, None)
        .await
        .unwrap()
    {
        Some(result) => result,
        None => return ApiError::ServerError("Document not found").into(),
    };
    let db_functions = DBFunctions {};
    let time_now = get_date_time(None);
    match suburb.build_schedule(Some(&connection), &db_functions, None).await {
        Ok(data) => {
            let relevant: Vec<TimeSlot> = data
                .times_off
                .into_iter()
                .filter(|time| time.time_slot_bound_validation(&time_now))
                .collect();
            if let Some(to_return) = relevant.get(0) {
                return ApiResponse::Ok(PredictiveSuburbStatsResponse {
                    times_off: vec![to_return.clone()],
                });
            } else {
                return ApiResponse::Ok(PredictiveSuburbStatsResponse { times_off: vec![] });
            }
        }
        Err(err) => err.into(),
    }
}

#[utoipa::path(get, path = "/api/fetchCurrentStage")]
#[get("/fetchCurrentStage")]
pub async fn get_current_stage<'a>(
    loadshedding_stage: &State<Option<Arc<RwLock<LoadSheddingStage>>>>,
) -> ApiResponse<'a, i32> {
    // query end
    ApiResponse::Ok(loadshedding_stage
        .inner()
        .as_ref()
        .clone()
        .unwrap()
        .read()
        .await
        .stage)
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "stage_log"]
pub struct LoadSheddingStage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing, skip_deserializing)]
    db: Option<Client>,
    stage: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoadsheddingData {
    pub start: SASTDateTime,
    pub end: SASTDateTime,
    #[serde(deserialize_with = "deserialize_stage")]
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
    pub id: i32,
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(skip)]
    #[serde(rename = "SP_CODE")]
    pub sp_code: f64,

    #[serde(skip)]
    #[serde(rename = "SP_CODE_st")]
    pub sp_code_st: String,

    #[serde(rename = "SP_NAME")]
    pub sp_name: String,

    #[serde(skip)]
    #[serde(rename = "MP_CODE")]
    pub mp_code: f64,

    #[serde(skip)]
    #[serde(rename = "MP_CODE_st")]
    pub mp_code_st: String,

    #[serde(rename = "MP_NAME")]
    pub mp_name: String,

    #[serde(rename = "MN_MDB_C")]
    pub mn_mdb_c: String,

    #[serde(skip)]
    #[serde(rename = "MN_CODE")]
    pub mn_code: f64,

    #[serde(skip)]
    #[serde(rename = "MN_CODE_st")]
    pub mn_code_st: String,

    #[serde(rename = "MN_NAME")]
    pub mn_name: String,

    #[serde(rename = "DC_MDB_C")]
    pub dc_mdb_c: String,

    #[serde(skip)]
    #[serde(rename = "DC_MN_C")]
    pub dc_mn_c: f64,

    #[serde(skip)]
    #[serde(rename = "DC_MN_C_st")]
    pub dc_mn_c_st: String,

    #[serde(rename = "DC_NAME")]
    pub dc_name: String,

    #[serde(rename = "PR_MDB_C")]
    pub pr_mdb_c: String,

    #[serde(skip)]
    #[serde(rename = "PR_CODE")]
    pub pr_code: f64,

    #[serde(rename = "PR_CODE_st")]
    pub pr_code_st: String,

    #[serde(rename = "PR_NAME")]
    pub pr_name: String,

    #[serde(skip)]
    #[serde(rename = "ALBERS_ARE")]
    pub albers_are: f64,

    #[serde(skip)]
    #[serde(rename = "Shape_Leng")]
    pub shape_leng: f64,

    #[serde(skip)]
    #[serde(rename = "Shape_Area")]
    pub shape_arek: f64,

    #[serde(rename = "PowerStatus")]
    pub power_status: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Entity,PartialEq)]
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
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json! {
    SuburbStatsRequest {
        suburb_id : 1245
    }
})]
pub struct SuburbStatsRequest {
    pub suburb_id: u32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SuburbStatsResponse {
    pub total_time: TotalTime,
    pub per_day_times: HashMap<String, TotalTime>,
    pub suburb: SuburbEntity,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PredictiveSuburbStatsResponse {
    times_off: Vec<TimeSlot>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimeSlot {
    start: i64,
    end: i64,
}

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct TotalTime {
    pub on: i32,
    pub off: i32,
}

impl std::ops::Add for MapDataDefaultResponse {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut merged = self;
        merged.map_polygons.extend(other.map_polygons);
        merged
    }
}

impl TotalTime {
    fn new() -> TotalTime {
        // total minutes
        TotalTime { on: 1440, off: 0 }
    }

    fn add_off_time(&mut self, minutes: i32) {
        self.off += minutes;
        self.on -= minutes;
    }
}

// entity implimentations:
fn get_date_time(time: Option<i64>) -> DateTime<FixedOffset> {
    // South African Standard Time Offset
    let sast = FixedOffset::east_opt(2 * 3600).unwrap();
    // get search time
    match time {
        Some(time) => {
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(time, 0).unwrap(), Utc)
                .with_timezone(&sast)
        }
        None => Local::now().with_timezone(&sast),
    }
}

// temporary database resturcturing
#[automock]
#[async_trait]
pub trait DBFunctionsTrait: Sync {
    async fn collect_schedules<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<TimeScheduleEntity>, ApiError<'static>>;
    async fn collect_groups<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<GroupEntity>, ApiError<'static>>;
    async fn collect_suburbs<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<SuburbEntity>, ApiError<'static>>;
    async fn collect_one_group<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<GroupEntity, ApiError<'static>>;
    async fn collect_stage_logs<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<LoadSheddingStage>, ApiError<'static>>;
    async fn collect_one_stage_log<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<LoadSheddingStage, ApiError<'static>>;
}

pub struct DBFunctions {}

#[async_trait]
impl DBFunctionsTrait for DBFunctions {
    async fn collect_schedules<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<TimeScheduleEntity>, ApiError<'static>> {
        let result = match TimeScheduleEntity::find(query, connection.unwrap(), options).await {
            Ok(groups) => groups.into_iter().map(|b| *b).collect(),
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        Ok(result)
    }

    async fn collect_suburbs<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<SuburbEntity>, ApiError<'static>> {
        let result = match SuburbEntity::find(query, connection.unwrap(), options).await {
            Ok(groups) => groups.into_iter().map(|b| *b).collect(),
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        Ok(result)
    }

    async fn collect_groups<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<GroupEntity>, ApiError<'static>> {
        let result = match GroupEntity::find(query, connection.unwrap(), options).await {
            Ok(groups) => groups.into_iter().map(|b| *b).collect(),
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        Ok(result)
    }
    async fn collect_one_group<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<GroupEntity, ApiError<'static>> {
        let result = match GroupEntity::find_one(query, connection.unwrap(), options).await {
            Some(group) => group,
            None => {
                warn!("Error, a suburb is not associated with a group");
                return Err(ApiError::ServerError(
                    "Group cannot be identified for specified suburb",
                ));
            }
        };
        Ok(result.deref().clone())
    }
    async fn collect_stage_logs<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<Vec<LoadSheddingStage>, ApiError<'static>> {
        let result = match LoadSheddingStage::find(query, connection.unwrap(), options).await {
            Ok(groups) => groups.into_iter().map(|b| *b).collect(),
            Err(err) => {
                log::error!("Unable to Collect suburbs from cursor {err}");
                return Err(ApiError::ServerError(
                    "Error occured on the server, sorry :<",
                ));
            }
        };
        Ok(result)
    }
    async fn collect_one_stage_log<'a>(
        &self,
        query: Document,
        connection: Option<&'a Database>,
        options: Option<FindOptions>,
    ) -> Result<LoadSheddingStage, ApiError<'static>> {
        let result = match LoadSheddingStage::find_one(query, connection.unwrap(), options).await {
            Some(group) => group,
            None => {
                warn!("Error, a suburb is not associated with a group");
                return Err(ApiError::ServerError(
                    "Group cannot be identified for specified suburb",
                ));
            }
        };
        Ok(result.deref().clone())
    }
}
// db functions end
impl LoadsheddingData {
    pub fn convert_to_loadsheddingstage(&self) -> LoadSheddingStage {
        return LoadSheddingStage {
            id: None,
            start_time: self.start.0.timestamp(),
            end_time: self.end.0.timestamp(),
            db: None,
            stage: self.stage,
        }
    }
}


impl TimeSlot {
    fn time_slot_bound_validation(&self, time_to_search: &DateTime<FixedOffset>) -> bool {
        let stamp = time_to_search.timestamp();
        if self.start <= stamp {
            if self.end > stamp {
                return true;
            }
        }
        return false;
    }
}

impl TimeScheduleEntity {
    pub fn timestamp_from_slot_times(
        &self,
        time_to_search: DateTime<FixedOffset>,
        start: bool,
    ) -> DateTime<FixedOffset> {
        let mut time = time_to_search
            .with_hour(if start {
                self.start_hour as u32
            } else {
                self.stop_hour as u32
            })
            .unwrap()
            .with_minute(if start {
                self.start_minute as u32
            } else {
                self.stop_minute as u32
            })
            .unwrap();
        if time < time_to_search {
            time = time.checked_add_signed(chrono::Duration::days(1)).unwrap();
        }
        time
    }
    fn is_within_timeslot(&self, time_to_search: &DateTime<FixedOffset>) -> bool {
        let mut maybe = false;
        if self.start_hour <= time_to_search.hour() as i32 {
            if self.stop_hour >= time_to_search.hour() as i32 {
                maybe = true;
                if self.stop_minute <= time_to_search.minute() as i32
                    && self.stop_hour == time_to_search.hour() as i32
                {
                    return false;
                }
                if self.start_minute > time_to_search.minute() as i32
                    && self.start_hour == time_to_search.hour() as i32
                {
                    return false;
                }
            }
        }
        return maybe;
    }
}

impl MunicipalityEntity {
    pub async fn get_regions_at_time(
        &self,
        stage: i32,
        time: Option<i64>,
        connection: Option<&Database>,
        db_functions: &dyn DBFunctionsTrait,
    ) -> Result<MapDataDefaultResponse, ApiError<'static>> {
        let mut suburbs_off = Vec::<SuburbEntity>::new();
        let time_to_search: DateTime<FixedOffset> = get_date_time(time);
        let mut geography = self.geometry.clone();

        // schedule query: all that fit the search time
        let query = doc! {
            "municipality": self.id.unwrap()
        };
        let unfiltered_schedules: Vec<TimeScheduleEntity> = match db_functions
            .collect_schedules(query, connection, None)
            .await
        {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
        let mut schedules: Vec<TimeScheduleEntity> = Vec::new();
        // filter schedules to relevant ones
        for schedule in unfiltered_schedules {
            let keep = schedule.is_within_timeslot(&time_to_search);
            println!("{:?}", time_to_search.hour());
            if keep {
                schedules.push(schedule);
            }
        }
        // schedule query end

        // suburbs query: all suburbs
        let query = doc! {
            "municipality" : self.id
        };

        let suburbs: Vec<SuburbEntity> =
            match db_functions.collect_suburbs(query, connection, None).await {
                Ok(data) => data,
                Err(err) => return Err(err),
            };
        // end of suburbs query

        // collect suburbs into a map for quick lookup and moving
        let mut suburbs: HashMap<ObjectId, SuburbEntity> = suburbs
            .into_iter()
            .map(|suburb| (suburb.id.unwrap(), suburb))
            .collect();

        // go through schedules
        for doc in schedules {
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
            let group_entities: Vec<GroupEntity> =
                match db_functions.collect_groups(query, connection, None).await {
                    Ok(data) => data,
                    Err(err) => return Err(err),
                };
            // groups query end

            // go through the relevant groups and place the affected suburbs into
            //  the suburbs_off array
            for group in group_entities {
                let removed: Vec<SuburbEntity> = group
                    .suburbs
                    .iter()
                    .filter_map(|key| suburbs.remove(key))
                    .collect();
                suburbs_off.extend(removed);
            }
        }
        // now we must mark all of our map stuff
        for feature in &mut geography.features {
            for suburb in &suburbs_off {
                if suburb.geometry.contains(&feature.id) {
                    feature.properties.power_status = Some("off".to_string());
                    break;
                }
            }

            if let None = feature.properties.power_status {
                for (_, suburb) in &suburbs {
                    if suburb.geometry.contains(&feature.id) {
                        feature.properties.power_status = Some("on".to_string());
                        break;
                    }
                }
            }
            if let None = feature.properties.power_status {
                feature.properties.power_status = Some("undefined".to_string());
            }
        }
        Ok(MapDataDefaultResponse {
            map_polygons: vec![geography],
        })
    }
}

impl SuburbEntity {
    pub async fn build_schedule(
        self,
        connection: Option<&Database>,
        db_functions: &dyn DBFunctionsTrait,
        time: Option<i64>,
    ) -> Result<PredictiveSuburbStatsResponse, ApiError<'static>> {
        let time_now = get_date_time(time)
            .with_second(0)
            .unwrap()
            .with_minute(0)
            .unwrap();
        println!("{:?}", time_now.timestamp());
        let mut response: Vec<TimeSlot> = Vec::new();
        let day_in_future =
            get_date_time(Some((Local::now() + chrono::Duration::days(1)).timestamp()));

        let (group, mut all_stages, schedule) = match self
            .collect_information(&time_now.timestamp(), connection, db_functions)
            .await
        {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
        all_stages.reverse();

        let mut time_to_search = time_now;
        println!("{:?}", time_to_search.timestamp());
        while time_to_search < day_in_future {
            let day = time_to_search.day() as i32;
            let time_slots: Vec<TimeScheduleEntity> = schedule
                .clone()
                .into_iter()
                .filter(|time| {
                    // check what time it falls under
                    time.is_within_timeslot(&time_to_search)
                })
                .collect();
            if all_stages.len() >= 2 {
                if all_stages[1].start_time <= time_to_search.timestamp() {
                    all_stages.remove(0);
                }
            }
            if let Some(slot) = self.add_time_checker(&all_stages[0], &time_slots, &group, &day) {
                let end_time = slot.timestamp_from_slot_times(time_to_search, false);
                time_to_search = slot.timestamp_from_slot_times(time_to_search, true);
                match response.last_mut() {
                    Some(time) => {
                        if time.end >= time_to_search.timestamp() {
                            time.end = end_time.timestamp();
                        } else {
                            response.push(TimeSlot {
                                start: time_to_search.timestamp(),
                                end: end_time.timestamp(),
                            });
                        }
                    }
                    None => response.push(TimeSlot {
                        start: time_to_search.timestamp(),
                        end: end_time.timestamp(),
                    }),
                }
                time_to_search = end_time;
            } else {
                time_to_search += chrono::Duration::minutes(30);
            }
        }
        return Ok(PredictiveSuburbStatsResponse {
            times_off: response,
        });
    }
    pub async fn get_total_time_down_stats(
        self,
        connection: Option<&Database>,
        db_functions: &dyn DBFunctionsTrait,
        time: Option<i64>
    ) -> Result<SuburbStatsResponse, ApiError<'static>> {
        // Time
        let mut down_time = 0;
        let mut daily_stats: HashMap<String, TotalTime> = HashMap::new();

        // get the relevant data
        let time_now = get_date_time(time);
        let one_week_ago = get_date_time(Some(
            (get_date_time(time) - chrono::Duration::weeks(1)).timestamp(),
        ));
        let (group, mut all_stages, schedule) = match self
            .collect_information(&one_week_ago.timestamp(), connection, db_functions)
            .await
        {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        // Time
        let mut time_to_search: DateTime<FixedOffset> = one_week_ago;
        time_to_search = time_to_search.with_minute(0).unwrap();

        // main logic loop
        while time_to_search <= time_now {
            let day = time_to_search.day() as i32;

            // get the timeslots for the current time interval
            let time_slots: Vec<TimeScheduleEntity> = schedule
                .clone()
                .into_iter()
                .filter(|time| {
                    // check what time it falls under
                    time.is_within_timeslot(&time_to_search)
                })
                .collect();
            // check next to see if its less than the current TTS
            if all_stages.len() >= 2 {
                if all_stages[1].start_time <= time_to_search.timestamp() {
                    all_stages.remove(0);
                }
            }

            // check if there exists a timeslot during which we have loadshedding, if there is, add time
            let add_time = self.add_time_checker(&all_stages[0], &time_slots, &group, &day);

            if let Some(_) = add_time {
                down_time += 30;
                let day = daily_stats
                    .entry(time_to_search.weekday().to_string())
                    .or_insert(TotalTime::new());
                day.add_off_time(30);
            }
            // update times
            time_to_search = time_to_search
                .checked_add_signed(Duration::minutes(30))
                .unwrap();
        }
        let total_time = 10080;
        let uptime = total_time - down_time;
        Ok(SuburbStatsResponse {
            total_time: TotalTime {
                on: uptime,
                off: down_time,
            },
            per_day_times: daily_stats,
            suburb: self,
        })
    }

    fn add_time_checker<'a>(
        &self,
        stage: &LoadSheddingStage,
        time_slots: &'a Vec<TimeScheduleEntity>,
        group: &GroupEntity,
        day: &i32,
    ) -> Option<&'a TimeScheduleEntity> {
        for time_slot in time_slots {
            let stages: Vec<&StageTimes> = time_slot
                .stages
                .iter()
                .filter(|x| x.stage <= stage.stage)
                .collect();
            for stage in stages {
                let slot = match stage.groups.get((day-1) as usize) {
                    Some(data) => {
                        if data.to_owned() == group.id.unwrap() {
                             Some(time_slot)
                        } else {
                            None
                        }
                    },
                    None => None
                };
                if let Some(data) = slot  {
                    return Some(data);
                }
            }
        }
        None
    }
    // returns the group accociated with this suburb,
    //  the stage_logs from and greater than a given time,
    //  and the timeschedules for the municpality of the suburb
    async fn collect_information(
        &self,
        from_time: &i64,
        connection: Option<&Database>,
        db_functions: &dyn DBFunctionsTrait,
    ) -> Result<(GroupEntity, Vec<LoadSheddingStage>, Vec<TimeScheduleEntity>), ApiError<'static>>
    {
        let query = doc! {
            "suburbs" : {
                "$in" : [self.id.unwrap()]
            }
        };
        let group: GroupEntity = match db_functions
            .collect_one_group(query, connection, None)
            .await
        {
            Ok(group) => group,
            Err(err) => {
                return Err(err);
            }
        };

        // get all the stage changes from the past week
        let query = doc! {
            "startTime": {
                "$gt": from_time
            }
        };
        let find_options = FindOptions::builder().sort(doc! { "startTime": 1 }).build();
        let mut all_stages = match db_functions
            .collect_stage_logs(query, connection, Some(find_options))
            .await
        {
            Ok(item) => item,
            Err(err) => {
                return Err(err);
            }
        };
        all_stages.reverse();

        // find first timestamp after one week ago
        let query = doc! {
            "startTime": {
                "$lte": from_time
            }
        };
        let find_options = FindOptions::builder()
            .sort(doc! { "startTime": -1 })
            .limit(1)
            .build();
        let first_stage_change = match db_functions
            .collect_one_stage_log(query, connection, Some(find_options))
            .await
        {
            Ok(cursor) => cursor,
            Err(err) => {
                return Err(err);
            }
        };
        all_stages.push(first_stage_change);

        // get the timeschedules
        let query = doc! {
            "municipality" : self.municipality,
        };
        let schedule = match db_functions
            .collect_schedules(query, connection, None)
            .await
        {
            Ok(item) => item,
            Err(err) => {
                return Err(err);
            }
        };
        Ok((group, all_stages, schedule))
    }
}

// Rocket State Loop Objects
impl LoadSheddingStage {
    pub async fn set_stage(&mut self) {
        // get the next thing from db
        let con = &self.db.as_ref().unwrap().database("production");
        let now = get_date_time(None).timestamp();
        let query = doc! {
            "startTime" : {
                "$lte" : now
            }
        };
        let filter = doc! {
            "startTime" : -1
        };
        let find_options = FindOneOptions::builder().sort(filter).build();
        let new_status: LoadSheddingStage = con
            .collection("stage_log")
            .find_one(query, find_options)
            .await
            .unwrap()
            .unwrap();
        println!("self is: {:?}", self);
        println!("new is: {:?}", new_status);
        self.end_time = new_status.end_time;
        self.start_time = new_status.start_time;
        self.stage = new_status.stage;
        println!("self is after operation: {:?}", self);
        //println!("{:?}", self);
    }

    pub async fn request_stage_data_update(&mut self) -> Result<i32, reqwest::Error> {
        loop {
            let stage = reqwest::get(
                "https://d42sspn7yra3u.cloudfront.net/eskom-load-shedding-extended-status.json",
            )
            .await?;
            if stage.status().is_success() {
                let text = stage.text().await?;
                let times: Vec<LoadsheddingData> = serde_json::from_str(&text).unwrap();
                self.log_stage_data(times).await;
                break;
            } else {
                warn!("Connection to https://d42sspn7yra3u.cloudfront.net/eskom-load-shedding-extended-status.json Dropped before any operations could take place. Check that url is still up {:?}", stage);
            }
            thread::sleep(std::time::Duration::from_secs(10));
        }
        Ok(self.stage)
    }

    async fn log_stage_data(&mut self, mut times: Vec<LoadsheddingData>) {
        if let Some(client) = &self.db.as_ref() {
            let db_con = &client.database("production");
            let query = doc! {
                "start_time" : -1
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
                    start_time: 0,
                    end_time: 0,
                    db: None,
                },
            };
            let latest_in_db = result.start_time;
            loop {
                let next = times.pop();
                if let Some(new_data) = next {
                    if latest_in_db >= new_data.start.0.timestamp() {
                        let _ = self.update_db_with_changes(new_data, db_con).await;
                    } else {
                        let _ = new_data.convert_to_loadsheddingstage().insert(db_con).await;
                    }
                } else {
                    break;
                }
            }
            self.set_stage().await;
        } else {
            return ();
        }
    }

    async fn update_db_with_changes(&self, new_data:LoadsheddingData, db_con:&Database) {
        // findone that matches our times.
        let query = doc! {
            "startTime" : new_data.start.0.timestamp(),
            "endTime" : new_data.end.0.timestamp()
        };
        //match LoadSheddingStage::find_one(query, db_con, None).await {
        match db_con
        .collection::<LoadSheddingStage>("stage_log")
        .find_one(query, None)
        .await
        .unwrap() {
            // if match
            // check similarities
            // if stage change: update
            Some(mut db_data) => {
                if db_data.stage != new_data.stage {
                    let update =
                        mongodb::options::UpdateModifications::Document(doc! {
                            "$set" : {"stage" : new_data.stage}
                        });
                    let _ = db_data.update(update, db_con).await;
                }
            }
            // else if no match
            // find all that encapsulate this new time.
            // delete all of them
            // insert
            None => {
                let filter = doc! {
                    "startTime" : {"$lt" : new_data.end.0.timestamp()},
                    "endTime" : {"$gt" : new_data.start.0.timestamp()}
                };
                // Can be optimized into a delete many
                match LoadSheddingStage::find(filter, db_con, None).await {
                    Ok(data) => {
                        for stage in data {
                            let _ = stage.delete(db_con).await;
                        }
                        let _ = new_data
                            .convert_to_loadsheddingstage()
                            .insert(db_con)
                            .await;
                    }
                    Err(_) => {
                        let _ = new_data
                            .convert_to_loadsheddingstage()
                            .insert(db_con)
                            .await;
                    }
                }
            }
        };
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
            start_time: 0,
            end_time: 0,
            db: None,
        }));
        let rocket = rocket.manage(Some(stage_info));
        Ok(rocket)
    }
    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let db = rocket.state::<Option<Client>>().unwrap();
        let stage_updater = rocket
            .state::<Option<Arc<RwLock<LoadSheddingStage>>>>()
            .unwrap();
        if let Some(stage) = stage_updater {
            {
                let mut stage_ref = stage.as_ref().clone().write().await;
                if let Some(db) = db {
                    stage_ref.set_db(&db.clone());
                }
            }
            let stage_info_ref = stage.clone();
            thread::spawn(move || {
                loop {
                    {
                        let stage_info = stage_info_ref.write();
                        let runtime = Runtime::new().unwrap();
                        let mut info = runtime.block_on(stage_info);
                        let stage = info.set_stage();
                        let _ = runtime.block_on(stage);
                    }
                    // Perform any other necessary processing on stage info
                    thread::sleep(std::time::Duration::from_secs(1600)); // Sleep for 20 mins
                }
            });
            let stage_info_ref = stage.clone();
            thread::spawn(move || {
                loop {
                    {
                        let stage_info = stage_info_ref.write();
                        let runtime = Runtime::new().unwrap();
                        let mut info = runtime.block_on(stage_info);
                        let stage = info.request_stage_data_update();
                        let _ = runtime.block_on(stage);
                    }
                    // Perform any other necessary processing on stage info
                    thread::sleep(std::time::Duration::from_secs(18000)); // Sleep for 5 hours
                }
            });
        }
    }
}

#[derive(Debug, Clone)]
pub struct SASTDateTime(pub DateTime<FixedOffset>);

const FORMAT: &str = "%Y-%m-%dT%H:%M";
impl<'de> Deserialize<'de> for SASTDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // get search time
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).unwrap();
        // hack for now because library is not being co-operative
        let convert_to_sast = dt.timestamp() - 2*3600;
        let sast = get_date_time(Some(convert_to_sast));
        println!("{:?}", sast);
        Ok(SASTDateTime(sast))
    }
}

fn deserialize_stage<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}
