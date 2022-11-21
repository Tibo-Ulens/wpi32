//! Operator Tokens

use std::fmt::{Display, Formatter, Result};

/// A tokentype to identify operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum OpToken {
	TernStart,
	TernAlt,
	LogicOr,
	LogicXor,
	LogicAnd,
	BitOr,
	BitXor,
	BitAnd,
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
	LogicNot,
	BitNot,
}

impl Display for OpToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::TernStart => write!(f, "?"),
			Self::TernAlt => write!(f, ":"),
			Self::LogicOr => write!(f, "||"),
			Self::LogicXor => write!(f, "^^"),
			Self::LogicAnd => write!(f, "&&"),
			Self::BitOr => write!(f, "|"),
			Self::BitXor => write!(f, "^"),
			Self::BitAnd => write!(f, "&"),
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
			Self::LogicNot => write!(f, "!"),
			Self::BitNot => write!(f, "~"),
		}
	}
}
