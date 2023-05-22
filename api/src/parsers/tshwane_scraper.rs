struct TswaneParser {
  excelURL: String,
  groupsURL: String,
}

impl Scraper for TswaneParser {
  fn run(&self) {
  }
  fn connect(&mut self) {
  }
  fn parseData(&self) {}
  fn default() -> Self {
    TswaneParser {
      excelURL: "https://www.tshwane.gov.za/wp-admin/admin-ajax.php?juwpfisadmin=false&action=wpfd&task=file.download&wpfd_category_id=293&wpfd_file_id=38390",
      groupsURL: "https://www.tshwane.gov.za/?page_id=1124#293-293-wpfd-top"
    }
  }
}
