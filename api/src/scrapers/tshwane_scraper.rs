/*
This is now legacy code which we will repurpouse in future if we decide to use AI
for power outage and out of date data detection
 */
use crate::{scraper::Group, scrapers::scraper::Scraper};
use log;
use reqwest;
use calamine;
use rocket::http::hyper::body::Bytes;
use scraper::{Html, Selector};
use std::{error::Error, fmt, io::{Cursor, Write, Seek}};
const TABLE_SELECTOR: &'static str = "#";
const TR_SELECTOR: &'static str = "tr";
const TD_SELECTOR: &'static str = "td";

fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

#[derive(Debug)]
pub enum TswaneScraperError {
    NoTablesDetected, //TODO may want to refactor into generic error
    NoDocumentToReadFrom,
    UnExpectedGroupInput,
}
struct TswaneScraper {
    excel_url: String,
    groups_url: String,
    document: Option<Html>,
	excel_data: Option<Cursor<Bytes>>
}

impl TswaneScraper {
	#[allow(unused)]
    fn new() -> Self {
        TswaneScraper {
			excel_url: String::from("https://www.tshwane.gov.za/wp-admin/admin-ajax.php?juwpfisadmin=false&action=wpfd&task=file.download&wpfd_category_id=293&wpfd_file_id=38390"),
			groups_url: String::from("https://www.tshwane.gov.za/?page_id=1124#293-293-wpfd-top"),
			document: None,
			excel_data: None
		}
    }
}

impl Scraper for TswaneScraper {
	#[allow(dead_code,unreachable_code)]
    fn run(self: Box<Self>) -> Result<Vec<Group>, Box<dyn Error>> {
		!todo!()
	}

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
		let client = reqwest::blocking::Client::new();
        let web_page = client.get(self.groups_url.clone()).send()?;
		if web_page.status().is_success() {
			let body = web_page.text()?;
			self.document = Some(Html::parse_document(&body));
		} else {
			// Some logs
		}

        let excel_data = client.get(self.excel_url.clone()).send()?;
		if excel_data.status().is_success() {
			self.excel_data = Some(Cursor::new(excel_data.bytes()?));
		} else {
			// Some logs
		}
		Ok(())
    }

	#[allow(non_snake_case)]
    fn parse_data(&self) -> Result<Vec<Group>, Box<dyn Error>> {
        let TABLE = make_selector(&TABLE_SELECTOR);
        let TR = make_selector(&TR_SELECTOR);
        let TD = make_selector(&TD_SELECTOR);

        let main_table: Option<_>;
        match &self.document {
            Some(doc) => {
                let table = doc
                    .select(&TABLE)
                    .max_by_key(|table| table.select(&TR).count());
                match table {
                    Some(value) => {
                        main_table = Some(value);
                    }
                    None => {
                        log::error!("No table was detected, check website content.");
                        return Err(Box::new(TswaneScraperError::NoTablesDetected));
                    }
                }
            }
            None => {
                // should never happen
                log::error!("The HTML that is supposed to be initialized is not");
                return Err(Box::new(TswaneScraperError::NoDocumentToReadFrom));
            }
        }

        let mut suburbs: Vec<Group> = Vec::new();
        // Find the columns we want
        if let Some(main_table) = main_table {
            for row in main_table.select(&TR) {
                let entries = row.select(&TD).collect::<Vec<_>>();
                if entries.len() != 2 {
                    // amount of cells expected for this row
                    // TODO add some form of log because this is not supposed to happen
                    continue;
                }
                // gets the areas and splits the commas
                let areas_affected: String = entries[0]
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();
                let areas: Vec<String> = areas_affected.split(",").map(|i| i.to_string()).collect();

                // read in the group these areas are a part of
                let group: Result<i32, std::num::ParseIntError> = entries[1]
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .parse::<i32>();

                if let Ok(group) = group {
                    // if we find a group add to it else make a new group
                    if let Some(existing_group) = suburbs.iter_mut().find(|x| x.group == group) {
                        for area in areas {
                            existing_group.add_suburb(area);
                        }
                    } else {
                        suburbs.push(Group {
                            suburbs: areas,
                            group: group,
                            stage: -1,
                            times: Vec::new(),
                        })
                    } // end if else block for adding to group
                } else {
                    log::warn!("There has been a change in the layout of the website, possible missing values.");
                    return Err(Box::new(TswaneScraperError::UnExpectedGroupInput));
                    // handle the error
                } // end if else block for error check
            } // end loop through rows
        }
        Ok(suburbs)
    }

    fn parse_secondary_data(&self,
        existingdata: &mut Vec<crate::scraper::Group>,
    ) -> Result<Vec<Group>, Box<dyn Error>> {

		match &self.excel_data {
			Some(data) => {
			// Create a temporary file
			let mut temp_file = tempfile::NamedTempFile::new()?;
			let bytes = data.clone().into_inner();
			temp_file.write_all(&bytes)?;
			temp_file.seek(std::io::SeekFrom::Start(0))?;

			// Open the workbook using the temporary file path
			let mut _workbook = calamine::open_workbook_auto(temp_file.path())?;

			// Cleanup: Delete the temporary file
			let _ = std::fs::remove_file(temp_file.path());
			}

			None => {

			}
		}
		let to_return = existingdata.clone();
		Ok(to_return)
    }
}

impl Error for TswaneScraperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}

impl fmt::Display for TswaneScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tswane Scraper Error: {:?}", self)
    }
}
