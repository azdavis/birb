fn main() {
  let file = std::env::args().nth(1).expect("could not get filename");
  let bs = std::fs::read(&file).expect("could not read file");
  match birb_core::get(&bs) {
    Ok(v) => println!("{:?}", v),
    Err(e) => eprintln!("{}", e),
  }
}
