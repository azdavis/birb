import React from "react";
import classNames from "../classNames";
import { get } from "../../wasm/pkg/birb_wasm";

function must<T>(x: T | null): T {
  if (x == null) {
    throw new Error("must(null)");
  }
  return x;
}

function safeGet(x: string): string {
  try {
    return get(x);
  } catch (e) {
    return String(e);
  }
}

const startingText = `// pretend this prints to standard output
fn print_nat(x: Nat): Nat affects Stdout ensures ret.gt(x) {
  x.add(1)
}

enum Option[T: Type] {
  some(T),
  none(()),
}

fn combine
  [T: Type, U: Type]
  (x: Option[T], y: Option[U])
  : Option[(T, U)]
{
  match (x, y) {
    (some(a), some(b)) { some[(T, U)]((a, b)) }
    (_, _) { none[(T, U)](()) }
  }
}

// try adding an 'affects' annotation here
fn main(): Nat {
  let num = match some[Nat](150).combine[Nat, Nat](some[Nat](151)) {
    some((a, b)) { a.mul(100).add(b) }
    none(()) { 321 }
  };
  let x = print_nat(num);
  x.sub(2)
}`;

export default function Sandbox() {
  const textarea = React.useRef<HTMLTextAreaElement | null>(null);
  const [text, setText] = React.useState<string>(startingText);
  const onSubmit = React.useCallback(
    (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      setText(must(textarea.current).value);
    },
    [setText, textarea],
  );
  return (
    <div>
      <form onSubmit={onSubmit}>
        <pre className="p-0-5-em bg-gray round-corners overflow-auto">{safeGet(text)}</pre>
        <textarea
          ref={textarea}
          className="resize-none d-block ff-mono fz-inherit w-100 h-20em bg-none color-inherit"
          defaultValue={startingText}
          spellCheck="false"
        ></textarea>
        <button
          type="submit"
          className={classNames(
            "d-block",
            "w-100",
            "fz-1-5-em",
            "p-0-5-em",
            "border-0",
            "cursor-pointer",
            "bg-green",
            "active-bg-light-green",
            "color-inherit",
          )}
        >
          Run
        </button>
      </form>
    </div>
  );
}
