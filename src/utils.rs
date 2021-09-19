use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::run_lighthouse_test::Config;

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
  // Open the file in read-only mode with buffer.
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `Config`.
  let config = serde_json::from_reader(reader)?;

  // Return the `Config` struct.
  Ok(config)
}
