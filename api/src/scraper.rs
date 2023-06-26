use std::collections::HashMap;

use crate::{
    api::ApiError,
    db::Entity,
    loadshedding::{GroupEntity, MunicipalityEntity, StageTimes, SuburbEntity, TimeScheduleEntity},
};
use bson::Bson;
use mongodb::Client;
use rocket::{serde::json::Json, Responder};
use serde::{Deserialize, Serialize};

#[derive(Responder)]
pub struct UploadResponse(String);

// Upload Request
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub groups: HashMap<i32, Vec<String>>,
    pub times: HashMap<String, HashMap<i32, Vec<i32>>>,
    pub municipality: String,
}

impl UploadRequest {
    pub async fn add_data(self, db: &Client, database: &str) -> Result<(), Json<ApiError<'static>>> {
        let mut groups = HashMap::new();
        for (group, group_suburbs) in self.groups {
            let mut suburbs: Vec<SuburbEntity> = Vec::new();
            let mut object_ids = Vec::new();
            for suburb in group_suburbs {
                suburbs.push(SuburbEntity {
                    id: None,
                    name: String::from(suburb),
                    geometry: None,
                })
            }
            for suburb in suburbs.iter() {
                let result = suburb.insert(&db.database(database)).await;
                if let Ok(result) = result {
                    if let Bson::ObjectId(object_id) = result.inserted_id {
                        object_ids.push(object_id);
                    }
                }
            }
            let group_entity = GroupEntity {
                id: None,
                suburbs: object_ids,
                number: group,
            };
            let result = group_entity.insert(&db.database(database)).await;
            if let Ok(result) = result {
                if let Bson::ObjectId(object_id) = result.inserted_id {
                    groups.insert(group, object_id);
                }
            }
        } // end of group for

        // we need refactoring, and we need it immediately
        let municipality = MunicipalityEntity {
            id: None,
            name: self.municipality,
        };
        let result = municipality.insert(&db.database(database)).await;
        if let Ok(result) = result {
            if let Bson::ObjectId(municipality_id) = result.inserted_id {
                for (time, stages) in self.times {
                    // strip
                    let stripped: String = time.chars().filter(|c| !c.is_whitespace()).collect();
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
                    }
                    let mut stages_for_time: Vec<StageTimes> = Vec::new();
                    for (stage, groups_in_time) in stages {
                        let mut group_ids = Vec::new();
                        for group in groups_in_time {
                            if let Some(group_id) = groups.get(&group) {
                                group_ids.push(*group_id);
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
                        start_hour: integer_times[0],
                        start_minute: integer_times[1],
                        stop_hour: integer_times[2],
                        stop_minute: integer_times[3],
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
        } else {
            return Err(Json(ApiError::ServerError(
                "For some reason the municpality was not added to the database, this could have happened elsewhere aswell.",
            )));
        }
        Ok(())
    }
}
