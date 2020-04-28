export default function classNames(...xs: (string | null)[]): string {
  const ys = [];
  for (const x of xs) {
    if (x !== null) {
      ys.push(x);
    }
  }
  return ys.join(" ");
}
