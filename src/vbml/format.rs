use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::board::{char::CharacterCode, BoardData};

use super::VbmlError;

lazy_static::lazy_static! {
  pub static ref PROPS_REGEX: regex::Regex = regex::Regex::new(r#"\{(\d+)\}"#).expect("failed to create regex");
  pub static ref TEMPLATE_REGEX: regex::Regex = regex::Regex::new(r#"\{(\d+)\}|\{\{([A-Za-z0-9]+)\}\}"#).expect("failed to create regex");
}

/// enum representing the horizontal justification of a component
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Justify {
  Center,
  Left,
  Right,
  Justified,
}

/// enum representing the vertical alignment of a component
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Align {
  Center,
  Top,
  Bottom,
  Justified,
  Absolute,
}

/// struct representing the absolute position of a component
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbsolutePosition {
  pub x: u32,
  pub y: u32,
}

/// struct representing the style of a component
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStyle {
  /// optional horizontal justification of the component as [`Justify`]
  pub justify: Option<Justify>,
  /// optional vertical alignment of the component as [`Align`]
  pub align: Option<Align>,
  /// optional height of the component (not the content)
  pub height: Option<u32>,
  /// optional width of the component (not the content)
  pub width: Option<u32>,
  /// optional absolute position of the component as [`AbsolutePosition`]
  pub absolute_position: Option<AbsolutePosition>,
}

/// # NOTE: VALUE IS IGNORED IN CURRENT IMPLEMENTATION
///
/// struct representing the style of a VBML component
///
/// in the future it could be used to change the size of the
/// resulting [`BoardData`] after parsing, but for now
/// the value of [`VbmlStyle`] is ignored. to change the size of
/// the resulting [`BoardData`], change the `ROWS` and `COLS` const generics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbmlStyle {
  pub height: Option<u32>,
  pub width: Option<u32>,
}

/// struct representing the props of a VBML component. these are derived from the
/// [`VbmlProps`] struct of a [`super::Vbml`] struct and are used to replace templates in the VBML components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VbmlProps(HashMap<String, String>);

impl VbmlProps {
  /// replaces any templates in the prop values with character values
  /// derived from [`CharacterCode`]
  pub fn replace_template(&self) -> HashMap<String, String> {
    self
      .0
      .iter()
      .map(|(k, v)| {
        (
          k.to_string(),
          PROPS_REGEX
            .replace_all(v, |caps: &regex::Captures| {
              let char_code = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
              let char: char = CharacterCode::from(char_code).into();

              format!("{}", char)
            })
            .to_string(),
        )
      })
      .collect()
  }
}

/// struct representing a VBML "raw" component.
///
/// a VBML "raw" component is a component that has no templates and is just a raw
/// array of character codes. \
/// the documentation says this is useful for setting backgrounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbmlRawComponent<const ROWS: usize, const COLS: usize> {
  /// optional style of the component as [`ComponentStyle`]. if not provided,
  /// a style component filled with None values will be used.
  #[serde(default)]
  pub style: ComponentStyle,
  /// the raw character codes of the component in format [`BoardData<ROWS, COLS>`].
  ///
  /// can be coerced from a raw [`crate::board::Board`] type.
  pub raw_characters: BoardData<ROWS, COLS>,
}

/// struct representing a VBML "template" component.
///
/// a VBML "template" component is a component that has templates that will be replaced
/// with values from the [`VbmlProps`] object in the VBML struct, or are raw strings that
/// will be placed into the board as is.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbmlTemplateComponent {
  /// optional style of the component as [`ComponentStyle`]. if not provided,
  /// a style component filled with None values will be used.
  #[serde(default)]
  pub style: ComponentStyle,
  /// the template string of the component. this can also be a raw string that does
  /// not need to be replaced with values from the [`VbmlProps`] object.
  pub template: String,
}

impl VbmlTemplateComponent {
  /// renders the template string of the component using the [`VbmlProps`] object.
  ///
  /// returns the rendered string or a [`VbmlError::Regex`] if there is an error.
  pub fn render(&self, props: Option<&HashMap<String, String>>) -> Result<String, VbmlError> {
    // tracing::trace!("template: {:?}; props: {:?}", &self.template, props);

    let string = TEMPLATE_REGEX
      .replace_all(&self.template, |caps: &regex::Captures| {
        if let Some(char_code) = caps.get(1) {
          let char_code = char_code.as_str().parse::<u8>().unwrap();
          let char: char = CharacterCode::from(char_code).into();

          return format!("{}", char);
        }

        if let Some(template) = caps.get(2) {
          if let Some(props) = props {
            if let Some(value) = props.get(template.as_str()) {
              return value.to_string();
            }
          }
        }

        "".to_string()
      })
      .to_string();

    Ok(string)
  }
}

/// enum representing a VBML component. can be either a raw component or a template component.
///
/// note that deserialization will fail if both `raw_characters` and `template` are provided in
/// a `VbmlComponent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum VbmlComponent<const ROWS: usize, const COLS: usize> {
  /// a component with `raw_characters` and no `template`
  Raw(VbmlRawComponent<ROWS, COLS>),
  /// a component with `template` and no `raw_characters`
  Template(VbmlTemplateComponent),
}

impl<const ROWS: usize, const COLS: usize> VbmlComponent<ROWS, COLS> {
  /// gets the rows of the component as a tuple of the number of rows, the widest row, and the
  /// character codes of the component.
  ///
  /// if the component is a template, the props will be used to replace the templates in the component.
  /// if the component is a raw component, the return will have just the height and width, and no character codes.
  /// the character codes of a raw component must be accessed separately.
  ///
  /// # returns
  /// - the number of rows in the component: `usize`
  /// - the widest row in the component: `usize`
  /// - the character codes of the component: `Option<[Vec<[CharacterCode]>; ROWS]>`
  ///
  /// as `(usize, usize, Option<[Vec<CharacterCode>; ROWS]>)`
  pub fn get_word_rows(
    &self,
    props: Option<&HashMap<String, String>>,
  ) -> (usize, usize, Option<[Vec<CharacterCode>; ROWS]>) {
    match self {
      VbmlComponent::Template(template) => {
        let style = self.get_style();

        let comp_height = style.height.unwrap_or(ROWS as u32) as usize;
        let comp_width = style.width.unwrap_or(COLS as u32) as usize;

        let mut text = template.render(props).unwrap_or(String::new());

        const ARRAY_REPEAT_VALUE: Vec<CharacterCode> = Vec::new();
        let mut text_mapping: [Vec<CharacterCode>; ROWS] = [ARRAY_REPEAT_VALUE; ROWS];

        if text.is_empty() {
          (0..comp_height).for_each(|i| (0..comp_width).for_each(|_| text_mapping[i].push(CharacterCode::Blank)));
          return (comp_height, comp_width, Some(text_mapping));
        }

        let mut remove_space = true;
        text = text
          .chars()
          .rev()
          .filter(|c| {
            if remove_space && *c == ' ' {
              return false;
            }

            remove_space = *c == '\n';
            true
          })
          .collect();
        text = text.chars().rev().collect();
        tracing::trace!("text: {:?}", text);

        let mut words = text.split_inclusive('\n').flat_map(|s| s.split(' ')).peekable();
        tracing::trace!("words: {:?}", words.clone().collect::<Vec<_>>());

        let mut row: usize = 0;
        let mut col: usize = 0;
        while let Some(word) = words.next() {
          let next_word = words.peek();
          tracing::trace!("word: {word}; next_word: {:?}; col: {col}; row: {row}", next_word);

          if word.len() > (comp_width - col) && word.len() < comp_width && word.chars().nth(0).unwrap_or(' ') != '\n' {
            col = 0;
            row += 1;
          }

          let mut ended_on_newline = false;
          for char in word.chars().map(CharacterCode::from) {
            tracing::trace!("char: {char}; col: {col}; row: {row}");
            if col >= comp_width {
              col = 0;
              row += 1;

              if char == CharacterCode::Newline {
                ended_on_newline = true;
                continue;
              }
            }

            if char == CharacterCode::Newline {
              col = 0;
              row += 1;
              ended_on_newline = true;
              continue;
            }

            if row >= comp_height {
              // panic!("row out of bounds");
              break;
            }

            text_mapping[row].push(char);
            col += 1;
          }

          if let Some(next_word) = next_word {
            if col < comp_width && next_word.len() < comp_width - col && !ended_on_newline {
              text_mapping[row].push(CharacterCode::Blank);
              col += 1;
            }
          }
        }

        let text_widest_width = text_mapping.iter().map(|row| row.len()).max().unwrap_or(0);

        (row + 1, text_widest_width, Some(text_mapping))
      }
      VbmlComponent::Raw(_) => (ROWS, COLS, None),
    }
  }

  /// gets a ref to the [`ComponentStyle`] of the component regardless of the type of component.
  pub fn get_style(&self) -> &ComponentStyle {
    match self {
      VbmlComponent::Raw(raw) => &raw.style,
      VbmlComponent::Template(template) => &template.style,
    }
  }
}
