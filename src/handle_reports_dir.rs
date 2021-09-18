use std::fs;

// TODO: Use relative file from command line
pub fn create_reports_dir() -> std::io::Result<()> {
  fs::create_dir_all("./reports")?;
  Ok(())
}
// TODO: Use relative file from command line
pub fn remove_reports_dir() -> std::io::Result<()> {
  if std::path::Path::new("./reports/").is_dir() {
    fs::remove_dir_all("./reports")?;
  }
  Ok(())
}
