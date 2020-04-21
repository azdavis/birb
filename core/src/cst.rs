use crate::ident::{BigIdent, Ident};

pub enum TopDefn {
  Type(TypeDefn),
  Struct(StructDefn),
  Enum(EnumDefn),
  Fn_(FnDefn),
}

pub struct Param<I, T> {
  ident: I,
  /// maybe not type, but instead kind.
  type_: T,
}

pub struct TypeDefn {
  name: BigIdent,
  params: Vec<Param<BigIdent, Kind>>,
  def: Type,
}

pub struct StructDefn {
  name: BigIdent,
  params: Vec<Param<BigIdent, Kind>>,
  fields: Vec<Param<Ident, Type>>,
}

pub struct EnumDefn {
  name: BigIdent,
  params: Vec<Param<BigIdent, Kind>>,
  cases: Vec<Param<Ident, Type>>,
}

pub struct FnDefn {
  name: Ident,
  big_params: Vec<Param<BigIdent, Kind>>,
  params: Vec<Param<Ident, Type>>,
  ret_type: Type,
  requires: Option<Expr>,
  ensures: Option<Expr>,
  body: Expr,
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
  name: BigIdent,
  params: Option<Vec<Param<BigIdent, Kind>>>,
  fields: Vec<Param<Ident, Type>>,
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
  pat: Pat,
  block: Block,
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
  stmts: Vec<Stmt>,
  expr: Expr,
}

pub enum Stmt {
  Let(Pat, Expr),
  Do(Expr),
}
