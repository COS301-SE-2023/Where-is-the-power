use std::{thread, time::Duration, sync::{Mutex, Arc}};

use crate::db::Entity;
use async_trait::async_trait;
use bson::oid::ObjectId;
use log::warn;
use macros::Entity;
use rocket::{fairing::{Fairing, Info, Kind, self}, Rocket};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

// Loadshedding Data Structs
pub struct StageUpdater;

#[derive(Debug)]
pub struct LoadSheddingStage {
    pub stage: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "groups"]
pub struct GroupEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub number: i32,
    pub suburbs: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "suburbs"]
pub struct SuburbEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
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
    pub municipality: ObjectId,
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
    pub groups: Vec<ObjectId>,
}

// GeoJSON structures
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

impl MunicipalityEntity {}

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
        let stage_info = Arc::new(Mutex::new(LoadSheddingStage {
			stage : 0
        }));

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