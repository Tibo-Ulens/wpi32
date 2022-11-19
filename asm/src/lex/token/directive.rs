use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DirectiveToken {
	Byte,
	Half,
	Word,
	Repeat,
	Equ,
}

impl Display for DirectiveToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Byte => write!(f, "$byte"),
			Self::Half => write!(f, "$half"),
			Self::Word => write!(f, "$word"),
			Self::Repeat => write!(f, "$repeat"),
			Self::Equ => write!(f, "$equ"),
		}
	}
}
