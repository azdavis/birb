//! The Birb standard library.

use crate::cst::{EnumDefn, Kinded, Param, TopDefn};
use crate::ident::{BigIdent, Ident};

/// The prelude of pre-defined types and such.
pub fn prelude() -> Vec<TopDefn> {
  vec![
    TopDefn::Enum(EnumDefn {
      name: BigIdent::new("Bool"),
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
      name: BigIdent::new("Int"),
      params: vec![],
      ctors: vec![],
    }),
    // intrinsic
    TopDefn::Enum(EnumDefn {
      name: BigIdent::new("Str"),
      params: vec![],
      ctors: vec![],
    }),
  ]
}
