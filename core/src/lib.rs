//! An implementation of Birb.

#![deny(missing_docs)]

pub mod cst;
pub mod error;
pub mod ident;
pub mod interpret;
pub mod lex;
pub mod parse;
pub mod statics;
pub mod token;

mod util;

/// Turn a sequence of bytes into a statically-checked sequence of top-level definitions.
pub fn get(bs: &[u8]) -> error::Result<Vec<cst::TopDefn>> {
  let ts = lex::get(bs)?;
  let ret = parse::get(&ts)?;
  statics::get(&ret)?;
  Ok(ret)
}

#[cfg(test)]
mod tests {
  use crate::cst::{
    Arm, Block, EnumDefn, Expr, Field, FnDefn, Kind, Kinded, Param, Pat, QualIdent, Stmt,
    StructDefn, TopDefn,
  };
  use crate::get;
  use crate::ident::{BigIdent, Ident};

  #[test]
  fn simple() {
    assert_eq!(
      get(include_bytes!("inputs/simple.b")).unwrap(),
      vec![
        TopDefn::Struct(StructDefn {
          name: BigIdent::new("Unit"),
          params: vec![],
          fields: vec![]
        }),
        TopDefn::Enum(EnumDefn {
          name: BigIdent::new("Void"),
          params: vec![],
          ctors: vec![],
        }),
        TopDefn::Fn_(FnDefn {
          name: Ident::new("main"),
          big_params: vec![],
          params: vec![],
          ret_type: Kinded::BigIdent(BigIdent::new("String"), vec![]),
          requires: None,
          ensures: None,
          body: Block {
            stmts: vec![],
            expr: Some(Expr::String_(String::from("hello")))
          },
        }),
      ]
    );
  }

  #[test]
  fn call() {
    assert_eq!(
      get(include_bytes!("inputs/call.b")).unwrap(),
      vec![
        TopDefn::Struct(StructDefn {
          name: BigIdent::new("Guy"),
          params: vec![Param {
            ident: BigIdent::new("T"),
            type_: Kind::Type,
          }],
          fields: vec![Param {
            ident: Ident::new("x"),
            type_: Kinded::BigIdent(BigIdent::new("T"), vec![]),
          }]
        }),
        TopDefn::Fn_(FnDefn {
          name: Ident::new("call"),
          big_params: vec![
            Param {
              ident: BigIdent::new("T"),
              type_: Kind::Type,
            },
            Param {
              ident: BigIdent::new("U"),
              type_: Kind::Type,
            },
            Param {
              ident: BigIdent::new("E"),
              type_: Kind::Effect,
            },
          ],
          params: vec![
            Param {
              ident: Ident::new("f"),
              type_: Kinded::Arrow(
                Kinded::BigIdent(BigIdent::new("T"), vec![]).into(),
                Kinded::Effectful(
                  Kinded::BigIdent(BigIdent::new("U"), vec![]).into(),
                  Kinded::BigIdent(BigIdent::new("E"), vec![]).into(),
                )
                .into()
              )
            },
            Param {
              ident: Ident::new("x"),
              type_: Kinded::BigIdent(BigIdent::new("T"), vec![]),
            }
          ],
          ret_type: Kinded::Effectful(
            Kinded::BigIdent(BigIdent::new("U"), vec![]).into(),
            Kinded::BigIdent(BigIdent::new("E"), vec![]).into(),
          ),
          requires: Some(Expr::QualIdent(QualIdent::Ident(Ident::new("true")))),
          ensures: Some(Expr::QualIdent(QualIdent::Ident(Ident::new("true")))),
          body: Block {
            stmts: vec![
              Stmt::Let(
                Pat::Wildcard,
                None,
                Expr::Struct(
                  BigIdent::new("Guy"),
                  vec![Kinded::BigIdent(BigIdent::new("T"), vec![])],
                  vec![Field::Ident(Ident::new("x"))],
                )
              ),
              Stmt::Let(
                Pat::Wildcard,
                None,
                Expr::Match(
                  Expr::Tuple(vec![]).into(),
                  vec![Arm {
                    pat: Pat::Tuple(vec![]),
                    block: Block {
                      stmts: vec![],
                      expr: Some(Expr::Tuple(vec![]))
                    }
                  }]
                )
              ),
              Stmt::Let(
                Pat::Wildcard,
                Some(Kinded::BigIdent(
                  BigIdent::new("Heh"),
                  vec![
                    Kinded::BigIdent(BigIdent::new("Nah"), vec![]),
                    Kinded::BigIdent(BigIdent::new("Dude"), vec![]),
                  ]
                )),
                Expr::Tuple(vec![])
              )
            ],
            expr: Some(Expr::MethodCall(
              Expr::QualIdent(QualIdent::Ident(Ident::new("x"))).into(),
              Ident::new("f"),
              vec![],
              vec![],
            )),
          }
          .into(),
        })
      ]
    );
  }
}
