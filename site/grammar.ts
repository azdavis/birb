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
