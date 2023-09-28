use std::collections::HashMap;

use crate::{
    api::ApiError,
    db::Entity,
    loadshedding::{GroupEntity, MunicipalityEntity, StageTimes, SuburbEntity, TimeScheduleEntity, GeoJson},
};
use mongodb::Client;
use rocket::{serde::json::Json, Responder};
use serde::{Deserialize, Serialize};

#[derive(Responder)]
pub struct UploadResponse(String);

// Upload Request
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub groups: HashMap<i32, HashMap<String, Vec<i32>>>,
    pub times: HashMap<String, HashMap<i32, Vec<i32>>>,
    pub municipality: String,
    pub geo_json: GeoJson
}

#[derive(Debug,Clone)]
pub struct Times {
    pub start_hour:i32,
    pub start_minute:i32,
    pub end_hour:i32,
    pub end_minute:i32
}

pub fn convert_to_ints(time_range:&str) -> Result<Times,Json<ApiError<'static>>> {
    let stripped: String = time_range.chars().filter(|c| !c.is_whitespace()).collect();
    let times: Vec<&str> = stripped.split("-").collect();
    if times.len() != 2 {
        return Err(Json(ApiError::ScraperUploadError(
            "Unexpected time range, your time ranges are not in the format: HH:MM-HH:MM. You potentially have an aditional \"-\"",
        )));
    }
    let mut integer_times = Vec::new();
    for timestring in times {
        // TODO string lenght validation
        let parts: Vec<&str> = timestring.split(":").collect();
        for part in parts {
            let integer_value:i32 = match part.parse() {
                Ok(hour) => hour,
                Err(_e) =>  return Err(Json(ApiError::ScraperUploadError(
                    "Error in time range, unable to convert to an integer. Please check that you are sending in the format \"HH:MM-HH:MM\"",
                )))
            };
            integer_times.push(integer_value);
        }
    }
    if integer_times.len() != 4 {
        return Err(Json(ApiError::ScraperUploadError(
            "Unexpected time range, your time ranges are not in the format: \"HH:MM-HH:MM\". You potentially have an additional : lingering somewhere.",
        )));
    } else {
        let potential_times = Times {start_hour:integer_times[0],start_minute:integer_times[1],end_hour:integer_times[2],end_minute:integer_times[3]};
        if potential_times.start_hour >= 24 {
            return Err(Json(ApiError::ScraperUploadError(
                "You have a malformed starting hour, please fix this, HH <= 23",
            )));
        } else if potential_times.end_hour > 24 || potential_times.end_hour == 0 {
            return Err(Json(ApiError::ScraperUploadError(
                "You have a malformed end hour, please fix this, 0 < HH <= 24",
            )));
        } else if potential_times.start_minute >= 60 {
            return Err(Json(ApiError::ScraperUploadError(
                "You have a malformed start minute, please fix this, MM <= 59",
            )));
        } else if potential_times.end_minute >= 60 {
            return Err(Json(ApiError::ScraperUploadError(
                "You have a malformed end minute, please fix this, MM <= 59",
            )));
        }
        Ok(potential_times)
    }
}

impl UploadRequest {
    pub async fn add_data(self, db: &Client, database: &str) -> Result<(), Json<ApiError<'static>>> {
        // we need refactoring, and we need it immediately
        let municipality = MunicipalityEntity {
            id: None,
            name: self.municipality,
            geometry: self.geo_json,
        };
        let result = municipality.insert(&db.database(database)).await;
        if let Ok(result) = result {
            // suburb insertion
            let municipality_id = result.inserted_id.as_object_id().unwrap();
            let mut groups = HashMap::new();
            for (group, group_suburbs) in self.groups {
                let mut suburbs: Vec<SuburbEntity> = Vec::new();
                let mut object_ids = Vec::new();
                for (suburb, geometry) in group_suburbs {
                    suburbs.push(SuburbEntity {
                        id: None,
                        municipality: municipality_id,
                        name: String::from(suburb),
                        geometry: geometry
                    })
                }
                for suburb in suburbs.iter() {
                    let result = suburb.insert(&db.database(database)).await;
                    if let Ok(result) = result {
                        object_ids.push(result.inserted_id.as_object_id().unwrap());
                    }
                }
                let group_entity = GroupEntity {
                    id: None,
                    suburbs: object_ids,
                    number: group,
                };
                let result = group_entity.insert(&db.database(database)).await;
                if let Ok(result) = result {
                    groups.insert(group, result.inserted_id.as_object_id().unwrap());
                }
            } // end of suburb for

            // timeschedule and group insertion
            for (time, stages) in self.times {
                // strip
                let times = convert_to_ints(&time);
                let converted_times = match times {
                    Ok(times) => times,
                    Err(e) => return Err(e)
                };
                let mut stages_for_time: Vec<StageTimes> = Vec::new();
                for (stage, groups_in_time) in stages {
                    let mut group_ids = Vec::new();
                    for group in groups_in_time {
                        if let Some(group_id) = groups.get(&group) {
                            group_ids.push(group_id.clone());
                        }
                    }
                    let stage_times = StageTimes {
                        stage: stage,
                        groups: group_ids,
                    };
                    stages_for_time.push(stage_times);
                }
                // make the object
                let schedule = TimeScheduleEntity {
                    id: None,
                    start_hour: converted_times.start_hour,
                    start_minute: converted_times.start_minute,
                    stop_hour: converted_times.end_hour,
                    stop_minute: converted_times.end_minute,
                    stages: stages_for_time,
                    municipality: municipality_id,
                };
                let _ = schedule.insert(&db.database(database)).await;
            } // end of times loop
        } else {
            return Err(Json(ApiError::ServerError(
                "For some reason the municpality was not added to the database, this could have happened elsewhere aswell.",
            )));
        }
    Ok(())
    }
}
