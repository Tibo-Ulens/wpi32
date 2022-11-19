//! AST immediate type definitions

use std::fmt::{Display, Formatter, Result};

use crate::lex::TokenType;

#[derive(Clone, Debug)]
pub(crate) struct Immediate<'s> {
	lhs: XorImmediate<'s>,
	rhs: Option<XorImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct XorImmediate<'s> {
	lhs: AndImmediate<'s>,
	rhs: Option<AndImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AndImmediate<'s> {
	lhs: EqImmediate<'s>,
	rhs: Option<EqImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct EqImmediate<'s> {
	op:  TokenType<'s>,
	lhs: OrdImmediate<'s>,
	rhs: Option<OrdImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct OrdImmediate<'s> {
	op:  TokenType<'s>,
	lhs: ShiftImmediate<'s>,
	rhs: Option<ShiftImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct ShiftImmediate<'s> {
	op:  TokenType<'s>,
	lhs: AddSubImmediate<'s>,
	rhs: Option<AddSubImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AddSubImmediate<'s> {
	op:  TokenType<'s>,
	lhs: MulDivRemImmediate<'s>,
	rhs: Option<MulDivRemImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct MulDivRemImmediate<'s> {
	op:  TokenType<'s>,
	lhs: Operand<'s>,
	rhs: Option<Operand<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) enum Operand<'s> {
	Label(&'s str),
	LocalLabel(&'s str),
	Number(isize),
	Immediate(Box<Immediate<'s>>),
}

#[derive(Clone, Debug)]
pub(crate) struct EqOp<'s>(TokenType<'s>);
#[derive(Clone, Debug)]
pub(crate) struct OrdOp<'s>(TokenType<'s>);
#[derive(Clone, Debug)]
pub(crate) struct ShiftOp<'s>(TokenType<'s>);
#[derive(Clone, Debug)]
pub(crate) struct AddSubOp<'s>(TokenType<'s>);
#[derive(Clone, Debug)]
pub(crate) struct MulDivRemOp<'s>(TokenType<'s>);

impl<'s> From<TokenType<'s>> for EqOp<'s> {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::OperatorEq | TokenType::OperatorNeq => Self(value),
			_ => unimplemented!(),
		}
	}
}

impl<'s> From<TokenType<'s>> for OrdOp<'s> {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::OperatorGt
			| TokenType::OperatorGte
			| TokenType::OperatorLt
			| TokenType::OperatorLte => Self(value),
			_ => unimplemented!(),
		}
	}
}

impl<'s> From<TokenType<'s>> for ShiftOp<'s> {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::OperatorLsl | TokenType::OperatorLsr | TokenType::OperatorAsr => Self(value),
			_ => unimplemented!(),
		}
	}
}

impl<'s> From<TokenType<'s>> for AddSubOp<'s> {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::OperatorPlus | TokenType::OperatorMinus => Self(value),
			_ => unimplemented!(),
		}
	}
}

impl<'s> From<TokenType<'s>> for MulDivRemOp<'s> {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::OperatorMul | TokenType::OperatorDiv | TokenType::OperatorRem => Self(value),
			_ => unimplemented!(),
		}
	}
}

// Display impls that flatten the deeply nested structur of [`Immediate`]s into
// a (hopefully) easy to read string

impl<'s> Display for Immediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} | {}", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for XorImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} ^ {}", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for AndImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} & {}", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for EqImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for OrdImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for ShiftImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for AddSubImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for MulDivRemImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for Operand<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Label(l) => write!(f, "{}", l),
			Self::LocalLabel(ll) => write!(f, "{}", ll),
			Self::Number(n) => write!(f, "{}", n),
			Self::Immediate(imm) => write!(f, "{}", imm),
		}
	}
}
