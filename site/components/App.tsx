import React from "react";
import Katex from "./Katex";
import { mul } from "../../core/pkg/birb_core";

export default function App() {
  return (
    <div>
      <h1>hello, world</h1>
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
