use std::fmt::{Display, Formatter};

use super::make_info_block;

#[derive(Debug)]
pub enum ParseError {
	UnexpectedEof {
		src_file: String,
		line:     usize,
		col:      usize,
		src_line: String,
	},
	UnexpectedToken {
		src_file: String,
		line:     usize,
		col:      usize,
		span:     usize,
		src_line: String,
		fnd:      String,
		ex:       String,
	},
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let repr = match self {
			Self::UnexpectedEof { src_file, line, col, src_line } => {
				make_info_block("unexpected end-of-file", src_file, *line, *col, 1, src_line)
			},
			Self::UnexpectedToken { src_file, line, col, span, src_line, fnd, ex } => {
				make_info_block(
					&format!("found unexpected token `{}`, expected `{}`", fnd, ex),
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
