use std::fmt::{Display, Formatter};

mod lex_error;
mod print;

pub use lex_error::LexError;
pub(crate) use print::make_info_block;

#[derive(Debug)]
pub enum Error {
	WrongFileType { found: String, expected: String },
	Io(std::io::Error),
	Lex(LexError),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::WrongFileType { found, expected } => {
				write!(
					f,
					"Wrong file type\nExpected a '{}' file, found a '{}' file",
					expected, found
				)
			},
			Self::Io(err) => write!(f, "{}", err),
			Self::Lex(err) => write!(f, "{}", err),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self { Self::Io(value) }
}

impl From<LexError> for Error {
	fn from(value: LexError) -> Self { Self::Lex(value) }
}
