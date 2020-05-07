//! The Birb standard library.

use crate::cst::{Block, EnumDefn, FnDefn, Kinded, Param, TopDefn};
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
    math_bin_op(ADD),
    math_bin_op(SUB),
    math_bin_op(MUL),
    math_bin_op(DIV),
  ]
}

fn math_bin_op(name: &str) -> TopDefn {
  TopDefn::Fn_(Box::new(FnDefn {
    name: Ident::new(name),
    big_params: vec![],
    params: vec![
      Param {
        ident: Ident::new("x"),
        type_: Kinded::Ident(Ident::new(NAT), vec![]),
      },
      Param {
        ident: Ident::new("y"),
        type_: Kinded::Ident(Ident::new(NAT), vec![]),
      },
    ],
    ret_type: Kinded::Ident(Ident::new(NAT), vec![]),
    requires: None,
    ensures: None,
    body: Block {
      stmts: vec![],
      expr: None,
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
