//! Top-level error types

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(super) enum Error {
	WrongFileType { found: String, expected: String },
	Assembler(super::AssemblerError),
	Simulator(super::SimulatorError),
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
			Self::Assembler(err) => write!(f, "{}", err),
			Self::Simulator(err) => write!(f, "{}", err),
		}
	}
}

impl From<super::AssemblerError> for Error {
	fn from(value: super::AssemblerError) -> Self { Self::Assembler(value) }
}

impl From<super::SimulatorError> for Error {
	fn from(value: super::SimulatorError) -> Self { Self::Simulator(value) }
}
