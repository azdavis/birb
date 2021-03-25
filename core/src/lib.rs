//! An implementation of Birb.

#![deny(missing_docs)]

#[cfg(test)]
mod tests;

pub mod cst;
pub mod elab;
pub mod error;
pub mod ident;
pub mod interpret;
pub mod lex;
pub mod parse;
pub mod statics;
pub mod std_lib;
pub mod token;

mod util;

/// Lex, parse, typecheck, and evaluate a Birb program.
pub fn get(bs: &[u8]) -> error::Result<interpret::Value> {
  use std::collections::HashMap;
  let ts = lex::get(bs)?;
  let mut top_defns = std_lib::top_defns();
  top_defns.append(&mut parse::get(&ts)?);
  let top_defns = elab::get(top_defns);
  statics::get(&top_defns)?;
  let cx: HashMap<_, _> = top_defns
    .into_iter()
    .map(|td| {
      let name = match &td {
        cst::TopDefn::Struct(defn) => defn.name.clone(),
        cst::TopDefn::Enum(defn) => defn.name.clone(),
        cst::TopDefn::Fn_(defn) => defn.name.clone(),
      };
      (name, td)
    })
    .collect();
  interpret::get(cx)
}
