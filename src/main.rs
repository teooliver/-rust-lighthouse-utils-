#![feature(str_split_as_str)]

mod constants;
use constants::*;

mod handle_reports_dir;
use handle_reports_dir::{create_reports_dir, remove_reports_dir};

mod read_json_files;
use read_json_files::*;

mod run_lighthouse_test;
use run_lighthouse_test::{run_lighthouse_test, Config};

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
pub struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    config: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    test_url: std::path::PathBuf,
    runs: i32,
    out_file_name: String,
}

fn main() {
    println!("REMOVING REPORTS DIR");
    remove_reports_dir().expect("Something went wrong");
    println!("CREATING REPORTS DIR");
    create_reports_dir().expect("Something went wrong");

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    println!("Running tests...");

    let config: Config = serde_json::from_str(&get_data()).unwrap();

    run_lighthouse_test(config);

    println!("Cool, All done.");
    println!("READING FILES");
    let json_files = JsonFiles::get_json_files();
    JsonFiles::get_metrics_from_file(json_files[0].to_string()).unwrap();
    let avarage = JsonFiles::get_avarage_perfomance(json_files);
    println!("{:?} ======>>>>>", avarage);
}
