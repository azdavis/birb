#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
  // punctuation
  LRound,
  RRound,
  LSquare,
  RSquare,
  LCurly,
  RCurly,
  Comma,
  Colon,
  Equal,
  // reserved words
  Affects,
  Become,
  Ensures,
  Enum,
  Exists,
  Fn_,
  Forall,
  If,
  Impl,
  Let,
  Match,
  Mut,
  Priv,
  Pub,
  Requires,
  Return,
  Struct,
  Trait,
  Type,
  Use,
  Where,
  // identifier
  Ident(String),
  // big identifier
  BigIdent(String),
  // number
  Number(u64),
  // string
  String_(String),
}

// these should be sorted longest first, then alphabetically

pub const PUNCT: [(&[u8], Token); 9] = [
  // 1
  (b",", Token::Comma),
  (b":", Token::Colon),
  (b"(", Token::LRound),
  (b")", Token::RRound),
  (b"[", Token::LSquare),
  (b"]", Token::RSquare),
  (b"{", Token::LCurly),
  (b"}", Token::RCurly),
  (b"=", Token::Equal),
];

pub const WORDS: [(&[u8], Token); 21] = [
  // 8
  (b"requires", Token::Requires),
  // 7
  (b"affects", Token::Affects),
  (b"ensures", Token::Ensures),
  // 6
  (b"become", Token::Become),
  (b"exists", Token::Exists),
  (b"forall", Token::Forall),
  (b"return", Token::Return),
  (b"struct", Token::Struct),
  // 5
  (b"match", Token::Match),
  (b"trait", Token::Trait),
  (b"where", Token::Where),
  // 4
  (b"enum", Token::Enum),
  (b"impl", Token::Impl),
  (b"priv", Token::Priv),
  (b"type", Token::Type),
  // 3
  (b"let", Token::Let),
  (b"mut", Token::Mut),
  (b"pub", Token::Pub),
  (b"use", Token::Use),
  // 2
  (b"fn", Token::Fn_),
  (b"if", Token::If),
];
