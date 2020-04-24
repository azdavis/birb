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
        ck_has_kind(&cx, &f.type_, Kind::Type)?;
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

fn get_kind(cx: &Cx, kinded: &Kinded) -> Result<Kind> {
  match kinded {
    Kinded::BigIdent(bi, args) => {
      let k = if let Some(si) = cx.structs.get(bi) {
        mk_params_kind(&si.params)
      } else if let Some(ei) = cx.enums.get(bi) {
        mk_params_kind(&ei.params)
      } else if let Some(k) = cx.big_vars.get(bi) {
        k.clone()
      } else {
        return Err(Error::UndefinedKind(bi.clone()));
      };
      if args.is_empty() {
        return Ok(k);
      }
      let (param, res) = if let Kind::Arrow(param, res) = k {
        (*param, *res)
      } else {
        return Err(Error::InvalidKindApp(bi.clone(), k));
      };
      // let arg = if args.len() == 1 { args[0].clone()}
      todo!()
    }
    Kinded::Tuple(ts) => {
      for t in ts {
        ck_has_kind(cx, t, Kind::Type)?;
      }
      Ok(Kind::Type)
    }
    Kinded::Set(es) => {
      for e in es {
        ck_has_kind(cx, e, Kind::Effect)?;
      }
      Ok(Kind::Effect)
    }
    Kinded::Arrow(t1, t2) => {
      ck_has_kind(cx, t1, Kind::Type)?;
      ck_has_kind(cx, t2, Kind::Type)?;
      Ok(Kind::Type)
    }
    Kinded::Effectful(t, e) => {
      ck_has_kind(cx, t, Kind::Type)?;
      ck_has_kind(cx, e, Kind::Effect)?;
      Ok(Kind::Type)
    }
  }
}

fn ck_has_kind(cx: &Cx, t: &Kinded, want: Kind) -> Result<()> {
  let got = get_kind(cx, t)?;
  if want == got {
    Ok(())
  } else {
    Err(Error::MismatchedKinds(t.clone(), want, got))
  }
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
