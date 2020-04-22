struct Guy[T: Type] { x: T }
fn call[
  T: Type,
  U: Type,
  E: Effect,
](
  f: T -> U affects {E},
  x: T,
):
  U affects {E}
  requires true
  ensures true
{
  // move semantics?
  let _ = Guy[T] { x }
  // match the empty tuple against the single-arm match
  // and upon match, evaluate to the empty tuple
  let _ = match () { () { () } }
  x.f()
}
