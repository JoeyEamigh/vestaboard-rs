#[cfg(feature = "parser")]
use libtest_mimic::Trial;
#[cfg(feature = "parser")]
use serde::Deserialize;
#[cfg(feature = "parser")]
use vestaboard::{
  board::{BoardData, FLAGSHIP_COLS, FLAGSHIP_ROWS},
  vbml::Vbml,
};

#[cfg(feature = "parser")]
mod common;

#[derive(Deserialize)]
#[cfg(feature = "parser")]
struct Template<const ROWS: usize, const COLS: usize> {
  name: String,
  data: Vbml<ROWS, COLS>,
  expect: BoardData<ROWS, COLS>,
}

#[cfg(feature = "parser")]
const TEMPLATES: &str = include_str!("./common/vbml.json");

#[cfg(feature = "parser")]
fn test_vbml_parse<const ROWS: usize, const COLS: usize>(
  template: Template<ROWS, COLS>,
) -> Result<(), libtest_mimic::Failed> {
  let parsed = template.data.parse().expect("failed to parse vbml");

  if parsed.0 == template.expect.0 {
    Ok(())
  } else {
    Err(libtest_mimic::Failed::from(format!(
      "expected:\n{}\ngot:\n{}",
      template.expect, parsed
    )))
  }
}

#[cfg(feature = "parser")]
fn main() {
  common::setup();

  let templates: Vec<Template<FLAGSHIP_ROWS, FLAGSHIP_COLS>> =
    serde_json::from_str(TEMPLATES).expect("failed to get templates");

  let tests = templates
    .into_iter()
    .map(|t| Trial::test(format!("parse::{}", &t.name), || test_vbml_parse(t)))
    .collect::<Vec<_>>();

  let args = libtest_mimic::Arguments::from_args();
  libtest_mimic::run(&args, tests).exit();
}

#[cfg(not(feature = "parser"))]
fn main() {
  eprintln!("this binary requires the `parser` feature to be enabled");
}
