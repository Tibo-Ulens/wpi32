#![feature(is_ascii_octdigit)]
#![feature(let_chains)]
#![feature(assert_matches)]

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::path::Path;

mod lex;
mod parse;

use common::{Error, LocationInfo};
use lex::{Lexer, Token};
use parse::Parser;
use ptree::print_tree;

use crate::parse::Node;

impl<'s> From<&Token<'s>> for LocationInfo {
	fn from(value: &Token<'s>) -> Self {
		Self {
			line:     value.line,
			col:      value.col,
			span:     value.span,
			src_line: value.source_line.to_string(),
		}
	}
}

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

	let _ = print_tree(&Node::from(&ast_root));

	Ok(())
}
