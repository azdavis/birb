//! Lexing.

use crate::error::{Error, Result};
use crate::ident::{BigIdent, Ident};
use crate::token::{Token, PUNCT, WORDS};

/// A guess.
const BYTES_PER_TOKEN: usize = 10;

/// Turn a sequence of bytes into a sequence of tokens.
pub fn get(bs: &[u8]) -> Result<Vec<Token>> {
  let mut i = 0;
  let n = bs.len();
  let mut ret = Vec::with_capacity(n / BYTES_PER_TOKEN);
  'outer: while i < n {
    // line comment
    if bs[i] == b'/' && i + 1 < n && bs[i + 1] == b'/' {
      i += 2;
      while i < n && bs[i] != b'\n' {
        i += 1;
      }
      i += 1;
      continue;
    }
    // whitespace
    if bs[i].is_ascii_whitespace() {
      i += 1;
      continue;
    }
    // punctuation
    for &(tok_bs, ref tok) in PUNCT.iter() {
      let tok_n = tok_bs.len();
      if i + tok_n <= n && bs[i..i + tok_n] == *tok_bs {
        ret.push(tok.clone());
        i += tok_n;
        continue 'outer;
      }
    }
    // reserved words
    for &(tok_bs, ref tok) in WORDS.iter() {
      let tok_n = tok_bs.len();
      // need to be careful to parse e.g. `returning` as not `return` and ident
      // `ing`, but ident `returning`
      if i + tok_n <= n
        && (i + tok_n == n || !is_ident_tl(bs[i + tok_n]))
        && bs[i..i + tok_n] == *tok_bs
      {
        ret.push(tok.clone());
        i += tok_n;
        continue 'outer;
      }
    }
    // identifier
    if bs[i].is_ascii_lowercase() {
      let s = i;
      i += 1;
      while i < n && is_ident_tl(bs[i]) {
        i += 1;
      }
      let tok_utf8 = std::str::from_utf8(&bs[s..i]).unwrap();
      ret.push(Token::Ident(Ident::new(tok_utf8)));
      continue 'outer;
    }
    // big identifier
    if bs[i].is_ascii_uppercase() {
      let s = i;
      i += 1;
      while i < n && bs[i].is_ascii_alphanumeric() {
        i += 1;
      }
      let tok_utf8 = std::str::from_utf8(&bs[s..i]).unwrap();
      ret.push(Token::BigIdent(BigIdent::new(tok_utf8)));
      continue 'outer;
    }
    // number
    if bs[i].is_ascii_digit() {
      let mut digits = vec![bs[i]];
      i += 1;
      while i < n {
        if bs[i].is_ascii_digit() {
          digits.push(bs[i]);
        } else if bs[i] != b'_' {
          break;
        }
        i += 1;
      }
      let tok_utf8 = std::str::from_utf8(&digits).unwrap();
      let tok_u64 = match tok_utf8.parse::<u64>() {
        Ok(x) => x,
        Err(e) => return Err(Error::InvalidNumber(e)),
      };
      ret.push(Token::Number(tok_u64));
      continue 'outer;
    }
    // string
    if bs[i] == b'"' {
      i += 1;
      let s = i;
      loop {
        if i >= n {
          return Err(Error::UnclosedString);
        }
        if bs[i] == b'"' {
          break;
        }
        i += 1;
      }
      let tok_utf8 = match std::str::from_utf8(&bs[s..i]) {
        Ok(x) => x,
        Err(e) => return Err(Error::InvalidUTF8(e)),
      };
      i += 1;
      ret.push(Token::String_(tok_utf8.to_owned()));
      continue 'outer;
    }
    // invalid byte
    return Err(Error::InvalidByte(bs[i]));
  }

  ret.shrink_to_fit();
  Ok(ret)
}

fn is_ident_tl(b: u8) -> bool {
  b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'
}

#[cfg(test)]
mod tests {
  use super::{get, BigIdent, Ident, Token as T};

  #[test]
  fn empty() {
    assert_eq!(get(b"").unwrap(), vec![]);
  }

  #[test]
  fn number() {
    assert_eq!(get(b"123").unwrap(), vec![T::Number(123)]);
  }

  #[test]
  fn number_leading_zeroes() {
    assert_eq!(get(b"00000123").unwrap(), vec![T::Number(123)]);
  }

  #[test]
  fn number_ok_underscores() {
    assert_eq!(get(b"123_456").unwrap(), vec![T::Number(123_456)]);
  }

  #[test]
  fn number_weird_underscores() {
    assert_eq!(get(b"1__2______3_____").unwrap(), vec![T::Number(123)]);
  }

  #[test]
  fn string() {
    assert_eq!(
      get(b"\"hey hey\"").unwrap(),
      vec![T::String_("hey hey".to_owned())]
    );
  }

  #[test]
  fn complex() {
    assert_eq!(
      get(b"return ( ) match{foo}Bar[123]").unwrap(),
      vec![
        T::Return,
        T::LRound,
        T::RRound,
        T::Match,
        T::LCurly,
        T::Ident(Ident::new("foo")),
        T::RCurly,
        T::BigIdent(BigIdent::new("Bar")),
        T::LSquare,
        T::Number(123),
        T::RSquare
      ]
    );
  }

  #[test]
  fn too_big_number() {
    assert!(get(b"999999999999999999999999999999999999").is_err());
  }

  #[test]
  fn unterminated_string() {
    assert!(get(b"\"foo bar").is_err());
  }

  #[test]
  fn comment() {
    assert_eq!(get(b"3// hi\n4").unwrap(), vec![T::Number(3), T::Number(4)]);
  }
}
