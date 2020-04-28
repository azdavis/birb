import React from "react";
import absurd from "../absurd";
import Sandbox from "./Sandbox";
import ConcreteSyntax from "./ConcreteSyntax";
import AbsSyntax from "./AbstractSyntax";
import Judgements from "./Judgements";
import classNames from "../classNames";

type Page = "Sandbox" | "Concrete Syntax" | "Abstract Syntax" | "Judgements";
const pages: Page[] = ["Sandbox", "Concrete Syntax", "Abstract Syntax", "Judgements"];

function switcher(page: Page) {
  switch (page) {
    case "Sandbox":
      return <Sandbox />;
    case "Concrete Syntax":
      return <ConcreteSyntax />;
    case "Abstract Syntax":
      return <AbsSyntax />;
    case "Judgements":
      return <Judgements />;
    default:
      return absurd(page);
  }
}

export default function App() {
  const [page, setPage] = React.useState<Page>("Sandbox");
  const navItem = React.useCallback(
    (x: Page) => (
      <div
        key={x}
        className={classNames(
          "p-0-5-em",
          "cursor-pointer",
          "td-underline",
          "round-corners",
          page === x ? "bg-gray" : null,
        )}
        onClick={() => {
          setPage(x);
        }}
      >
        {x}
      </div>
    ),
    [page, setPage],
  );
  return (
    <div>
      <h1>Birb</h1>
      <p>Ariel Davis (azdavis), Vivian Huang (vivianh)</p>
      <nav className="d-flex jc-center">{pages.map(navItem)}</nav>
      {switcher(page)}
    </div>
  );
}
