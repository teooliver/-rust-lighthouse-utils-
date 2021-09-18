#![feature(str_split_as_str)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod handle_reports_dir;
use handle_reports_dir::{create_reports_dir, remove_reports_dir};

mod read_json_files;
use read_json_files::*;

mod run_lighthouse_test;
use run_lighthouse_test::{run_lighthouse_test, Config};

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
    // println!("REMOVING REPORTS DIR");
    remove_reports_dir().expect("Couldn't remove dir");
    // println!("CREATING REPORTS DIR");
    create_reports_dir().expect("Couldn't create dir");
    let args = Cli::from_args();

    // if args.config {
    let path_to_config = args.config.unwrap();
    let json_config = read_config_from_file(path_to_config).unwrap();
    // } else {
    // let config = Config {
    //     url: args.url.unwrap(),
    //     dir: args.dir,
    //     runs: args.runs,
    //     out_file_name: args.out_file_name,
    // };
    // }

    println!("{:#?}", json_config)

    // let cli_config = args.config;
    // if let Some(cli_config) = cli_config {
    //     // config = serde_json::from_str(cli_config).unwrap();
    // } else {
    //     config = args;
    // }
    // println!("{:?}", cli_config);

    // println!("Running tests...");

    // run_lighthouse_test(config);

    // println!("Cool, All done.");
    // println!("READING FILES");
    // let json_files = JsonFiles::get_json_files();
    // JsonFiles::get_metrics_from_file(json_files[0].to_string()).unwrap();
    // let avarage = JsonFiles::get_avarage_perfomance(json_files);
    // println!("{:?} ======>>>>>", avarage);
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Cli`.
    let u = serde_json::from_reader(reader)?;

    // Return the `Cli`.
    Ok(u)
}
