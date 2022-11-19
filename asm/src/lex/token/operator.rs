use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OperatorToken {
	TernStart,
	TernAlt,
	LogicOr,
	LogicXor,
	LogicAnd,
	Or,
	Xor,
	And,
	Eq,
	Neq,
	Lt,
	Lte,
	Gt,
	Gte,
	Lsl,
	Lsr,
	Asr,
	Plus,
	Minus,
	Mul,
	Div,
	Rem,
}

impl Display for OperatorToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::TernStart => write!(f, "?"),
			Self::TernAlt => write!(f, ":"),
			Self::LogicOr => write!(f, "||"),
			Self::LogicXor => write!(f, "^^"),
			Self::LogicAnd => write!(f, "&&"),
			Self::Or => write!(f, "|"),
			Self::Xor => write!(f, "^"),
			Self::And => write!(f, "&"),
			Self::Eq => write!(f, "=="),
			Self::Neq => write!(f, "!="),
			Self::Lt => write!(f, "<"),
			Self::Lte => write!(f, "<="),
			Self::Gt => write!(f, ">"),
			Self::Gte => write!(f, ">="),
			Self::Lsl => write!(f, "<<"),
			Self::Lsr => write!(f, ">>"),
			Self::Asr => write!(f, ">>>"),
			Self::Plus => write!(f, "+"),
			Self::Minus => write!(f, "-"),
			Self::Mul => write!(f, "*"),
			Self::Div => write!(f, "/"),
			Self::Rem => write!(f, "%"),
		}
	}
}
