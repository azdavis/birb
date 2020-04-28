import React from "react";
import { Judgement, typeKind, effKind, valueType, compType, compStep } from "../data/judgements";
import Katex from "./Katex";

function judgement(x: Judgement) {
  return (
    <React.Fragment key={x.name}>
      <h4>{x.name}</h4>
      <Katex block v={x.math} />
    </React.Fragment>
  );
}

export default function Judgements() {
  return (
    <div>
      <h2>Judgements</h2>
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
      <h3>
        <Katex v="\Gamma \vdash \tau : \kappa" />
      </h3>
      {typeKind.map(judgement)}
      <h3>
        <Katex v="\Gamma \vdash E : \kappa" />
      </h3>
      {effKind.map(judgement)}
      <h3>
        <Katex v="\Gamma \vdash v : \tau" />
      </h3>
      {valueType.map(judgement)}
      <h3>
        <Katex v="\Gamma \vdash c : \tau!E" />
      </h3>
      {compType.map(judgement)}
      <h3>
        <Katex v="c \mapsto c'" />
      </h3>
      {compStep.map(judgement)}
    </div>
  );
}
