import React from "react";
import Grammar from "./Grammar";
import tokens from "../data/tokens";
import { special, reserved, syntax } from "../data/syntax";

function monoWord(x: string) {
  return (
    <code className="p-lr-0-5-em" key={x}>
      {x}
    </code>
  );
}

export default function ConcreteSyntax() {
  return (
    <div>
      <h2>Lexical Tokens</h2>
      <p>Birb source code is first lexed into tokens, separated by whitespace or comments.</p>
      <h3>Special Tokens</h3>
      <p>Some tokens are composed of special characters. They are listed below.</p>
      <div className="d-flex flex-wrap jc-center">{special.map(monoWord)}</div>
      <h3>Reserved Words</h3>
      <p>The reserved words are below. An identifier may not be a reserved word.</p>
      <div className="d-flex flex-wrap jc-center">{reserved.map(monoWord)}</div>
      <h3>Other Tokens</h3>
      <p>The other tokens are strings, numbers, and identifiers.</p>
      <Grammar g={tokens} />
      <h2>Grammar</h2>
      <Grammar g={syntax} />
    </div>
  );
}
