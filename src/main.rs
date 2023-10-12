  use std::process;
use near_cli_new::{run};

  fn main() {
        println!("NEAR CLI - New Project Generator");
        println!("*--------------------------------*");
      if let Err(e) = run(){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}


