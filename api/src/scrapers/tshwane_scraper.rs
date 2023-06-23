use reqwest;
use scraper::{Html, Selector};
use crate::{scrapers::scraper::Scraper, scraper::Group};
const TABLE_SELECTOR: &'static str = "#";
const TR_SELECTOR: &'static str = "tr";
const TD_SELECTOR: &'static str = "td";

struct TswaneHTMLRow {
    suburb_and_extension: String,
    group: i32
}

struct TswaneScraper {
  excel_url: String,
  groups_url: String,
  document:Option<Html>
}

fn make_selector(selector: &str) -> Selector {
  Selector::parse(selector).unwrap()
}

impl Scraper for TswaneScraper {
  fn run(self:Box<Self>) {
  }

  fn connect(&mut self) {
    let web_page = reqwest::blocking::get(self.groups_url.clone()).unwrap().text().unwrap();
    self.document = Some(Html::parse_document(&web_page));
  }

  fn parse_data(&self) {
    let TABLE = make_selector(&TABLE_SELECTOR);
    let TR = make_selector(&TR_SELECTOR);
    let TD = make_selector(&TD_SELECTOR);
    let main_table = self.document.select(&TABLE).max_by_key(|table| {
        table.select(&TR).count()
    }).expect("No tables found in document?");

    let mut suburbs:Vec<Group> = Vec::new();
    // Find the columns we want
    for row in main_table.select(&TR) {
        // Need to collect this into a Vec<> because we're going to be iterating over it
        // multiple times.
        let entries = row.select(&TD).collect::<Vec<_>>();
        if entries.len() != 2 { // amount of rows
            // TODO add some form of log because this is not supposed to happen
            continue
        }
        // gets the areas and splits the commas
        let areasAffected:String = entries[0].text().collect::<Vec<_>>().join("").trim().to_string();
        let areas:Vec<String> = areasAffected.split(",").map(|i| i.to_string()).collect();
        let group:Result<i32,std::num::ParseIntError> = entries[1].text().collect::<Vec<_>>().join("").trim().parse::<i32>();
        if let Err(error) = group {
            // handle the error
        } else {
          // if we find a group add to it else make a new group
          if let Some(existingGroup) = suburbs.iter_mut().find(|x| x.group == group) {
            for area in areas {
              existingGroup.addSuburb(area);
            }
          } else {
            suburbs.append(Group {
              suburbs: areas.as_mut(),
              group:group,
              stage:-1,
              times: Vec::new()
            })
          }
        }

        for area in areas {

        }
    }
    Ok(suburbs)
  }
}

impl TswaneScraper {
  fn default() -> Self {
    TswaneScraper {
      excel_url: String::from("https://www.tshwane.gov.za/wp-admin/admin-ajax.php?juwpfisadmin=false&action=wpfd&task=file.download&wpfd_category_id=293&wpfd_file_id=38390"),
      groups_url: String::from("https://www.tshwane.gov.za/?page_id=1124#293-293-wpfd-top"),
      document: None
    }
  }
}