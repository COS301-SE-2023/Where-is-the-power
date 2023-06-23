use crate::scraper;
pub trait Scraper {
  fn run(self:Box<Self>) -> Result<Vec<scraper::Group>>; // add error
  fn connect(self: &mut Self);
  fn parse_data(&self);
}