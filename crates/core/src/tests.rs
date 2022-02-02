use crate::cst::{
  Arm, Block, EnumDefn, Expr, Field, FnDefn, Kind, Kinded, Param, Pat, Stmt, StructDefn, TopDefn,
};
use crate::error::Result;
use crate::ident::Ident;

fn get(bs: &[u8]) -> Result<Vec<TopDefn>> {
  let ts = crate::lex::get(bs)?;
  crate::parse::get(&ts)
}

#[test]
fn simple() {
  assert_eq!(
    get(include_bytes!("inputs/simple.txt")).unwrap(),
    vec![
      TopDefn::Struct(StructDefn {
        name: Ident::new("Unit"),
        params: vec![],
        fields: vec![]
      }),
      TopDefn::Enum(EnumDefn {
        name: Ident::new("Void"),
        params: vec![],
        ctors: vec![],
      }),
      TopDefn::Fn_(Box::new(FnDefn {
        name: Ident::new("main"),
        big_params: vec![],
        params: vec![],
        ret_type: Kinded::Ident(Ident::new("Str"), vec![]),
        requires: None,
        ensures: None,
        body: Block {
          stmts: vec![],
          expr: Some(Expr::String_(String::from("hello")))
        },
      })),
    ]
  );
}

#[test]
fn call() {
  assert_eq!(
    get(include_bytes!("inputs/call.txt")).unwrap(),
    vec![
      TopDefn::Struct(StructDefn {
        name: Ident::new("Guy"),
        params: vec![Param {
          ident: Ident::new("T"),
          type_: Kind::Type,
        }],
        fields: vec![Param {
          ident: Ident::new("x"),
          type_: Kinded::Ident(Ident::new("T"), vec![]),
        }]
      }),
      TopDefn::Fn_(Box::new(FnDefn {
        name: Ident::new("call"),
        big_params: vec![
          Param {
            ident: Ident::new("T"),
            type_: Kind::Type,
          },
          Param {
            ident: Ident::new("U"),
            type_: Kind::Type,
          },
          Param {
            ident: Ident::new("E"),
            type_: Kind::Effect,
          },
        ],
        params: vec![
          Param {
            ident: Ident::new("f"),
            type_: Kinded::Arrow(
              Kinded::Ident(Ident::new("T"), vec![]).into(),
              Kinded::Effectful(
                Kinded::Ident(Ident::new("U"), vec![]).into(),
                Kinded::Ident(Ident::new("E"), vec![]).into(),
              )
              .into()
            )
          },
          Param {
            ident: Ident::new("x"),
            type_: Kinded::Ident(Ident::new("T"), vec![]),
          }
        ],
        ret_type: Kinded::Effectful(
          Kinded::Ident(Ident::new("U"), vec![]).into(),
          Kinded::Ident(Ident::new("E"), vec![]).into(),
        ),
        requires: Some(Expr::Ident(Ident::new("true"))),
        ensures: Some(Expr::Ident(Ident::new("true"))),
        body: Block {
          stmts: vec![
            Stmt::Let(
              Pat::Wildcard,
              None,
              Expr::Struct(
                Ident::new("Guy"),
                vec![Kinded::Ident(Ident::new("T"), vec![])],
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
              Some(Kinded::Ident(
                Ident::new("Heh"),
                vec![
                  Kinded::Ident(Ident::new("Nah"), vec![]),
                  Kinded::Ident(Ident::new("Dude"), vec![]),
                ]
              )),
              Expr::Tuple(vec![])
            )
          ],
          expr: Some(Expr::MethodCall(
            Expr::Ident(Ident::new("x")).into(),
            Ident::new("f"),
            vec![],
            vec![],
          )),
        }
        .into(),
      }))
    ]
  );
}
