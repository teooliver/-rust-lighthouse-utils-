#![feature(str_split_as_str)]

use colour::green_ln;
use serde::{Deserialize, Serialize};
mod utils;
use utils::{create_reports_dir, read_config_from_file, remove_reports_dir, write_results_to_json};

mod read_json_files;
use read_json_files::*;

mod run_lighthouse_tests;
use run_lighthouse_tests::{run_lighthouse_tests, Config};

use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct Cli {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: Option<std::path::PathBuf>,
    #[structopt(short, long = "test-url", required_if("config", "None"))]
    url: Option<String>,
    #[structopt(short = "r", long = "runs", default_value = "2")]
    runs: i32,
    #[structopt(short = "o", long = "out", default_value = "website")]
    out_file_name: String,
    #[structopt(short = "dir", long, default_value = "reports")]
    dir: String,
}

fn main() {
    remove_reports_dir().expect("Couldn't remove dir");
    create_reports_dir().expect("Couldn't create dir");
    let args = Cli::from_args();
    let args_config = args.config;
    let config: Config;

    if let Some(args_config) = args_config {
        let path_to_config = args_config;
        config = read_config_from_file(path_to_config).unwrap();
    } else {
        config = Config {
            url: args.url.unwrap(),
            dir: args.dir,
            runs: args.runs,
            out_file_name: args.out_file_name,
        };
    }

    green_ln!("Running tests...");

    run_lighthouse_tests(config);

    let json_files = JsonFiles::get_json_files();
    let avarage = JsonFiles::get_avarage_perfomance(json_files);

    write_results_to_json(avarage);

    green_ln!("Cool, All done.");
}
