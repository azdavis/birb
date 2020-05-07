import React from "react";
import * as A from "../abstract-syntax";
import Katex from "./Katex";
import abstractSyntax from "../data/abstract-syntax";
import "./AbstractSyntax.css";

function alternativeIdx(a: A.Alternative, idx: number) {
  return (
    <React.Fragment key={idx}>
      <div className="AbstractSyntax__Sep ta-c">{idx === 0 ? "=" : "|"}</div>
      <div className="AbstractSyntax__Math">
        <Katex v={a.math} />
      </div>
      <div className="AbstractSyntax__Text">{a.text}</div>
    </React.Fragment>
  );
}

function production(gi: A.Production) {
  return (
    <React.Fragment key={gi.text}>
      <div className="AbstractSyntax__Name">
        {gi.text} <Katex v={gi.math} />
      </div>
      {gi.def.map(alternativeIdx)}
      <div className="AbstractSyntax__Space h-1-em" />
    </React.Fragment>
  );
}

export default function AbstractSyntax() {
  return <div className="AbstractSyntax">{abstractSyntax.map(production)}</div>;
}
