use super::{get, Ident, Token as T};

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
      T::BigIdent(Ident::new("Bar")),
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
