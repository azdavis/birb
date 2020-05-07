import React from "react";
import classNames from "../classNames";
import { get } from "../../wasm/pkg/birb_wasm";

function must<T>(x: T | null): T {
  if (x == null) {
    throw new Error("must(null)");
  }
  return x;
}

const startingText = "hey";

export default function Sandbox() {
  const textarea = React.useRef<HTMLTextAreaElement | null>(null);
  const [text, setText] = React.useState<string>(startingText);
  const onSubmit = React.useCallback(
    (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      const { current } = textarea;
      setText(must(current).value);
    },
    [setText, textarea],
  );
  return (
    <div>
      <form onSubmit={onSubmit}>
        <div>{get(text)}</div>
        <textarea
          ref={textarea}
          className="resize-none d-block ff-mono fz-inherit w-100 h-10em bg-none color-inherit"
          defaultValue={startingText}
        ></textarea>
        <input
          type="submit"
          value="Run"
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
        />
      </form>
    </div>
  );
}
