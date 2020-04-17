import React from "react";
import Katex from "./Katex";
import reserved from "../data/reserved";
import { mul } from "../../core/pkg/birb_core";

function monoWord(x: string) {
  return (
    <code className="p-0-5" key={x}>
      {x}
    </code>
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
      <p>the answer is {mul(3, 4)}.</p>
      <Katex
        block
        v="\frac{
          \Gamma, x : \tau \vdash e : \rho
        }{
          \Gamma \vdash \lambda (x : \tau) e : \tau \rightarrow \rho
        }"
      />
    </div>
  );
}
