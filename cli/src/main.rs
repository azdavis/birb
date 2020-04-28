const BIG_STACK_SIZE: usize = 180 * 1024 * 1024;

fn run() -> birb_core::error::Result<()> {
  let file = std::env::args().nth(1).expect("could not get filename");
  let bs = std::fs::read(&file).expect("could not read file");
  birb_core::get(&bs)
}

fn main() {
  match std::thread::Builder::new()
    .name("run".to_string())
    .stack_size(BIG_STACK_SIZE)
    .spawn(run)
    .expect("could not spawn thread")
    .join()
  {
    Ok(Ok(v)) => println!("{:?}", v),
    Ok(Err(e)) => println!("error: {}", e),
    Err(e) => eprintln!("panic: {:?}", e),
  }
}
