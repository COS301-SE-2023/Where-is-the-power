use std::error::Error;

use crate::scraper;

pub trait Scraper {
    fn run(self: Box<Self>) -> Result<Vec<scraper::Group>, Box<dyn Error>>;
    fn connect(self: &mut Self) -> Result<(),Box<dyn Error>>;
    fn parse_data(&self) -> Result<Vec<scraper::Group>, Box<dyn Error>>;
    fn parse_secondary_data(
        &self,
        existingdata: &mut Vec<scraper::Group>,
    ) -> Result<Vec<scraper::Group>, Box<dyn Error>>; // optional
}
