import React from "react";
import { Judgement, typeKind, effKind, valueType, compType, compStep } from "../data/judgements";
import Katex from "./Katex";

function judgement(x: Judgement) {
  return (
    <div key={x.name} className="overflow-auto">
      <h4>{x.name}</h4>
      <Katex block v={x.math} />
    </div>
  );
}

export default function Judgements() {
  return (
    <div>
      <p>We introduce the judgements</p>
      <ol>
        <li>
          <Katex v="\Gamma \vdash \tau : \kappa" />, the kinding judgement for types.
        </li>
        <li>
          <Katex v="\Gamma \vdash E : \kappa" />, the kinding judgement for effects.
        </li>
        <li>
          <Katex v="\Gamma \vdash v : \tau" />, the typing judgement for values, which do not step
          and do not engender effects.
        </li>
        <li>
          <Katex v="\Gamma \vdash c : \tau!E" />, the typing judgement for computations, which do
          step and may engender effects.
        </li>
        <li>
          <Katex v="c \mapsto c'" />, the stepping judgement for computations.
        </li>
      </ol>
      <h2>
        <Katex v="\Gamma \vdash \tau : \kappa" />
      </h2>
      {typeKind.map(judgement)}
      <h2>
        <Katex v="\Gamma \vdash E : \kappa" />
      </h2>
      {effKind.map(judgement)}
      <h2>
        <Katex v="\Gamma \vdash v : \tau" />
      </h2>
      {valueType.map(judgement)}
      <h2>
        <Katex v="\Gamma \vdash c : \tau!E" />
      </h2>
      {compType.map(judgement)}
      <h2>
        <Katex v="c \mapsto c'" />
      </h2>
      {compStep.map(judgement)}
    </div>
  );
}
