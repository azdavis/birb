import React from "react";

export default function About() {
  return (
    <div>
      <h2>About</h2>
      <p>
        Birb is a programming language. It is designed and implemented by Vivian Huang (vivianh) and
        Ariel Davis (azdavis). Its source code is available on{" "}
        <a href="https://github.com/azdavis/birb">GitHub</a>.
      </p>
      <h2>Features</h2>
      <h3>Compile-time effects checking</h3>
      <p>
        A function must statically declare all of the effects that it may use. If a caller calls a
        function that uses effects that the caller itself does not have access to, the compiler
        rejects the code before it runs.
      </p>
      <h3>Contracts</h3>
      <p>
        Pre-conditions and post-conditions can be added to a function with <code>requires</code> and{" "}
        <code>ensures</code>, respectively. These contracts are checked dynamically at runtime, and
        execution is halted if they are not satisfied.
      </p>
      <h3>Unified function call syntax</h3>
      <p>
        Birb permits writing function calls like <code>f(x)</code> or <code>g(a, b, c)</code>. But
        Birb also supports a "method call" syntactic sugar. Thus <code>x.f()</code> is exactly
        equivalent to <code>f(x)</code>. And <code>a.g(b, c)</code> is exactly equivalent to{" "}
        <code>g(a, b, c)</code>. And so on.
      </p>
      <h3>Rich type system</h3>
      <p>
        Birb has some built-in types. Nat is the type of natural numbers, and Str is the type of
        strings.
      </p>
      <p>
        Birb allows for user-definable structs (product types) and enums (sum types). It also has
        tuples (anonymous products types).
      </p>
      <p>Birb supports generic types and functions via parametric polymorphism.</p>
    </div>
  );
}
