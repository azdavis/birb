import React from "react";
import * as G from "../grammar";
import Katex from "../components/Katex";
import absurd from "../absurd";
import "./Grammar.css";

type Props = {
  g: G.Grammar;
};

const eps = <Katex v="\epsilon" />;

function prodAnd(p: G.Prod, idx: number) {
  const sp = idx === 0 ? "" : " ";
  return (
    <React.Fragment key={idx}>
      {sp}
      {prod(p)}
    </React.Fragment>
  );
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
      return p.prods.map(prodAnd);
    default:
      return absurd(p);
  }
}

function orOpt(p: G.Prod, idx: number) {
  return (
    <React.Fragment key={idx}>
      <div className="Grammar__Sep ta-c">{idx === 0 ? "=" : "|"}</div>
      <div className="Grammar__Prod">{prod(p)}</div>
    </React.Fragment>
  );
}

function grammarItem(gi: G.GrammarItem) {
  return (
    <React.Fragment key={gi.name}>
      <div className="Grammar__Name">
        <em>{gi.name}</em>
      </div>
      {gi.def.map(orOpt)}
      <div className="Grammar__Space h-1-em" />
    </React.Fragment>
  );
}

export default function Grammar({ g }: Props) {
  return <div className="Grammar">{g.map(grammarItem)}</div>;
}
