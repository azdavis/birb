import React from "react";
import katex from "katex";

type Props = {
  v: string;
  block?: boolean;
};

type DangerousHTML = {
  __html: string;
};

function fromTeXtoHTML(tex: string, displayMode: boolean): DangerousHTML {
  return { __html: katex.renderToString(tex, { displayMode }) };
}

export default function Katex({ v, block }: Props) {
  return <span dangerouslySetInnerHTML={fromTeXtoHTML(v, block === true)} />;
}
