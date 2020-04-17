import React from "react";
import ReactDOM from "react-dom";
import Katex from "./components/Katex";
import { mul } from "../core/pkg/birb_core";
import "./index.css";

function App() {
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

ReactDOM.render(<App />, document.getElementById("root"));
