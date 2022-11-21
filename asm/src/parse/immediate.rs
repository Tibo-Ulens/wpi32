//! [`Parser`] functions to parse [`Immediate`] expressions

use common::{LocationInfo, ParseError};

use super::ast::{
	AddSubImmediate,
	AddSubOp,
	AndImmediate,
	EqImmediate,
	EqOp,
	Immediate,
	LogicAndImmediate,
	LogicOrImmediate,
	LogicXorImmediate,
	MulDivRemImmediate,
	MulDivRemOp,
	Operand,
	OrImmediate,
	OrdImmediate,
	OrdOp,
	ShiftImmediate,
	ShiftOp,
	UnaryImmediate,
	UnaryOp,
	XorImmediate,
};
use super::Parser;
use crate::lex::{OpToken, TokenType};

// !!!
//
// THIS IS ALMOST CERTAINLY ONE OF THE LEAST EFFICIENT WAYS TO DO THIS
// I ONLY DID IT LIKE THIS TO SIMPLIFY TRANSLATING THE GRAMMAR INTO CODE
// (and so i don't need to implement the shunting-yard algorithm)
//
// !!!

impl<'s> Parser<'s> {
	/// Recursively parse an immediate expresion
	pub(super) fn parse_immediate<'r>(&'r mut self) -> Result<Immediate<'s>, ParseError> {
		let lhs = self.parse_logicor_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::TernStart) {
			Some(self.parse_ternary()?)
		} else {
			None
		};

		Ok(Immediate { lhs, rhs })
	}

	/// Parse the consequent and alternate in a ternary expression
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::Op(OpToken::TernStart)`]
	fn parse_ternary<'r>(
		&'r mut self,
	) -> Result<(LogicOrImmediate<'s>, LogicOrImmediate<'s>), ParseError> {
		// Consume the `?` TernStart token
		// Unwrap is safe as [`self.peek()`] is [`Some`]
		assert_eq!(self.peek().unwrap().t, TokenType::Op(OpToken::TernStart));
		self.next().unwrap();

		let cons = self.parse_logicor_immediate()?;

		let peek = self.peek()?;
		if peek.t != TokenType::Op(OpToken::TernAlt) {
			return Err(ParseError::UnexpectedToken {
				src_file: self.source_file.to_string(),
				location: Box::new(LocationInfo::from(peek)),
				fnd:      peek.t.to_string(),
				ex:       OpToken::TernAlt.to_string(),
			});
		}
		// Consume the `:` TernAlt token
		// Unwrap is safe as [`self.peek()`] is [`Some`]
		self.next().unwrap();

		let alt = self.parse_logicor_immediate()?;

		Ok((cons, alt))
	}

	fn parse_logicor_immediate<'r>(&'r mut self) -> Result<LogicOrImmediate<'s>, ParseError> {
		let lhs = self.parse_logicxor_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::LogicOr) {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(self.parse_logicxor_immediate()?)
		} else {
			None
		};

		Ok(LogicOrImmediate { lhs, rhs })
	}

	fn parse_logicxor_immediate<'r>(&'r mut self) -> Result<LogicXorImmediate<'s>, ParseError> {
		let lhs = self.parse_logicand_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::LogicXor) {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(self.parse_logicand_immediate()?)
		} else {
			None
		};

		Ok(LogicXorImmediate { lhs, rhs })
	}

	fn parse_logicand_immediate<'r>(&'r mut self) -> Result<LogicAndImmediate<'s>, ParseError> {
		let lhs = self.parse_or_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::LogicAnd) {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(self.parse_or_immediate()?)
		} else {
			None
		};

		Ok(LogicAndImmediate { lhs, rhs })
	}

	fn parse_or_immediate<'r>(&'r mut self) -> Result<OrImmediate<'s>, ParseError> {
		let lhs = self.parse_xor_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::BitOr) {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(self.parse_xor_immediate()?)
		} else {
			None
		};

		Ok(OrImmediate { lhs, rhs })
	}

	fn parse_xor_immediate<'r>(&'r mut self) -> Result<XorImmediate<'s>, ParseError> {
		let lhs = self.parse_and_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::BitXor) {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(self.parse_and_immediate()?)
		} else {
			None
		};

		Ok(XorImmediate { lhs, rhs })
	}

	fn parse_and_immediate<'r>(&'r mut self) -> Result<AndImmediate<'s>, ParseError> {
		let lhs = self.parse_eq_immediate()?;

		let peek = self.peek()?;
		let rhs = if peek.t == TokenType::Op(OpToken::BitAnd) {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(self.parse_eq_immediate()?)
		} else {
			None
		};

		Ok(AndImmediate { lhs, rhs })
	}

	fn parse_eq_immediate<'r>(&'r mut self) -> Result<EqImmediate<'s>, ParseError> {
		let lhs = self.parse_ord_immediate()?;

		let peek = self.peek()?;
		let (op, rhs) = match &peek.t {
			t @ TokenType::Op(OpToken::Eq) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(EqOp::from(t)), Some(self.parse_ord_immediate()?))
			},
			t @ TokenType::Op(OpToken::Neq) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(EqOp::from(t)), Some(self.parse_ord_immediate()?))
			},
			_ => (None, None),
		};

		Ok(EqImmediate { lhs, op, rhs })
	}

	fn parse_ord_immediate<'r>(&'r mut self) -> Result<OrdImmediate<'s>, ParseError> {
		let lhs = self.parse_shift_immediate()?;

		let peek = self.peek()?;
		let (op, rhs) = match &peek.t {
			t @ TokenType::Op(OpToken::Lt) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(OrdOp::from(t)), Some(self.parse_shift_immediate()?))
			},
			t @ TokenType::Op(OpToken::Lte) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(OrdOp::from(t)), Some(self.parse_shift_immediate()?))
			},
			t @ TokenType::Op(OpToken::Gt) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(OrdOp::from(t)), Some(self.parse_shift_immediate()?))
			},
			t @ TokenType::Op(OpToken::Gte) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(OrdOp::from(t)), Some(self.parse_shift_immediate()?))
			},
			_ => (None, None),
		};

		Ok(OrdImmediate { lhs, op, rhs })
	}

	fn parse_shift_immediate<'r>(&'r mut self) -> Result<ShiftImmediate<'s>, ParseError> {
		let lhs = self.parse_addsub_immediate()?;

		let peek = self.peek()?;
		let (op, rhs) = match &peek.t {
			t @ TokenType::Op(OpToken::Lsl) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(ShiftOp::from(t)), Some(self.parse_addsub_immediate()?))
			},
			t @ TokenType::Op(OpToken::Lsr) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(ShiftOp::from(t)), Some(self.parse_addsub_immediate()?))
			},
			t @ TokenType::Op(OpToken::Asr) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(ShiftOp::from(t)), Some(self.parse_addsub_immediate()?))
			},
			_ => (None, None),
		};

		Ok(ShiftImmediate { lhs, op, rhs })
	}

	fn parse_addsub_immediate<'r>(&'r mut self) -> Result<AddSubImmediate<'s>, ParseError> {
		let lhs = self.parse_muldivrem_immediate()?;

		let peek = self.peek()?;
		let (op, rhs) = match &peek.t {
			t @ TokenType::Op(OpToken::Plus) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(AddSubOp::from(t)), Some(self.parse_muldivrem_immediate()?))
			},
			t @ TokenType::Op(OpToken::Minus) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(AddSubOp::from(t)), Some(self.parse_muldivrem_immediate()?))
			},
			_ => (None, None),
		};

		Ok(AddSubImmediate { lhs, op, rhs })
	}

	fn parse_muldivrem_immediate<'r>(&'r mut self) -> Result<MulDivRemImmediate<'s>, ParseError> {
		let lhs = self.parse_unary_immediate()?;

		let peek = self.peek()?;
		let (op, rhs) = match &peek.t {
			t @ TokenType::Op(OpToken::Mul) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(MulDivRemOp::from(t)), Some(self.parse_unary_immediate()?))
			},
			t @ TokenType::Op(OpToken::Div) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(MulDivRemOp::from(t)), Some(self.parse_unary_immediate()?))
			},
			t @ TokenType::Op(OpToken::Rem) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				(Some(MulDivRemOp::from(t)), Some(self.parse_unary_immediate()?))
			},
			_ => (None, None),
		};

		Ok(MulDivRemImmediate { lhs, op, rhs })
	}

	fn parse_unary_immediate<'r>(&'r mut self) -> Result<UnaryImmediate<'s>, ParseError> {
		let peek = self.peek()?;
		let op = match &peek.t {
			TokenType::Op(OpToken::Plus | OpToken::Minus | OpToken::BitNot | OpToken::LogicNot) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				let next = self.next().unwrap();
				Some(UnaryOp::from(&next.t))
			},
			_ => None,
		};

		let rhs = self.parse_operand()?;

		Ok(UnaryImmediate { op, rhs })
	}

	fn parse_operand<'r>(&'r mut self) -> Result<Operand<'s>, ParseError> {
		let peek = self.peek()?;
		let operand = match &peek.t {
			TokenType::Label(l) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				Operand::Label(l)
			},
			TokenType::LocalLabel(ll) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				Operand::LocalLabel(ll)
			},
			TokenType::LitNum(n) => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();
				Operand::Number(*n)
			},
			TokenType::SymLeftParen => {
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();

				let imm = self.parse_immediate()?;

				let close_peek = self.peek()?;
				if close_peek.t != TokenType::SymRightParen {
					return Err(ParseError::UnclosedParenthesis {
						src_file:       self.source_file.to_string(),
						close_location: Box::new(LocationInfo::from(close_peek)),
						open_location:  Box::new(LocationInfo::from(peek)),
					});
				}

				// Take the closing paren
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();

				Operand::Immediate(Box::new(imm))
			},
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
					fnd:      peek.t.to_string(),
					ex:       "LABEL or LOCAL_LABEL or NUMBER or IMMEDIATE".to_string(),
				});
			},
		};

		Ok(operand)
	}
}
