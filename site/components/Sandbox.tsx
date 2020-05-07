import React from "react";
import { mul } from "../../wasm/pkg/birb_wasm";

const startingNum = 3;
const otherNum = 5;

export default function Sandbox() {
  const textarea = React.useRef<HTMLTextAreaElement | null>(null);
  const [err, setErr] = React.useState<string | null>(null);
  const [num, setNum] = React.useState<number>(startingNum);
  const onSubmit = React.useCallback(
    (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      const { current } = textarea;
      if (current === null) {
        setErr("no textarea element found");
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
    [setErr, setNum, textarea],
  );
  return (
    <div>
      <h2>Sandbox</h2>
      <form onSubmit={onSubmit}>
        <div>
          {otherNum} * {num} = {mul(otherNum, num)}
        </div>
        <textarea ref={textarea}></textarea>
        <input type="submit" value="Run" />
        {err === null ? null : <div className="c-red fw-bold">{err}</div>}
      </form>
    </div>
  );
}
