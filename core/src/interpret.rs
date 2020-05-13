//! Interpretation.

use crate::cst::{Block, Expr, Field, Pat, Stmt, TopDefn};
use crate::error::{Error, Result};
use crate::ident::Ident;
use crate::std_lib as birb_std_lib;
use crate::util::SliceDisplay;
use std::collections::HashMap;
use std::fmt;

/// Steps the expression `main()` in the given context to a value. Requires that the context be
/// statically checked and have a main function.
pub fn get(cx: HashMap<Ident, TopDefn>) -> Result<Value> {
  let main = &cx[&Ident::new("main")];
  let main = match main {
    TopDefn::Fn_(x) => x,
    TopDefn::Struct(..) | TopDefn::Enum(..) => unreachable!(),
  };
  block_eval(&main.body, HashMap::new(), &cx)
}

fn block_eval(
  b: &Block,
  mut m: HashMap<Ident, Value>,
  cx: &HashMap<Ident, TopDefn>,
) -> Result<Value> {
  for s in b.stmts.iter() {
    let (pat, expr) = match s {
      Stmt::Let(p, _, e) => (p, e),
    };
    let val = expr_eval(expr, &m, cx)?;
    let mm = pat_match(pat, &val);
    let mm = mm.unwrap();
    m.extend(mm);
  }
  expr_eval(b.expr.as_ref().unwrap(), &m, cx)
}

fn pat_match(p: &Pat, v: &Value) -> Option<HashMap<Ident, Value>> {
  match (p, v) {
    (Pat::Wildcard, _) => Some(HashMap::new()),
    (Pat::String_(x), Value::String_(y)) => {
      if x == y {
        Some(HashMap::new())
      } else {
        None
      }
    }
    (Pat::Number(x), Value::Number(y)) => {
      if x == y {
        Some(HashMap::new())
      } else {
        None
      }
    }
    (Pat::Tuple(xs), Value::Tuple(ys)) => {
      let mut m = HashMap::new();
      for (x, y) in xs.iter().zip(ys) {
        match pat_match(x, y) {
          None => return None,
          Some(x) => m.extend(x),
        }
      }
      Some(m)
    }
    (Pat::Ctor(x, p), Value::Ctor(y, q)) => {
      if x == y {
        pat_match(&*p, &*q)
      } else {
        None
      }
    }
    (Pat::Ident(i), _) => {
      let mut m = HashMap::new();
      m.insert(i.clone(), v.clone());
      Some(m)
    }
    _ => None,
  }
}

fn expr_eval(e: &Expr, m: &HashMap<Ident, Value>, cx: &HashMap<Ident, TopDefn>) -> Result<Value> {
  let ret = match e {
    Expr::String_(x) => Value::String_(x.clone()),
    Expr::Number(x) => Value::Number(x.clone()),
    Expr::Tuple(xs) => {
      let mut t = Vec::with_capacity(xs.len());
      for x in xs {
        t.push(expr_eval(x, m, cx)?);
      }
      Value::Tuple(t)
    }
    Expr::Struct(w, _, xs) => {
      let mut t = Vec::with_capacity(xs.len());
      for x in xs {
        match x {
          Field::Ident(i) => t.push(Field::IdentAnd(
            i.clone(),
            expr_eval(&Expr::Ident(i.clone()), m, cx)?,
          )),
          Field::IdentAnd(i, j) => t.push(Field::IdentAnd(i.clone(), expr_eval(j, m, cx)?)),
        };
      }
      Value::Struct(w.clone(), t)
    }
    Expr::Ident(i) => m[i].clone(),
    Expr::FnCall(i, _, xs) => {
      let mut vs = Vec::with_capacity(xs.len());
      for x in xs {
        vs.push(expr_eval(x, m, cx)?);
      }
      if *i == Ident::new(birb_std_lib::ADD) {
        return Ok(nat_math_op(vs, |x, y| x + y));
      }
      if *i == Ident::new(birb_std_lib::SUB) {
        return Ok(nat_math_op(vs, |x, y| x - y));
      }
      if *i == Ident::new(birb_std_lib::MUL) {
        return Ok(nat_math_op(vs, |x, y| x * y));
      }
      if *i == Ident::new(birb_std_lib::DIV) {
        return Ok(nat_math_op(vs, |x, y| x / y));
      }
      if *i == Ident::new(birb_std_lib::EQ) {
        return Ok(nat_cmp_op(vs, |x, y| x == y));
      }
      if *i == Ident::new(birb_std_lib::LT) {
        return Ok(nat_cmp_op(vs, |x, y| x < y));
      }
      if *i == Ident::new(birb_std_lib::GT) {
        return Ok(nat_cmp_op(vs, |x, y| x > y));
      }
      if *i == Ident::new(birb_std_lib::AND) {
        return Ok(bool_op(vs, |x, y| x && y));
      }
      if *i == Ident::new(birb_std_lib::OR) {
        return Ok(bool_op(vs, |x, y| x || y));
      }
      match cx.get(i) {
        Some(TopDefn::Fn_(f)) => {
          let mut m = m.clone();
          for (p, v) in f.params.iter().zip(vs) {
            m.insert(p.ident.clone(), v);
          }
          if let Some(req) = &f.requires {
            let e = expr_eval(req, &m, cx)?;
            if !get_bool(e) {
              return Err(Error::RequiresFailed(i.clone()));
            }
          }
          let ret = block_eval(&f.body, m.clone(), cx)?;
          if let Some(ens) = &f.ensures {
            m.insert(Ident::new("ret"), ret.clone());
            let e = expr_eval(ens, &m, cx)?;
            if !get_bool(e) {
              return Err(Error::EnsuresFailed(i.clone()));
            }
          }
          ret
        }
        Some(TopDefn::Struct(..)) | Some(TopDefn::Enum(..)) => unreachable!(),
        None => {
          let v = vs.pop().unwrap();
          assert!(vs.is_empty());
          Value::Ctor(i.clone(), v.into())
        }
      }
    }
    Expr::FieldGet(e, i) => {
      let v = expr_eval(e, m, cx)?;
      match v {
        Value::Struct(_, fs) => {
          for f in fs {
            match f {
              Field::IdentAnd(q, t) => {
                if q == *i {
                  return Ok(t);
                }
              }
              Field::Ident(..) => unreachable!(),
            }
          }
          unreachable!()
        }
        _ => unreachable!(),
      }
    }
    Expr::MethodCall(..) => unreachable!("eval method call"),
    Expr::Match(e, xs) => {
      let v = expr_eval(e, m, cx)?;
      for x in xs {
        match pat_match(&x.pat, &v) {
          Some(map) => {
            let mut m = m.clone();
            m.extend(map);
            return block_eval(&x.block, m, cx);
          }
          None => continue,
        }
      }
      return Err(Error::NonExhaustiveMatch);
    }
    Expr::Block(b) => block_eval(&*b, m.clone(), cx)?,
  };
  Ok(ret)
}

fn get_number(val: Value) -> u64 {
  match val {
    Value::Number(n) => n,
    _ => unreachable!(),
  }
}

fn mk_bool(b: bool) -> Value {
  Value::Ctor(
    Ident::new(if b { "true" } else { "false" }),
    Value::Tuple(vec![]).into(),
  )
}

fn get_bool(val: Value) -> bool {
  match val {
    Value::Ctor(name, val) => {
      assert_eq!(*val, Value::Tuple(vec![]));
      if name == Ident::new("true") {
        return true;
      }
      if name == Ident::new("false") {
        return false;
      }
      unreachable!()
    }
    _ => unreachable!(),
  }
}

fn nat_math_op<F>(mut vs: Vec<Value>, f: F) -> Value
where
  F: FnOnce(u64, u64) -> u64,
{
  let y = get_number(vs.pop().unwrap());
  let x = get_number(vs.pop().unwrap());
  assert!(vs.is_empty());
  return Value::Number(f(x, y));
}

fn nat_cmp_op<F>(mut vs: Vec<Value>, f: F) -> Value
where
  F: FnOnce(u64, u64) -> bool,
{
  let y = get_number(vs.pop().unwrap());
  let x = get_number(vs.pop().unwrap());
  assert!(vs.is_empty());
  return mk_bool(f(x, y));
}

fn bool_op<F>(mut vs: Vec<Value>, f: F) -> Value
where
  F: FnOnce(bool, bool) -> bool,
{
  let y = get_bool(vs.pop().unwrap());
  let x = get_bool(vs.pop().unwrap());
  assert!(vs.is_empty());
  return mk_bool(f(x, y));
}

/// A value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
  /// A string literal, like `"x"`.
  String_(String),
  /// A number (integer) literal, like `3`.
  Number(u64),
  /// A tuple, like `(1, "e")`.
  Tuple(Vec<Value>),
  /// A struct expression, like `Foo { x: 3 }`.
  Struct(Ident, Vec<Field<Value>>),
  /// A constructor, like `some(3)`.
  Ctor(Ident, Box<Value>),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::String_(s) => write!(f, "\"{}\"", s),
      Self::Number(n) => n.fmt(f),
      Self::Tuple(vs) => SliceDisplay::new("(", vs, ")").fmt(f),
      Self::Struct(name, fs) => write!(f, "{} {{ {} }}", name, SliceDisplay::new("", fs, ""),),
      Self::Ctor(name, v) => write!(f, "{}({})", name, v),
    }
  }
}
