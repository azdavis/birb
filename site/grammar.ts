import absurd from "./absurd";

export type Grammar = Production[];

export type Production = {
  name: string;
  def: Alternative[];
};

export type Alternative =
  | { t: "Empty" }
  | { t: "Comment"; msg: string }
  | { t: "Term"; val: string }
  | { t: "NonTerm"; val: string }
  | { t: "And"; as: Alternative[] };

export const e: Alternative = { t: "Empty" };

export function c(msg: string): Alternative {
  return { t: "Comment", msg };
}

export function t(val: string): Alternative {
  return { t: "Term", val };
}

export function n(val: string): Alternative {
  return { t: "NonTerm", val };
}

export function a(...as: Alternative[]): Alternative {
  return { t: "And", as };
}

function union<T>(xs: Set<T>, ys: Set<T>): Set<T> {
  const ret = new Set<T>();
  function add(x: T) {
    ret.add(x);
  }
  xs.forEach(add);
  ys.forEach(add);
  return ret;
}

function altTerminal(ac: Set<string>, a: Alternative): Set<string> {
  switch (a.t) {
    case "Empty":
    case "Comment":
    case "NonTerm":
      return ac;
    case "Term":
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
