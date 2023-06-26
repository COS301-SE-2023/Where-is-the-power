use rocket::Responder;
use serde::{Deserialize, Serialize};
use crate::scrapers::scraper::Scraper;

// Upload Request
#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub groups: Vec<GroupData>,
    pub times: Vec<TimeRange>,
}

#[derive(Responder)]
pub struct UploadResponse(String);

#[derive(Serialize, Deserialize,Debug)]
pub struct GroupData {
    pub group: i32,
    pub suburbs: Vec<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct TimeRange {
    timePeriod: String,
    stageData: Vec<StageData>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct StageData {
    stage: String,
    groupsOnDay: Vec<i32>,
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

#[allow(dead_code)]
struct StartScrapers {
    scrapers: Vec<Box<dyn Scraper>>,
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
