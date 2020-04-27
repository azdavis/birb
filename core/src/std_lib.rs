//! The Birb standard library.

use crate::cst::{EnumDefn, Kinded, Param, TopDefn};
use crate::ident::Ident;
use std::collections::HashSet;

/// The prelude of pre-defined types and such.
pub fn prelude() -> Vec<TopDefn> {
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
      name: Ident::new(INT),
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

/// The name of the built-in integer type.
pub const INT: &str = "Int";

/// The name of the built-in string type.
pub const STR: &str = "Str";

/// The effects.
pub fn effects() -> HashSet<Ident> {
  let mut ret = HashSet::new();
  ret.insert(Ident::new("Stdout"));
  ret
}
