use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "grep, but mini.")]
pub struct Config {
	#[structopt(help = "Query to search for in input")]
	pub query: String,
	#[structopt(
		parse(from_os_str),
		help = "File input or path to file input to search (leave out for standard input)"
	)]
	pub filename: Option<PathBuf>,
	#[structopt(
		short,
		long,
		help = "Do a case insensitive search"
	)]
	pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = get_contents(config.filename)?;

	let results = search(&config.query, &contents, config.ignore_case)?;

	let mut handle = io::BufWriter::new(io::stdout());
	for line in results {
		writeln!(handle, "{}", line)?;
	}

	Ok(())
}

pub fn search(
	query: &str,
	contents: &str,
	ignore_case: bool
) -> Result<Vec<String>, Box<dyn Error>> {
	let query = if ignore_case { query.to_lowercase() } else { String::from(query) };
	let mut results = Vec::new();

  for line in contents.lines() {
		if !ignore_case && line.contains(&query) ||
		ignore_case && line.to_lowercase().contains(&query) {
			let s = highlight_match(line, &query)?;
			results.push(s);
		}
	}

	Ok(results)
}

fn get_contents(filename: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
	let mut buffer = String::new();

	if filename.is_some() {
		let path = filename.unwrap();
		let f = File::open(path)?;
		let mut reader = io::BufReader::new(f);
		reader.read_to_string(&mut buffer)?;
	} else {
		let mut reader = io::BufReader::new(io::stdin());
		reader.read_to_string(&mut buffer)?;
	}

	Ok(buffer)
}

fn highlight_match(line: &str, query: &str) -> Result<String, String> {
	let red = "\x1b[0;31m";
	let no_color = "\x1b[0m";

	let start_index = line.find(&query)
		.or_else(|| line.to_lowercase().find(&query));

	let start_index = match start_index {
		None => return Err("cannot read lines".to_string()),
		Some(index) => index
	};

	let end_index = start_index + query.len() + red.len();

	let mut s = String::from(line);
	s.insert_str(start_index, red);
	s.insert_str(end_index, no_color);

	Ok(s)
}