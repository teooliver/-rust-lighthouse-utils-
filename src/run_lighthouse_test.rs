use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Config {
  websites: WebsiteOptions,
  out_file_name: String,
  reports_folder: String,
  run_limit: i8,
}

#[derive(Serialize, Deserialize)]
struct WebsiteOptions {
  control_url: String,
  test_url: String,
}

pub fn run_lighthouse_test(config: Config) {
  let mut runs = 0;
  while runs < config.run_limit {
    let mut control_output = Command::new("lighthouse");
    control_output.arg(format!("{}", config.websites.control_url.as_str()));
    control_output.arg("--quiet");
    control_output.arg("--chrome-flags=\"--headless\"");
    control_output.arg("--only-categories=\"performance\"");
    control_output.arg("--output=\"json\"");
    control_output.arg("--output=\"html\"");
    control_output.arg(format!(
      "--output-path=./{}/{}-control-v{}",
      config.reports_folder.as_str(),
      config.out_file_name.as_str(),
      runs
    ));
    control_output.status().expect("failed to execute process");

    runs += 1;
  }
}
