import absurd from "./absurd";

export type Grammar = Production[];

export type Production = {
  name: string;
  def: Alternative[];
};

export type Alternative =
  | { t: "Empty" }
  | { t: "Comment"; msg: string }
  | { t: "Terminal"; val: string }
  | { t: "NonTerminal"; val: string }
  | { t: "And"; as: Alternative[] };

export const e: Alternative = { t: "Empty" };

export function c(msg: string): Alternative {
  return { t: "Comment", msg };
}

export function t(val: string): Alternative {
  return { t: "Terminal", val };
}

export function n(val: string): Alternative {
  return { t: "NonTerminal", val };
}

export function a(...as: Alternative[]): Alternative {
  return { t: "And", as };
}

function altTerminal(ac: Set<string>, a: Alternative): Set<string> {
  switch (a.t) {
    case "Empty":
    case "Comment":
    case "NonTerminal":
      return ac;
    case "Terminal":
      ac.add(a.val);
      return ac;
    case "And":
      return altsTerminals(ac, a.as);
    default:
      return absurd(a);
  }
}

function altsTerminals(ac: Set<string>, as: Alternative[]): Set<string> {
  return as.reduce(altTerminal, ac);
}

function prodTerminals(ac: Set<string>, prod: Production): Set<string> {
  return altsTerminals(ac, prod.def);
}

export function terminals(g: Grammar): Set<string> {
  return g.reduce(prodTerminals, new Set());
}

function altNonTerminal(ac: Set<string>, a: Alternative): Set<string> {
  switch (a.t) {
    case "Empty":
    case "Comment":
    case "Terminal":
      return ac;
    case "NonTerminal":
      ac.add(a.val);
      return ac;
    case "And":
      return altsNonTerminals(ac, a.as);
    default:
      return absurd(a);
  }
}

function altsNonTerminals(ac: Set<string>, as: Alternative[]): Set<string> {
  return as.reduce(altNonTerminal, ac);
}

function prodNonTerminals(ac: Set<string>, prod: Production): Set<string> {
  const added = altsNonTerminals(new Set(), prod.def);
  // recursive reference doesn't count
  added.delete(prod.name);
  return union(ac, added);
}

function nonTerminals(g: Grammar): Set<string> {
  return g.reduce(prodNonTerminals, new Set());
}

function prodName(p: Production): string {
  return p.name;
}

function minus(xs: Set<string>, ys: Set<string>): Set<string> {
  const ret = new Set(xs);
  for (const y of ys) {
    ret.delete(y);
  }
  return ret;
}

function intersect(xs: Set<string>, ys: Set<string>): Set<string> {
  const ret = new Set<string>();
  for (const x of xs) {
    if (ys.has(x)) {
      ret.add(x);
    }
  }
  return ret;
}

function union(xs: Set<string>, ys: Set<string>): Set<string> {
  return new Set([...xs, ...ys]);
}

function dupes(xs: string[]): string[] {
  const seen = new Set<string>();
  const ret = [];
  for (const x of xs) {
    if (seen.has(x)) {
      ret.push(x);
    }
    seen.add(x);
  }
  return ret;
}

export function verify(imported: Set<string>, exported: Set<string>, g: Grammar): Grammar {
  const ref = nonTerminals(g);
  const defArr = g.map(prodName);
  const reDef = dupes(defArr);
  if (reDef.length !== 0) {
    throw new Error(`reDef: ${reDef.join(", ")}`);
  }
  const def = new Set(defArr);
  const needImport = minus(ref, union(imported, def));
  const uselessImport = union(intersect(imported, def), minus(imported, ref));
  if (needImport.size !== 0) {
    throw new Error(`needImport: ${[...needImport].join(", ")}`);
  }
  if (uselessImport.size !== 0) {
    throw new Error(`uselessImport: ${[...uselessImport].join(", ")}`);
  }
  const needExport = minus(def, union(ref, exported));
  const uselessExport = union(intersect(exported, ref), minus(exported, def));
  if (needExport.size !== 0) {
    throw new Error(`needExport: ${[...needExport].join(", ")}`);
  }
  if (uselessExport.size !== 0) {
    throw new Error(`uselessExport: ${[...uselessExport].join(", ")}`);
  }
  return g;
}
