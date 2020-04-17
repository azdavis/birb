import React from "react";
import * as A from "../abs-syntax";
import Katex from "./Katex";
import "./AbsSyntax.css";

type Props = {
  a: A.AbsSyntax;
};

function alternativeIdx(a: A.Alternative, idx: number) {
  return (
    <React.Fragment key={idx}>
      <div className="AbsSyntax__Sep ta-c">{idx === 0 ? "=" : "|"}</div>
      <div className="AbsSyntax__Math">
        <Katex v={a.math} />
      </div>
      <div className="AbsSyntax__Text">{a.text}</div>
    </React.Fragment>
  );
}

function production(gi: A.Production) {
  return (
    <React.Fragment key={gi.text}>
      <div className="AbsSyntax__Name">
        {gi.text} <Katex v={gi.math} />
      </div>
      {gi.def.map(alternativeIdx)}
      <div className="AbsSyntax__Space h-1-em" />
    </React.Fragment>
  );
}

export default function AbsSyntax({ a }: Props) {
  return <div className="AbsSyntax">{a.map(production)}</div>;
}
