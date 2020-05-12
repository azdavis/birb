import React from "react";

export default function About() {
  return (
    <div>
      <h2>About</h2>
      <p>
        Birb is a programming language. It is designed and implemented by Vivian Huang (vivianh) and
        Ariel Davis (azdavis).
      </p>
      <p>
        One headline feature of Birb is compile-time effects checking. A function must statically
        declare all of the effects that it uses. If a caller calls a function that uses effects that
        the caller does not have access to, the compiler rejects the code before it runs.
      </p>
      <p>
        Birb also features contracts. Pre-conditions and post-conditions can be added to a function
        with <code>requires</code> and <code>ensures</code>, respectively. These contracts are
        checked dynamically at runtime, and execution is halted if they fail.
      </p>
      <p>
        Birb also has some of the usual features of a general-purpose programming languages, like
        user-definable product types (structs), sum types (enums), and functions. It has some
        built-in types. Nat is the type of natural numbers, and Str is the type of strings. It also
        has tuples.
      </p>
    </div>
  );
}
