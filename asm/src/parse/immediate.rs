//! AST immediate type definitions

use std::fmt::{Display, Formatter, Result};
use std::ops::Deref;

use crate::lex::{OpToken, TokenType};

#[derive(Clone, Debug)]
pub(crate) struct Immediate<'s> {
	lhs: LogicOrImmediate<'s>,
	rhs: Option<(LogicOrImmediate<'s>, LogicOrImmediate<'s>)>,
}

#[derive(Clone, Debug)]
pub(crate) struct LogicOrImmediate<'s> {
	lhs: LogicXorImmediate<'s>,
	rhs: Option<LogicXorImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct LogicXorImmediate<'s> {
	lhs: LogicAndImmediate<'s>,
	rhs: Option<LogicAndImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct LogicAndImmediate<'s> {
	lhs: OrImmediate<'s>,
	rhs: Option<OrImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct OrImmediate<'s> {
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
	op:  EqOp,
	lhs: OrdImmediate<'s>,
	rhs: Option<OrdImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct OrdImmediate<'s> {
	op:  OrdOp,
	lhs: ShiftImmediate<'s>,
	rhs: Option<ShiftImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct ShiftImmediate<'s> {
	op:  ShiftOp,
	lhs: AddSubImmediate<'s>,
	rhs: Option<AddSubImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AddSubImmediate<'s> {
	op:  AddSubOp,
	lhs: MulDivRemImmediate<'s>,
	rhs: Option<MulDivRemImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct MulDivRemImmediate<'s> {
	op:  MulDivRemOp,
	lhs: UnaryImmediate<'s>,
	rhs: Option<UnaryImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct UnaryImmediate<'s> {
	op:  Option<UnaryOp>,
	rhs: Operand<'s>,
}

#[derive(Clone, Debug)]
pub(crate) enum Operand<'s> {
	Label(&'s str),
	LocalLabel(&'s str),
	Number(isize),
	Immediate(Box<Immediate<'s>>),
}

#[derive(Clone, Debug)]
pub(crate) struct EqOp(OpToken);
#[derive(Clone, Debug)]
pub(crate) struct OrdOp(OpToken);
#[derive(Clone, Debug)]
pub(crate) struct ShiftOp(OpToken);
#[derive(Clone, Debug)]
pub(crate) struct AddSubOp(OpToken);
#[derive(Clone, Debug)]
pub(crate) struct MulDivRemOp(OpToken);
#[derive(Clone, Debug)]
pub(crate) struct UnaryOp(OpToken);

impl<'s> From<TokenType<'s>> for EqOp {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Eq) | TokenType::Op(o @ OpToken::Neq) => Self(o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for EqOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<TokenType<'s>> for OrdOp {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Gt)
			| TokenType::Op(o @ OpToken::Gte)
			| TokenType::Op(o @ OpToken::Lt)
			| TokenType::Op(o @ OpToken::Lte) => Self(o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for OrdOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<TokenType<'s>> for ShiftOp {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Lsl)
			| TokenType::Op(o @ OpToken::Lsr)
			| TokenType::Op(o @ OpToken::Asr) => Self(o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for ShiftOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<TokenType<'s>> for AddSubOp {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Plus) | TokenType::Op(o @ OpToken::Minus) => Self(o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for AddSubOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<TokenType<'s>> for MulDivRemOp {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Mul)
			| TokenType::Op(o @ OpToken::Div)
			| TokenType::Op(o @ OpToken::Rem) => Self(o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for MulDivRemOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<TokenType<'s>> for UnaryOp {
	fn from(value: TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Plus)
			| TokenType::Op(o @ OpToken::Minus)
			| TokenType::Op(o @ OpToken::LogicNot)
			| TokenType::Op(o @ OpToken::BitNot) => Self(o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for UnaryOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

// Display impls that flatten the deeply nested structur of [`Immediate`]s into
// a (hopefully) easy to read string

impl<'s> Display for Immediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} ? {} : {}", self.lhs, rhs.0, rhs.1)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for LogicOrImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} || {}", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for LogicXorImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} ^^ {}", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for LogicAndImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} && {}", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for OrImmediate<'s> {
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
			write!(f, "{} {} {}", self.lhs, *self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for OrdImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, *self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for ShiftImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, *self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for AddSubImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, *self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for MulDivRemImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "{} {} {}", self.lhs, *self.op, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for UnaryImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op {
			write!(f, "{} {}", op.deref(), self.rhs)
		} else {
			write!(f, "{}", self.rhs)
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
