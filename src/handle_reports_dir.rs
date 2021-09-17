use std::fs;

use crate::constants::get_folder;

// TODO: Use relative file from command line
pub fn create_reports_dir() -> std::io::Result<()> {
  fs::create_dir_all(get_folder())?;
  Ok(())
}
// TODO: Use relative file from command line
pub fn remove_reports_dir() -> std::io::Result<()> {
  fs::remove_dir_all(get_folder())?;
  Ok(())
}
