//! Concrete syntax trees.

use crate::ident::{BigIdent, Ident};
use crate::util::SliceDisplay;
use std::fmt;

/// A top-level definition.
#[derive(Debug, PartialEq, Eq)]
pub enum TopDefn {
  /// A struct (product type) definition.
  Struct(StructDefn),
  /// An enum (sum type) definition.
  Enum(EnumDefn),
  /// A function definition.
  Fn_(FnDefn),
}

/// A parameter (also reused as a struct field). A pair of identifier and type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param<I, T> {
  /// The identifier.
  pub ident: I,
  /// The type.
  pub type_: T,
}

/// A struct (product type) definition.
#[derive(Debug, PartialEq, Eq)]
pub struct StructDefn {
  /// The name.
  pub name: BigIdent,
  /// The generic type/effect parameters. Will be empty iff no params were written in the source.
  pub params: Vec<Param<BigIdent, Kind>>,
  /// The fields.
  pub fields: Vec<Param<Ident, Kinded>>,
}

/// An enum (sum type) definition.
#[derive(Debug, PartialEq, Eq)]
pub struct EnumDefn {
  /// The name.
  pub name: BigIdent,
  /// The generic type/effect parameters. Will be empty iff no params were written in the source.
  pub params: Vec<Param<BigIdent, Kind>>,
  /// The constructors (also called variants) of the enum.
  pub ctors: Vec<Param<Ident, Kinded>>,
}

/// A function definition.
#[derive(Debug, PartialEq, Eq)]
pub struct FnDefn {
  /// The name.
  pub name: Ident,
  /// The generic type/effect parameters. Will be empty iff no params were written in the source.
  pub big_params: Vec<Param<BigIdent, Kind>>,
  /// The value parameters.
  pub params: Vec<Param<Ident, Kinded>>,
  /// The return type. Even functions which 'return nothing' have one.
  pub ret_type: Kinded,
  /// The requires annotation, the pre-condition just before the function is called.
  pub requires: Option<Expr>,
  /// The ensures annotation, the post-condition just after the function returns.
  pub ensures: Option<Expr>,
  /// The body.
  pub body: Block,
}

/// A kind. The most common kind is Type, but we also have Effect. We also have arrow kinds (like
/// for generic types) and tuple kinds (for convenience).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kind {
  /// The kind of types.
  Type,
  /// The kind of effects.
  Effect,
  /// A tuple of kinds.
  Tuple(Vec<Kind>),
  /// An arrow kind.
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

/// Something that is kinded, aka a type or an effect.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kinded {
  /// The Vec<Kinded> will be empty iff no args were written in the source.
  BigIdent(BigIdent, Vec<Kinded>),
  /// A tuple. The Kinded inside must each have kind Type.
  Tuple(Vec<Kinded>),
  /// A set. The Kinded inside must each have kind Effect.
  Set(Vec<Kinded>),
  /// An arrow kind. The Kinded inside must each have kind Type.
  Arrow(Box<Kinded>, Box<Kinded>),
  /// An effectful kind. The left Kinded is a Type and the right is an Effect.
  Effectful(Box<Kinded>, Box<Kinded>),
}

impl fmt::Display for Kinded {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BigIdent(bi, args) => {
        write!(f, "{}", bi)?;
        if !args.is_empty() {
          SliceDisplay::new("[", args, "]").fmt(f)?;
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

/// A qualified identifier.
#[derive(Debug, PartialEq, Eq)]
pub enum QualIdent {
  /// Just a regular identifier.
  Ident(Ident),
  /// A big identifier and an identifier. Used to refer to a constructor of an enum.
  More(BigIdent, Ident),
}

/// An expression.
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
  /// A string literal, like `"x"`.
  String_(String),
  /// A number (integer) literal, like `3`.
  Number(u64),
  /// A tuple, like `(1, "e")`.
  Tuple(Vec<Expr>),
  /// A struct expression, like `Foo { x: 3 }`.
  Struct(BigIdent, Vec<Kinded>, Vec<Field<Expr>>),
  /// A qualified identifier, like `a` or `Bar::b`.
  QualIdent(QualIdent),
  /// A function call, like `f(x)`.
  FnCall(QualIdent, Vec<Kinded>, Vec<Expr>),
  /// A field get, like `x.bar`.
  FieldGet(Box<Expr>, Ident),
  /// A function call written like a method call, like `x.f()`. Semantically equivalent to `f(x)`.
  MethodCall(Box<Expr>, Ident, Vec<Kinded>, Vec<Expr>),
  /// A return expression, like `return x`.
  Return(Box<Expr>),
  /// A match expression, like `match x { 3 => 4, _ => 5 }`.
  Match(Box<Expr>, Vec<Arm>),
  /// A block, like `{ let x = 3; x + 4 }`.
  Block(Box<Block>),
}

/// An arm of a match expression.
#[derive(Debug, PartialEq, Eq)]
pub struct Arm {
  /// The pattern to match on.
  pub pat: Pat,
  /// The block evaluated if the match succeeds.
  pub block: Block,
}

/// A pattern.
#[derive(Debug, PartialEq, Eq)]
pub enum Pat {
  /// A wildcard, like `_`.
  Wildcard,
  /// A string literal, like `"x"`.
  String_(String),
  /// A number literal, like `3`.
  Number(u64),
  /// A tuple, like `(4, x)`.
  Tuple(Vec<Pat>),
  /// A struct pattern, like `Foo { x, y: 3 }`.
  Struct(BigIdent, Vec<Field<Pat>>),
  /// A constructor pattern, like `Bar::b(x)`.
  Ctor(QualIdent, Box<Pat>),
  /// An identifier pattern, like `x`.
  Ident(Ident),
  /// An or pattern, like `3 | 4`.
  Or(Box<Pat>, Box<Pat>),
}

/// A field.
#[derive(Debug, PartialEq, Eq)]
pub enum Field<T> {
  /// An identifier. Shorthand for `x: x`.
  Ident(Ident),
  /// An identifier and something else, like `x: 3` or `x: y`.
  IdentAnd(Ident, T),
}

/// A block.
#[derive(Debug, PartialEq, Eq)]
pub struct Block {
  /// The statements before the expression at the end.
  pub stmts: Vec<Stmt>,
  /// The expression.
  pub expr: Option<Expr>,
}

/// A statement.
#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
  /// A let-binding, which may define some variables, and may be type-annotated.
  Let(Pat, Option<Kinded>, Expr),
}
