//! Static verification.

use crate::cst::{Arm, Block, Expr, Field, Kind, Kinded, Param, Pat, Stmt, TopDefn};
use crate::error::{Error, Result};
use crate::ident::{BigIdent, Ident};
use crate::std_lib::{BOOL, INT, STR};
use std::collections::{HashMap, HashSet};

/// Checks whether the sequence of top-level definitions is statically well-formed.
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

#[derive(Clone)]
struct StructInfo {
  params: Vec<Param<BigIdent, Kind>>,
  fields: HashMap<Ident, Kinded>,
}

struct EnumInfo {
  params: Vec<Param<BigIdent, Kind>>,
  ctors: HashMap<Ident, Kinded>,
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
      let mut fields = HashMap::with_capacity(struct_.fields.len());
      for p in struct_.fields.iter() {
        ck_has_kind(&cx, &p.type_, Kind::Type)?;
        if fields.insert(p.ident.clone(), p.type_.clone()).is_some() {
          return Err(Error::DuplicateField(struct_.name.clone(), p.ident.clone()));
        }
      }
      for p in struct_.params.iter() {
        cx.big_vars.remove(&p.ident).unwrap();
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
      for p in enum_.params.iter() {
        cx.big_vars.insert(p.ident.clone(), p.type_.clone());
      }
      let mut ctors = HashMap::with_capacity(enum_.ctors.len());
      for p in enum_.ctors.iter() {
        ck_has_kind(&cx, &p.type_, Kind::Type)?;
        if ctors.insert(p.ident.clone(), p.type_.clone()).is_some()
          || cx.fns.contains_key(&p.ident)
          || cx
            .enums
            .iter()
            .any(|(_, info)| info.ctors.contains_key(&p.ident))
        {
          return Err(Error::DuplicateFnOrCtor(p.ident.clone()));
        }
      }
      for p in enum_.params.iter() {
        cx.big_vars.remove(&p.ident).unwrap();
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
      for p in fn_.big_params.iter() {
        cx.big_vars.insert(p.ident.clone(), p.type_.clone());
      }
      for p in fn_.params.iter() {
        ck_has_kind(&cx, &p.type_, Kind::Type)?;
        cx.vars.insert(p.ident.clone(), p.type_.clone());
      }
      // TODO check the requires, ensures, and body
      for p in fn_.big_params.iter() {
        cx.big_vars.remove(&p.ident).unwrap();
      }
      for p in fn_.params.iter() {
        cx.vars.remove(&p.ident).unwrap();
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
  Ok(cx)
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
        return Err(Error::InvalidKindedApp(bi.clone(), k));
      };
      let mut arg_kinds = Vec::with_capacity(args.len());
      for arg in args {
        arg_kinds.push(get_kind(cx, arg)?);
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

fn ck_has_kind(cx: &Cx, kinded: &Kinded, want: Kind) -> Result<()> {
  let got = get_kind(cx, kinded)?;
  if want == got {
    Ok(())
  } else {
    Err(Error::MismatchedKinds(want, got))
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

fn get_expr_type(mut cx: Cx, expr: &Expr) -> Result<(Cx, Kinded)> {
  match expr {
    Expr::String_(_) => Ok((cx, Kinded::BigIdent(BigIdent::new(STR), vec![]))),
    Expr::Number(_) => Ok((cx, Kinded::BigIdent(BigIdent::new(INT), vec![]))),
    Expr::Tuple(es) => {
      let mut ts = Vec::with_capacity(es.len());
      for e in es {
        let ans = get_expr_type(cx, e)?;
        cx = ans.0;
        ts.push(ans.1);
      }
      Ok((cx, Kinded::Tuple(ts)))
    }
    Expr::Struct(name, args, fields) => {
      let info = match cx.structs.get(name) {
        Some(x) => x.clone(),
        None => return Err(Error::UndefinedType(name.clone())),
      };
      if info.params.len() != args.len() {
        return Err(Error::WrongNumKindedArgs(
          name.clone(),
          info.params.len(),
          args.len(),
        ));
      }
      for (p, a) in info.params.iter().zip(args) {
        ck_has_kind(&cx, a, p.type_.clone())?;
      }
      for f in fields {
        let (x, ans) = match f {
          Field::Ident(x) => (x, get_expr_type(cx, &Expr::Ident(x.clone()))?),
          Field::IdentAnd(x, e) => (x, get_expr_type(cx, e)?),
        };
        cx = ans.0;
        match info.fields.get(x) {
          None => return Err(Error::UndefinedField(name.clone(), x.clone())),
          Some(t) => todo!(),
        }
      }
      todo!()
    }
    Expr::Ident(name) => todo!(),
    Expr::FnCall(name, big_args, args) => todo!(),
    Expr::FieldGet(struct_, name) => todo!(),
    Expr::MethodCall(receiver, name, big_args, args) => todo!(),
    Expr::Return(expr) => todo!(),
    Expr::Match(expr, arms) => todo!(),
    Expr::Block(block) => todo!(),
  }
}

fn ck_expr_has_type(cx: Cx, e: &Expr, want: Kinded) -> Result<Cx> {
  let (cx, got) = get_expr_type(cx, e)?;
  if want == got {
    Ok(cx)
  } else {
    Err(Error::MismatchedTypes(want, got))
  }
}
