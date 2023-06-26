use std::collections::HashMap;

use bson::Bson;
use mongodb::Client;
use rocket::Responder;
use serde::{Deserialize, Serialize};
use crate::{loadshedding::{SuburbEntity, GroupEntity}, db::Entity};

#[derive(Responder)]
pub struct UploadResponse(String);

// Upload Request
#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub groups: HashMap<i32,Vec<String>>,
    pub times: HashMap<String,HashMap<i32,Vec<i32>>>,
    pub municipality: String
}

// this is for the clone of a group,
// however if we clone a group we want to delete these
// values anyway so there must be a better way of doing this.
#[derive(Clone)]
#[allow(dead_code)]
pub struct LoadSheddingPeriod {
    start: i32,
    end: i32,
}

#[derive(Clone)]
pub struct Group {
    pub suburbs: Vec<String>,
    pub group: i32,
    pub stage: i32,
    pub times: Vec<Box<LoadSheddingPeriod>>,
}

impl Group {
    pub fn add_suburb(&mut self, suburb: String) {
        self.suburbs.push(suburb);
    }

    #[allow(dead_code)]
    pub fn change_times(&mut self, new_times: Vec<Box<LoadSheddingPeriod>>) {
        self.times = new_times;
    }
}

impl UploadRequest {
    pub async fn add_data(self, db:&Client) {
        let mut groups = Vec::new();
        for (group,group_suburbs) in self.groups {
            let mut suburbs: Vec<SuburbEntity> =  Vec::new();
            let mut object_ids = Vec::new();
            for suburb in group_suburbs {
                suburbs.push(SuburbEntity {
                    id:None,
                    name: String::from(suburb),
                    geometry: None
                })
            }
            for suburb in suburbs.iter() {
                let result = suburb.insert(&db.database("staging")).await;
                if let Ok(result) = result {
                    if let Bson::ObjectId(object_id) = result.inserted_id {
                        object_ids.push(object_id);
                    }
                }
            }
            let group = GroupEntity {id:None, suburbs: object_ids, number:group};
            groups.push(group.insert(&db.database("staging")).await);
        }
    }
}