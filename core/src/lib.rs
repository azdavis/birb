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
          ret_type: Type::BigIdent(BigIdent::new("String"), vec![]),
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
            type_: Kind::BigIdent(BigIdent::new("Type")),
          }],
          fields: vec![Param {
            ident: Ident::new("x"),
            type_: Type::BigIdent(BigIdent::new("T"), vec![]),
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
                Type::BigIdent(BigIdent::new("T"), vec![]).into(),
                Type::Effectful(
                  Type::BigIdent(BigIdent::new("U"), vec![]).into(),
                  Effect {
                    idents: vec![BigIdent::new("E")]
                  }
                )
                .into()
              )
            },
            Param {
              ident: Ident::new("x"),
              type_: Type::BigIdent(BigIdent::new("T"), vec![]),
            }
          ],
          ret_type: Type::Effectful(
            Type::BigIdent(BigIdent::new("U"), vec![]).into(),
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
                    vec![]
                  ))],
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
                Some(Type::BigIdent(
                  BigIdent::new("Heh"),
                  vec![
                    TypeOrEffect::Type(Type::BigIdent(BigIdent::new("Nah"), vec![])),
                    TypeOrEffect::Effect(Effect {
                      idents: vec![BigIdent::new("Dude")]
                    })
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
