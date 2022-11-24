//! Operator Tokens

use std::fmt::{Display, Formatter, Result};

/// A tokentype to identify operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum OpToken {
	/// `+`
	Plus,
	/// `-`
	Minus,
	/// `*`
	Star,
	/// `/`
	Slash,
	/// `%`
	Percent,
	/// `?`
	Question,
	/// `:`
	Colon,
	/// `$`
	Dollar,
	/// `||`
	LogicOr,
	/// `^^`
	LogicXor,
	/// `&&`
	LogicAnd,
	/// `!`
	Exclamation,
	/// `|`
	BitOr,
	/// `^`
	BitXor,
	/// `&`
	BitAnd,
	/// `~`
	BitNot,
	/// `==`
	Eq,
	/// `!=`
	Neq,
	/// `<`
	Lt,
	/// `<=`
	Lte,
	/// `>`
	Gt,
	/// `>=`
	Gte,
	/// `<<`
	Lsl,
	/// `>>`
	Lsr,
	/// `>>>`
	Asr,

	// These will never be produced by the lexer and exist purely to simplify
	// parsing immediate expressions
	/// `-`
	UnaryMinus,
	/// `(`
	LeftParen,
}

impl Display for OpToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Dollar => write!(f, "$"),
			Self::Question => write!(f, "?"),
			Self::Colon => write!(f, ":"),
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
			Self::Star => write!(f, "*"),
			Self::Slash => write!(f, "/"),
			Self::Percent => write!(f, "%"),
			Self::Exclamation => write!(f, "!"),
			Self::BitNot => write!(f, "~"),
			Self::UnaryMinus => write!(f, "-"),
			Self::LeftParen => write!(f, "("),
		}
	}
}

impl OpToken {
	/// Check if this token is an arithmetic/logic operator
	pub(crate) fn is_al_operator(&self) -> bool {
		matches!(
			self,
			Self::Question
				| Self::Colon | Self::LogicOr
				| Self::LogicXor | Self::LogicAnd
				| Self::BitOr | Self::BitXor
				| Self::BitAnd | Self::Eq
				| Self::Neq | Self::Lt
				| Self::Lte | Self::Gt
				| Self::Gte | Self::Lsl
				| Self::Lsr | Self::Asr
				| Self::Plus | Self::Minus
				| Self::Star | Self::Slash
				| Self::Percent | Self::Exclamation
				| Self::BitNot
		)
	}

	/// Check if this token is right associative or not
	pub(crate) fn is_right_associative(&self) -> bool {
		matches!(self, Self::UnaryMinus | Self::Question | Self::Colon)
	}

	/// Get the precedence of this token
	///
	/// 0 -> highest precedence
	pub(crate) fn get_precedence(&self) -> u8 {
		match self {
			Self::Question => 0,
			Self::Colon => 0,
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
			Self::Star => 11,
			Self::Slash => 11,
			Self::Percent => 11,
			Self::Exclamation => 12,
			Self::BitNot => 12,
			Self::UnaryMinus => 12,

			_ => unreachable!(),
		}
	}
}
