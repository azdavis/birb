use crate::cst::{
  Arm, Block, Effect, Expr, Field, Kind, Param, Pat, QualIdent, Stmt, TopDefn, Type, TypeOrEffect,
};
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
  vars: HashMap<Ident, Type>,
  kinds: HashSet<BigIdent>,
  effects: HashSet<BigIdent>,
}

struct TypeInfo {
  params: Vec<Param<BigIdent, Kind>>,
  def: Type,
}

struct StructInfo {
  params: Vec<Param<BigIdent, Kind>>,
  fields: Vec<Param<Ident, Type>>,
}

struct EnumInfo {
  params: Vec<Param<BigIdent, Kind>>,
  ctors: Vec<Param<Ident, Type>>,
}

struct FnInfo {
  big_params: Vec<Param<BigIdent, Kind>>,
  params: Vec<Param<Ident, Type>>,
  ret_type: Type,
}

fn ck_top_defn(mut cx: Cx, td: &TopDefn) -> Result<Cx> {
  match td {
    TopDefn::Struct(struct_) => {
      for p in struct_.params.iter() {
        cx.big_vars.insert(p.ident.clone(), p.type_.clone());
      }
      for f in struct_.fields.iter() {
        ck_type(&cx, &f.type_)?;
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

fn ck_type(cx: &Cx, t: &Type) -> Result<()> {
  match t {
    Type::BigIdent(bi, tes) => {
      // TODO
      let k = if let Some(ti) = cx.structs.get(bi) {
        mk_params_kind(&ti.params)
      } else if let Some(ti) = cx.enums.get(bi) {
        mk_params_kind(&ti.params)
      } else if let Some(k) = cx.big_vars.get(bi) {
        k.clone()
      } else {
        return Err(Error::UndefinedType(bi.clone()));
      };
      todo!()
    }
    Type::Tuple(ts) => {
      for t in ts {
        ck_type(cx, t)?;
      }
    }
    Type::Arrow(t1, t2) => {
      ck_type(cx, t1)?;
      ck_type(cx, t2)?;
    }
    Type::Effectful(t, e) => {
      ck_type(cx, t)?;
      ck_effect(cx, e)?;
    }
  }
  Ok(())
}

fn ck_effect(cx: &Cx, eff: &Effect) -> Result<()> {
  for e in eff.idents.iter() {
    let k = match cx.big_vars.get(e) {
      Some(k) => k,
      None => return Err(Error::UndefinedEffect(e.clone())),
    };
    if *k != Kind::Effect {
      return Err(Error::MismatchedEffectKinds(e.clone(), k.clone()));
    }
  }
  Ok(())
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
