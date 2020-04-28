import React from "react";
import { mul } from "../../wasm/pkg/birb_wasm";

const startingNum = 3;
const otherNum = 5;

export default function Sandbox() {
  const input = React.useRef<HTMLInputElement | null>(null);
  const [err, setErr] = React.useState<string | null>(null);
  const [num, setNum] = React.useState<number>(startingNum);
  const onSubmit = React.useCallback(
    (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      const { current } = input;
      if (current === null) {
        setErr("no input element found");
        return;
      }
      const num = parseInt(current.value, 10);
      if (isNaN(num)) {
        setErr("unable to parse number");
        return;
      }
      setErr(null);
      setNum(num);
    },
    [setErr, setNum, input],
  );
  return (
    <div>
      <h2>Sandbox</h2>
      <form onSubmit={onSubmit}>
        <div>
          {otherNum} * {num} = {mul(otherNum, num)}
        </div>
        <input type="text" ref={input} defaultValue={startingNum} />
        <input type="submit" value="Run" />
        {err === null ? null : <div className="c-red fw-bold">{err}</div>}
      </form>
    </div>
  );
}
