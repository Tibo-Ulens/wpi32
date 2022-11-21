use std::fmt::{Display, Formatter};

use super::{make_info_block, LocationInfo};

#[derive(Debug)]
pub enum ParseError {
	UnexpectedEof {
		src_file: String,
		location: Box<LocationInfo>,
	},
	UnexpectedToken {
		src_file: String,
		location: Box<LocationInfo>,
		fnd:      String,
		ex:       String,
	},
	UnclosedParenthesis {
		src_file:       String,
		close_location: Box<LocationInfo>,
		open_location:  Box<LocationInfo>,
	},
	UnclosedBracket {
		src_file:       String,
		close_location: Box<LocationInfo>,
		open_location:  Box<LocationInfo>,
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
				make_info_block(
					"unexpected end-of-file",
					src_file,
					location.line,
					location.col,
					1,
					&location.src_line,
				)
			},
			Self::UnexpectedToken { src_file, location, fnd, ex } => {
				make_info_block(
					&format!("found unexpected token `{}`, expected `{}`", fnd, ex),
					src_file,
					location.line,
					location.col,
					location.span,
					&location.src_line,
				)
			},
			Self::UnclosedParenthesis { src_file, close_location, open_location } => {
				let err = make_info_block(
					"expected closing parenthesis",
					src_file,
					close_location.line,
					close_location.col,
					close_location.span,
					&close_location.src_line,
				);
				let origin = make_info_block(
					"unclosed parenthesis",
					src_file,
					open_location.line,
					open_location.col,
					open_location.span,
					&open_location.src_line,
				);

				format!("{}\n{}", err, origin)
			},
			Self::UnclosedBracket { src_file, close_location, open_location } => {
				let err = make_info_block(
					"expected closing bracket",
					src_file,
					close_location.line,
					close_location.col,
					close_location.span,
					&close_location.src_line,
				);
				let origin = make_info_block(
					"unclosed bracket",
					src_file,
					open_location.line,
					open_location.col,
					open_location.span,
					&open_location.src_line,
				);

				format!("{}\n{}", err, origin)
			},
			Self::InvalidOrderingSpecifier { src_file, location, spec } => {
				make_info_block(
					&format!("invalid ordering specifier `{:?}`", spec),
					src_file,
					location.line,
					location.col,
					location.span,
					&location.src_line,
				)
			},
		};

		write!(f, "{}", repr)
	}
}
