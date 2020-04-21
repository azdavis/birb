use crate::ident::{BigIdent, Ident};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
  // punctuation
  Arrow,
  Bar,
  Colon,
  ColonColon,
  Comma,
  Dot,
  Equal,
  LCurly,
  LRound,
  LSquare,
  Plus,
  RCurly,
  RRound,
  RSquare,
  Underscore,
  // reserved words
  Affects,
  Do,
  Ensures,
  Enum,
  Fn_,
  Let,
  Match,
  Requires,
  Return,
  Struct,
  Type,
  // other
  BigIdent(BigIdent),
  Ident(Ident),
  Number(u64),
  String_(String),
}

impl Token {
  pub fn desc(&self) -> &'static str {
    match *self {
      // punctuation
      Self::Arrow => "->",
      Self::Bar => "|",
      Self::Colon => ":",
      Self::ColonColon => "::",
      Self::Comma => ",",
      Self::Dot => ".",
      Self::Equal => "=",
      Self::LCurly => "{",
      Self::LRound => "(",
      Self::LSquare => "[",
      Self::Plus => "+",
      Self::RCurly => "}",
      Self::RRound => ")",
      Self::RSquare => "]",
      Self::Underscore => "_",
      // reserved words
      Self::Affects => "affects",
      Self::Do => "do",
      Self::Ensures => "ensures",
      Self::Enum => "enum",
      Self::Fn_ => "fn",
      Self::Let => "let",
      Self::Match => "match",
      Self::Requires => "requires",
      Self::Return => "return",
      Self::Struct => "struct",
      Self::Type => "type",
      // other
      Self::BigIdent(..) => "a big identifier",
      Self::Ident(..) => "an identifier",
      Self::Number(..) => "a number",
      Self::String_(..) => "a string",
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::BigIdent(ref x) => write!(f, "{}", x),
      Self::Ident(ref x) => write!(f, "{}", x),
      Self::Number(ref n) => write!(f, "{}", n),
      Self::String_(ref s) => write!(f, "{:?}", s),
      _ => write!(f, "{}", self.desc()),
    }
  }
}

// these should be sorted longest first, then alphabetically

pub const PUNCT: [(&[u8], Token); 15] = [
  // 2
  (b"->", Token::Arrow),
  (b"::", Token::ColonColon),
  // 1
  (b"_", Token::Underscore),
  (b",", Token::Comma),
  (b":", Token::Colon),
  (b"(", Token::LRound),
  (b")", Token::RRound),
  (b"[", Token::LSquare),
  (b"]", Token::RSquare),
  (b"{", Token::LCurly),
  (b"}", Token::RCurly),
  (b"+", Token::Plus),
  (b"=", Token::Equal),
  (b"|", Token::Bar),
  (b".", Token::Dot),
];

pub const WORDS: [(&[u8], Token); 11] = [
  // 8
  (b"requires", Token::Requires),
  // 7
  (b"affects", Token::Affects),
  (b"ensures", Token::Ensures),
  // 6
  (b"return", Token::Return),
  (b"struct", Token::Struct),
  // 5
  (b"match", Token::Match),
  // 4
  (b"enum", Token::Enum),
  (b"type", Token::Type),
  // 3
  (b"let", Token::Let),
  // 2
  (b"do", Token::Do),
  (b"fn", Token::Fn_),
];
