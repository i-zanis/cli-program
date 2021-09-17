#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "yow!";
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
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  vec![]
}