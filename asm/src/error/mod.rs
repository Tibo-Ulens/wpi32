//! Assembler error types and utility functions

use std::fmt::{Display, Formatter};

mod lex_error;
mod macro_error;
mod parse_error;
mod print;

pub use lex_error::LexError;
pub use macro_error::MacroError;
pub use parse_error::ParseError;

use crate::lex::Token;

/// Any possible error produced during assembly
#[derive(Debug)]
pub enum Error {
	/// Wrapper around [`std::io::Error`]
	Io(std::io::Error),
	/// An error produced by the [`Lexer`](crate::lex::Lexer)
	Lex(LexError),
	/// An error produced by the [`Parser`](crate::parse::Parser)
	Parse(ParseError),
}

/// Information on where exactly an error occured, can be generated from
/// Lexer tokens
#[derive(Debug)]
pub struct LocationInfo {
	line:     usize,
	col:      usize,
	span:     usize,
	src_line: String,
}

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

impl LocationInfo {
	fn new(line: usize, col: usize, span: usize, src_line: &str) -> Self {
		Self { line, col, span, src_line: src_line.to_string() }
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Io(err) => write!(f, "{}", err),
			Self::Lex(err) => write!(f, "{}", err),
			Self::Parse(err) => write!(f, "{}", err),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self { Self::Io(value) }
}

impl From<LexError> for Error {
	fn from(value: LexError) -> Self { Self::Lex(value) }
}

impl From<ParseError> for Error {
	fn from(value: ParseError) -> Self { Self::Parse(value) }
}
