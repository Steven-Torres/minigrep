use minigrep::*;

#[test]
fn case_sensitive() {
  let query = "duct";
  let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

  assert_eq!(
    vec![("safe, fast, productive.", 15, 19)],
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

  assert_eq!(
    vec![
      ("Rust:", 0, 4),
      ("Trust me.", 1, 5)
    ],
    search(query, contents, true).unwrap()
  );
}