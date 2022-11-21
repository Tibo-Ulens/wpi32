//! Directive Tokens
use std::fmt::{Display, Formatter, Result};

/// A tokentype to identify directives
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DirToken {
	Section,
	Bytes,
	Halves,
	Words,
	ResBytes,
	ResHalves,
	ResWords,
	Repeat,
	Const,
}

impl Display for DirToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Section => write!(f, "#SECTION"),
			Self::Bytes => write!(f, "#BYTES"),
			Self::Halves => write!(f, "#HALVES"),
			Self::Words => write!(f, "#WORDS"),
			Self::ResBytes => write!(f, "#RES_BYTES"),
			Self::ResHalves => write!(f, "#RES_HALVES"),
			Self::ResWords => write!(f, "#RES_WORDS"),
			Self::Repeat => write!(f, "#REPEAT"),
			Self::Const => write!(f, "#CONST"),
		}
	}
}
