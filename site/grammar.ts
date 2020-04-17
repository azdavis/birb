export type Grammar = GrammarItem[];

export type GrammarItem = {
  name: string;
  // a big or
  def: Prod[];
};

export type Prod =
  | { t: "Empty" }
  | { t: "Term"; val: string }
  | { t: "NonTerm"; val: string }
  | { t: "And"; prods: Prod[] };

export const e: Prod = { t: "Empty" };

export function t(val: string): Prod {
  return { t: "Term", val };
}

export function n(val: string): Prod {
  return { t: "NonTerm", val };
}

export function a(...prods: Prod[]): Prod {
  return { t: "And", prods };
}
