//! Directive Tokens
use std::fmt::{Display, Formatter, Result};

/// A tokentype to identify directives
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DirToken {
	Section,
	Regular(RegularDirective),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum RegularDirective {
	Bytes,
	Halves,
	Words,
	ResBytes,
	ResHalves,
	ResWords,
	Const,
}

impl Display for DirToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Section => write!(f, "#SECTION"),
			Self::Regular(RegularDirective::Bytes) => write!(f, "#BYTES"),
			Self::Regular(RegularDirective::Halves) => write!(f, "#HALVES"),
			Self::Regular(RegularDirective::Words) => write!(f, "#WORDS"),
			Self::Regular(RegularDirective::ResBytes) => write!(f, "#RES_BYTES"),
			Self::Regular(RegularDirective::ResHalves) => write!(f, "#RES_HALVES"),
			Self::Regular(RegularDirective::ResWords) => write!(f, "#RES_WORDS"),
			Self::Regular(RegularDirective::Const) => write!(f, "#CONST"),
		}
	}
}
