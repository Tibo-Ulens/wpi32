//! [`Parser`] functions to parse [`Instruction`]s

use common::{LocationInfo, ParseError};

use super::ast::{AddrOffset, Address, Immediate, Instruction, OffsetOperator, OrderingTarget};
use super::Parser;
use crate::lex::{InstToken, OpToken, RegToken, TokenType};

impl<'s> Parser<'s> {
	/// Parse any valid [`Instruction`]
	///
	/// Assumes the current [`Token`] has [`TokenType`] [`TokenType::Inst`]
	pub(super) fn parse_instruction<'r>(&'r mut self) -> Result<Instruction<'s>, ParseError> {
		// Unwrap is assumed to be safe
		let instruction_token = self.next().unwrap();

		match &instruction_token.t {
			TokenType::Inst(InstToken::Addi) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Addi { dest, src, imm })
			},
			TokenType::Inst(InstToken::Andi) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Andi { dest, src, imm })
			},
			TokenType::Inst(InstToken::Ori) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Ori { dest, src, imm })
			},
			TokenType::Inst(InstToken::Xori) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Xori { dest, src, imm })
			},
			TokenType::Inst(InstToken::Lsli) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Lsli { dest, src, imm })
			},
			TokenType::Inst(InstToken::Lsri) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Lsri { dest, src, imm })
			},
			TokenType::Inst(InstToken::Asri) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Asri { dest, src, imm })
			},
			TokenType::Inst(InstToken::Slti) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Slti { dest, src, imm })
			},
			TokenType::Inst(InstToken::Sltiu) => {
				let (dest, src, imm) = self.parse_rri()?;

				Ok(Instruction::Sltiu { dest, src, imm })
			},

			TokenType::Inst(InstToken::Add) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Add { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Sub) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Sub { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::And) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::And { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Or) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Or { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Xor) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Xor { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Lsl) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Lsl { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Lsr) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Lsr { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Asr) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Asr { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Slt) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Slt { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Sltu) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Sltu { dest, src1, src2 })
			},

			TokenType::Inst(InstToken::Lui) => {
				let (dest, imm) = self.parse_ri()?;

				Ok(Instruction::Lui { dest, imm })
			},
			TokenType::Inst(InstToken::Auipc) => {
				let (dest, imm) = self.parse_ri()?;

				Ok(Instruction::Auipc { dest, imm })
			},

			TokenType::Inst(InstToken::Jal) => {
				let (dest, offset) = self.parse_ri()?;

				Ok(Instruction::Jal { dest, offset })
			},
			TokenType::Inst(InstToken::Jalr) => {
				let (dest, base, offset) = self.parse_rri()?;

				Ok(Instruction::Jalr { dest, base, offset })
			},

			TokenType::Inst(InstToken::Beq) => {
				let (src1, src2, offset) = self.parse_rri()?;

				Ok(Instruction::Beq { src1, src2, offset })
			},
			TokenType::Inst(InstToken::Bne) => {
				let (src1, src2, offset) = self.parse_rri()?;

				Ok(Instruction::Bne { src1, src2, offset })
			},
			TokenType::Inst(InstToken::Blt) => {
				let (src1, src2, offset) = self.parse_rri()?;

				Ok(Instruction::Blt { src1, src2, offset })
			},
			TokenType::Inst(InstToken::Bltu) => {
				let (src1, src2, offset) = self.parse_rri()?;

				Ok(Instruction::Bltu { src1, src2, offset })
			},
			TokenType::Inst(InstToken::Bge) => {
				let (src1, src2, offset) = self.parse_rri()?;

				Ok(Instruction::Bge { src1, src2, offset })
			},
			TokenType::Inst(InstToken::Bgeu) => {
				let (src1, src2, offset) = self.parse_rri()?;

				Ok(Instruction::Bgeu { src1, src2, offset })
			},

			TokenType::Inst(InstToken::Lw) => {
				let (dest, addr) = self.parse_ra()?;

				Ok(Instruction::Lw { dest, addr })
			},
			TokenType::Inst(InstToken::Lh) => {
				let (dest, addr) = self.parse_ra()?;

				Ok(Instruction::Lh { dest, addr })
			},
			TokenType::Inst(InstToken::Lhu) => {
				let (dest, addr) = self.parse_ra()?;

				Ok(Instruction::Lhu { dest, addr })
			},
			TokenType::Inst(InstToken::Lb) => {
				let (dest, addr) = self.parse_ra()?;

				Ok(Instruction::Lb { dest, addr })
			},
			TokenType::Inst(InstToken::Lbu) => {
				let (dest, addr) = self.parse_ra()?;

				Ok(Instruction::Lbu { dest, addr })
			},

			TokenType::Inst(InstToken::Sw) => {
				let (dest, src) = self.parse_ar()?;

				Ok(Instruction::Sw { dest, src })
			},
			TokenType::Inst(InstToken::Sh) => {
				let (dest, src) = self.parse_ar()?;

				Ok(Instruction::Sh { dest, src })
			},
			TokenType::Inst(InstToken::Sb) => {
				let (dest, src) = self.parse_ar()?;

				Ok(Instruction::Sb { dest, src })
			},

			TokenType::Inst(InstToken::Fence) => {
				let (pred, succ) = self.parse_oo()?;

				Ok(Instruction::Fence { pred, succ })
			},
			TokenType::Inst(InstToken::FenceTso) => {
				let (pred, succ) = self.parse_oo()?;

				Ok(Instruction::FenceTso { pred, succ })
			},

			TokenType::Inst(InstToken::Ecall) => Ok(Instruction::Ecall),
			TokenType::Inst(InstToken::Ebreak) => Ok(Instruction::Ebreak),

			TokenType::Inst(InstToken::Fencei) => Ok(Instruction::Fencei),

			TokenType::Inst(InstToken::Csrrw) => {
				let (dest, src, target) = self.parse_rri()?;

				Ok(Instruction::Csrrw { dest, src, target })
			},
			TokenType::Inst(InstToken::Csrrs) => {
				let (dest, src, target) = self.parse_rri()?;

				Ok(Instruction::Csrrs { dest, src, target })
			},
			TokenType::Inst(InstToken::Csrrc) => {
				let (dest, src, target) = self.parse_rri()?;

				Ok(Instruction::Csrrc { dest, src, target })
			},

			TokenType::Inst(InstToken::Csrrwi) => {
				let (dest, src, target) = self.parse_rii()?;

				Ok(Instruction::Csrrwi { dest, src, target })
			},
			TokenType::Inst(InstToken::Csrrsi) => {
				let (dest, src, target) = self.parse_rii()?;

				Ok(Instruction::Csrrsi { dest, src, target })
			},
			TokenType::Inst(InstToken::Csrrci) => {
				let (dest, src, target) = self.parse_rii()?;

				Ok(Instruction::Csrrci { dest, src, target })
			},

			TokenType::Inst(InstToken::Mul) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Mul { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Mulh) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Mulh { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Mulhu) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Mulhu { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Mulhsu) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Mulhsu { dest, src1, src2 })
			},

			TokenType::Inst(InstToken::Div) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Div { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Divu) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Divu { dest, src1, src2 })
			},

			TokenType::Inst(InstToken::Rem) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Rem { dest, src1, src2 })
			},
			TokenType::Inst(InstToken::Remu) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				Ok(Instruction::Remu { dest, src1, src2 })
			},

			_ => unreachable!(),
		}
	}

	/// Parse 2 [`RegToken`]s, followed by an [`Immediate`]
	fn parse_rri<'r>(&'r mut self) -> Result<(RegToken, RegToken, Immediate<'s>), ParseError> {
		let reg1 = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let reg2 = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let imm = self.parse_immediate()?;

		Ok((reg1, reg2, imm))
	}

	/// Parse 3 [`RegToken`]s
	fn parse_rrr(&mut self) -> Result<(RegToken, RegToken, RegToken), ParseError> {
		let reg1 = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let reg2 = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let reg3 = self.parse_register()?;

		Ok((reg1, reg2, reg3))
	}

	/// Parse a [`RegToken`] followed by an [`Immediate`]
	fn parse_ri<'r>(&'r mut self) -> Result<(RegToken, Immediate<'s>), ParseError> {
		let reg = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let imm = self.parse_immediate()?;

		Ok((reg, imm))
	}

	/// Parse a [`RegToken`] followed by an [`Address`]
	fn parse_ra<'r>(&'r mut self) -> Result<(RegToken, Address<'s>), ParseError> {
		let reg = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let addr = self.parse_address()?;

		Ok((reg, addr))
	}

	/// Parse an [`Address`] followed by a [`RegToken`]
	fn parse_ar<'r>(&'r mut self) -> Result<(Address<'s>, RegToken), ParseError> {
		let addr = self.parse_address()?;
		self.expect(TokenType::SymComma)?;

		let reg = self.parse_register()?;

		Ok((addr, reg))
	}

	/// Parse a [`RegToken`], followed by 2 [`Immediate`]s
	fn parse_rii<'r>(&'r mut self) -> Result<(RegToken, Immediate<'s>, Immediate<'s>), ParseError> {
		let reg1 = self.parse_register()?;
		self.expect(TokenType::SymComma)?;

		let imm1 = self.parse_immediate()?;
		self.expect(TokenType::SymComma)?;

		let imm2 = self.parse_immediate()?;

		Ok((reg1, imm1, imm2))
	}

	/// Parse 2 [`OrderingTarget`]s
	fn parse_oo(&mut self) -> Result<(OrderingTarget, OrderingTarget), ParseError> {
		let ord1 = self.parse_ordering_target()?;
		self.expect(TokenType::SymComma)?;

		let ord2 = self.parse_ordering_target()?;

		Ok((ord1, ord2))
	}

	/// Parse an [`Address`] calculation consisting of:
	///  - A left bracket
	///  - A base [`register`](RegToken)
	///  - An optional [`AddrOffset`]
	///  - A right bracket
	fn parse_address<'r>(&'r mut self) -> Result<Address<'s>, ParseError> {
		let open_peek = self.peek()?;
		self.expect(TokenType::SymLeftBracket)?;

		let base = self.parse_register()?;

		let op_peek = self.peek()?;
		let offset = match &op_peek.t {
			TokenType::Op(o @ OpToken::Plus | o @ OpToken::Minus) => {
				// Take operator token
				// Unwrap is safe as [`self.peek()`] is [`Some`]
				self.next().unwrap();

				let offset = self.parse_immediate()?;

				Some(AddrOffset { op: OffsetOperator::from(o), offset })
			},
			_ => None,
		};

		let close_peek = self.peek()?;
		if close_peek.t != TokenType::SymRightBracket {
			return Err(ParseError::UnclosedParenthesis {
				src_file:       self.source_file.to_string(),
				close_location: Box::new(LocationInfo::from(close_peek)),
				open_location:  Box::new(LocationInfo::from(open_peek)),
			});
		}

		Ok(Address { base, offset })
	}

	/// Parse a single [`register`](RegToken) identifier
	fn parse_register(&mut self) -> Result<RegToken, ParseError> {
		let next = self.next()?;
		match next.t {
			TokenType::Reg(reg) => Ok(reg),
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(next)),
					fnd:      next.t.to_string(),
					ex:       "REGISTER".to_string(),
				})
			},
		}
	}

	/// Parse a single [`OrderingTarget`]
	fn parse_ordering_target(&mut self) -> Result<OrderingTarget, ParseError> {
		let ord_label = self.next()?;
		match ord_label.t {
			TokenType::Label(l) => {
				let flags = OrderingTarget::from(l);

				if flags.is_empty() {
					return Err(ParseError::InvalidOrderingSpecifier {
						src_file: self.source_file.to_string(),
						location: Box::new(LocationInfo::from(ord_label)),
						spec:     l.to_string(),
					});
				}

				Ok(flags)
			},
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(ord_label)),
					fnd:      ord_label.t.to_string(),
					ex:       "ORDERING_TARGET".to_string(),
				})
			},
		}
	}
}
