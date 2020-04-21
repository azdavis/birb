use crate::cst::TopDefn;
use crate::error::{Error, Result};
use crate::token::Token;
use std::fmt;

pub fn get(ts: &[Token]) -> Result<Vec<TopDefn>> {
  let mut i = 0;
  let mut ds = Vec::new();
  while i < ts.len() {
    let (j, d) = top_defn(i, ts)?;
    ds.push(d);
    i = j;
  }
  Ok(ds)
}

#[derive(Debug)]
pub enum Found {
  EOF,
  Token(Token),
}

impl fmt::Display for Found {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::EOF => write!(f, "end of file"),
      Self::Token(ref t) => t.fmt(f),
    }
  }
}

fn top_defn(i: usize, ts: &[Token]) -> Result<(usize, TopDefn)> {
  if let Ok(i) = eat(i, ts, Token::Type) {
    todo!()
  }
  if let Ok(i) = eat(i, ts, Token::Struct) {
    todo!()
  }
  if let Ok(i) = eat(i, ts, Token::Enum) {
    todo!()
  }
  if let Ok(i) = eat(i, ts, Token::Fn_) {
    todo!()
  }
  err(i, ts, "a top-level definition")
}

// helpers

fn eat(i: usize, ts: &[Token], t: Token) -> Result<usize> {
  let f = found(i, ts);
  if let Found::Token(ref got) = f {
    if t == *got {
      return Ok(i + 1);
    }
  }
  Err(Error::Parse(t.desc(), f))
}

fn ident(i: usize, ts: &[Token]) -> Result<(usize, Ident)> {
  let f = found(i, ts);
  if let Found::Token(Token::Ident(id)) = f {
    return Ok((i + 1, id));
  }
  Err(Error::Parse("an identifier", f))
}

fn big_ident(i: usize, ts: &[Token]) -> Result<(usize, BigIdent)> {
  let f = found(i, ts);
  if let Found::Token(Token::BigIdent(id)) = f {
    return Ok((i + 1, id));
  }
  Err(Error::Parse("a big identifier", f))
}

fn number(i: usize, ts: &[Token]) -> Result<(usize, u64)> {
  let f = found(i, ts);
  if let Found::Token(Token::Number(n)) = f {
    return Ok((i + 1, n));
  }
  Err(Error::Parse("a number", f))
}

fn string(i: usize, ts: &[Token]) -> Result<(usize, String)> {
  let f = found(i, ts);
  if let Found::Token(Token::String_(s)) = f {
    return Ok((i + 1, s));
  }
  Err(Error::Parse("a string", f))
}

fn found(i: usize, ts: &[Token]) -> Found {
  match ts.get(i) {
    Some(t) => Found::Token(t.clone()),
    None => Found::EOF,
  }
}

fn err<T>(i: usize, ts: &[Token], expected: &'static str) -> Result<T> {
  Err(Error::Parse(expected, found(i, ts)))
}
