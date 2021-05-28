use std::io::{self, Read, Write};
use std::process::Command;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "grep, but mini.")]
pub struct Config {
	#[structopt(help = "Query to search for in input", required_unless = "update")]
	pub query: Option<String>,
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
	#[structopt(short, long, help = "Update to the latest version of minigrep")]
	pub update: bool
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	if config.update && config.query.is_none() {
		println!("Updating...");

		Command::new("sh")
			.args(&[
				"-c",
				"curl -fsSL https://raw.githubusercontent.com/Steven-Torres/minigrep/main/install.sh | sh"
			])
			.output()?;
			
		println!("Minigrep successfully updated!");
		return Ok(());
	}

	let contents = get_contents(config.filename)?;

	if config.query.is_some() {
		let results = search(&config.query.unwrap(), &contents, config.ignore_case)?;
	
		let mut handle = io::BufWriter::new(io::stdout());
		for line in results {
			let line = highlight_match(line)?;
			writeln!(handle, "{}", line)?;
		}
	}

	Ok(())
}

pub fn search<'a>(
	query: &str,
	contents: &'a str,
	ignore_case: bool
) -> Result<Vec<(&'a str, usize, usize)>, Box<dyn Error>> {
	let query = if ignore_case { query.to_lowercase() } else { String::from(query) };
	let mut results = Vec::new();

  for line in contents.lines() {
		if is_match(ignore_case, line, &query) {
			let (start_index, end_index) = get_indexes(line, &query)?;
			let m = (line, start_index, end_index);
			results.push(m);
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

fn is_match(ignore_case: bool, line: &str, query: &str) -> bool {
	!ignore_case && line.contains(query) ||
	ignore_case && line.to_lowercase().contains(query)
}

fn get_indexes(line: &str, query: &str) -> Result<(usize, usize), String> {
	let start_index = line.find(&query)
		.or_else(|| line.to_lowercase().find(&query));

	let start_index = match start_index {
		None => return Err("cannot read lines".to_string()),
		Some(index) => index
	};

	let end_index = start_index + query.len();

	Ok((start_index, end_index))
}

fn highlight_match(match_info: (&str, usize, usize)) -> Result<String, String> {
	let red = "\x1b[0;31m";
	let no_color = "\x1b[0m";

	let (s, start_index, end_index) = match_info;

	let mut s = s.to_string();

	s.insert_str(end_index, no_color);
	s.insert_str(start_index, red);

	Ok(s)
}