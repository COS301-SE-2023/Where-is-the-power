use reqwest;
use scraper::Html;
use crate::scrapers::scraper::Scraper;
struct TswaneScraper {
  excel_url: String,
  groups_url: String,
}

impl Scraper for TswaneScraper {
  fn run(self:Box<Self>) {
      
  }
  fn connect(&mut self) {
    let web_page = reqwest::get(self.groups_url.clone());
    //let document = Html::parse_document(webPage.text());
  }
  fn parse_data(&self) {
      
  }
}

impl TswaneScraper {
  fn default() -> Self {
    TswaneScraper {
      excel_url: String::from("https://www.tshwane.gov.za/wp-admin/admin-ajax.php?juwpfisadmin=false&action=wpfd&task=file.download&wpfd_category_id=293&wpfd_file_id=38390"),
      groups_url: String::from("https://www.tshwane.gov.za/?page_id=1124#293-293-wpfd-top")
    }
  }
}