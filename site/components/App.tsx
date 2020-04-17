import React from "react";
import Katex from "./Katex";
import Grammar from "./Grammar";
import reserved from "../data/reserved";
import tokens from "../data/tokens";
import syntax from "../data/syntax";
import AbsSyntax from "./AbsSyntax";
import absSyntax from "../data/abs-syntax";
import {
  Judgement,
  typeKind,
  effKind,
  valueType,
  compType,
  compStep,
} from "../data/judgements";
import { mul } from "../../core/pkg/birb_core";

function monoWord(x: string) {
  return (
    <code className="p-lr-0-5-em" key={x}>
      {x}
    </code>
  );
}

function judgement(x: Judgement) {
  return (
    <React.Fragment key={x.name}>
      <h4>{x.name}</h4>
      <Katex block v={x.math} />
    </React.Fragment>
  );
}

export default function App() {
  return (
    <div>
      <h1>Birb</h1>
      <p>Ariel Davis (azdavis), Vivian Huang (vivianh)</p>
      <p>
        This is the specification for Birb, a programming language featuring an
        effects system and channels.
      </p>
      <h2>Concrete Syntax</h2>
      <h3>Reserved Words</h3>
      <p>
        The reserved words follow. An identifier may not be a reserved word. Not
        all reserved words are currently used in the grammar.
      </p>
      <div className="d-flex flex-wrap jc-center">{reserved.map(monoWord)}</div>
      <h3>Lexical Tokens</h3>
      <Grammar g={tokens} />
      <h3>Grammar</h3>
      <p>
        Arbitrary comments and whitespace may be interspersed between tokens.
        Sometimes, at least 1 element of whitespace is required, e.g. to
        separate a reserved word from an identifier.
      </p>
      <Grammar g={syntax} />
      <h2>Abstract Syntax</h2>
      <AbsSyntax a={absSyntax} />
      <h2>Judgement Forms</h2>
      <p>We introduce the judgements</p>
      <ol>
        <li>
          <Katex v="\Gamma \vdash \tau : \kappa" />, the kinding judgement for
          types.
        </li>
        <li>
          <Katex v="\Gamma \vdash E : \kappa" />, the kinding judgement for
          effects.
        </li>
        <li>
          <Katex v="\Gamma \vdash v : \tau" />, the typing judgement for values,
          which do not step and do not engender effects.
        </li>
        <li>
          <Katex v="\Gamma \vdash c : \tau!E" />, the typing judgement for
          computations, which do step and may engender effects.
        </li>
        <li>
          <Katex v="c \mapsto c'" />, the stepping judgement for computations.
        </li>
      </ol>
      <h2>Judgement Definitions</h2>
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
      <p>the answer is {mul(3, 4)}.</p>
    </div>
  );
}
