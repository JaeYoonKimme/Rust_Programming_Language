use std::env;
use std::process;

use minigrep::Config;

fn main() {
	let config = Config::new(&args).unwrap_or_else(|err| {
			let args: Vec<String> = eng::args().collect();

			eprintln!("Problem parsing arguments: {}",err);
			process::exit(1);
			});


	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}",e);
		process::exit(1);
	}
}
