import { Grammar, t, n, a, e, terminals, verify } from "../grammar";

const imported = new Set(["big-ident", "ident", "string", "number"]);
const exported = new Set(["program"]);

const syntax: Grammar = verify(imported, exported, [
  { name: "program", def: [e, a(n("top-defn"), n("program"))] },
  {
    name: "top-defn",
    def: [
      a(t("struct"), n("big-ident"), n("big-param-list-opt"), t("{"), n("param-list"), t("}")),
      a(t("enum"), n("big-ident"), n("big-param-list-opt"), t("{"), n("ctor-list"), t("}")),
      a(
        t("fn"),
        n("ident"),
        n("big-param-list-opt"),
        t("("),
        n("param-list"),
        t(")"),
        t(":"),
        n("kinded"),
        n("requires-clause"),
        n("ensures-clause"),
        n("block"),
      ),
    ],
  },
  {
    name: "big-param-list-opt",
    def: [e, a(t("["), n("big-param-list"), t("]"))],
  },
  {
    name: "big-param-list",
    def: [e, n("big-param"), a(n("big-param"), t(","), n("big-param-list"))],
  },
  {
    name: "big-param",
    def: [a(n("big-ident"), t(":"), n("kind"))],
  },
  { name: "kind", def: [a(n("kind-hd"), n("kind-arrow"))] },
  {
    name: "kind-hd",
    def: [n("big-ident"), a(t("("), n("kind-list"), t(")"))],
  },
  {
    name: "kind-arrow",
    def: [e, a(t("->"), n("kind"))],
  },
  {
    name: "kind-list",
    def: [e, n("kind"), a(n("kind"), t(","), n("kind-list"))],
  },
  {
    name: "kinded",
    def: [a(n("kinded-hd"), n("kinded-tl"))],
  },
  {
    name: "kinded-hd",
    def: [a(n("big-ident"), n("kinded-args-opt")), a(t("("), n("kinded-list"), t(")"))],
  },
  {
    name: "kinded-tl",
    def: [e, a(t("->"), n("kinded")), a(t("affects"), n("kinded"))],
  },
  {
    name: "kinded-args-opt",
    def: [e, a(t("["), n("kinded-list"), t("]"))],
  },
  {
    name: "kinded-list",
    def: [e, n("kinded"), a(n("kinded"), t(","), n("kinded-list"))],
  },
  {
    name: "ctor-list",
    def: [e, n("ctor"), a(n("ctor"), t(","), n("ctor-list"))],
  },
  {
    name: "ctor",
    def: [a(n("ident"), t("("), n("kinded"), t(")"))],
  },
  {
    name: "param-list",
    def: [e, n("param"), a(n("param"), t(","), n("param-list"))],
  },
  {
    name: "param",
    def: [a(n("ident"), t(":"), n("kinded"))],
  },
  { name: "requires-clause", def: [e, a(t("requires"), n("expr"))] },
  { name: "ensures-clause", def: [e, a(t("ensures"), n("expr"))] },
  { name: "block", def: [a(t("{"), n("block-inner"), t("}"))] },
  { name: "block-inner", def: [e, n("expr"), a(n("stmt"), n("block-inner"))] },
  {
    name: "stmt",
    def: [a(t("let"), n("pat"), n("type-annotation"), t("="), n("expr"), t(";"))],
  },
  {
    name: "type-annotation",
    def: [e, a(t(":"), n("kinded"))],
  },
  {
    name: "pat",
    def: [a(n("pat-hd"), n("pat-or"))],
  },
  {
    name: "pat-hd",
    def: [
      t("_"),
      n("string"),
      n("number"),
      a(t("("), n("pat-list"), t(")")),
      a(n("big-ident"), t("{"), n("field-pat-list"), t("}")),
      a(n("qual-ident"), t("("), n("pat"), t(")")),
      n("ident"),
    ],
  },
  { name: "pat-list", def: [e, n("pat"), a(n("pat"), t(","), n("pat-list"))] },
  { name: "pat-or", def: [e, a(t("|"), n("pat"))] },
  {
    name: "field-pat-list",
    def: [e, n("field-pat"), a(n("field-pat"), t(","), n("field-pat-list"))],
  },
  { name: "field-pat", def: [n("ident"), a(n("ident"), t(":"), n("pat"))] },
  {
    name: "expr",
    def: [a(n("expr-hd"), n("expr-tl-list"))],
  },
  {
    name: "expr-hd",
    def: [
      n("string"),
      n("number"),
      a(t("("), n("expr-list"), t(")")),
      a(n("big-ident"), n("kinded-args-opt"), t("{"), n("field-expr-list"), t("}")),
      a(n("qual-ident"), n("call-opt")),
      a(t("return"), n("expr")),
      a(t("match"), n("expr"), t("{"), n("arm-list"), t("}")),
      n("block"),
    ],
  },
  {
    name: "expr-tl-list",
    def: [e, a(t("."), n("ident"), n("call-opt"), n("expr-tl-list"))],
  },
  {
    name: "qual-ident",
    def: [n("ident"), a(n("big-ident"), t("::"), n("ident"))],
  },
  {
    name: "call-opt",
    def: [e, a(n("kinded-args-opt"), t("("), n("expr-list"), t(")"))],
  },
  {
    name: "expr-list",
    def: [e, n("expr"), a(n("expr"), t(","), n("expr-list"))],
  },
  {
    name: "field-expr-list",
    def: [e, n("field-expr"), a(n("field-expr"), t(","), n("field-expr-list"))],
  },
  { name: "field-expr", def: [n("ident"), a(n("ident"), t(":"), n("expr"))] },
  { name: "arm-list", def: [e, a(n("arm"), n("arm-list"))] },
  { name: "arm", def: [a(n("pat"), n("block"))] },
]);

const special: string[] = [];
const reserved: string[] = [];
const alpha = /^[a-z]+$/;

terminals(syntax).forEach((t) => {
  if (alpha.test(t)) {
    reserved.push(t);
  } else {
    special.push(t);
  }
});

special.sort();
reserved.sort();

export { special, reserved, syntax };
