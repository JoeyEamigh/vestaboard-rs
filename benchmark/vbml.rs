#[cfg(feature = "parser")]
use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(feature = "parser")]
use serde::Deserialize;
#[cfg(feature = "parser")]
use vestaboard::{board, vbml::Vbml};

#[cfg(feature = "parser")]
const ROWS: usize = board::FLAGSHIP_ROWS;
#[cfg(feature = "parser")]
const COLS: usize = board::FLAGSHIP_COLS;

#[cfg(feature = "parser")]
const TEMPLATES: &str = include_str!("./data/templates.json");

#[cfg(feature = "parser")]
#[derive(Clone, Deserialize)]
struct Template<const ROWS: usize, const COLS: usize> {
  name: String,
  data: Vbml<ROWS, COLS>,
}

#[cfg(feature = "parser")]
fn vbml_bench(c: &mut Criterion) {
  let data = serde_json::from_str::<Vec<Template<ROWS, COLS>>>(TEMPLATES).expect("could not parse templates");

  let mut group = c.benchmark_group("vbml");

  for template in data {
    group.bench_function(&template.name, |b| b.iter(|| template.data.parse()));
  }

  group.finish();
}

#[cfg(feature = "parser")]
criterion_group!(vbml, vbml_bench);
#[cfg(feature = "parser")]
criterion_main!(vbml);

#[cfg(not(feature = "parser"))]
fn main() {
  eprintln!("this binary requires the `parser` feature to be enabled");
}
