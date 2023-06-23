use crate::scrapers::scraper::Scraper;
pub struct LoadSheddingPeriod {
  start:i32,
  end:i32
}

#[derive(Clone)]
pub struct Group {
  suburbs: Vec<String>,
  pub group: i32,
  pub stage: i32,
  times: Vec<Box<LoadSheddingPeriod>>,
}

impl Group {
  pub fn addSuburb(self:&Self, suburb:String) {
    self.suburbs.append(suburb);
  }

  pub fn changeTimes(self:&Self, newTimes:Vec<Box<LoadSheddingPeriod>>) {
    self.times = newTimes;
  }
}

struct StartScrapers {
  scrapers: Vec<Box<dyn Scraper>>
}