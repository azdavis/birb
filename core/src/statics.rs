use crate::cst::{Arm, Block, Expr, Field, Kind, Kinded, Param, Pat, QualIdent, Stmt, TopDefn};
use crate::error::{Error, Result};
use crate::ident::{BigIdent, Ident};
use std::collections::{HashMap, HashSet};

pub fn get(top_defns: &[TopDefn]) -> Result<()> {
  let mut cx = Cx::default();
  cx.effects.insert(BigIdent::new("Stdout"));
  Ok(())
}

#[derive(Default)]
struct Cx {
  structs: HashMap<BigIdent, StructInfo>,
  enums: HashMap<BigIdent, EnumInfo>,
  fns: HashMap<Ident, FnInfo>,
  big_vars: HashMap<BigIdent, Kind>,
  vars: HashMap<Ident, Kinded>,
  effects: HashSet<BigIdent>,
}

struct StructInfo {
  params: Vec<Param<BigIdent, Kind>>,
  fields: Vec<Param<Ident, Kinded>>,
}

struct EnumInfo {
  params: Vec<Param<BigIdent, Kind>>,
  ctors: Vec<Param<Ident, Kinded>>,
}

struct FnInfo {
  big_params: Vec<Param<BigIdent, Kind>>,
  params: Vec<Param<Ident, Kinded>>,
  ret_type: Kinded,
}

fn ck_top_defn(mut cx: Cx, td: &TopDefn) -> Result<Cx> {
  match td {
    TopDefn::Struct(struct_) => {
      for p in struct_.params.iter() {
        cx.big_vars.insert(p.ident.clone(), p.type_.clone());
      }
      for f in struct_.fields.iter() {
        ck_kinded(&cx, &f.type_)?;
      }
      for p in struct_.params.iter() {
        cx.big_vars.remove(&p.ident).unwrap();
      }
      cx.structs.insert(
        struct_.name.clone(),
        StructInfo {
          params: struct_.params.clone(),
          fields: struct_.fields.clone(),
        },
      );
      Ok(cx)
    }
    TopDefn::Enum(enum_) => todo!(),
    TopDefn::Fn_(fn_) => todo!(),
  }
}

fn ck_kinded(cx: &Cx, t: &Kinded) -> Result<Kind> {
  match t {
    Kinded::BigIdent(bi, tes) => todo!(),
    Kinded::Tuple(ts) => {
      for t in ts {
        ck_kinded(cx, t)?;
      }
    }
    Kinded::Arrow(t1, t2) => {
      ck_kinded(cx, t1)?;
      ck_kinded(cx, t2)?;
    }
    Kinded::Effectful(t, e) => {
      ck_kinded(cx, t)?;
      ck_kinded(cx, e)?;
    }
  }
  todo!()
}

fn mk_params_kind(params: &[Param<BigIdent, Kind>]) -> Kind {
  if params.is_empty() {
    Kind::Type
  } else if params.len() == 1 {
    Kind::Arrow(params[0].type_.clone().into(), Kind::Type.into())
  } else {
    Kind::Arrow(
      Kind::Tuple(params.iter().map(|p| p.type_.clone()).collect()).into(),
      Kind::Type.into(),
    )
  }
}
