#![allow(missing_docs)]

use std::fmt::{Display, Formatter};

use super::print::{make_info_body, make_info_header};
use super::LocationInfo;

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
		found:    char,
		expected: Vec<char>,
	},
	RawUnexpectedSymbol {
		src_file: String,
		line:     usize,
		col:      usize,
		src_line: String,
		found:    char,
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
}

impl Display for LexError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let repr = match self {
			Self::UnexpectedEof { src_file, line, col, src_line } => {
				let location = LocationInfo::new(*line, *col, 1, src_line);
				let mut pretty_err =
					make_info_header("unexpected end-of-file", src_file, &location);

				pretty_err.push_str(&make_info_body(None, &location));

				pretty_err
			},
			Self::UnexpectedSymbol { src_file, line, col, src_line, found, expected } => {
				let location = LocationInfo::new(*line, *col, 1, src_line);
				let mut pretty_err = make_info_header(
					&format!(
						"found unexpected symbol `{:?}`, expected `{:?}`",
						found,
						expected
							.iter()
							.map(|c| c.to_string())
							.collect::<Vec<String>>()
							.join(" or ")
					),
					src_file,
					&location,
				);

				pretty_err.push_str(&make_info_body(None, &location));

				pretty_err
			},
			Self::RawUnexpectedSymbol { src_file, line, col, src_line, found } => {
				let location = LocationInfo::new(*line, *col, 1, src_line);
				let mut pretty_err = make_info_header(
					&format!("found unexpected symbol `{:?}`", found),
					src_file,
					&location,
				);

				pretty_err.push_str(&make_info_body(None, &location));

				pretty_err
			},
			Self::InvalidNumber { src_file, line, col, span, src_line } => {
				let location = LocationInfo::new(*line, *col, *span, src_line);
				let mut pretty_err = make_info_header("invalid number", src_file, &location);

				pretty_err.push_str(&make_info_body(None, &location));

				pretty_err
			},
			Self::InvalidEscape { src_file, line, col, span, src_line } => {
				let location = LocationInfo::new(*line, *col, *span, src_line);
				let mut pretty_err =
					make_info_header("invalid escape sequence", src_file, &location);

				pretty_err.push_str(&make_info_body(None, &location));

				pretty_err
			},
		};

		write!(f, "{}", repr)
	}
}
