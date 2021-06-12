use std::{
	io::{self, Read},
	path::PathBuf,
	error::Error,
	fs
};

pub fn get_contents(filename: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
	let mut buffer = String::new();

	if let Some(path) = filename {
		buffer = fs::read_to_string(path)?;
	} else {
		let mut reader = io::stdin();
		reader.read_to_string(&mut buffer)?;
	}

	Ok(buffer)
}

pub fn is_match(ignore_case: bool, line: &str, query: &str) -> bool {
	if ignore_case {
		line.to_lowercase().contains(query)
	} else {
		line.contains(query)
	}
}

pub fn get_indexes(line: &str, query: &str) -> Result<(usize, usize), String> {
	let start_index = line.find(&query)
		.or_else(|| line.to_lowercase().find(&query));

	let start_index = match start_index {
		None => return Err(String::from("cannot read lines")),
		Some(index) => index
	};

	let end_index = start_index + query.len();

	Ok((start_index, end_index))
}

pub fn highlight_match(match_info: (&str, usize, usize)) -> Result<String, String> {
	let red = "\x1b[0;31m";
	let no_color = "\x1b[0m";

	let (s, start_index, end_index) = match_info;

	let mut s = s.to_string();

	s.insert_str(end_index, no_color);
	s.insert_str(start_index, red);

	Ok(s)
}