#![allow(dead_code)]
#![allow(clippy::large_enum_variant)]

use comfy_table::{Cell, Table};
use serde::Deserialize;
use thousands::Separable;

#[derive(Debug, Deserialize)]
struct JSBenchmarkResult {
  name: String,
  #[serde(rename = "totalTime")]
  total_time: f64,
  min: f64,
  max: f64,
  hz: f64,
  period: f64,
  mean: f64,
  variance: f64,
  sd: f64,
  sem: f64,
  df: f64,
  critical: f64,
  moe: f64,
  rme: f64,
  p75: f64,
  p99: f64,
  p995: f64,
  p999: f64,
}

#[derive(Deserialize)]
struct Throughput {
  per_iteration: u64,
  unit: String,
}

#[derive(Deserialize)]
enum ChangeType {
  NoChange,
  Improved,
  Regressed,
}

#[derive(Deserialize)]
struct ConfidenceInterval {
  estimate: f64,
  lower_bound: f64,
  upper_bound: f64,
  unit: String,
}

#[derive(Deserialize)]
struct ChangeDetails {
  mean: ConfidenceInterval,
  median: ConfidenceInterval,
  change: ChangeType,
}

#[derive(Deserialize)]
struct RustSingleBenchmarkResult {
  id: String,
  reason: String,
  report_directory: String,
  iteration_count: Vec<u64>,
  measured_values: Vec<f64>,
  unit: String,
  throughput: Vec<Throughput>,
  typical: ConfidenceInterval,
  mean: ConfidenceInterval,
  median: ConfidenceInterval,
  median_abs_dev: ConfidenceInterval,
  slope: Option<ConfidenceInterval>,
  change: Option<ChangeDetails>,
}

#[derive(Deserialize)]
struct BenchmarkGroupComplete {
  reason: String,
  group_name: String,
  benchmarks: Vec<String>,
  report_directory: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RustBenchmarkResult {
  Group(BenchmarkGroupComplete),
  Single(RustSingleBenchmarkResult),
}

#[derive(Clone, Deserialize)]
struct Template {
  name: String,
}

struct Benchmark<'a> {
  name: String,
  js: &'a JSBenchmarkResult,
  rust: &'a RustBenchmarkResult,
}

const TEMPLATES: &str = include_str!("./data/templates.json");
const JS_RESULT: &str = include_str!("./out/js.json");
const RUST_RESULT: &str = include_str!("./out/rust.json");

fn main() {
  let templates = serde_json::from_str::<Vec<Template>>(TEMPLATES)
    .expect("could not parse templates")
    .into_iter()
    .map(|t| t.name)
    .collect::<Vec<_>>();
  let js_result = serde_json::from_str::<Vec<JSBenchmarkResult>>(JS_RESULT).expect("could not parse js result");
  let rust_result = serde_json::from_str::<Vec<RustBenchmarkResult>>(RUST_RESULT).expect("could not parse rust result");

  let benchmarks = templates
    .into_iter()
    .map(|name| {
      let js = js_result
        .iter()
        .find(|r| r.name == name)
        .expect("could not find js result");
      let rust = rust_result
        .iter()
        .find(|r| {
          if let RustBenchmarkResult::Single(r) = r {
            r.id == format!("vbml/{}", name)
          } else {
            false
          }
        })
        .expect("could not find rust result");

      Benchmark { name, js, rust }
    })
    .collect::<Vec<_>>();

  let mut table = Table::new();

  table.set_header(vec![
    Cell::new("test"),
    Cell::new("js μs/it"),
    Cell::new("rs μs/it"),
    Cell::new("diff"),
    Cell::new("% speed"),
  ]);

  for benchmark in benchmarks {
    let js = benchmark.js;
    let rust = if let RustBenchmarkResult::Single(rust) = benchmark.rust {
      rust
    } else {
      panic!("unexpected rust result");
    };

    // js is in ms, rust is in ns, need to convert to μs
    let js_mean = js.mean * 1_000.0;
    let rust_mean = rust.mean.estimate / 1_000.0;

    let difference = rust_mean - js_mean;

    let mut difference_cell = Cell::new(&format!("{:.2}μs", difference));
    let mut percentage_cell = Cell::new(&format!(
      "{}%",
      ((js_mean / rust_mean) * 100.0).round().separate_with_commas()
    ));

    if difference > 0.0 {
      difference_cell = difference_cell.fg(comfy_table::Color::Red);
      percentage_cell = percentage_cell.fg(comfy_table::Color::Red);
    } else if difference < 0.0 {
      difference_cell = difference_cell.fg(comfy_table::Color::Green);
      percentage_cell = percentage_cell.fg(comfy_table::Color::Green);
    }

    table.add_row(vec![
      Cell::new(&benchmark.name),
      Cell::new(&format!("{:.2}μs", js_mean)),
      Cell::new(&format!("{:.2}μs", rust_mean)),
      difference_cell,
      percentage_cell,
    ]);
  }

  println!("{table}");
}
