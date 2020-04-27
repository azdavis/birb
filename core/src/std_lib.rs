//! The Birb standard library.

use crate::cst::{EnumDefn, Kinded, Param, TopDefn};
use crate::ident::{BigIdent, Ident};

/// The prelude of pre-defined types and such.
pub fn prelude() -> Vec<TopDefn> {
  vec![
    TopDefn::Enum(EnumDefn {
      name: BigIdent::new(BOOL),
      params: vec![],
      ctors: vec![
        Param {
          ident: Ident::new(TRUE),
          type_: Kinded::Tuple(vec![]),
        },
        Param {
          ident: Ident::new(FALSE),
          type_: Kinded::Tuple(vec![]),
        },
      ],
    }),
    // intrinsic
    TopDefn::Enum(EnumDefn {
      name: BigIdent::new(INT),
      params: vec![],
      ctors: vec![],
    }),
    // intrinsic
    TopDefn::Enum(EnumDefn {
      name: BigIdent::new(STR),
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

/// The name of the built-in true value, one of the boolean values.
pub const TRUE: &str = "true";

/// The name of the built-in false value, the other boolean value.
pub const FALSE: &str = "false";
