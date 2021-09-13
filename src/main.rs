use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let (query, file_name) = parse_config(&args);

  print!("Seeking {}", config.query);
  print!("In the file {}", config.file_name);

  let contents = fs::read_to_string(config.file_name)
    .expect("There was an unknown error");
}

struct Config {
  query: String,
  file_name: String,
}

fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let file_name = args[2].clone();

  Config { query, file_name }
}

fn new(args: &[String]) -> Result<Config, &str> {
  if args.len() < 3 {
    return Err("Insufficient arguments");
  }
  let query = args[1].clone();
  let file_name = args[2].clone();

  Ok(Config {query, file_name})
}



