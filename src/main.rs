extern crate structopt;

use std::fs;
use std::error::Error;
use std::process;
use std::path::PathBuf;
use structopt::StructOpt;
use chrono::{DateTime, Local};

#[derive(StructOpt, Debug)]
struct Opt {
	/// Output file
	#[structopt(default_value = ".", parse(from_os_str))]
	path: PathBuf,
}

fn main() {
	let opt = Opt::from_args();
	if let Err(ref e) = run(&opt.path) {
			println!("{}", e);
			process::exit(1);
	}
}

fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let file_name = entry.path();

			let metadata = entry.metadata()?;
			let size = metadata.len();
			let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

			println!(
				"{:>5} {} {:?}",
				size,
				modified.format("%_d %b %H:%M").to_string(),
				file_name
			);
		}
	}
	Ok(())
}