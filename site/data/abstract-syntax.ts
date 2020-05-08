import { AbstractSyntax } from "../abstract-syntax";

const abstractSyntax: AbstractSyntax = [
  {
    text: "Kind",
    math: String.raw`\kappa`,
    def: [
      { math: String.raw`\textsf{Type}`, text: "type" },
      { math: String.raw`\textsf{Eff}`, text: "effect" },
      { math: String.raw`\kappa_1 \rightarrow \kappa_2`, text: "arrow" },
    ],
  },
  {
    text: "Type",
    math: String.raw`\tau`,
    def: [
      { math: String.raw`t`, text: "variable" },
      { math: String.raw`\textsf{Nat}`, text: "natural number" },
      { math: String.raw`\textsf{Str}`, text: "string" },
      {
        math: String.raw`\langle L_i : \tau_i \rangle`,
        text: "labeled product",
      },
      { math: String.raw`[ L_i : \tau_i ]`, text: "labeled sum" },
      { math: String.raw`\tau_1 \rightarrow \tau_2!E`, text: "arrow" },
      { math: String.raw`\forall (t : \kappa) \ \tau`, text: "universal" },
      { math: String.raw`\lambda (t : \kappa) \ \tau`, text: "function" },
      { math: String.raw`\tau_1[\tau_2]`, text: "application" },
    ],
  },
  {
    text: "Effect",
    math: "E",
    def: [
      { math: String.raw`e`, text: "variable" },
      {
        math: String.raw`\langle L_i : \tau_{i,1} \rightarrow \tau_{i,2} \rangle`,
        text: "labeled product",
      },
      { math: String.raw`\forall (e : \kappa) \ E`, text: "universal" },
      { math: String.raw`\lambda (e : \kappa) \ E`, text: "function" },
      { math: String.raw`E_1[E_2]`, text: "application" },
    ],
  },
  {
    text: "Value",
    math: "v",
    def: [
      { math: String.raw`x`, text: "variable" },
      { math: String.raw`\overline{n}`, text: "natural number" },
      { math: String.raw`\overline{s}`, text: "string" },
      {
        math: String.raw`\langle L_i \hookrightarrow v_i \rangle`,
        text: "labeled product",
      },
      { math: String.raw`L \cdot v`, text: "labeled sum" },
      { math: String.raw`\lambda (x : \tau) \ c`, text: "function" },
      { math: String.raw`\Lambda (t : \kappa) \ v`, text: "big function" },
    ],
  },
  {
    text: "Computation",
    math: "c",
    def: [
      { math: String.raw`\textsf{pure} \ v`, text: "enter the monad" },
      {
        math: String.raw`\textsf{bind} \ x \leftarrow c_1 \ \textsf{in} \ c_2`,
        text: "sequencing",
      },
      { math: String.raw`v \cdot L`, text: "projection" },
      {
        math: String.raw`\textsf{match} \ v \ \{ L_i \cdot x_i \hookrightarrow c_i \}`,
        text: "case analysis",
      },
      { math: String.raw`v_1(v_2)`, text: "value application" },
      { math: String.raw`v[\tau]`, text: "type application" },
      { math: String.raw`v[E]`, text: "effect application" },
    ],
  },
];

export default abstractSyntax;
