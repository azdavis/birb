import React from "react";
import absurd from "../absurd";
import Sandbox from "./Sandbox";
import About from "./About";
import ConcreteSyntax from "./ConcreteSyntax";
import AbsSyntax from "./AbstractSyntax";
import Judgements from "./Judgements";
import classNames from "../classNames";

type Page = "About" | "Sandbox" | "Concrete Syntax" | "Abstract Syntax" | "Judgements";
const pages: Page[] = ["About", "Sandbox", "Concrete Syntax", "Abstract Syntax", "Judgements"];

function switcher(page: Page) {
  switch (page) {
    case "About":
      return <About />;
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
  const [page, setPage] = React.useState<Page>("About");
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
      <nav className="d-flex flex-wrap jc-center p-0-5-em">{pages.map(navItem)}</nav>
      {switcher(page)}
    </div>
  );
}
