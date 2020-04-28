//! The Birb standard library.

use crate::cst::{EnumDefn, Kinded, Param, TopDefn};
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
    // intrinsic
    TopDefn::Enum(EnumDefn {
      name: Ident::new(NAT),
      params: vec![],
      ctors: vec![],
    }),
    // intrinsic
    TopDefn::Enum(EnumDefn {
      name: Ident::new(STR),
      params: vec![],
      ctors: vec![],
    }),
  ]
}

/// The name of the built-in boolean type.
pub const BOOL: &str = "Bool";

/// The name of the built-in natural number type.
pub const NAT: &str = "Nat";

/// The name of the built-in string type.
pub const STR: &str = "Str";

/// The pre-defined effects.
pub fn effects() -> HashSet<Ident> {
  let mut ret = HashSet::new();
  ret.insert(Ident::new("Stdout"));
  ret
}
