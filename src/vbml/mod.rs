//! # VBML Parser (requires `parser` feature)
//!
//! Vestaboard Markup Language (VBML) is a simple markup language for creating Vestaboard messages.
//! it is a JSON object that adheres to the type [`Vbml<ROWS, COLS>`]. learn more at <https://docs.vestaboard.com/docs/vbml>.
//!
//! this module provides the [`Vbml`] struct for deserializing VBML from JSON and parsing it into a [`BoardData<ROWS, COLS>`]
//! to be sent to a Vestaboard. it can also be used to build a [`Board<ROWS, COLS>`] directly, if desired.
//!
//! if ROWS and COLS are not provided, the flagship board size (6x22) will be used.
//!
//! # example
//! ```
//! let vbml_string = "{\"props\":{},\"style\":{},\"components\":[]}"; // any valid VBML string
//! let vbml: Vbml<ROWS, COLS> = vbml_string.parse().unwrap();
//! let board: Board<ROWS, COLS> = vbml.parse().unwrap();
//! ```

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::board::{Board, BoardData, FLAGSHIP_COLS, FLAGSHIP_ROWS};

mod format;
pub use format::{
  AbsolutePosition, Align, ComponentStyle, Justify, VbmlComponent, VbmlProps, VbmlRawComponent, VbmlStyle,
  VbmlTemplateComponent,
};

/// a Vestaboard Markup Language (VBML) object
///
/// # attributes
/// - `props`: [`VbmlProps`] - data that will be used to replace templates in the VBML components
/// - `style`: [`VbmlStyle`] - optional style for the VBML (NOTE: DATA HERE IS CURRENTLY IGNORED)
/// - `components`: a Vec of [`VbmlComponent`]s that make up the VBML
///
/// # type parameters
/// - `ROWS`: the number of rows in the board
/// - `COLS`: the number of columns in the board
///
/// # methods
/// - [`Vbml::parse`] - parses the VBML into a [`BoardData<ROWS, COLS>`] for use in api calls
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vbml<const ROWS: usize = FLAGSHIP_ROWS, const COLS: usize = FLAGSHIP_COLS> {
  /// data that will be used to replace templates in the VBML components
  pub props: Option<VbmlProps>,
  /// optional style for the VBML (NOTE: DATA HERE IS CURRENTLY IGNORED)
  pub style: Option<VbmlStyle>,
  /// a Vec of [`VbmlComponent`]s that make up the VBML
  pub components: Vec<VbmlComponent<ROWS, COLS>>,
}

impl<const ROWS: usize, const COLS: usize> Vbml<ROWS, COLS> {
  /// parses the VBML into a [`BoardData<ROWS, COLS>`] for use in api calls
  ///
  /// this implementation has been tested against the official Vestaboard VBML parser,
  /// but may still have edge cases that are not covered. if you encounter any issues,
  /// please open an issue on the repository.
  ///
  /// # errors
  /// - [`VbmlError::Regex`] if there is an error with regex replacement of template during parse
  pub fn parse(&self) -> Result<BoardData<ROWS, COLS>, VbmlError> {
    let mut board: Board<ROWS, COLS> = BoardData::<ROWS, COLS>::default().into();
    let mut components = self.components.to_vec();
    components.sort_by(
      |a, b| match (a.get_style().absolute_position, b.get_style().absolute_position) {
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Equal,
      },
    );

    let props = self.props.as_ref().map(|props| props.replace_template());

    let mut cur_row: usize = 0;
    let mut max_row: usize = 0;
    let mut cur_col: usize = 0;

    for component in &components {
      let style = component.get_style();
      let component_height = style.height.unwrap_or(ROWS as u32) as usize;
      let component_width = style.width.unwrap_or(COLS as u32) as usize;
      let (content_height, content_widest_width, content) = component.get_word_rows(props.as_ref());
      tracing::trace!(
        "component_height: {component_height}; component_width: {component_width}; content_height: {content_height}; content_widest_width: {content_widest_width};",
      );
      tracing::trace!("content: {:?}", content);

      if cur_col + component_width > COLS {
        cur_col = 0;
        cur_row = max_row;
      }

      if let Some(absolute) = &style.absolute_position {
        cur_row = absolute.y as usize;
        cur_col = absolute.x as usize;
      }

      // indexed within the component
      let mut starting_row = 0;
      match &style.align {
        Some(Align::Center) => starting_row = ((component_height - content_height) as f64 / 2.0).floor() as usize,
        Some(Align::Bottom) => starting_row = component_height - content_height,
        Some(Align::Justified) => starting_row = ((component_height - content_height) as f64 / 2.0).ceil() as usize,
        _ => {}
      }

      match content {
        Some(content_rows) => {
          for (row_offset, content_row) in content_rows.iter().filter(|row| !row.is_empty()).enumerate() {
            // indexed within the component
            let mut starting_col = 0;

            match &style.justify {
              Some(Justify::Center) => starting_col = ((component_width - content_row.len()) as f64 / 2.0) as usize,
              Some(Justify::Right) => starting_col = component_width - content_row.len(),
              Some(Justify::Justified) => {
                starting_col = ((component_width - content_widest_width) as f64 / 2.0) as usize
              }
              _ => {}
            }

            let row = cur_row + starting_row + row_offset;
            for (col_offset, content_col) in content_row.iter().enumerate() {
              let col = cur_col + starting_col + col_offset;
              tracing::trace!("row: {row}; col: {col}; content_col: {content_col};",);

              if row >= ROWS || col >= COLS {
                // panic!("row or col out of bounds");
                tracing::error!("row or col out of bounds");
                continue;
              }

              board[row][col] = (*content_col).into();
            }
          }

          cur_col += component_width;
          max_row = max_row.max(cur_row + component_height);
        }
        None => {
          if let VbmlComponent::Raw(raw) = component {
            board = *raw.raw_characters
          }
        }
      };
    }

    Ok(board.into())
  }
}

impl<const ROWS: usize, const COLS: usize> std::str::FromStr for Vbml<ROWS, COLS> {
  type Err = VbmlError;

  /// deserializes a VBML string into a [`Vbml<ROWS, COLS>`]
  ///
  /// # example
  /// ```
  /// let string = "{\"props\":{},\"style\":{},\"components\":[]}"; // a valid VBML string
  /// let vbml: Vbml<ROWS, COLS> = string.parse().unwrap();
  /// ```
  ///
  /// # errors
  /// - [`VbmlError::Deserialize`] if the string cannot be deserialized
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    serde_json::from_str(s).map_err(VbmlError::Deserialize)
  }
}

impl<const ROWS: usize, const COLS: usize> TryFrom<Vbml<ROWS, COLS>> for String {
  type Error = VbmlError;

  /// serializes a [`Vbml<ROWS, COLS>`] into a VBML string
  ///
  /// # example
  /// ```
  /// let vbml = Vbml::<ROWS, COLS> {
  ///  props: None,
  ///  style: None,
  ///  components: vec![],
  /// };
  /// let string: String = vbml.try_into().unwrap();
  /// ```
  ///
  /// # errors
  /// - [`VbmlError::Serialize`] if the VBML cannot be serialized
  fn try_from(value: Vbml<ROWS, COLS>) -> Result<Self, Self::Error> {
    serde_json::to_string(&value).map_err(VbmlError::Serialize)
  }
}

impl<const ROWS: usize, const COLS: usize> TryFrom<Vbml<ROWS, COLS>> for Board<ROWS, COLS> {
  type Error = VbmlError;

  /// parses a [`Vbml<ROWS, COLS>`] into a [`Board<ROWS, COLS>`]
  ///
  /// this is a convenience method for calling [`Vbml::parse`] directly
  fn try_from(value: Vbml<ROWS, COLS>) -> Result<Self, Self::Error> {
    Ok(value.parse()?.into())
  }
}

/// error type for VBML
/// - [`VbmlError::Deserialize`] if there is an error deserializing the VBML
/// - [`VbmlError::Serialize`] if there is an error serializing the VBML
/// - [`VbmlError::Regex`] if there is an error with regex replacement of template during parse
#[derive(Error, Debug)]
pub enum VbmlError {
  /// failed to deserialize into VBML
  #[error("failed to deserialize into VBML")]
  Deserialize(serde_json::Error),
  /// failed to serialize from VBML
  #[error("failed to serialize from VBML")]
  Serialize(serde_json::Error),
  /// failed to replace VBML template
  #[error("failed to replace VBML template")]
  Regex(#[from] regex::Error),
}
