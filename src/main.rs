use std::env;
use std::process;

use minigrep::Config;
use cli_program::Config;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Parsing arguments error: {}", err);
    process::exit(1);
  });

  if let Err(e) = minigrep::run(config) {
    eprintln!("Error: {}", e);

    process::exit(1);
  }
}
