use crate::ident::{BigIdent, Ident};
use crate::util::SliceDisplay;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TopDefn {
  Struct(StructDefn),
  Enum(EnumDefn),
  Fn_(FnDefn),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param<I, T> {
  pub ident: I,
  /// maybe not type, but instead kind.
  pub type_: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructDefn {
  pub name: BigIdent,
  /// will be empty iff no params were written in the source
  pub params: Vec<Param<BigIdent, Kind>>,
  pub fields: Vec<Param<Ident, Kinded>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumDefn {
  pub name: BigIdent,
  /// will be empty iff no params were written in the source
  pub params: Vec<Param<BigIdent, Kind>>,
  pub ctors: Vec<Param<Ident, Kinded>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FnDefn {
  pub name: Ident,
  /// will be empty iff no params were written in the source
  pub big_params: Vec<Param<BigIdent, Kind>>,
  pub params: Vec<Param<Ident, Kinded>>,
  pub ret_type: Kinded,
  pub requires: Option<Expr>,
  pub ensures: Option<Expr>,
  pub body: Block,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kind {
  Type,
  Effect,
  Tuple(Vec<Kind>),
  Arrow(Box<Kind>, Box<Kind>),
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Type => write!(f, "Type"),
      Self::Effect => write!(f, "Effect"),
      Self::Tuple(ts) => SliceDisplay::new("(", ts, ")").fmt(f),
      Self::Arrow(k1, k2) => write!(f, "({}) -> ({})", k1, k2),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kinded {
  /// the Vec<Kinded> will be empty iff no args were written in the source
  BigIdent(BigIdent, Vec<Kinded>),
  Tuple(Vec<Kinded>),
  Set(Vec<Kinded>),
  Arrow(Box<Kinded>, Box<Kinded>),
  Effectful(Box<Kinded>, Box<Kinded>),
}

impl fmt::Display for Kinded {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BigIdent(bi, tes) => {
        write!(f, "{}", bi)?;
        if !tes.is_empty() {
          SliceDisplay::new("[", tes, "]").fmt(f)?;
        }
        Ok(())
      }
      Self::Tuple(ts) => SliceDisplay::new("(", ts, ")").fmt(f),
      Self::Set(ts) => SliceDisplay::new("{", ts, "}").fmt(f),
      Self::Arrow(k1, k2) => write!(f, "({}) -> ({})", k1, k2),
      Self::Effectful(t, e) => write!(f, "({}) affects ({})", t, e),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructExpr {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub fields: Vec<Param<Ident, Kinded>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum QualIdent {
  Ident(Ident),
  More(BigIdent, Ident),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
  String_(String),
  Number(u64),
  Tuple(Vec<Expr>),
  Struct(BigIdent, Vec<Kinded>, Vec<Field<Expr>>),
  QualIdent(QualIdent),
  FnCall(QualIdent, Vec<Kinded>, Vec<Expr>),
  FieldGet(Box<Expr>, Ident),
  MethodCall(Box<Expr>, Ident, Vec<Kinded>, Vec<Expr>),
  Return(Box<Expr>),
  Match(Box<Expr>, Vec<Arm>),
  Block(Box<Block>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arm {
  pub pat: Pat,
  pub block: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pat {
  Wildcard,
  String_(String),
  Number(u64),
  Tuple(Vec<Pat>),
  Struct(BigIdent, Vec<Field<Pat>>),
  Enum(QualIdent, Box<Pat>),
  Ident(Ident),
  Or(Box<Pat>, Box<Pat>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Field<T> {
  Ident(Ident),
  IdentAnd(Ident, T),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
  pub stmts: Vec<Stmt>,
  pub expr: Option<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
  Let(Pat, Option<Kinded>, Expr),
}
