//! module for working with Vestaboard board representations.
//!
//! the flagship Vestaboard is a 6x22 board, but this module is designed to be flexible for future board sizes
//! as comments in the [Vestaboard VBML JavaScript Library](https://github.com/Vestaboard/vbml/blob/a8a3f9d9b1fbc03d21743fd631cc2d15a3fa206e/src/index.ts#L10)
//! indicate that there are plans for more sizes.
//!
//! a raw [`Board`] is a 2D array of u8 values, where each value represents a character code. it takes two
//! const generics, `ROWS` and `COLS`, to represent the size of the board. this library currently defaults
//! to the size of the flagship board, but that may change in a future release if a different Vestaboard
//! size is released.
//!
//! the [`BoardData`] struct wraps a raw [`Board`] and provides a more ergonomic interface for working with
//! it, including display, serialization, and deserialization. printing a [`BoardData`] will display the
//! board as a grid of characters, with a border around the edges. each character is padded to be two spaces
//! to be the same width as emoji characters.
//!
//! the [`BoardData`] struct also implements a variety of From and TryFrom traits to make it easier to work
//! with board data.

use std::ops::Deref;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

pub(crate) mod char;
pub use char::CharacterCode;

lazy_static::lazy_static! {
  static ref BOARD_REGEX: regex::Regex = Regex::new("[^0-9,]").expect("failed to create regex");
}

/// the number of rows in the flagship Vestaboard
pub const FLAGSHIP_ROWS: usize = 6;
/// the number of columns in the flagship Vestaboard
pub const FLAGSHIP_COLS: usize = 22;

/// representation of a Vestaboard with character codes
pub type TextBoard<const ROWS: usize, const COLS: usize> = [[char; COLS]; ROWS];
/// representation of a Vestaboard with u8 values
pub type Board<const ROWS: usize = FLAGSHIP_ROWS, const COLS: usize = FLAGSHIP_COLS> = [[u8; COLS]; ROWS];

/// a Vestaboard representation with character codes. defaults to the flagship board size.
/// wraps a raw [`Board`].
///
/// # type parameters
/// - `ROWS`: the number of rows in the board
/// - `COLS`: the number of columns in the board
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoardData<const ROWS: usize = FLAGSHIP_ROWS, const COLS: usize = FLAGSHIP_COLS>(
  #[serde_as(as = "[[_; COLS]; ROWS]")] pub Board<ROWS, COLS>,
);

impl<const ROWS: usize, const COLS: usize> Default for BoardData<ROWS, COLS> {
  /// creates a new [`BoardData`] with the specified `ROWS` and `COLS` with all values set to [`CharacterCode::Blank`] (0)
  fn default() -> Self {
    BoardData([[0; COLS]; ROWS])
  }
}

impl<const ROWS: usize, const COLS: usize> From<TextBoard<ROWS, COLS>> for BoardData<ROWS, COLS> {
  /// constructs a new [`BoardData`] from a [`TextBoard`] type
  fn from(value: TextBoard<ROWS, COLS>) -> Self {
    value.map(|row| row.map(|c| CharacterCode::from(c) as u8)).into()
  }
}

impl<const ROWS: usize, const COLS: usize> Deref for BoardData<ROWS, COLS> {
  type Target = Board<ROWS, COLS>;

  /// dereferences the [`BoardData`] to a raw [`Board`] type
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<const ROWS: usize, const COLS: usize> From<Board<ROWS, COLS>> for BoardData<ROWS, COLS> {
  /// constructs a new [`BoardData`] from a raw [`Board`] type
  fn from(raw: Board<ROWS, COLS>) -> Self {
    BoardData(raw)
  }
}

impl<const ROWS: usize, const COLS: usize> PartialEq<BoardData<ROWS, COLS>> for Board<ROWS, COLS> {
  fn eq(&self, other: &BoardData<ROWS, COLS>) -> bool {
    self == &other.0
  }
}

impl<const ROWS: usize, const COLS: usize> PartialEq<Board<ROWS, COLS>> for BoardData<ROWS, COLS> {
  fn eq(&self, other: &Board<ROWS, COLS>) -> bool {
    &self.0 == other
  }
}

impl<const ROWS: usize, const COLS: usize> std::str::FromStr for BoardData<ROWS, COLS> {
  type Err = BoardError;

  /// attempts to parse a string into a [`BoardData`]. input should be string representation
  /// of a board, with each cell separated by a comma.
  ///
  /// # errors
  /// - [`BoardError::TooManyRows`] if there are too many rows in the input
  /// - [`BoardError::TooManyCols`] if there are too many columns in the input
  /// - [`BoardError::InvalidChar`] if there is an invalid character in the input
  ///
  /// # examples
  /// ```
  /// let string = "[[0,0,0,...],[0,0,0,...],...]"; // where stringified array has correct ROWS and COLS
  /// let board: BoardData<ROWS, COLS> = string.parse().unwrap();
  /// ```
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut board: Board<ROWS, COLS> = [[0; COLS]; ROWS];
    let s = BOARD_REGEX.replace_all(s, "");

    for (i, val) in s.split(',').enumerate() {
      let row = i / FLAGSHIP_COLS;
      let col = i % FLAGSHIP_COLS;

      if row >= FLAGSHIP_ROWS || col >= FLAGSHIP_COLS && val.is_empty() {
        continue;
      }

      if row >= FLAGSHIP_ROWS {
        return Err(BoardError::TooManyRows);
      }

      if col >= FLAGSHIP_COLS {
        return Err(BoardError::TooManyCols);
      }

      board[row][col] = val.parse().map_err(|_| BoardError::InvalidChar(val.to_string()))?;
    }

    Ok(BoardData(board))
  }
}

impl<const ROWS: usize, const COLS: usize> From<BoardData<ROWS, COLS>> for Board<ROWS, COLS> {
  /// converts a [`BoardData`] into a raw [`Board`] type
  fn from(val: BoardData<ROWS, COLS>) -> Self {
    val.0
  }
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Display for BoardData<ROWS, COLS> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, " {}", "-".repeat(COLS * 2))?;

    for row in self.0.iter() {
      for (col_idx, col) in row.iter().enumerate() {
        if col_idx == 0 {
          write!(f, "|")?;
        }

        CharacterCode::from(*col).fmt(f)?;

        if col_idx == COLS - 1 {
          write!(f, "|")?;
        }
      }
      writeln!(f)?;
    }

    writeln!(f, " {}", "-".repeat(COLS * 2))?;

    Ok(())
  }
}

/// error type for the Vestaboard board module
/// - [`BoardError::TooManyRows`] if there are too many rows in the input
/// - [`BoardError::TooManyCols`] if there are too many columns in the input
/// - [`BoardError::InvalidChar`] if there is an invalid character in the input
/// - [`BoardError::Regex`] if there is an error with regex parsing of board data
/// - [`BoardError::InvalidLength`] if the length of the input is invalid
#[derive(Error, Debug)]
pub enum BoardError {
  /// too many rows in the input
  #[error("too many rows in the input")]
  TooManyRows,
  /// too many columns in the input
  #[error("too many columns in the input")]
  TooManyCols,
  /// invalid character in the input, see the wrapped string for the invalid character
  #[error("invalid character in the input: {0}")]
  InvalidChar(String),
  /// regex error (usually while parsing replacement of board data), see the wrapped regex::Error for more details
  #[error("regex error: {0}")]
  Regex(#[from] regex::Error),
  /// invalid board length
  #[error("invalid length")]
  InvalidLength,
}
