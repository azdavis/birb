import { Grammar, t, n, a, e, terminals } from "../grammar";

const syntax: Grammar = [
  { name: "program", def: [e, a(n("top-defn"), n("program"))] },
  {
    name: "top-defn",
    def: [n("type-defn"), n("struct-defn"), n("enum-defn"), n("fn-defn")],
  },
  {
    name: "type-defn",
    def: [a(t("type"), n("big-ident"), n("big-param-list-opt"), t("="), n("type"))],
  },
  {
    name: "struct-defn",
    def: [a(t("struct"), n("big-ident"), n("big-param-list-opt"), t("{"), n("field-list"), t("}"))],
  },
  {
    name: "enum-defn",
    def: [a(t("enum"), n("big-ident"), n("big-param-list-opt"), t("{"), n("ctor-list"), t("}"))],
  },
  {
    name: "fn-defn",
    def: [
      a(
        t("fn"),
        n("ident"),
        n("big-param-list-opt"),
        t("("),
        n("param-list"),
        t(")"),
        t(":"),
        n("type"),
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
  { name: "kind", def: [a(n("kind-hd"), n("kind-tl"))] },
  {
    name: "kind-hd",
    def: [n("big-ident"), a(t("("), n("kind-list"), t(")"))],
  },
  {
    name: "kind-tl",
    def: [e, a(t("->"), n("kind"))],
  },
  {
    name: "kind-list",
    def: [e, n("kind"), a(n("kind"), t(","), n("kind-list"))],
  },
  {
    name: "type",
    def: [a(n("type-hd"), n("type-tl"))],
  },
  {
    name: "type-hd",
    def: [n("big-ident"), a(t("("), n("type-list"), t(")"))],
  },
  {
    name: "type-tl",
    def: [e, a(t("->"), n("type")), a(t("affects"), n("effect"))],
  },
  {
    name: "type-list",
    def: [e, n("type"), a(n("type"), t(","), n("type-list"))],
  },
  {
    name: "field-list",
    def: [
      e,
      a(n("ident"), t(":"), n("type")),
      a(n("ident"), t(":"), n("type"), t(","), n("field-list")),
    ],
  },
  {
    name: "ctor-list",
    def: [e, n("ctor"), a(n("ctor"), t(","), n("ctor-list"))],
  },
  {
    name: "ctor",
    def: [a(n("ident"), t("("), n("type"), t(")"))],
  },
  {
    name: "effect",
    def: [a(t("{"), n("effect-list"), t("}"))],
  },
  {
    name: "effect-list",
    def: [e, n("big-ident"), a(n("big-ident"), t(","), n("effect-list"))],
  },
  {
    name: "param-list",
    def: [e, n("param"), a(n("param"), t(","), n("param-list"))],
  },
  {
    name: "param",
    def: [a(n("ident"), t(":"), n("type"))],
  },
  { name: "requires-clause", def: [e, a(t("requires"), n("expr"))] },
  { name: "ensures-clause", def: [e, a(t("ensures"), n("expr"))] },
  { name: "block", def: [a(t("{"), n("block-inner"), t("}"))] },
  { name: "block-inner", def: [e, n("expr"), a(n("stmt"), n("block-inner"))] },
  {
    name: "stmt",
    def: [a(t("let"), n("pat"), t("="), n("expr"))],
  },
  {
    name: "pat",
    def: [a(n("pat-hd"), n("pat-tl"))],
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
  { name: "pat-tl", def: [e, a(t("|"), n("pat")), a(t(":"), n("type"))] },
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
      a(n("big-ident"), n("type-effect-args-opt"), t("{"), n("field-expr-list"), t("}")),
      a(n("qual-ident"), n("call-opt")),
      a(t("return"), n("expr")),
      a(t("match"), n("expr"), t("{"), n("arm-list"), t("}")),
      n("block"),
    ],
  },
  { name: "expr-tl-list", def: [e, a(t("."), n("ident"), n("call-opt"), n("expr-tail-list"))] },
  {
    name: "qual-ident",
    def: [n("ident"), a(n("big-ident"), t("::"), n("ident"))],
  },
  {
    name: "call-opt",
    def: [e, a(n("type-effect-args-opt"), t("("), n("expr-list"), t(")"))],
  },
  {
    name: "type-effect-args-opt",
    def: [e, a(t("["), n("type-effect-list"), t("]"))],
  },
  {
    name: "type-effect-list",
    def: [e, n("type-effect"), a(n("type-effect"), t(","), n("type-effect-list"))],
  },
  {
    name: "type-effect",
    def: [n("type"), n("effect")],
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
];

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
