use crate::ident::{BigIdent, Ident};

pub enum TopDefn {
  Type(TypeDefn),
  Struct(StructDefn),
  Enum(EnumDefn),
  Fn_(FnDefn),
}

pub struct Param<I, T> {
  pub ident: I,
  /// maybe not type, but instead kind.
  pub type_: T,
}

pub struct TypeDefn {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub def: Type,
}

pub struct StructDefn {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub fields: Vec<Param<Ident, Type>>,
}

pub struct EnumDefn {
  pub name: BigIdent,
  pub params: Vec<Param<BigIdent, Kind>>,
  pub ctors: Vec<Param<Ident, Type>>,
}

pub struct FnDefn {
  pub name: Ident,
  pub big_params: Vec<Param<BigIdent, Kind>>,
  pub params: Vec<Param<Ident, Type>>,
  pub ret_type: Type,
  pub requires: Option<Expr>,
  pub ensures: Option<Expr>,
  pub body: Expr,
}

pub enum Kind {
  BigIdent(BigIdent),
  Tuple(Vec<Kind>),
  Arrow(Box<Kind>, Box<Kind>),
}

pub enum Type {
  BigIdent(BigIdent),
  Tuple(Vec<Type>),
  Arrow(Box<Type>, Box<Type>),
  Effectful(Box<Type>, Vec<BigIdent>),
}

pub struct StructExpr {
  pub name: BigIdent,
  pub params: Option<Vec<Param<BigIdent, Kind>>>,
  pub fields: Vec<Param<Ident, Type>>,
}

pub enum IdentPath {
  Ident(Ident),
  More(BigIdent, Ident),
}

pub enum Expr {
  String_(String),
  Number(u64),
  Tuple(Vec<Expr>),
  Struct(BigIdent, Option<Vec<TypeOrEffect>>, Vec<Expr>),
  Var(IdentPath),
  FnCall(IdentPath, Option<Vec<TypeOrEffect>>, Vec<Expr>),
  FieldGet(IdentPath, Ident),
  MethodCall(IdentPath, Ident, Option<Vec<TypeOrEffect>>, Vec<Expr>),
  Return(Box<Expr>),
  Match(Box<Expr>, Vec<Arm>),
  Block(Box<Block>),
}

pub enum TypeOrEffect {
  /// couldn't determine at parse time whether this was a type or effect
  BigIdent(BigIdent),
  /// the type inside will not be BigIdent
  Type(Type),
  /// the vec will have len >= 2
  Effect(Vec<BigIdent>),
}

pub struct Arm {
  pub pat: Pat,
  pub block: Block,
}

pub enum Pat {
  Wildcard,
  String_(String),
  Number(u64),
  Tuple(Vec<Pat>),
  Struct(BigIdent, Vec<FieldPat>),
  Enum(IdentPath, Box<Pat>),
  Ident(Ident),
  Or(Box<Pat>, Box<Pat>),
  TypeAnnotation(Box<Pat>, Type),
}

pub enum FieldPat {
  Ident(Ident),
  IdentPat(Ident, Pat),
}

pub struct Block {
  pub stmts: Vec<Stmt>,
  pub expr: Expr,
}

pub enum Stmt {
  Let(Pat, Expr),
  Do(Expr),
}
