/// the character codes that can be displayed on the Vestaboard
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CharacterCode {
  Blank = 0,
  A = 1,
  B = 2,
  C = 3,
  D = 4,
  E = 5,
  F = 6,
  G = 7,
  H = 8,
  I = 9,
  J = 10,
  K = 11,
  L = 12,
  M = 13,
  N = 14,
  O = 15,
  P = 16,
  Q = 17,
  R = 18,
  S = 19,
  T = 20,
  U = 21,
  V = 22,
  W = 23,
  X = 24,
  Y = 25,
  Z = 26,
  One = 27,
  Two = 28,
  Three = 29,
  Four = 30,
  Five = 31,
  Six = 32,
  Seven = 33,
  Eight = 34,
  Nine = 35,
  Zero = 36,
  ExclamationMark = 37,
  AtSign = 38,
  PoundSign = 39,
  DollarSign = 40,
  LeftParen = 41,
  RightParen = 42,
  Hyphen = 44,
  PlusSign = 46,
  Ampersand = 47,
  EqualsSign = 48,
  Semicolon = 49,
  Colon = 50,
  SingleQuote = 52,
  DoubleQuote = 53,
  PercentSign = 54,
  Comma = 55,
  Period = 56,
  Slash = 59,
  QuestionMark = 60,
  DegreeSign = 62,
  Red = 63,
  Orange = 64,
  Yellow = 65,
  Green = 66,
  Blue = 67,
  Violet = 68,
  White = 69,
  Black = 70,
  Filled = 71,
  Newline = 100,
}

impl From<u8> for CharacterCode {
  /// converts a `u8` to a [`CharacterCode`]
  fn from(code: u8) -> CharacterCode {
    match code {
      0 => CharacterCode::Blank,
      1 => CharacterCode::A,
      2 => CharacterCode::B,
      3 => CharacterCode::C,
      4 => CharacterCode::D,
      5 => CharacterCode::E,
      6 => CharacterCode::F,
      7 => CharacterCode::G,
      8 => CharacterCode::H,
      9 => CharacterCode::I,
      10 => CharacterCode::J,
      11 => CharacterCode::K,
      12 => CharacterCode::L,
      13 => CharacterCode::M,
      14 => CharacterCode::N,
      15 => CharacterCode::O,
      16 => CharacterCode::P,
      17 => CharacterCode::Q,
      18 => CharacterCode::R,
      19 => CharacterCode::S,
      20 => CharacterCode::T,
      21 => CharacterCode::U,
      22 => CharacterCode::V,
      23 => CharacterCode::W,
      24 => CharacterCode::X,
      25 => CharacterCode::Y,
      26 => CharacterCode::Z,
      27 => CharacterCode::One,
      28 => CharacterCode::Two,
      29 => CharacterCode::Three,
      30 => CharacterCode::Four,
      31 => CharacterCode::Five,
      32 => CharacterCode::Six,
      33 => CharacterCode::Seven,
      34 => CharacterCode::Eight,
      35 => CharacterCode::Nine,
      36 => CharacterCode::Zero,
      37 => CharacterCode::ExclamationMark,
      38 => CharacterCode::AtSign,
      39 => CharacterCode::PoundSign,
      40 => CharacterCode::DollarSign,
      41 => CharacterCode::LeftParen,
      42 => CharacterCode::RightParen,
      44 => CharacterCode::Hyphen,
      46 => CharacterCode::PlusSign,
      47 => CharacterCode::Ampersand,
      48 => CharacterCode::EqualsSign,
      49 => CharacterCode::Semicolon,
      50 => CharacterCode::Colon,
      52 => CharacterCode::SingleQuote,
      53 => CharacterCode::DoubleQuote,
      54 => CharacterCode::PercentSign,
      55 => CharacterCode::Comma,
      56 => CharacterCode::Period,
      59 => CharacterCode::Slash,
      60 => CharacterCode::QuestionMark,
      62 => CharacterCode::DegreeSign,
      63 => CharacterCode::Red,
      64 => CharacterCode::Orange,
      65 => CharacterCode::Yellow,
      66 => CharacterCode::Green,
      67 => CharacterCode::Blue,
      68 => CharacterCode::Violet,
      69 => CharacterCode::White,
      70 => CharacterCode::Black,
      71 => CharacterCode::Filled,
      100 => CharacterCode::Newline,
      _ => CharacterCode::Blank,
    }
  }
}

impl From<CharacterCode> for u8 {
  /// converts a [`CharacterCode`] to a `u8`
  fn from(code: CharacterCode) -> u8 {
    code as u8
  }
}

impl From<CharacterCode> for char {
  /// converts a [`CharacterCode`] to a `char`
  fn from(val: CharacterCode) -> Self {
    code_to_char(val as u8)
  }
}

impl From<char> for CharacterCode {
  /// converts a `char` to a [`CharacterCode`]
  fn from(c: char) -> Self {
    char_to_code(c.to_ascii_uppercase()).into()
  }
}

impl std::fmt::Display for CharacterCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let c = code_to_char((*self).into());
    if matches!(c, '🟥' | '🟧' | '🟨' | '🟩' | '🟦' | '🟪' | '⬜' | '⬛') {
      write!(f, "{:^1}", c)
    } else {
      write!(f, "{:^2}", c)
    }
  }
}

/// converts a `u8` character code to a `char`
fn code_to_char(code: u8) -> char {
  match code {
    0 => ' ',
    1 => 'A',
    2 => 'B',
    3 => 'C',
    4 => 'D',
    5 => 'E',
    6 => 'F',
    7 => 'G',
    8 => 'H',
    9 => 'I',
    10 => 'J',
    11 => 'K',
    12 => 'L',
    13 => 'M',
    14 => 'N',
    15 => 'O',
    16 => 'P',
    17 => 'Q',
    18 => 'R',
    19 => 'S',
    20 => 'T',
    21 => 'U',
    22 => 'V',
    23 => 'W',
    24 => 'X',
    25 => 'Y',
    26 => 'Z',
    27 => '1',
    28 => '2',
    29 => '3',
    30 => '4',
    31 => '5',
    32 => '6',
    33 => '7',
    34 => '8',
    35 => '9',
    36 => '0',
    37 => '!',
    38 => '@',
    39 => '#',
    40 => '$',
    41 => '(',
    42 => ')',
    44 => '-',
    46 => '+',
    47 => '&',
    48 => '=',
    49 => ';',
    50 => ':',
    52 => '\'',
    53 => '"',
    54 => '%',
    55 => ',',
    56 => '.',
    59 => '/',
    60 => '?',
    62 => '°',
    63 => '🟥',
    64 => '🟧',
    65 => '🟨',
    66 => '🟩',
    67 => '🟦',
    68 => '🟪',
    69 => '⬜',
    70 => '⬛',
    // todo: is FILLED necessary? - https://github.com/Vestaboard/vbml/blob/a8a3f9d9b1fbc03d21743fd631cc2d15a3fa206e/src/characterCodesToAscii.ts#L14
    71 => '⬜',
    100 => '\n',
    _ => ' ',
  }
}

/// converts a `char`` to a `u8` character code
fn char_to_code(c: char) -> u8 {
  match c {
    ' ' => 0,
    'A' => 1,
    'B' => 2,
    'C' => 3,
    'D' => 4,
    'E' => 5,
    'F' => 6,
    'G' => 7,
    'H' => 8,
    'I' => 9,
    'J' => 10,
    'K' => 11,
    'L' => 12,
    'M' => 13,
    'N' => 14,
    'O' => 15,
    'P' => 16,
    'Q' => 17,
    'R' => 18,
    'S' => 19,
    'T' => 20,
    'U' => 21,
    'V' => 22,
    'W' => 23,
    'X' => 24,
    'Y' => 25,
    'Z' => 26,
    '1' => 27,
    '2' => 28,
    '3' => 29,
    '4' => 30,
    '5' => 31,
    '6' => 32,
    '7' => 33,
    '8' => 34,
    '9' => 35,
    '0' => 36,
    '!' => 37,
    '@' => 38,
    '#' => 39,
    '$' => 40,
    '(' => 41,
    ')' => 42,
    '-' => 44,
    '+' => 46,
    '&' => 47,
    '=' => 48,
    ';' => 49,
    ':' => 50,
    '\'' => 52,
    '"' => 53,
    '%' => 54,
    ',' => 55,
    '.' => 56,
    '/' => 59,
    '?' => 60,
    '°' => 62,
    '🟥' => 63,
    '🟧' => 64,
    '🟨' => 65,
    '🟩' => 66,
    '🟦' => 67,
    '🟪' => 68,
    '⬜' => 69,
    '⬛' => 70,
    '\n' => 100,
    _ => 0,
  }
}
