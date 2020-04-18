import { Grammar, t, c, n, a, e } from "../grammar";

const tokens: Grammar = [
  { name: "comment", def: [a(t("//"), n("comment-tl"))] },
  { name: "comment-tl", def: [c("any character except newline")] },
  { name: "big-ident", def: [a(n("upper"), n("big-ident-tl"))] },
  {
    name: "big-ident-tl",
    def: [e, a(n("big-ident-tl-one"), n("big-ident-tl"))],
  },
  { name: "big-ident-tl-one", def: [n("upper"), n("lower"), n("digit")] },
  { name: "ident", def: [a(n("lower"), n("ident-tl"))] },
  { name: "ident-tl", def: [e, a(n("ident-tl-one"), n("ident-tl"))] },
  { name: "ident-tl-one", def: [n("lower"), n("number"), t("_")] },
  { name: "string", def: [a(t('"'), n("string-inner"), t('"'))] },
  {
    name: "string-inner",
    def: [e, a(n("string-inner-one"), n("string-inner"))],
  },
  {
    name: "string-inner-one",
    def: [a(c("any character except"), t('"'))],
  },
  {
    name: "number",
    def: [a(n("digit"), n("number-tl"))],
  },
  {
    name: "number-tl",
    def: [e, a(n("number-tl-one"), n("number-tl"))],
  },
  {
    name: "number-tl-one",
    def: [n("digit"), t("_")],
  },
  {
    name: "upper",
    def: [t("A"), t("B"), c("..."), t("Z")],
  },
  {
    name: "lower",
    def: [t("a"), t("b"), c("..."), t("z")],
  },
  {
    name: "digit",
    def: [t("0"), t("1"), c("..."), t("9")],
  },
];

export default tokens;
