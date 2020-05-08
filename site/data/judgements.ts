// TODO define union of effects?
// TODO define subst?
// TODO define subset relation for effects?

export type Judgement = {
  name: string;
  math: string;
};

export const typeKind: Judgement[] = [
  {
    name: "T-Var",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma, t : \kappa \vdash t : \kappa
}`,
  },
  {
    name: "T-Nat",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma \vdash \textsf{Nat} : \textsf{Type}
}`,
  },
  {
    name: "T-Str",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma \vdash \textsf{Str} : \textsf{Type}
}`,
  },
  {
    name: "T-Prod",
    math: String.raw`\frac{
  (\forall i) \ \Gamma \vdash \tau_i : \textsf{Type}
}{
  \Gamma \vdash \langle L_i : \tau_i \rangle : \textsf{Type}
}`,
  },
  {
    name: "T-Sum",
    math: String.raw`\frac{
  (\forall i) \ \Gamma \vdash \tau_i : \textsf{Type}
}{
  \Gamma \vdash [ L_i : \tau_i ] : \textsf{Type}
}`,
  },
  {
    name: "T-Arrow",
    math: String.raw`\frac{
  \Gamma \vdash \tau_1 : \textsf{Type} \hspace{1em}
  \ \Gamma \vdash \tau_2 : \textsf{Type} \hspace{1em}
  \Gamma \vdash E : \textsf{Eff}
}{
  \Gamma \vdash \tau_1 \rightarrow \tau_2!E : \textsf{Type}
}`,
  },
  {
    name: "T-Forall",
    math: String.raw`\frac{
  \Gamma, t : \kappa \vdash \tau : \textsf{Type}
}{
  \Gamma \vdash \forall (t : \kappa) \ \tau : \textsf{Type}
}`,
  },
  {
    name: "T-Func",
    math: String.raw`\frac{
  \Gamma, t : \kappa_1 \vdash \tau : \kappa_2
}{
  \Gamma \vdash \lambda (t : \kappa_1) \ \tau : \kappa_1 \rightarrow \kappa_2
}`,
  },
  {
    name: "T-App",
    math: String.raw`\frac{
  \Gamma \vdash \tau_1 : \kappa_1 \rightarrow \kappa_2 \hspace{1em}
  \Gamma \vdash \tau_2 : \kappa_1
}{
  \Gamma \vdash \tau_1[\tau_2] : \kappa_2
}`,
  },
  {
    name: "T-Sender",
    math: String.raw`\frac{
  \Gamma \vdash \tau : \textsf{Type}
}{
  \Gamma \vdash \textsf{Sender}[\tau] : \textsf{Type}
}`,
  },
  {
    name: "T-Receiver",
    math: String.raw`\frac{
  \Gamma \vdash \tau : \textsf{Type}
}{
  \Gamma \vdash \textsf{Receiver}[\tau] : \textsf{Type}
}`,
  },
];

export const effKind: Judgement[] = [
  {
    name: "E-Var",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma, e : \kappa \vdash e : \kappa
}`,
  },
  {
    name: "E-Prod",
    math: String.raw`\frac{
  (\forall i) \ \Gamma \vdash \tau_{i,1} : \textsf{Type} \hspace{1em}
  (\forall i) \ \Gamma \vdash \tau_{i,2} : \textsf{Type} \hspace{1em}
}{
  \Gamma \vdash
  \langle L_i : \tau_{i,1} \rightarrow \tau_{i,2} \rangle
  : \textsf{Eff}
}`,
  },
  {
    name: "E-Forall",
    math: String.raw`\frac{
  \Gamma, e : \kappa \vdash E : \textsf{Eff}
}{
  \Gamma \vdash \forall (e : \kappa) \ E : \textsf{Eff}
}`,
  },
  {
    name: "E-Func",
    math: String.raw`\frac{
  \Gamma, t : \kappa_1 \vdash E : \kappa_2
}{
  \Gamma \vdash \lambda (t : \kappa_1) \ E : \kappa_1 \rightarrow \kappa_2
}`,
  },
  {
    name: "E-App",
    math: String.raw`\frac{
  \Gamma \vdash E_1 : \kappa_1 \rightarrow \kappa_2 \hspace{1em}
  \Gamma \vdash E_2 : \kappa_1
}{
  \Gamma \vdash E_1[E_2] : \kappa_2
}`,
  },
];

export const valueType: Judgement[] = [
  {
    name: "V-Var",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma, x : \tau \vdash x : \tau
}`,
  },
  {
    name: "V-Nat",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma \vdash \overline{n} : \textsf{Nat}
}`,
  },
  {
    name: "V-Str",
    math: String.raw`\frac{
  % axiom
}{
  \Gamma \vdash \overline{s} : \textsf{Str}
}`,
  },
  {
    name: "V-Prod",
    math: String.raw`\frac{
  (\forall i) \ \Gamma \vdash v_i : \tau_i
}{
  \Gamma \vdash \langle L_i \hookrightarrow v_i \rangle :
  \langle L_i : \tau_i \rangle
}`,
  },
  {
    name: "V-Sum",
    math: String.raw`\frac{
  (\exists j, 0 \leq j \leq i) \ \Gamma \vdash v : \tau_j
}{
  \Gamma \vdash L_j \cdot v : [ L_i : \tau_i ]
}`,
  },
  {
    name: "V-Func",
    math: String.raw`\frac{
  \Gamma \vdash \tau_1 : \textsf{Type} \hspace{1em}
  \Gamma, x : \tau_1 \vdash c : \tau_2!E
}{
  \Gamma \vdash \lambda (x : \tau_1) \ c : \tau_1 \rightarrow \tau_2!E
}`,
  },
  {
    name: "V-BigFunc",
    math: String.raw`\frac{
  \Gamma, t : \kappa \vdash v : \tau
}{
  \Gamma \vdash \Lambda (t : \kappa) \ v : \forall (t : \kappa) \ \tau
}`,
  },
];

export const compType: Judgement[] = [
  {
    name: "C-Pure",
    math: String.raw`\frac{
  \Gamma \vdash v : \tau
}{
  \Gamma \vdash \textsf{pure} \ v : \tau!\langle \rangle
}`,
  },
  {
    name: "C-Bind",
    math: String.raw`\frac{
  \Gamma \vdash c_1 : \tau_1!E_1 \hspace{1em}
  \Gamma, x : \tau_1 \vdash c_2 : \tau_2!E_2 \hspace{1em}
  E = E_1 \cup E_2
}{
  \Gamma \vdash \textsf{bind} \ x \leftarrow c_1 \ \textsf{in} \ c_2 :
  \tau_2!E
}`,
  },
  {
    name: "C-Proj",
    math: String.raw`\frac{
  \Gamma \vdash v : \langle L_i : \tau_i \rangle \hspace{1em}
  \exists j, 0 \leq j \leq i
}{
  \Gamma \vdash v \cdot L_j : \tau_j!\langle \rangle
}`,
  },
  {
    name: "C-Match",
    math: String.raw`\frac{
  \Gamma \vdash v : [ L_i : \tau_i ] \hspace{1em}
  (\forall i) \ \Gamma, x_i : \tau_i \vdash c_i : \tau!E_i \hspace{1em}
  E = \bigcup E_i
}{
  \Gamma \vdash \textsf{match} \ v \ \{
  L_i \cdot x_i \hookrightarrow c_i
  \} : \tau!E
}`,
  },
  {
    name: "C-App",
    math: String.raw`\frac{
  \Gamma \vdash v_1 : \tau_1 \rightarrow \tau_2!E \hspace{1em}
  \Gamma \vdash v_2 : \tau_1
}{
  \Gamma \vdash v_1(v_2) : \tau_2!E
}`,
  },
  {
    name: "C-TypeApp",
    math: String.raw`\frac{
  \Gamma \vdash v : \forall (t : \kappa) \ \tau_2 \hspace{1em}
  \Gamma \vdash \tau_1 : \kappa
}{
  \Gamma \vdash v[\tau_1] : ([\tau_1/t] \tau_2)!\langle \rangle
}`,
  },
  {
    name: "C-EffApp",
    math: String.raw`\frac{
  \Gamma \vdash v : \forall (e : \kappa) \ E_2 \hspace{1em}
  \Gamma \vdash E_1 : \kappa
}{
  \Gamma \vdash v[E_1] : ([E_1/e] E_2)!\langle \rangle
}`,
  },
];

export const compStep: Judgement[] = [
  {
    name: "S-BindArg",
    math: String.raw`\frac{
  c_1 \mapsto c_1'
}{
  \textsf{bind} \ x \leftarrow c_1 \ \textsf{in} \ c_2
  \mapsto
  \textsf{bind} \ x \leftarrow c_1' \ \textsf{in} \ c_2
}`,
  },
  {
    name: "S-Bind",
    math: String.raw`\frac{
  % axiom
}{
  \textsf{bind} \ x \leftarrow \textsf{pure} \ v \ \textsf{in} \ c_2
  \mapsto
  [v/x] c_2
}`,
  },
  {
    name: "S-Proj",
    math: String.raw`\frac{
  \exists j, 0 \leq j \leq i
}{
  \langle L_i \hookrightarrow v_i \rangle \cdot L_j \mapsto \textsf{pure} \ v_j
}`,
  },
  {
    name: "S-Match",
    math: String.raw`\frac{
  \exists j, 0 \leq j \leq i
}{
  \textsf{match} \ L_j \cdot v \ \{
    L_i \cdot x_i \hookrightarrow c_i
  \}
  \mapsto
  [v/x_j] c_j
}`,
  },
  {
    name: "S-App",
    math: String.raw`\frac{
  % axiom
}{
  (\lambda (x : \tau) c) (v_2) \mapsto [v_2/x] c
}`,
  },
  {
    name: "S-TypeApp",
    math: String.raw`\frac{
  % axiom
}{
  (\Lambda (t : \kappa) v)[\tau] \mapsto \textsf{pure} \ [\tau/t] v
}`,
  },
  {
    name: "S-EffApp",
    math: String.raw`\frac{
  % axiom
}{
  (\Lambda (t : \kappa) v)[E] \mapsto \textsf{pure} \ [E/t] v
}`,
  },
];
