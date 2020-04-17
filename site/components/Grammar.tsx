import React from "react";
import * as G from "../grammar";
import Katex from "../components/Katex";
import absurd from "../absurd";
import "./Grammar.css";

type Props = {
  g: G.Grammar;
};

const eps = <Katex v="\epsilon" />;

function andItem(a: G.Alternative, idx: number) {
  const sp = idx === 0 ? "" : " ";
  return (
    <React.Fragment key={idx}>
      {sp}
      {alternative(a)}
    </React.Fragment>
  );
}

function alternative(a: G.Alternative): React.ReactNode {
  switch (a.t) {
    case "Empty":
      return eps;
    case "Comment":
      return a.msg;
    case "Term":
      return <code>{a.val}</code>;
    case "NonTerm":
      return <em>{a.val}</em>;
    case "And":
      return a.as.map(andItem);
    default:
      return absurd(a);
  }
}

function alternativeIdx(p: G.Alternative, idx: number) {
  return (
    <React.Fragment key={idx}>
      <div className="Grammar__Sep ta-c">{idx === 0 ? "=" : "|"}</div>
      <div className="Grammar__Prod">{alternative(p)}</div>
    </React.Fragment>
  );
}

function production(gi: G.Production) {
  return (
    <React.Fragment key={gi.name}>
      <div className="Grammar__Name">
        <em>{gi.name}</em>
      </div>
      {gi.def.map(alternativeIdx)}
      <div className="Grammar__Space h-1-em" />
    </React.Fragment>
  );
}

export default function Grammar({ g }: Props) {
  return <div className="Grammar">{g.map(production)}</div>;
}
