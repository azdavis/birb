//! Static verification.

use crate::cst::{Arm, Block, Expr, Field, Kind, Kinded, Param, Pat, Stmt, TopDefn};
use crate::error::{Error, Result};
use crate::ident::Ident;
use crate::std_lib::{effects, BOOL, INT, STR};
use std::collections::{HashMap, HashSet};

/// Checks whether the sequence of top-level definitions is statically well-formed.
pub fn get(top_defns: &[TopDefn]) -> Result<()> {
  let mut cx = Cx::default();
  let mut var_cx = VarCx::default();
  cx.effects = effects();
  for td in top_defns {
    ck_top_defn(&mut cx, &mut var_cx, td)?;
    assert!(var_cx.big_vars.is_empty());
    assert!(var_cx.vars.is_empty());
  }
  Ok(())
}

#[derive(Default)]
struct Cx {
  structs: HashMap<Ident, StructInfo>,
  enums: HashMap<Ident, EnumInfo>,
  fns: HashMap<Ident, FnInfo>,
  effects: HashSet<Ident>,
}

#[derive(Default)]
struct VarCx {
  big_vars: HashMap<Ident, Kind>,
  vars: HashMap<Ident, Kinded>,
}

#[derive(Clone)]
struct StructInfo {
  params: Vec<Param<Ident, Kind>>,
  fields: HashMap<Ident, Kinded>,
}

struct EnumInfo {
  params: Vec<Param<Ident, Kind>>,
  ctors: HashMap<Ident, Kinded>,
}

#[derive(Clone)]
struct FnInfo {
  big_params: Vec<Param<Ident, Kind>>,
  params: Vec<Param<Ident, Kinded>>,
  ret_type: Kinded,
}

fn ck_ident(cx: &Cx, id: &Ident) -> Result<()> {
  if cx.fns.contains_key(id) || cx.enums.iter().any(|(_, info)| info.ctors.contains_key(id)) {
    Err(Error::DuplicateIdentifier(id.clone()))
  } else {
    Ok(())
  }
}

fn ck_big_ident(cx: &Cx, bi: &Ident) -> Result<()> {
  if cx.structs.contains_key(bi) || cx.enums.contains_key(bi) || cx.effects.contains(bi) {
    Err(Error::DuplicateIdentifier(bi.clone()))
  } else {
    Ok(())
  }
}

fn ck_top_defn(cx: &mut Cx, var_cx: &mut VarCx, td: &TopDefn) -> Result<()> {
  match td {
    TopDefn::Struct(struct_) => {
      ck_big_ident(&cx, &struct_.name)?;
      for p in struct_.params.iter() {
        if var_cx
          .big_vars
          .insert(p.ident.clone(), p.type_.clone())
          .is_some()
        {
          return Err(Error::DuplicateIdentifier(p.ident.clone()));
        }
      }
      let mut fields = HashMap::with_capacity(struct_.fields.len());
      for p in struct_.fields.iter() {
        ck_has_kind(&cx, &var_cx, &p.type_, Kind::Type)?;
        if fields.insert(p.ident.clone(), p.type_.clone()).is_some() {
          return Err(Error::DuplicateField(struct_.name.clone(), p.ident.clone()));
        }
      }
      for p in struct_.params.iter() {
        var_cx.big_vars.remove(&p.ident).unwrap();
      }
      cx.structs.insert(
        struct_.name.clone(),
        StructInfo {
          params: struct_.params.clone(),
          fields,
        },
      );
    }
    TopDefn::Enum(enum_) => {
      ck_big_ident(&cx, &enum_.name)?;
      for p in enum_.params.iter() {
        if var_cx
          .big_vars
          .insert(p.ident.clone(), p.type_.clone())
          .is_some()
        {
          return Err(Error::DuplicateIdentifier(p.ident.clone()));
        }
      }
      let mut ctors = HashMap::with_capacity(enum_.ctors.len());
      for p in enum_.ctors.iter() {
        ck_ident(&cx, &p.ident)?;
        ck_has_kind(&cx, &var_cx, &p.type_, Kind::Type)?;
        ctors.insert(p.ident.clone(), p.type_.clone());
      }
      for p in enum_.params.iter() {
        var_cx.big_vars.remove(&p.ident).unwrap();
      }
      cx.enums.insert(
        enum_.name.clone(),
        EnumInfo {
          params: enum_.params.clone(),
          ctors,
        },
      );
    }
    TopDefn::Fn_(fn_) => {
      ck_ident(&cx, &fn_.name)?;
      for p in fn_.big_params.iter() {
        if var_cx
          .big_vars
          .insert(p.ident.clone(), p.type_.clone())
          .is_some()
        {
          return Err(Error::DuplicateIdentifier(p.ident.clone()));
        }
      }
      for p in fn_.params.iter() {
        ck_has_kind(&cx, &var_cx, &p.type_, Kind::Type)?;
        if var_cx
          .vars
          .insert(p.ident.clone(), p.type_.clone())
          .is_some()
        {
          return Err(Error::DuplicateIdentifier(p.ident.clone()));
        }
      }
      ck_has_kind(&cx, &var_cx, &fn_.ret_type, Kind::Type)?;
      // TODO check the requires, ensures, and body
      for p in fn_.big_params.iter() {
        var_cx.big_vars.remove(&p.ident).unwrap();
      }
      for p in fn_.params.iter() {
        var_cx.vars.remove(&p.ident).unwrap();
      }
      cx.fns.insert(
        fn_.name.clone(),
        FnInfo {
          big_params: fn_.big_params.clone(),
          params: fn_.params.clone(),
          ret_type: fn_.ret_type.clone(),
        },
      );
    }
  }
  Ok(())
}

fn get_kind(cx: &Cx, var_cx: &VarCx, kinded: &Kinded) -> Result<Kind> {
  match kinded {
    Kinded::Ident(bi, args) => {
      let k = if let Some(si) = cx.structs.get(bi) {
        mk_params_kind(&si.params)
      } else if let Some(ei) = cx.enums.get(bi) {
        mk_params_kind(&ei.params)
      } else if let Some(k) = var_cx.big_vars.get(bi) {
        k.clone()
      } else {
        return Err(Error::UndefinedIdentifier(bi.clone()));
      };
      if args.is_empty() {
        return Ok(k);
      }
      let (param, res) = if let Kind::Arrow(param, res) = k {
        (*param, *res)
      } else {
        return Err(Error::InvalidKindedApp(bi.clone(), k));
      };
      let mut arg_kinds = Vec::with_capacity(args.len());
      for arg in args {
        arg_kinds.push(get_kind(cx, var_cx, arg)?);
      }
      let arg_kind = if arg_kinds.len() == 1 {
        arg_kinds.pop().unwrap()
      } else {
        Kind::Tuple(arg_kinds)
      };
      if param == arg_kind {
        Ok(res)
      } else {
        // not the best error message. whatever.
        Err(Error::MismatchedKinds(param, arg_kind))
      }
    }
    Kinded::Tuple(ts) => {
      for t in ts {
        ck_has_kind(cx, var_cx, t, Kind::Type)?;
      }
      Ok(Kind::Type)
    }
    Kinded::Set(es) => {
      for e in es {
        ck_has_kind(cx, var_cx, e, Kind::Effect)?;
      }
      Ok(Kind::Effect)
    }
    Kinded::Arrow(t1, t2) => {
      ck_has_kind(cx, var_cx, t1, Kind::Type)?;
      ck_has_kind(cx, var_cx, t2, Kind::Type)?;
      Ok(Kind::Type)
    }
    Kinded::Effectful(t, e) => {
      ck_has_kind(cx, var_cx, t, Kind::Type)?;
      ck_has_kind(cx, var_cx, e, Kind::Effect)?;
      Ok(Kind::Type)
    }
  }
}

fn ck_has_kind(cx: &Cx, var_cx: &VarCx, kinded: &Kinded, want: Kind) -> Result<()> {
  let got = get_kind(cx, var_cx, kinded)?;
  if want == got {
    Ok(())
  } else {
    Err(Error::MismatchedKinds(want, got))
  }
}

fn mk_params_kind(params: &[Param<Ident, Kind>]) -> Kind {
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

fn get_expr_type(cx: &Cx, var_cx: &VarCx, expr: &Expr) -> Result<Kinded> {
  match expr {
    Expr::String_(_) => Ok(Kinded::Ident(Ident::new(STR), vec![])),
    Expr::Number(_) => Ok(Kinded::Ident(Ident::new(INT), vec![])),
    Expr::Tuple(es) => {
      let mut ts = Vec::with_capacity(es.len());
      for e in es {
        ts.push(get_expr_type(cx, var_cx, e)?);
      }
      Ok(Kinded::Tuple(ts))
    }
    Expr::Struct(name, args, fields) => {
      let info = match cx.structs.get(name) {
        Some(x) => x.clone(),
        None => return Err(Error::UndefinedIdentifier(name.clone())),
      };
      if info.params.len() != args.len() {
        return Err(Error::WrongNumArgs(
          name.clone(),
          info.params.len(),
          args.len(),
        ));
      }
      for (p, a) in info.params.iter().zip(args) {
        ck_has_kind(&cx, &var_cx, a, p.type_.clone())?;
      }
      let mut fields_seen = HashSet::with_capacity(info.fields.len());
      for f in fields {
        let (x, got) = match f {
          Field::Ident(x) => (x, get_expr_type(cx, var_cx, &Expr::Ident(x.clone()))?),
          Field::IdentAnd(x, e) => (x, get_expr_type(cx, var_cx, e)?),
        };
        let want = match info.fields.get(x) {
          None => return Err(Error::NoSuchField(name.clone(), x.clone())),
          Some(t) => t,
        };
        if *want != got {
          return Err(Error::MismatchedTypes(want.clone(), got));
        }
        if !fields_seen.insert(x) {
          return Err(Error::DuplicateField(name.clone(), x.clone()));
        }
      }
      Ok(Kinded::Ident(name.clone(), args.clone()))
    }
    Expr::Ident(name) => {
      if let Some(t) = var_cx.vars.get(name) {
        return Ok(t.clone());
      }
      // TODO we currently forbid bare function and constructor names
      Err(Error::UndefinedIdentifier(name.clone()))
    }
    Expr::FnCall(name, big_args, args) => {
      let info = if let Some(info) = cx.fns.get(name) {
        info.clone()
      } else if let Some(info) = cx.enums.iter().find_map(|(enum_name, enum_info)| {
        enum_info.ctors.get(name).map(|type_| FnInfo {
          big_params: enum_info.params.clone(),
          params: vec![Param {
            ident: Ident::new("_"),
            type_: type_.clone(),
          }],
          ret_type: Kinded::Ident(enum_name.clone(), vec![]),
        })
      }) {
        info
      } else {
        return Err(Error::UndefinedIdentifier(name.clone()));
      };
      if info.big_params.len() != big_args.len() {
        return Err(Error::WrongNumArgs(
          name.clone(),
          info.big_params.len(),
          big_args.len(),
        ));
      }
      let mut big_vars = HashMap::with_capacity(big_args.len());
      for (p, a) in info.big_params.iter().zip(big_args) {
        ck_has_kind(&cx, var_cx, a, p.type_.clone())?;
        ck_big_ident(&cx, &p.ident)?;
        big_vars.insert(p.ident.clone(), a.clone());
      }
      if info.params.len() != args.len() {
        return Err(Error::WrongNumArgs(
          name.clone(),
          info.params.len(),
          args.len(),
        ));
      }
      // TODO handle generics
      assert!(info.big_params.is_empty());
      todo!()
    }
    Expr::FieldGet(struct_, name) => {
      let type_ = get_expr_type(cx, var_cx, struct_)?;
      let (bi, ks) = if let Kinded::Ident(bi, ks) = type_ {
        (bi, ks)
      } else {
        return Err(Error::NotStruct(name.clone()));
      };
      todo!()
    }
    Expr::MethodCall(receiver, name, big_args, args) => todo!(),
    Expr::Return(expr) => todo!(),
    Expr::Match(expr, arms) => todo!(),
    Expr::Block(block) => todo!(),
  }
}
