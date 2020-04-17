export type AbsSyntax = Production[];

export type Production = {
  text: string;
  math: string;
  def: Alternative[];
};

export type Alternative = {
  math: string;
  text: string;
};
