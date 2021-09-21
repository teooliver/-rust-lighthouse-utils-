use regex::Regex;
use rustils::parse::short::usize_to_i16;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metrics {
  perfomance: f32,
  first_contentful_paint: f32,
  speed_index: f32,
  largest_contentful_paint: f32,
  interactive: f32,
  total_blocking_time: f32,
  cumulative_layout_shift: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunsResult {
  runs: Vec<Metrics>,
  avarage: Metrics,
}

pub struct JsonFiles {}

impl JsonFiles {
  pub fn get_json_files() -> Vec<String> {
    let re = Regex::new(r".*\.json").unwrap();

    let mut json_files: Vec<String> = vec![];
    let path = Path::new("./reports");

    for entry in path.read_dir().expect("read_dir call failed") {
      match entry {
        Ok(entry) => {
          let file = entry.path();
          let str_file = file.to_str();

          if let Some(str_file) = str_file {
            if re.is_match(str_file) {
              json_files.push(String::from(str_file));
            }
          }
        }
        _ => panic!("Something went wrong"),
      }
    }
    json_files
  }

  pub fn get_metrics_from_file(path: String) -> Result<Metrics, Box<dyn Error>> {
    // open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // read the json contents of the file as an instance of `Value`.
    let metrics: Value = serde_json::from_reader(reader)?;

    let perfomance = &metrics["categories"]["performance"]["score"];
    let fcp = &metrics["audits"]["first-contentful-paint"]["displayValue"];
    let si = &metrics["audits"]["speed-index"]["displayValue"];
    let lcp = &metrics["audits"]["largest-contentful-paint"]["displayValue"];
    let interactive = &metrics["audits"]["interactive"]["displayValue"];
    let tbt = &metrics["audits"]["total-blocking-time"]["displayValue"];
    let cls = &metrics["audits"]["cumulative-layout-shift"]["displayValue"];

    let fcp_string = fcp.as_str().unwrap();
    let fcp_clean: Vec<&str> = fcp_string.split("s").collect();

    let si_string = si.as_str().unwrap();
    let si_clean: Vec<&str> = si_string.split("s").collect();

    let lcp_string = lcp.as_str().unwrap();
    let lcp_clean: Vec<&str> = lcp_string.split("s").collect();

    let intercatice_string = interactive.as_str().unwrap();
    let interactive_clean: Vec<&str> = intercatice_string.split("s").collect();

    let tbt_string = tbt.as_str().unwrap();
    let tbt_clean: Vec<&str> = tbt_string.split("ms").collect();

    let clc_string = cls.as_str().unwrap();
    let cls_clean = clc_string.trim();

    let metrics_obj = Metrics {
      perfomance: Value::to_string(perfomance).parse::<f32>().unwrap(),
      first_contentful_paint: fcp_clean[0].trim().parse::<f32>().unwrap(),
      speed_index: si_clean[0].trim().parse::<f32>().unwrap(),
      largest_contentful_paint: lcp_clean[0].trim().parse::<f32>().unwrap(),
      interactive: interactive_clean[0].trim().parse::<f32>().unwrap(),
      total_blocking_time: tbt_clean[0].trim().parse::<f32>().unwrap(),
      cumulative_layout_shift: cls_clean.parse::<f32>().unwrap(),
    };

    Ok(metrics_obj)
  }

  pub fn get_avarage_perfomance(paths: Vec<String>) -> RunsResult {
    let mut avarage_obj = Metrics {
      perfomance: 0.0,
      first_contentful_paint: 0.0,
      speed_index: 0.0,
      largest_contentful_paint: 0.0,
      interactive: 0.0,
      total_blocking_time: 0.0,
      cumulative_layout_shift: 0.0,
    };

    let runs = usize_to_i16(paths.len()) as f32;

    let mut run_results = RunsResult {
      runs: Vec::new(),
      avarage: avarage_obj.clone(),
    };

    for path in paths {
      let metrics = JsonFiles::get_metrics_from_file(path).unwrap();
      run_results.runs.push(metrics.clone());
      // QUESTION: Coundln't find a good way of iterating trought the a Struct,
      // maybe this should be a Struct from the start? Or maybe I should implement my own
      // iterator for this struct?
      // i.e. for metric in metrics {
      //   println!("{:?} ======>>>>>", metric);
      // }

      avarage_obj.perfomance = avarage_obj.perfomance + metrics.perfomance;

      avarage_obj.first_contentful_paint =
        avarage_obj.first_contentful_paint + metrics.first_contentful_paint;

      avarage_obj.speed_index = avarage_obj.speed_index + metrics.speed_index;

      avarage_obj.largest_contentful_paint =
        avarage_obj.largest_contentful_paint + metrics.largest_contentful_paint;

      avarage_obj.interactive = avarage_obj.interactive + metrics.interactive;

      avarage_obj.total_blocking_time =
        avarage_obj.total_blocking_time + metrics.total_blocking_time;

      avarage_obj.cumulative_layout_shift =
        avarage_obj.cumulative_layout_shift + metrics.cumulative_layout_shift;
    }

    avarage_obj.perfomance /= runs;

    avarage_obj.first_contentful_paint /= runs;

    avarage_obj.speed_index /= runs;

    avarage_obj.largest_contentful_paint /= runs;

    avarage_obj.interactive /= runs;

    avarage_obj.total_blocking_time /= runs;

    avarage_obj.cumulative_layout_shift /= runs;

    run_results.avarage = avarage_obj.clone();

    run_results
  }
}
