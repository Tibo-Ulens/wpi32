#![allow(missing_docs)]

use std::fmt::{Display, Formatter};

use super::print::{make_info_body, make_info_header};
use super::LocationInfo;

#[derive(Debug)]
pub enum ParseError {
	UnexpectedEof {
		src_file: String,
		location: Box<LocationInfo>,
	},
	UnexpectedToken {
		src_file: String,
		location: Box<LocationInfo>,
		found:    String,
		expected: String,
	},
	UnclosedDelimiter {
		src_file:       String,
		delim_type:     String,
		found:          String,
		close_location: Box<LocationInfo>,
		open_location:  Box<LocationInfo>,
	},
	UnmatchedCloseParenthesis {
		src_file: String,
		location: Box<LocationInfo>,
	},
	InvalidOrderingSpecifier {
		src_file: String,
		location: Box<LocationInfo>,
		spec:     String,
	},
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let repr = match self {
			Self::UnexpectedEof { src_file, location } => {
				let mut pretty_err = make_info_header("unexpected end-of-file", src_file, location);

				pretty_err.push_str(&make_info_body(None, location));

				pretty_err
			},
			Self::UnexpectedToken { src_file, location, found, expected } => {
				let mut pretty_err = make_info_header(
					&format!("found unexpected token `{}`, expected `{}`", found, expected),
					src_file,
					location,
				);

				pretty_err.push_str(&make_info_body(None, location));

				pretty_err
			},
			Self::UnclosedDelimiter {
				src_file,
				delim_type,
				found,
				close_location,
				open_location,
			} => {
				let mut pretty_err = make_info_header(
					&format!("found unexpected token `{}`, expected closing {}", found, delim_type),
					src_file,
					close_location,
				);

				pretty_err.push_str(&make_info_body(None, close_location));

				pretty_err.push_str(&make_info_body(
					Some(&format!("unclosed {}", delim_type)),
					open_location,
				));

				pretty_err
			},
			Self::UnmatchedCloseParenthesis { src_file, location } => {
				let mut pretty_err =
					make_info_header("unmatched closing parenthesis", src_file, location);

				pretty_err.push_str(&make_info_body(None, location));

				pretty_err
			},
			Self::InvalidOrderingSpecifier { src_file, location, spec } => {
				let mut pretty_err = make_info_header(
					&format!("invalid ordering specifier `{:?}`", spec),
					src_file,
					location,
				);

				pretty_err.push_str(&make_info_body(None, location));

				pretty_err
			},
		};

		write!(f, "{}", repr)
	}
}
