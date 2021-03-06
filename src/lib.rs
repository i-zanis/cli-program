use std::{fs, env};
use std::error::Error;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "alright";
    let contents = "\
I'm in love alri-i-i-ight, with my crazy beautiful life
With the parties, the disasters
With my friends all pretty and plastered
Every night we're down to go out
Wakin' up on a different couch
Till the next night, on the next flight
Yeah, I guess we're doin' alright.
Ke$ha";
    assert_eq!(vec!["crazy, cyanide, Crashdïet."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "hIgH";
    let contents = "\
  Every single night we fight to get to get a little high on life.";
    assert_eq!(
      vec!["high, life."],
      search_case_insensitive(query, contents)
    );
  }
}

pub struct Config {
  pub query: String,
  pub file_name: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("Insufficient amount of arguments");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      file_name,
      case_sensitive,
    })
  }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line)
    }
  }
  result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }
  Ok(())
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

