import React from "react";
import * as G from "../grammar";
import Katex from "../components/Katex";
import absurd from "../absurd";
import "./Grammar.css";

type Props = {
  g: G.Grammar;
};

const eps = <Katex v="\epsilon" />;

function prodKey(p: G.Prod, idx: number) {
  // space is intentional for And
  return <React.Fragment key={idx}>{prod(p)} </React.Fragment>;
}

function prod(p: G.Prod): React.ReactNode {
  switch (p.t) {
    case "Empty":
      return eps;
    case "Comment":
      return p.msg;
    case "Term":
      return <code>{p.val}</code>;
    case "NonTerm":
      return <em>{p.val}</em>;
    case "And":
      return p.prods.map(prodKey);
    default:
      return absurd(p);
  }
}

function orOpt(p: G.Prod, idx: number) {
  return (
    <React.Fragment key={idx}>
      <div className="GrammarItem__Sep ta-c">{idx === 0 ? "=" : "|"}</div>
      <div className="GrammarItem__Prod">{prod(p)}</div>
    </React.Fragment>
  );
}

function grammarItem(gi: G.GrammarItem) {
  return (
    <div key={gi.name} className="mb-1-em">
      <div>
        <em>{gi.name}</em>
      </div>
      <div className="GrammarItem">{gi.def.map(orOpt)}</div>
    </div>
  );
}

export default function Grammar({ g }: Props) {
  return <div className="Grammar">{g.map(grammarItem)}</div>;
}
