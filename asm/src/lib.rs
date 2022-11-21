//! # Assembler
//!
//! The assembler is responsible for converting a text file into a binary that
//! can be executed by the simulator <br>
//! This is achieved using the following steps:
//!  - Lexing: The [`Lexer`] converts the raw text of the input file into a series of abstracted
//!    [`Token`]s that more conveniently represent their underlying data
//!  - Parsing: The [`Parser`] converts the stream of [`Token`]s generated by the [`Lexer`] into a
//!    structured representation called an Abstract Syntax Tree (AST) (see [parse::ast])
//!
//!  - TODO: Label resolution: Resolve labels to their final value
//!  - TODO: Fold constants: Calculate immediates at runtime
//!  - TODO: Code generation: Output binary data

#![warn(missing_docs)]
#![feature(is_ascii_octdigit)]
#![feature(let_chains)]
#![feature(assert_matches)]

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod error;
pub mod lex;
pub mod parse;

use error::Error;
use lex::{Lexer, Token};
use parse::Parser;
use ptree::print_tree;

use crate::parse::Node;

/// Assemble a file at the given input path into a binary, and write it to the
/// file given by the output path
///
/// See the [module level documentation](self) for more info
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
