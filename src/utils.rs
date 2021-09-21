use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::read_json_files::RunsResult;
use crate::run_lighthouse_tests::Config;

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
  // Open the file in read-only mode with buffer.
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `Config`.
  let config = serde_json::from_reader(reader)?;

  // Return the `Config` struct.
  Ok(config)
}

pub fn create_reports_dir() -> std::io::Result<()> {
  fs::create_dir_all("./reports")?;
  Ok(())
}

pub fn remove_reports_dir() -> std::io::Result<()> {
  if std::path::Path::new("./reports/").is_dir() {
    fs::remove_dir_all("./reports")?;
  }
  Ok(())
}

pub fn write_results_to_json(avarage: RunsResult) {
  serde_json::to_writer_pretty(&File::create("./reports/report.json").unwrap(), &avarage).unwrap();
}
