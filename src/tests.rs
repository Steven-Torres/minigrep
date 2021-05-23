use minigrep::*;

#[test]
fn case_sensitive() {
  let query = "duct";
  let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

  let red = "\x1b[0;31m";
  let no_color = "\x1b[0m";

  assert_eq!(
    vec![String::from(format!("safe, fast, pro{}duct{}ive.", red, no_color))],
    search(query, contents, false).unwrap()
  );
}

#[test]
fn case_insensitive() {
  let query = "rUsT";
  let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

  let red = "\x1b[0;31m";
  let no_color = "\x1b[0m";

  assert_eq!(
    vec![
      String::from(format!("{}Rust{}:", red, no_color)),
      String::from(format!("T{}rust{} me.", red, no_color))
      ],
    search(query, contents, true).unwrap()
  );
}