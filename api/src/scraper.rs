trait Scraper {
  fn run(&self);
  fn connect(&mut self);
  fn parseData(&self);
  fn default() -> Self;
}

struct LoadSheddingPeriod {
  start:int,
  end:int
}

struct Group {
  suburbs: std::vec<String>,
  group: int,
  stage: int,
  times: std::vec<Box<LoadSheddingPeriod>>,
}
