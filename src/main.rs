#![feature(str_split_as_str)]

use serde::{Deserialize, Serialize};
use std::fs::File;

mod utils;
use utils::{create_reports_dir, read_config_from_file, remove_reports_dir};

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
    let args_config = args.config;
    let config: Config;

    if let Some(args_config) = args_config {
        let path_to_config = args_config;
        config = read_config_from_file(path_to_config).unwrap();
    } else {
        // QUESTION: If I were using typescript, I would probably use the Partial type, since I need the
        // same object as the `Cli` struct but without the `config` field;
        config = Config {
            url: args.url.unwrap(),
            dir: args.dir,
            runs: args.runs,
            out_file_name: args.out_file_name,
        };
    }

    println!("Running tests...");

    run_lighthouse_test(config);

    println!("Cool, All done.");
    println!("READING FILES");
    let json_files = JsonFiles::get_json_files();
    JsonFiles::get_metrics_from_file(json_files[0].to_string()).unwrap();
    let avarage = JsonFiles::get_avarage_perfomance(json_files);

    //use generate reports::write_avarage_to_json
    serde_json::to_writer_pretty(&File::create("./reports/report.json").unwrap(), &avarage)
        .unwrap();
}
