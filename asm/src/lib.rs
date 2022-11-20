#![feature(is_ascii_octdigit)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]
#![feature(assert_matches)]

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::path::Path;

mod lex;
mod parse;

use common::Error;
use lex::{Lexer, Token};
use parse::Parser;

pub fn assemble(input_path: &Path, _output_path: &Path) -> Result<(), Error> {
	let src_file = input_path.to_string_lossy().to_string();
	let mut file = File::open(input_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let lexer = Lexer::new(&src_file, &contents);
	let tokens: Vec<Token> = lexer.into_iter().collect::<Result<Vec<Token>, Error>>()?;

	for token in &tokens {
		debug!("{:?}", token);
	}

	let mut parser = Parser::new(&src_file, &tokens);
	let ast_root = parser.parse()?;

	debug!("{:?}", ast_root);

	Ok(())
}
