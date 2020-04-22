pub mod cst;
pub mod error;
pub mod ident;
pub mod lex;
pub mod parse;
pub mod statics;
pub mod token;

mod util;

pub fn get(bs: &[u8]) -> error::Result<Vec<cst::TopDefn>> {
  let ts = lex::get(bs)?;
  parse::get(&ts)
}

#[cfg(test)]
mod tests {
  use crate::cst::{
    Arm, Block, Effect, EnumDefn, Expr, Field, FnDefn, Kind, Param, Pat, QualIdent, Stmt,
    StructDefn, TopDefn, Type, TypeOrEffect,
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
          params: Vec::new(),
          fields: Vec::new()
        }),
        TopDefn::Enum(EnumDefn {
          name: BigIdent::new("Void"),
          params: Vec::new(),
          ctors: Vec::new(),
        }),
        TopDefn::Fn_(FnDefn {
          name: Ident::new("main"),
          big_params: Vec::new(),
          params: Vec::new(),
          ret_type: Type::BigIdent(BigIdent::new("String"), Vec::new()),
          requires: None,
          ensures: None,
          body: Block {
            stmts: Vec::new(),
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
            type_: Kind::BigIdent(BigIdent::new("Type")),
          }],
          fields: vec![Param {
            ident: Ident::new("x"),
            type_: Type::BigIdent(BigIdent::new("T"), Vec::new()),
          }]
        }),
        TopDefn::Fn_(FnDefn {
          name: Ident::new("call"),
          big_params: vec![
            Param {
              ident: BigIdent::new("T"),
              type_: Kind::BigIdent(BigIdent::new("Type"))
            },
            Param {
              ident: BigIdent::new("U"),
              type_: Kind::BigIdent(BigIdent::new("Type"))
            },
            Param {
              ident: BigIdent::new("E"),
              type_: Kind::BigIdent(BigIdent::new("Effect"))
            },
          ],
          params: vec![
            Param {
              ident: Ident::new("f"),
              type_: Type::Arrow(
                Type::BigIdent(BigIdent::new("T"), Vec::new()).into(),
                Type::Effectful(
                  Type::BigIdent(BigIdent::new("U"), Vec::new()).into(),
                  Effect {
                    idents: vec![BigIdent::new("E")]
                  }
                )
                .into()
              )
            },
            Param {
              ident: Ident::new("x"),
              type_: Type::BigIdent(BigIdent::new("T"), Vec::new()),
            }
          ],
          ret_type: Type::Effectful(
            Type::BigIdent(BigIdent::new("U"), Vec::new()).into(),
            Effect {
              idents: vec![BigIdent::new("E")]
            }
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
                  vec![TypeOrEffect::Type(Type::BigIdent(
                    BigIdent::new("T"),
                    Vec::new()
                  ))],
                  vec![Field::Ident(Ident::new("x"))],
                )
              ),
              Stmt::Let(
                Pat::Wildcard,
                None,
                Expr::Match(
                  Expr::Tuple(Vec::new()).into(),
                  vec![Arm {
                    pat: Pat::Tuple(Vec::new()),
                    block: Block {
                      stmts: Vec::new(),
                      expr: Some(Expr::Tuple(Vec::new()))
                    }
                  }]
                )
              )
            ],
            expr: Some(Expr::MethodCall(
              Expr::QualIdent(QualIdent::Ident(Ident::new("x"))).into(),
              Ident::new("f"),
              Vec::new(),
              Vec::new(),
            )),
          }
          .into(),
        })
      ]
    );
  }
}
