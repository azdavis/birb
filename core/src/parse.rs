use crate::cst::{
  Arm, Block, Effect, EnumDefn, Expr, Field, FnDefn, Kind, Param, Pat, QualIdent, Stmt, StructDefn,
  TopDefn, Type, TypeOrEffect,
};
use crate::error::{Error, Result};
use crate::ident::{BigIdent, Ident};
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
    match self {
      Self::EOF => write!(f, "end of file"),
      Self::Token(t) => write!(f, "`{}`", t),
    }
  }
}

// grammar items

fn top_defn(i: usize, ts: &[Token]) -> Result<(usize, TopDefn)> {
  if let Ok(i) = eat(i, ts, Token::Struct) {
    let (i, name) = big_ident(i, ts)?;
    let (i, params) = big_param_list_opt(i, ts)?;
    let i = eat(i, ts, Token::LCurly)?;
    let (i, fields) = comma_sep(i, ts, field)?;
    let i = eat(i, ts, Token::RCurly)?;
    return Ok((
      i,
      TopDefn::Struct(StructDefn {
        name,
        params,
        fields,
      }),
    ));
  }
  if let Ok(i) = eat(i, ts, Token::Enum) {
    let (i, name) = big_ident(i, ts)?;
    let (i, params) = big_param_list_opt(i, ts)?;
    let i = eat(i, ts, Token::LCurly)?;
    let (i, ctors) = comma_sep(i, ts, ctor)?;
    let i = eat(i, ts, Token::RCurly)?;
    return Ok((
      i,
      TopDefn::Enum(EnumDefn {
        name,
        params,
        ctors,
      }),
    ));
  }
  if let Ok(i) = eat(i, ts, Token::Fn_) {
    let (i, name) = ident(i, ts)?;
    let (i, big_params) = big_param_list_opt(i, ts)?;
    let i = eat(i, ts, Token::LRound)?;
    let (i, params) = comma_sep(i, ts, param)?;
    let i = eat(i, ts, Token::RRound)?;
    let i = eat(i, ts, Token::Colon)?;
    let (i, ret_type) = type_(i, ts)?;
    let (i, requires) = requires_clause(i, ts)?;
    let (i, ensures) = ensures_clause(i, ts)?;
    let (i, body) = block(i, ts)?;
    return Ok((
      i,
      TopDefn::Fn_(FnDefn {
        name,
        big_params,
        params,
        ret_type,
        requires,
        ensures,
        body,
      }),
    ));
  }
  err(i, ts, "a top-level definition")
}

fn big_param(i: usize, ts: &[Token]) -> Result<(usize, Param<BigIdent, Kind>)> {
  let (i, bi) = big_ident(i, ts)?;
  let i = eat(i, ts, Token::Colon)?;
  let (i, k) = kind(i, ts)?;
  Ok((
    i,
    Param {
      ident: bi,
      type_: k,
    },
  ))
}

fn big_param_list_opt(i: usize, ts: &[Token]) -> Result<(usize, Vec<Param<BigIdent, Kind>>)> {
  let i = match eat(i, ts, Token::LSquare) {
    Ok(i) => i,
    Err(_) => return Ok((i, Vec::new())),
  };
  let (i, ret) = comma_sep(i, ts, big_param)?;
  let i = eat(i, ts, Token::RSquare)?;
  Ok((i, ret))
}

fn kind(i: usize, ts: &[Token]) -> Result<(usize, Kind)> {
  let (i, k) = kind_hd(i, ts)?;
  if let Ok(i) = eat(i, ts, Token::Arrow) {
    let (i, k2) = kind(i, ts)?;
    return Ok((i, Kind::Arrow(k.into(), k2.into())));
  }
  Ok((i, k))
}

fn kind_hd(i: usize, ts: &[Token]) -> Result<(usize, Kind)> {
  if let Ok((i, bi)) = big_ident(i, ts) {
    return Ok((i, Kind::BigIdent(bi)));
  }
  if let Ok(i) = eat(i, ts, Token::LRound) {
    let (i, ks) = comma_sep(i, ts, kind)?;
    let i = eat(i, ts, Token::RRound)?;
    return Ok((i, Kind::Tuple(ks)));
  }
  err(i, ts, "a kind")
}

fn type_(i: usize, ts: &[Token]) -> Result<(usize, Type)> {
  let (i, t) = type_hd(i, ts)?;
  if let Ok(i) = eat(i, ts, Token::Arrow) {
    let (i, t2) = type_(i, ts)?;
    return Ok((i, Type::Arrow(t.into(), t2.into())));
  }
  if let Ok(i) = eat(i, ts, Token::Affects) {
    let (i, e) = effect(i, ts)?;
    return Ok((i, Type::Effectful(t.into(), e)));
  }
  Ok((i, t))
}

fn type_hd(i: usize, ts: &[Token]) -> Result<(usize, Type)> {
  if let Ok((i, bi)) = big_ident(i, ts) {
    return Ok((i, Type::BigIdent(bi)));
  }
  if let Ok(i) = eat(i, ts, Token::LRound) {
    let (i, ks) = comma_sep(i, ts, type_)?;
    let i = eat(i, ts, Token::RRound)?;
    return Ok((i, Type::Tuple(ks)));
  }
  err(i, ts, "a type")
}

fn field(i: usize, ts: &[Token]) -> Result<(usize, Param<Ident, Type>)> {
  let (i, id) = ident(i, ts)?;
  let i = eat(i, ts, Token::Colon)?;
  let (i, t) = type_(i, ts)?;
  Ok((
    i,
    Param {
      ident: id,
      type_: t,
    },
  ))
}

fn ctor(i: usize, ts: &[Token]) -> Result<(usize, Param<Ident, Type>)> {
  let (i, id) = ident(i, ts)?;
  let i = eat(i, ts, Token::LRound)?;
  let (i, t) = type_(i, ts)?;
  let i = eat(i, ts, Token::RRound)?;
  Ok((
    i,
    Param {
      ident: id,
      type_: t,
    },
  ))
}

fn effect(i: usize, ts: &[Token]) -> Result<(usize, Effect)> {
  let i = eat(i, ts, Token::LCurly)?;
  let (i, idents) = comma_sep(i, ts, big_ident)?;
  let i = eat(i, ts, Token::RCurly)?;
  Ok((i, Effect { idents }))
}

fn param(i: usize, ts: &[Token]) -> Result<(usize, Param<Ident, Type>)> {
  let (i, id) = ident(i, ts)?;
  let i = eat(i, ts, Token::Colon)?;
  let (i, t) = type_(i, ts)?;
  Ok((
    i,
    Param {
      ident: id,
      type_: t,
    },
  ))
}

fn requires_clause(i: usize, ts: &[Token]) -> Result<(usize, Option<Expr>)> {
  let i = match eat(i, ts, Token::Requires) {
    Ok(i) => i,
    Err(_) => return Ok((i, None)),
  };
  let (i, e) = expr(i, ts)?;
  Ok((i, Some(e)))
}

fn ensures_clause(i: usize, ts: &[Token]) -> Result<(usize, Option<Expr>)> {
  let i = match eat(i, ts, Token::Ensures) {
    Ok(i) => i,
    Err(_) => return Ok((i, None)),
  };
  let (i, e) = expr(i, ts)?;
  Ok((i, Some(e)))
}

fn block(i: usize, ts: &[Token]) -> Result<(usize, Block)> {
  let mut i = eat(i, ts, Token::LCurly)?;
  let mut stmts = Vec::new();
  while let Ok((j, s)) = stmt(i, ts) {
    stmts.push(s);
    i = j;
  }
  let (i, e) = match expr(i, ts) {
    Ok((i, e)) => (i, Some(e)),
    Err(_) => (i, None),
  };
  let i = eat(i, ts, Token::RCurly)?;
  Ok((i, Block { stmts, expr: e }))
}

fn stmt(i: usize, ts: &[Token]) -> Result<(usize, Stmt)> {
  let i = eat(i, ts, Token::Let)?;
  let (i, p) = pat(i, ts)?;
  let i = eat(i, ts, Token::Equal)?;
  let (i, e) = expr(i, ts)?;
  return Ok((i, Stmt::Let(p, e)));
}

fn pat(i: usize, ts: &[Token]) -> Result<(usize, Pat)> {
  let (i, p) = pat_hd(i, ts)?;
  if let Ok(i) = eat(i, ts, Token::Bar) {
    let (i, p2) = pat(i, ts)?;
    return Ok((i, Pat::Or(p.into(), p2.into())));
  }
  if let Ok(i) = eat(i, ts, Token::Colon) {
    let (i, t) = type_(i, ts)?;
    return Ok((i, Pat::TypeAnnotation(p.into(), t)));
  }
  Ok((i, p))
}

fn pat_hd(i: usize, ts: &[Token]) -> Result<(usize, Pat)> {
  if let Ok(i) = eat(i, ts, Token::Underscore) {
    return Ok((i, Pat::Wildcard));
  }
  if let Ok((i, s)) = string(i, ts) {
    return Ok((i, Pat::String_(s)));
  }
  if let Ok((i, n)) = number(i, ts) {
    return Ok((i, Pat::Number(n)));
  }
  if let Ok(i) = eat(i, ts, Token::LRound) {
    let (i, ps) = comma_sep(i, ts, pat)?;
    let i = eat(i, ts, Token::RRound)?;
    return Ok((i, Pat::Tuple(ps)));
  }
  if let Ok((i, bi)) = big_ident(i, ts) {
    let i = eat(i, ts, Token::LCurly)?;
    let (i, fps) = comma_sep(i, ts, field_pat)?;
    let i = eat(i, ts, Token::RCurly)?;
    return Ok((i, Pat::Struct(bi, fps)));
  }
  if let Ok((i, ip)) = qual_ident(i, ts) {
    let i = eat(i, ts, Token::LRound)?;
    let (i, p) = pat(i, ts)?;
    let i = eat(i, ts, Token::RRound)?;
    return Ok((i, Pat::Enum(ip, p.into())));
  }
  if let Ok((i, id)) = ident(i, ts) {
    return Ok((i, Pat::Ident(id.into())));
  }
  err(i, ts, "a pattern")
}

fn field_pat(i: usize, ts: &[Token]) -> Result<(usize, Field<Pat>)> {
  let (i, id) = ident(i, ts)?;
  match eat(i, ts, Token::Colon) {
    Ok(i) => {
      let (i, p) = pat(i, ts)?;
      Ok((i, Field::IdentAnd(id, p)))
    }
    Err(_) => Ok((i, Field::Ident(id))),
  }
}

fn expr(i: usize, ts: &[Token]) -> Result<(usize, Expr)> {
  let (mut i, mut e) = expr_hd(i, ts)?;
  while let Ok(j) = eat(i, ts, Token::Dot) {
    let (j, id) = ident(j, ts)?;
    let (j, co) = call_opt(j, ts)?;
    e = match co {
      None => Expr::FieldGet(e.into(), id),
      Some((tes, es)) => Expr::MethodCall(e.into(), id, tes, es),
    };
    i = j;
  }
  Ok((i, e))
}

fn expr_hd(i: usize, ts: &[Token]) -> Result<(usize, Expr)> {
  if let Ok((i, s)) = string(i, ts) {
    return Ok((i, Expr::String_(s)));
  }
  if let Ok((i, n)) = number(i, ts) {
    return Ok((i, Expr::Number(n)));
  }
  if let Ok(i) = eat(i, ts, Token::LRound) {
    let (i, es) = comma_sep(i, ts, expr)?;
    let i = eat(i, ts, Token::RRound)?;
    return Ok((i, Expr::Tuple(es)));
  }
  if let Ok((i, bi)) = big_ident(i, ts) {
    let (i, tes, _) = type_effect_args_opt(i, ts)?;
    let i = eat(i, ts, Token::LCurly)?;
    let (i, fes) = comma_sep(i, ts, field_expr)?;
    let i = eat(i, ts, Token::RCurly)?;
    return Ok((i, Expr::Struct(bi, tes, fes)));
  }
  if let Ok((i, ip)) = qual_ident(i, ts) {
    let (i, co) = call_opt(i, ts)?;
    return match co {
      None => Ok((i, Expr::QualIdent(ip))),
      Some((tes, es)) => Ok((i, Expr::FnCall(ip, tes, es))),
    };
  }
  if let Ok(i) = eat(i, ts, Token::Return) {
    let (i, e) = expr(i, ts)?;
    return Ok((i, Expr::Return(e.into())));
  }
  if let Ok(i) = eat(i, ts, Token::Match) {
    let (i, e) = expr(i, ts)?;
    let i = eat(i, ts, Token::LCurly)?;
    let (i, arms) = arm_list(i, ts)?;
    let i = eat(i, ts, Token::RCurly)?;
    return Ok((i, Expr::Match(e.into(), arms)));
  }
  if let Ok((i, b)) = block(i, ts) {
    return Ok((i, Expr::Block(b.into())));
  }
  err(i, ts, "an expression")
}

fn qual_ident(i: usize, ts: &[Token]) -> Result<(usize, QualIdent)> {
  if let Ok((i, id)) = ident(i, ts) {
    return Ok((i, QualIdent::Ident(id)));
  }
  if let Ok((i, bi)) = big_ident(i, ts) {
    let i = eat(i, ts, Token::ColonColon)?;
    let (i, id) = ident(i, ts)?;
    return Ok((i, QualIdent::More(bi, id)));
  }
  err(i, ts, "a qualified identifier")
}

fn call_opt(i: usize, ts: &[Token]) -> Result<(usize, Option<(Vec<TypeOrEffect>, Vec<Expr>)>)> {
  let (i, tes, got) = type_effect_args_opt(i, ts)?;
  let i = match eat(i, ts, Token::LRound) {
    Ok(i) => i,
    Err(e) => return if got { Err(e) } else { Ok((i, None)) },
  };
  let (i, es) = comma_sep(i, ts, expr)?;
  let i = eat(i, ts, Token::RRound)?;
  Ok((i, Some((tes, es))))
}

fn type_effect_args_opt(i: usize, ts: &[Token]) -> Result<(usize, Vec<TypeOrEffect>, bool)> {
  let i = match eat(i, ts, Token::LSquare) {
    Ok(i) => i,
    Err(_) => return Ok((i, Vec::new(), false)),
  };
  let (i, tes) = comma_sep(i, ts, type_effect)?;
  let i = eat(i, ts, Token::RSquare)?;
  Ok((i, tes, true))
}

fn type_effect(i: usize, ts: &[Token]) -> Result<(usize, TypeOrEffect)> {
  if let Ok((i, t)) = type_(i, ts) {
    return Ok((i, TypeOrEffect::Type(t)));
  }
  if let Ok((i, e)) = effect(i, ts) {
    return Ok((i, TypeOrEffect::Effect(e)));
  }
  err(i, ts, "a type or effect")
}

fn field_expr(i: usize, ts: &[Token]) -> Result<(usize, Field<Expr>)> {
  let (i, id) = ident(i, ts)?;
  match eat(i, ts, Token::Colon) {
    Ok(i) => {
      let (i, e) = expr(i, ts)?;
      Ok((i, Field::IdentAnd(id, e)))
    }
    Err(_) => Ok((i, Field::Ident(id))),
  }
}

fn arm_list(mut i: usize, ts: &[Token]) -> Result<(usize, Vec<Arm>)> {
  let mut ret = Vec::new();
  while let Ok((j, p)) = pat(i, ts) {
    let (j, b) = block(j, ts)?;
    ret.push(Arm { pat: p, block: b });
    i = j;
  }
  Ok((i, ret))
}

// helpers

fn comma_sep<F, U>(mut i: usize, ts: &[Token], mut f: F) -> Result<(usize, Vec<U>)>
where
  F: FnMut(usize, &[Token]) -> Result<(usize, U)>,
{
  let mut ret = Vec::new();
  while let Ok((j, y)) = f(i, ts) {
    ret.push(y);
    i = j;
    i = match eat(j, ts, Token::Comma) {
      Ok(i) => i,
      Err(_) => break,
    };
  }
  Ok((i, ret))
}

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
