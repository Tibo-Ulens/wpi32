//! AST immediate type definitions

#![allow(missing_docs)]

use std::fmt::{Display, Formatter, Result};
use std::ops::Deref;

use crate::lex::{OpToken, TokenType};

/// An immediate value
///
/// This can range from a single number to a complex expression referencing
/// labels and constants
///
/// *EBNF not given as it is too chonky, look at the docs folder for grammar*
#[derive(Clone, Debug)]
pub struct Immediate<'s> {
	pub lhs: LogicOrImmediate<'s>,
	pub rhs: Option<(LogicOrImmediate<'s>, LogicOrImmediate<'s>)>,
}

#[derive(Clone, Debug)]
pub struct LogicOrImmediate<'s> {
	pub lhs: LogicXorImmediate<'s>,
	pub rhs: Option<LogicXorImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct LogicXorImmediate<'s> {
	pub lhs: LogicAndImmediate<'s>,
	pub rhs: Option<LogicAndImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct LogicAndImmediate<'s> {
	pub lhs: OrImmediate<'s>,
	pub rhs: Option<OrImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct OrImmediate<'s> {
	pub lhs: XorImmediate<'s>,
	pub rhs: Option<XorImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct XorImmediate<'s> {
	pub lhs: AndImmediate<'s>,
	pub rhs: Option<AndImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct AndImmediate<'s> {
	pub lhs: EqImmediate<'s>,
	pub rhs: Option<EqImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct EqImmediate<'s> {
	pub lhs: OrdImmediate<'s>,
	pub op:  Option<EqOp>,
	pub rhs: Option<OrdImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct OrdImmediate<'s> {
	pub lhs: ShiftImmediate<'s>,
	pub op:  Option<OrdOp>,
	pub rhs: Option<ShiftImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct ShiftImmediate<'s> {
	pub lhs: AddSubImmediate<'s>,
	pub op:  Option<ShiftOp>,
	pub rhs: Option<AddSubImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct AddSubImmediate<'s> {
	pub lhs: MulDivRemImmediate<'s>,
	pub op:  Option<AddSubOp>,
	pub rhs: Option<MulDivRemImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct MulDivRemImmediate<'s> {
	pub lhs: UnaryImmediate<'s>,
	pub op:  Option<MulDivRemOp>,
	pub rhs: Option<UnaryImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub struct UnaryImmediate<'s> {
	pub op:  Option<UnaryOp>,
	pub rhs: Operand<'s>,
}

#[derive(Clone, Debug)]
pub enum Operand<'s> {
	Label(&'s str),
	LocalLabel(&'s str),
	Number(isize),
	Immediate(Box<Immediate<'s>>),
}

#[derive(Clone, Debug)]
pub struct EqOp(OpToken);
#[derive(Clone, Debug)]
pub struct OrdOp(OpToken);
#[derive(Clone, Debug)]
pub struct ShiftOp(OpToken);
#[derive(Clone, Debug)]
pub struct AddSubOp(OpToken);
#[derive(Clone, Debug)]
pub struct MulDivRemOp(OpToken);
#[derive(Clone, Debug)]
pub struct UnaryOp(OpToken);

impl<'s> From<&TokenType<'s>> for EqOp {
	fn from(value: &TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Eq) | TokenType::Op(o @ OpToken::Neq) => Self(*o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for EqOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<&TokenType<'s>> for OrdOp {
	fn from(value: &TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Gt)
			| TokenType::Op(o @ OpToken::Gte)
			| TokenType::Op(o @ OpToken::Lt)
			| TokenType::Op(o @ OpToken::Lte) => Self(*o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for OrdOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<&TokenType<'s>> for ShiftOp {
	fn from(value: &TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Lsl)
			| TokenType::Op(o @ OpToken::Lsr)
			| TokenType::Op(o @ OpToken::Asr) => Self(*o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for ShiftOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<&TokenType<'s>> for AddSubOp {
	fn from(value: &TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Plus) | TokenType::Op(o @ OpToken::Minus) => Self(*o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for AddSubOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<&TokenType<'s>> for MulDivRemOp {
	fn from(value: &TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Mul)
			| TokenType::Op(o @ OpToken::Div)
			| TokenType::Op(o @ OpToken::Rem) => Self(*o),
			_ => unimplemented!(),
		}
	}
}

impl Deref for MulDivRemOp {
	type Target = OpToken;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'s> From<&TokenType<'s>> for UnaryOp {
	fn from(value: &TokenType<'s>) -> Self {
		match value {
			TokenType::Op(o @ OpToken::Plus)
			| TokenType::Op(o @ OpToken::Minus)
			| TokenType::Op(o @ OpToken::LogicNot)
			| TokenType::Op(o @ OpToken::BitNot) => Self(*o),
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
			write!(f, "({}) ? ({}) : ({})", self.lhs, rhs.0, rhs.1)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for LogicOrImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "({} || {})", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for LogicXorImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "({} ^^ {})", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for LogicAndImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "({} && {})", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for OrImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "({} | {})", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for XorImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "({} ^ {})", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for AndImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(rhs) = &self.rhs {
			write!(f, "({} & {})", self.lhs, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for EqImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op
			&& let Some (rhs) = &self.rhs
		{
			write!(f, "({} {} {})", self.lhs, op.0, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for OrdImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op
			&& let Some(rhs) = &self.rhs
		{
			write!(f, "({} {} {})", self.lhs, op.0, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for ShiftImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op
			&& let Some(rhs) = &self.rhs
		{
			write!(f, "({} {} {})", self.lhs, op.0, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for AddSubImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op
			&& let Some(rhs) = &self.rhs
		{
			write!(f, "({} {} {})", self.lhs, op.0, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for MulDivRemImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op
			&& let Some(rhs) = &self.rhs
		{
			write!(f, "({} {} {})", self.lhs, op.0, rhs)
		} else {
			write!(f, "{}", self.lhs)
		}
	}
}

impl<'s> Display for UnaryImmediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Some(op) = &self.op {
			write!(f, "{} ({})", op.deref(), self.rhs)
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
			Self::Immediate(imm) => write!(f, "({})", imm),
		}
	}
}
