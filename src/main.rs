use std::process;

use minigrep::Config;
use structopt::StructOpt;

fn main() {
	let config = Config::from_args();

	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}