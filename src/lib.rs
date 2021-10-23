use std::{
	error::Error,
	io::{self, Write},
	path::PathBuf,
};
use structopt::StructOpt;

mod utils;
use utils::{get_contents, get_indexes, highlight_match, is_match};

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
	#[structopt(short, long, help = "Do a case insensitive search")]
	pub ignore_case: bool,
	#[structopt(short, long, help = "Exclude color codes from output")]
	pub no_color: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = get_contents(config.filename)?;

	if config.query.is_some() {
		let results = search(&config.query.unwrap(), &contents, config.ignore_case)?;
		let mut handle = io::BufWriter::new(io::stdout());
		for line in results {
			let line = if config.no_color {
				String::from(line.0)
			} else {
				highlight_match(line)?
			};
			writeln!(handle, "{}", line)?;
		}
	}

	Ok(())
}

pub fn search<'a>(
	query: &str,
	contents: &'a str,
	ignore_case: bool,
) -> Result<Vec<(&'a str, usize, usize)>, Box<dyn Error>> {
	let query = if ignore_case {
		query.to_lowercase()
	} else {
		String::from(query)
	};
	let mut results = Vec::new();

	for line in contents.lines() {
		if is_match(ignore_case, line, &query) {
			let (start_index, end_index) = get_indexes(line, &query)?;
			results.push((line, start_index, end_index));
		}
	}

	Ok(results)
}
