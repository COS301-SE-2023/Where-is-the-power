use crate::scrapers::scraper::Scraper;
struct LoadSheddingPeriod {
  start:i32,
  end:i32
}

struct Group {
  suburbs: Vec<String>,
  group: i32,
  stage: i32,
  times: Vec<Box<LoadSheddingPeriod>>,
}

struct StartScrapers {
  scrapers: Vec<Box<dyn Scraper>>
}