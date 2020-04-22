use crate::cst::{
  Arm, Block, Effect, Expr, Field, Kind, Param, Pat, QualIdent, Stmt, TopDefn, Type, TypeOrEffect,
};
use crate::error::{Error, Result};
use crate::ident::{BigIdent, Ident};
use std::collections::{HashMap, HashSet};

pub fn get(top_defns: &[TopDefn]) -> Result<()> {
  let mut cx = Cx::default();
  cx.kinds.insert(type_ident());
  cx.kinds.insert(effect_ident());
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
        ck_kind(&cx, &p.type_)?;
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

fn ck_kind(cx: &Cx, k: &Kind) -> Result<()> {
  match k {
    Kind::BigIdent(bi) => {
      if cx.kinds.contains(bi) {
        Ok(())
      } else {
        Err(Error::UndefinedKind(bi.clone()))
      }
    }
    Kind::Arrow(k1, k2) => {
      ck_kind(cx, k1)?;
      ck_kind(cx, k2)?;
      Ok(())
    }
    Kind::Tuple(ks) => {
      for k in ks {
        ck_kind(cx, k)?;
      }
      Ok(())
    }
  }
}

fn ck_type(cx: &Cx, t: &Type) -> Result<Kind> {
  let tk = type_kind();
  match t {
    Type::BigIdent(bi) => {
      if let Some(ti) = cx.structs.get(bi) {
        return Ok(mk_arrow_kind(&ti.params));
      }
      if let Some(ti) = cx.enums.get(bi) {
        return Ok(mk_arrow_kind(&ti.params));
      }
      if let Some(k) = cx.big_vars.get(bi) {
        return Ok(k.clone());
      }
      Err(Error::UndefinedType(bi.clone()))
    }
    Type::Tuple(ts) => {
      for t in ts {
        let k = ck_type(cx, t)?;
        if k != tk {
          return Err(Error::MismatchedTypeKinds(t.clone(), k));
        }
      }
      Ok(tk)
    }
    Type::Arrow(t1, t2) => {
      let k1 = ck_type(cx, t1)?;
      if k1 != tk {
        return Err(Error::MismatchedTypeKinds((**t1).clone(), k1));
      }
      let k2 = ck_type(cx, t2)?;
      if k2 != tk {
        return Err(Error::MismatchedTypeKinds((**t2).clone(), k2));
      }
      Ok(tk)
    }
    Type::Effectful(t, e) => {
      let kt = ck_type(cx, t)?;
      if kt != tk {
        return Err(Error::MismatchedTypeKinds((**t).clone(), kt));
      }
      ck_effect(cx, e)?;
      Ok(tk)
    }
  }
}

fn ck_effect(cx: &Cx, eff: &Effect) -> Result<()> {
  let ek = effect_kind();
  for e in eff.idents.iter() {
    let k = match cx.big_vars.get(e) {
      Some(k) => k,
      None => return Err(Error::UndefinedEffect(e.clone())),
    };
    if *k != ek {
      return Err(Error::MismatchedEffectKinds(e.clone(), k.clone()));
    }
  }
  Ok(())
}

fn mk_arrow_kind(params: &[Param<BigIdent, Kind>]) -> Kind {
  Kind::Arrow(
    Kind::Tuple(params.iter().map(|p| p.type_.clone()).collect()).into(),
    type_kind().into(),
  )
}

fn type_kind() -> Kind {
  Kind::BigIdent(type_ident())
}

fn type_ident() -> BigIdent {
  BigIdent::new("Type")
}

fn effect_kind() -> Kind {
  Kind::BigIdent(effect_ident())
}

fn effect_ident() -> BigIdent {
  BigIdent::new("Effect")
}
