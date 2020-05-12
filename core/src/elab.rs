//! Desugaring. AKA 77 lines just to get rid of MethodCall.

use crate::cst::{Arm, Block, Expr, Field, FnDefn, Stmt, TopDefn};

/// Does the conversion.
pub fn get(top_defns: Vec<TopDefn>) -> Vec<TopDefn> {
  top_defns.into_iter().map(get_top_defn).collect()
}

fn get_top_defn(top_defn: TopDefn) -> TopDefn {
  match top_defn {
    TopDefn::Struct(..) | TopDefn::Enum(..) => top_defn,
    TopDefn::Fn_(fn_) => TopDefn::Fn_(get_fn(*fn_).into()),
  }
}

fn get_fn(func: FnDefn) -> FnDefn {
  FnDefn {
    requires: func.requires.map(get_expr),
    ensures: func.ensures.map(get_expr),
    body: get_block(func.body),
    ..func
  }
}

fn get_expr(expr: Expr) -> Expr {
  match expr {
    Expr::String_(..) | Expr::Number(..) | Expr::Ident(..) => expr,
    Expr::Tuple(exprs) => Expr::Tuple(exprs.into_iter().map(get_expr).collect()),
    Expr::Struct(name, args, fields) => {
      Expr::Struct(name, args, fields.into_iter().map(get_field).collect())
    }
    Expr::FnCall(name, big_args, args) => {
      Expr::FnCall(name, big_args, args.into_iter().map(get_expr).collect())
    }
    Expr::FieldGet(expr, field) => Expr::FieldGet(get_expr(*expr).into(), field),
    Expr::MethodCall(recv, name, big_args, mut args) => {
      // the one thing we _actually_ do: get rid of MethodCall
      args.insert(0, *recv);
      Expr::FnCall(name, big_args, args.into_iter().map(get_expr).collect())
    }
    Expr::Match(head, arms) => Expr::Match(
      get_expr(*head).into(),
      arms.into_iter().map(get_arm).collect(),
    ),
    Expr::Block(blk) => Expr::Block(get_block(*blk).into()),
  }
}

fn get_block(block: Block) -> Block {
  Block {
    stmts: block.stmts.into_iter().map(get_stmt).collect(),
    expr: block.expr.map(get_expr),
  }
}

fn get_field(field: Field<Expr>) -> Field<Expr> {
  match field {
    Field::Ident(..) => field,
    Field::IdentAnd(id, expr) => Field::IdentAnd(id, get_expr(expr)),
  }
}

fn get_arm(arm: Arm) -> Arm {
  Arm {
    pat: arm.pat,
    block: get_block(arm.block),
  }
}

fn get_stmt(stmt: Stmt) -> Stmt {
  match stmt {
    Stmt::Let(pat, typ, expr) => Stmt::Let(pat, typ, get_expr(expr)),
  }
}
