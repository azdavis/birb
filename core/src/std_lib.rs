//! The Birb standard library.

#![allow(missing_docs)]

use crate::cst::{Block, EnumDefn, Expr, FnDefn, Kinded, Param, TopDefn};
use crate::ident::Ident;
use std::collections::HashSet;

/// The pre-defined top definitions. This isn't the prettiest way to do this.
pub fn top_defns() -> Vec<TopDefn> {
  vec![
    TopDefn::Enum(EnumDefn {
      name: Ident::new(BOOL),
      params: vec![],
      ctors: vec![
        Param {
          ident: Ident::new("true"),
          type_: Kinded::Tuple(vec![]),
        },
        Param {
          ident: Ident::new("false"),
          type_: Kinded::Tuple(vec![]),
        },
      ],
    }),
    TopDefn::Enum(EnumDefn {
      name: Ident::new(NAT),
      params: vec![],
      ctors: vec![],
    }),
    TopDefn::Enum(EnumDefn {
      name: Ident::new(STR),
      params: vec![],
      ctors: vec![],
    }),
    bin_op(ADD, NAT, NAT, NAT, Expr::Number(0)),
    bin_op(SUB, NAT, NAT, NAT, Expr::Number(0)),
    bin_op(MUL, NAT, NAT, NAT, Expr::Number(0)),
    bin_op(DIV, NAT, NAT, NAT, Expr::Number(0)),
    bin_op(EQ, NAT, NAT, BOOL, fake_bool()),
    bin_op(LT, NAT, NAT, BOOL, fake_bool()),
    bin_op(GT, NAT, NAT, BOOL, fake_bool()),
    bin_op(AND, BOOL, BOOL, BOOL, fake_bool()),
    bin_op(OR, BOOL, BOOL, BOOL, fake_bool()),
  ]
}

fn fake_bool() -> Expr {
  Expr::FnCall(Ident::new("true"), vec![], vec![Expr::Tuple(vec![])])
}

fn bin_op(name: &str, lhs_type: &str, rhs_type: &str, ret_type: &str, ret_val: Expr) -> TopDefn {
  TopDefn::Fn_(Box::new(FnDefn {
    name: Ident::new(name),
    big_params: vec![],
    params: vec![
      Param {
        ident: Ident::new("lhs"),
        type_: Kinded::Ident(Ident::new(lhs_type), vec![]),
      },
      Param {
        ident: Ident::new("rhs"),
        type_: Kinded::Ident(Ident::new(rhs_type), vec![]),
      },
    ],
    ret_type: Kinded::Ident(Ident::new(ret_type), vec![]),
    requires: None,
    ensures: None,
    body: Block {
      stmts: vec![],
      expr: Some(ret_val),
    },
  }))
}

pub const BOOL: &str = "Bool";
pub const NAT: &str = "Nat";
pub const STR: &str = "Str";
pub const ADD: &str = "add";
pub const SUB: &str = "sub";
pub const MUL: &str = "mul";
pub const DIV: &str = "div";
pub const EQ: &str = "eq";
pub const LT: &str = "lt";
pub const GT: &str = "gt";
pub const AND: &str = "and";
pub const OR: &str = "or";

/// The pre-defined effects.
pub fn effects() -> HashSet<Ident> {
  let mut ret = HashSet::new();
  ret.insert(Ident::new("Stdin"));
  ret.insert(Ident::new("Stdout"));
  ret.insert(Ident::new("Stderr"));
  ret.insert(Ident::new("Randomness"));
  ret
}
