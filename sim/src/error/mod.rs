//! Simulator error types and utility functions

use std::fmt::{Display, Formatter};

/// Any possible error produced during simulation
#[derive(Debug)]
pub enum Error {
	/// Wrapper around [`std::io::Error`]
	Io(std::io::Error),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Io(err) => write!(f, "{}", err),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self { Self::Io(value) }
}
