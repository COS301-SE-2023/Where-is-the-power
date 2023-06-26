use crate::db::Entity;
use bson::oid::ObjectId;
use macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "groups"]
pub struct GroupEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub suburbs: Vec<ObjectId>
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "suburbs"]
pub struct SuburbEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub name: String,
    pub geometry: Option<Vec<CoordinatePoint>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "timeschedule"]
pub struct TimeScheduleEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub start: i32,
    pub stop: i32,
    pub stages: Vec<StageTimes>,
    pub municipality: ObjectId
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "municiplaity"]
pub struct MunicipalityEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StageTimes {
  pub stage: i32,
  pub groups: Vec<ObjectId>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoordinatePoint {
    long:f32,
    lat:f32
}