use std::env;
use std::process;
use cli_program::Config;

fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Parsing arguments error: {}", err);
    process::exit(1);
  });

  if let Err(e) = cli_program::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
