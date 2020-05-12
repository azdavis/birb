//! The Birb standard library.

use crate::cst::{Block, EnumDefn, Expr, FnDefn, Kinded, Param, TopDefn};
use crate::ident::Ident;
use std::collections::HashSet;

/// The pre-defined top definitions.
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
    bin_op(ADD, NAT, NAT, NAT),
    bin_op(SUB, NAT, NAT, NAT),
    bin_op(MUL, NAT, NAT, NAT),
    bin_op(DIV, NAT, NAT, NAT),
  ]
}

fn bin_op(name: &str, lhs_type: &str, rhs_type: &str, ret_type: &str) -> TopDefn {
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
      expr: Some(Expr::Number(0)),
    },
  }))
}

/// The name of the built-in boolean type.
pub const BOOL: &str = "Bool";

/// The name of the built-in natural number type.
pub const NAT: &str = "Nat";

/// The name of the built-in string type.
pub const STR: &str = "Str";

/// The function 'add'.
pub const ADD: &str = "add";

/// The function 'sub'.
pub const SUB: &str = "sub";

/// The function 'mul'.
pub const MUL: &str = "mul";

/// The function 'div'.
pub const DIV: &str = "div";

/// The pre-defined effects.
pub fn effects() -> HashSet<Ident> {
  let mut ret = HashSet::new();
  ret.insert(Ident::new("Stdin"));
  ret.insert(Ident::new("Stdout"));
  ret.insert(Ident::new("Stderr"));
  ret.insert(Ident::new("Randomness"));
  ret
}
