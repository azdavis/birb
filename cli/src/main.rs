fn run() -> Result<birb_core::interpret::Value, Box<dyn std::error::Error>> {
  let file = match std::env::args().nth(1) {
    Some(x) => x,
    None => return Err("give exactly 1 argument (a filename)".into()),
  };
  let bs = std::fs::read(&file)?;
  let res = birb_core::get(&bs)?;
  Ok(res)
}

fn main() {
  match run() {
    Ok(v) => println!("{}", v),
    Err(e) => {
      eprintln!("error: {}", e);
      std::process::exit(1)
    }
  }
}
