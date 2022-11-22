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
	UnaryMinus,

	/// This will never be produced by the lexer and exists purely simplify
	/// parsing immediate expressions
	LeftParen,
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
			Self::UnaryMinus => write!(f, "-"),
			Self::LeftParen => write!(f, "("),
		}
	}
}

impl OpToken {
	/// Check if this token is right associative or not
	pub(crate) fn is_right_associative(&self) -> bool {
		matches!(self, Self::UnaryMinus | Self::TernStart | Self::TernAlt)
	}

	/// Get the precedence of this token
	///
	/// 0 -> highest precedence
	pub(crate) fn get_precedence(&self) -> u8 {
		match self {
			Self::TernStart => 0,
			Self::TernAlt => 0,
			Self::LogicOr => 1,
			Self::LogicXor => 2,
			Self::LogicAnd => 3,
			Self::BitOr => 4,
			Self::BitXor => 5,
			Self::BitAnd => 6,
			Self::Eq => 7,
			Self::Neq => 7,
			Self::Lt => 8,
			Self::Lte => 8,
			Self::Gt => 8,
			Self::Gte => 8,
			Self::Lsl => 9,
			Self::Lsr => 9,
			Self::Asr => 9,
			Self::Plus => 10,
			Self::Minus => 10,
			Self::Mul => 11,
			Self::Div => 11,
			Self::Rem => 11,
			Self::LogicNot => 12,
			Self::BitNot => 12,
			Self::UnaryMinus => 12,

			Self::LeftParen => 0,
		}
	}
}
