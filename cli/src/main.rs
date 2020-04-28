fn main() {
  let file = std::env::args().nth(1).expect("could not get filename");
  let bs = std::fs::read(&file).expect("could not read file");
  let res = birb_core::get(&bs).expect("interpretation failed");
  println!("{:?}", res);
}
