use crate::scrapers::scraper::Scraper;

// this is for the clone of a group,
// however if we clone a group we want to delete these
// values anyway so there must be a better way of doing this.
#[derive(Clone)] 
#[allow(dead_code)]
pub struct LoadSheddingPeriod {
  start:i32,
  end:i32
}

#[derive(Clone)]
pub struct Group {
  pub suburbs: Vec<String>,
  pub group: i32,
  pub stage: i32,
  pub times: Vec<Box<LoadSheddingPeriod>>,
}

impl Group {
  pub fn add_suburb(&mut self, suburb:String) {
    self.suburbs.push(suburb);
  }

  #[allow(dead_code)]
  pub fn change_times(&mut self, new_times:Vec<Box<LoadSheddingPeriod>>) {
    self.times = new_times;
  }
}

#[allow(dead_code)]
struct StartScrapers {
  scrapers: Vec<Box<dyn Scraper>>
}