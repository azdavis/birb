use crate::ident::{BigIdent, Ident};
use crate::util::SliceDisplay;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TopDefn {
  Type(TypeDefn),
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
pub struct TypeDefn {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub def: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructDefn {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub fields: Vec<Param<Ident, Type>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumDefn {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub ctors: Vec<Param<Ident, Type>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FnDefn {
  pub name: Ident,
  pub big_params: Vec<Param<BigIdent, Kind>>,
  pub params: Vec<Param<Ident, Type>>,
  pub ret_type: Type,
  pub requires: Option<Expr>,
  pub ensures: Option<Expr>,
  pub body: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kind {
  BigIdent(BigIdent),
  Tuple(Vec<Kind>),
  Arrow(Box<Kind>, Box<Kind>),
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BigIdent(bi) => write!(f, "{}", bi),
      Self::Tuple(ts) => SliceDisplay::new("(", ts, ")").fmt(f),
      Self::Arrow(k1, k2) => write!(f, "({}) -> ({})", k1, k2),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
  BigIdent(BigIdent),
  Tuple(Vec<Type>),
  Arrow(Box<Type>, Box<Type>),
  Effectful(Box<Type>, Effect),
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BigIdent(bi) => write!(f, "{}", bi),
      Self::Tuple(ts) => SliceDisplay::new("(", ts, ")").fmt(f),
      Self::Arrow(k1, k2) => write!(f, "({}) -> ({})", k1, k2),
      Self::Effectful(t, e) => write!(f, "({}) affects {}", t, e),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Effect {
  pub idents: Vec<BigIdent>,
}

impl fmt::Display for Effect {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    SliceDisplay::new("{", &self.idents, "}").fmt(f)
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructExpr {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub fields: Vec<Param<Ident, Type>>,
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
  Struct(BigIdent, Vec<TypeOrEffect>, Vec<Field<Expr>>),
  QualIdent(QualIdent),
  FnCall(QualIdent, Vec<TypeOrEffect>, Vec<Expr>),
  FieldGet(Box<Expr>, Ident),
  MethodCall(Box<Expr>, Ident, Vec<TypeOrEffect>, Vec<Expr>),
  Return(Box<Expr>),
  Match(Box<Expr>, Vec<Arm>),
  Block(Box<Block>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeOrEffect {
  Type(Type),
  Effect(Effect),
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
  TypeAnnotation(Box<Pat>, Type),
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
  Let(Pat, Expr),
}
