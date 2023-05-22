pub trait Scraper {
  fn run(self:Box<Self>);
  fn connect(self: &mut Self);
  fn parse_data(&self);
}