//! Static verification.

use crate::cst::{Arm, Block, Expr, Field, Kind, Kinded, Param, Pat, Stmt, TopDefn};
use crate::error::{Error, Result};
use crate::ident::Ident;
use crate::std_lib::{effects, NAT, STR};
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
  let has_main = top_defns.iter().any(|td| match td {
    TopDefn::Fn_(info) => info.name == Ident::new("main"),
    _ => false,
  });
  if !has_main {
    return Err(Error::NoMain);
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

#[derive(Default, Clone)]
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
      if fn_.requires.is_some() {
        todo!("requires");
      }
      if fn_.ensures.is_some() {
        todo!("requires");
      }
      let got = get_block_type(cx, var_cx.clone(), &fn_.body)?;
      if fn_.ret_type != got {
        return Err(Error::MismatchedTypes(fn_.ret_type.clone(), got));
      }
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

// TODO check effects
fn get_expr_type(cx: &Cx, var_cx: &VarCx, expr: &Expr) -> Result<Kinded> {
  match expr {
    Expr::String_(_) => Ok(str_type()),
    Expr::Number(_) => Ok(nat_type()),
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
        big_vars.insert(p.ident.clone(), a.clone());
      }
      if info.params.len() != args.len() {
        return Err(Error::WrongNumArgs(
          name.clone(),
          info.params.len(),
          args.len(),
        ));
      }
      for (p, a) in info.params.iter().zip(args) {
        let want = subst_kinded(&big_vars, p.type_.clone());
        let got = get_expr_type(cx, var_cx, a)?;
        if want != got {
          return Err(Error::MismatchedTypes(want, got));
        }
      }
      Ok(subst_kinded(&big_vars, info.ret_type))
    }
    Expr::FieldGet(struct_, field) => {
      let type_ = get_expr_type(cx, var_cx, struct_)?;
      let (name, args) = match type_ {
        Kinded::Ident(name, args) => (name, args),
        _ => return Err(Error::NotStruct(field.clone())),
      };
      let info = match cx.structs.get(&name) {
        Some(x) => x,
        None => return Err(Error::NotStruct(field.clone())),
      };
      let field_type = match info.fields.get(field) {
        Some(x) => x,
        None => return Err(Error::NoSuchField(name.clone(), field.clone())),
      };
      assert_eq!(info.params.len(), args.len());
      let big_vars: HashMap<_, _> = info
        .params
        .iter()
        .zip(args)
        .map(|(p, a)| (p.ident.clone(), a))
        .collect();
      Ok(subst_kinded(&big_vars, field_type.clone()))
    }
    Expr::MethodCall(receiver, name, big_args, args) => {
      let receiver = (**receiver).clone();
      let args: Vec<_> = std::iter::once(receiver).chain(args.clone()).collect();
      let expr = Expr::FnCall(name.clone(), big_args.clone(), args);
      get_expr_type(cx, var_cx, &expr)
    }
    Expr::Return(..) => todo!("return"),
    Expr::Match(head, arms) => {
      let head_type = get_expr_type(cx, var_cx, head)?;
      let mut iter = arms.iter();
      // TODO exhaustiveness
      let res_type = match iter.next() {
        Some(arm) => get_arm_type(cx, var_cx.clone(), arm, &head_type)?,
        None => todo!("empty match"),
      };
      for arm in iter {
        let got = get_arm_type(cx, var_cx.clone(), arm, &head_type)?;
        if res_type != got {
          return Err(Error::MismatchedTypes(res_type, got));
        }
      }
      Ok(res_type)
    }
    Expr::Block(block) => get_block_type(cx, var_cx.clone(), block),
  }
}

fn subst_kinded(vars: &HashMap<Ident, Kinded>, kinded: Kinded) -> Kinded {
  match kinded {
    Kinded::Ident(id, args) => match vars.get(&id) {
      None => Kinded::Ident(id, args),
      Some(var_kinded) => {
        if args.is_empty() {
          var_kinded.clone()
        } else {
          match var_kinded {
            Kinded::Ident(var_id, var_args) => {
              assert!(var_args.is_empty());
              Kinded::Ident(var_id.clone(), args)
            }
            _ => unreachable!(),
          }
        }
      }
    },
    Kinded::Tuple(ts) => Kinded::Tuple(ts.into_iter().map(|t| subst_kinded(vars, t)).collect()),
    Kinded::Set(es) => Kinded::Set(es.into_iter().map(|e| subst_kinded(vars, e)).collect()),
    Kinded::Arrow(t1, t2) => Kinded::Arrow(
      subst_kinded(vars, *t1).into(),
      subst_kinded(vars, *t2).into(),
    ),
    Kinded::Effectful(t, e) => {
      Kinded::Effectful(subst_kinded(vars, *t).into(), subst_kinded(vars, *e).into())
    }
  }
}

fn match_pat(cx: &Cx, pat: &Pat, typ: &Kinded) -> Result<HashMap<Ident, Kinded>> {
  match pat {
    Pat::Wildcard => Ok(HashMap::new()),
    Pat::String_(_) => {
      let got = str_type();
      if *typ == got {
        Ok(HashMap::new())
      } else {
        Err(Error::MismatchedTypes(typ.clone(), got))
      }
    }
    Pat::Number(_) => {
      let got = nat_type();
      if *typ == got {
        Ok(HashMap::new())
      } else {
        Err(Error::MismatchedTypes(typ.clone(), got))
      }
    }
    Pat::Tuple(pats) => {
      let types = match typ {
        Kinded::Tuple(x) => x,
        _ => return Err(Error::InvalidPattern(typ.clone())),
      };
      if pats.len() != types.len() {
        return Err(Error::InvalidPattern(typ.clone()));
      }
      let mut ret = HashMap::new();
      for (p, t) in pats.iter().zip(types) {
        ret = match union_no_dupe(ret, match_pat(cx, p, t)?) {
          Ok(x) => x,
          Err(id) => return Err(Error::DuplicateIdentifier(id)),
        };
      }
      Ok(ret)
    }
    Pat::Struct(..) => todo!("struct pattern"),
    Pat::Ctor(ctor_name, pat) => {
      let (enum_name, args) = match typ {
        Kinded::Ident(enum_name, args) => (enum_name, args),
        _ => return Err(Error::InvalidPattern(typ.clone())),
      };
      let info = match cx.enums.get(enum_name) {
        Some(x) => x,
        None => return Err(Error::InvalidPattern(typ.clone())),
      };
      let ctor_type = match info.ctors.get(ctor_name) {
        Some(x) => x,
        None => return Err(Error::InvalidPattern(typ.clone())),
      };
      assert_eq!(info.params.len(), args.len());
      let big_vars: HashMap<_, _> = info
        .params
        .iter()
        .zip(args)
        .map(|(p, a)| (p.ident.clone(), a.clone()))
        .collect();
      match_pat(cx, &**pat, &subst_kinded(&big_vars, ctor_type.clone()))
    }
    Pat::Ident(name) => {
      let mut ret = HashMap::new();
      ret.insert(name.clone(), typ.clone());
      Ok(ret)
    }
    Pat::Or(..) => todo!("or pattern"),
  }
}

fn get_arm_type(cx: &Cx, mut var_cx: VarCx, arm: &Arm, typ: &Kinded) -> Result<Kinded> {
  var_cx.vars.extend(match_pat(cx, &arm.pat, typ)?);
  get_block_type(cx, var_cx, &arm.block)
}

fn get_block_type(cx: &Cx, mut var_cx: VarCx, blk: &Block) -> Result<Kinded> {
  for stmt in blk.stmts.iter() {
    match stmt {
      Stmt::Let(pat, typ, expr) => {
        let got = get_expr_type(cx, &var_cx, expr)?;
        if let Some(typ) = typ {
          if *typ != got {
            return Err(Error::MismatchedTypes(typ.clone(), got));
          }
        }
        var_cx.vars.extend(match_pat(cx, pat, &got)?);
      }
    }
  }
  match &blk.expr {
    None => todo!("block with no expr"),
    Some(e) => get_expr_type(cx, &var_cx, e),
  }
}

fn str_type() -> Kinded {
  Kinded::Ident(Ident::new(STR), vec![])
}

fn nat_type() -> Kinded {
  Kinded::Ident(Ident::new(NAT), vec![])
}

fn union_no_dupe<K, V, S>(
  mut xs: HashMap<K, V, S>,
  ys: HashMap<K, V, S>,
) -> std::result::Result<HashMap<K, V, S>, K>
where
  K: std::hash::Hash + Eq,
  S: std::hash::BuildHasher,
{
  for (k, v) in ys {
    if xs.contains_key(&k) {
      return Err(k);
    }
    assert!(xs.insert(k, v).is_none());
  }
  Ok(xs)
}
