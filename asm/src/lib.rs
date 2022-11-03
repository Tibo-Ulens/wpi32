#![feature(is_ascii_octdigit)]

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::path::Path;

mod lex;

use common::Error;
use lex::Lexer;

pub fn assemble(input_path: &Path, _output_path: &Path) -> Result<(), Error> {
	let mut file = File::open(input_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let lexer = Lexer::new(&contents);
	for token in lexer {
		debug!("{}", token?);
	}

	Ok(())
}
