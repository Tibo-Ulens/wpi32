#![allow(missing_docs)]

use std::fmt::{Display, Formatter};

use super::make_info_block;

/// An error produced by the [`Lexer`](crate::lex::Lexer)
#[derive(Debug)]
pub enum LexError {
	UnexpectedEof {
		src_file: String,
		line:     usize,
		col:      usize,
		src_line: String,
	},
	UnexpectedSymbol {
		src_file: String,
		line:     usize,
		col:      usize,
		src_line: String,
		fnd:      char,
		ex:       char,
	},
	RawUnexpectedSymbol {
		src_file: String,
		line:     usize,
		col:      usize,
		src_line: String,
		fnd:      char,
	},
	InvalidNumber {
		src_file: String,
		line:     usize,
		col:      usize,
		span:     usize,
		src_line: String,
	},
	InvalidEscape {
		src_file: String,
		line:     usize,
		col:      usize,
		span:     usize,
		src_line: String,
	},
	InvalidDirective {
		src_file: String,
		line:     usize,
		col:      usize,
		span:     usize,
		src_line: String,
		dir:      String,
	},
}

impl Display for LexError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let repr = match self {
			Self::UnexpectedEof { src_file, line, col, src_line } => {
				make_info_block("unexpected end-of-file", src_file, *line, *col, 1, src_line)
			},
			Self::UnexpectedSymbol { src_file, line, col, src_line, fnd, ex } => {
				make_info_block(
					&format!("found unexpected symbol '{:?}', expected '{:?}'", fnd, ex),
					src_file,
					*line,
					*col,
					1,
					src_line,
				)
			},
			Self::RawUnexpectedSymbol { src_file, line, col, src_line, fnd } => {
				make_info_block(
					&format!("found unexpected symbol '{:?}'", fnd),
					src_file,
					*line,
					*col,
					1,
					src_line,
				)
			},
			Self::InvalidNumber { src_file, line, col, span, src_line } => {
				make_info_block("invalid number", src_file, *line, *col, *span, src_line)
			},
			Self::InvalidEscape { src_file, line, col, span, src_line } => {
				make_info_block("invalid escape sequence", src_file, *line, *col, *span, src_line)
			},
			Self::InvalidDirective { src_file, line, col, span, src_line, dir } => {
				make_info_block(
					&format!("invalid directive '{:?}'", dir),
					src_file,
					*line,
					*col,
					*span,
					src_line,
				)
			},
		};

		write!(f, "{}", repr)
	}
}
