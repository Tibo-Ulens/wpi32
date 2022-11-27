//! [`Parser`] functions to parse [`Instruction`]s

use super::ast::{AddrOffset, Address, Immediate, Instruction, OffsetOperator, OrderingTarget};
use super::Parser;
use crate::error::{LocationInfo, ParseError};
use crate::lex::{
	BranchInstruction,
	CsrInstruction,
	CsriInstruction,
	InstToken,
	LoadInstruction,
	MdrInstruction,
	OpToken,
	RegToken,
	RriInstruction,
	RrrInstruction,
	StoreInstruction,
	TokenType,
};

impl<'s> Parser<'s> {
	/// Parse any valid [`Instruction`]
	///
	/// Assumes the current [`Token`](crate::lex::Token) has [`TokenType`]
	/// [`TokenType::Inst`]
	pub(super) fn parse_instruction<'r>(&'r mut self) -> Result<Instruction<'s>, ParseError> {
		// Unwrap is assumed to be safe
		let instruction_token = self.next().unwrap();

		match &instruction_token.t {
			TokenType::Inst(InstToken::Rri(rri_inst)) => {
				let (dest, src, imm) = self.parse_rri()?;

				match rri_inst {
					RriInstruction::Addi => Ok(Instruction::Addi { dest, src, imm }),
					RriInstruction::Andi => Ok(Instruction::Andi { dest, src, imm }),
					RriInstruction::Ori => Ok(Instruction::Ori { dest, src, imm }),
					RriInstruction::Xori => Ok(Instruction::Xori { dest, src, imm }),
					RriInstruction::Lsli => Ok(Instruction::Lsli { dest, src, imm }),
					RriInstruction::Lsri => Ok(Instruction::Lsri { dest, src, imm }),
					RriInstruction::Asri => Ok(Instruction::Asri { dest, src, imm }),
					RriInstruction::Slti => Ok(Instruction::Slti { dest, src, imm }),
					RriInstruction::Sltiu => Ok(Instruction::Sltiu { dest, src, imm }),
				}
			},
			TokenType::Inst(InstToken::Rrr(rrr_inst)) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				match rrr_inst {
					RrrInstruction::Add => Ok(Instruction::Add { dest, src1, src2 }),
					RrrInstruction::Sub => Ok(Instruction::Sub { dest, src1, src2 }),
					RrrInstruction::And => Ok(Instruction::And { dest, src1, src2 }),
					RrrInstruction::Or => Ok(Instruction::Or { dest, src1, src2 }),
					RrrInstruction::Xor => Ok(Instruction::Xor { dest, src1, src2 }),
					RrrInstruction::Lsl => Ok(Instruction::Lsl { dest, src1, src2 }),
					RrrInstruction::Lsr => Ok(Instruction::Lsr { dest, src1, src2 }),
					RrrInstruction::Asr => Ok(Instruction::Asr { dest, src1, src2 }),
					RrrInstruction::Slt => Ok(Instruction::Slt { dest, src1, src2 }),
					RrrInstruction::Sltu => Ok(Instruction::Sltu { dest, src1, src2 }),
				}
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
			TokenType::Inst(InstToken::Branch(b_inst)) => {
				let (src1, src2, offset) = self.parse_rri()?;

				match b_inst {
					BranchInstruction::Beq => Ok(Instruction::Beq { src1, src2, offset }),
					BranchInstruction::Bne => Ok(Instruction::Bne { src1, src2, offset }),
					BranchInstruction::Blt => Ok(Instruction::Blt { src1, src2, offset }),
					BranchInstruction::Bltu => Ok(Instruction::Bltu { src1, src2, offset }),
					BranchInstruction::Bge => Ok(Instruction::Bge { src1, src2, offset }),
					BranchInstruction::Bgeu => Ok(Instruction::Bgeu { src1, src2, offset }),
				}
			},
			TokenType::Inst(InstToken::Load(l_inst)) => {
				let (dest, addr) = self.parse_ra()?;

				match l_inst {
					LoadInstruction::Lw => Ok(Instruction::Lw { dest, addr }),
					LoadInstruction::Lh => Ok(Instruction::Lh { dest, addr }),
					LoadInstruction::Lhu => Ok(Instruction::Lhu { dest, addr }),
					LoadInstruction::Lb => Ok(Instruction::Lb { dest, addr }),
					LoadInstruction::Lbu => Ok(Instruction::Lbu { dest, addr }),
				}
			},
			TokenType::Inst(InstToken::Store(s_inst)) => {
				let (dest, src) = self.parse_ar()?;

				match s_inst {
					StoreInstruction::Sw => Ok(Instruction::Sw { dest, src }),
					StoreInstruction::Sh => Ok(Instruction::Sh { dest, src }),
					StoreInstruction::Sb => Ok(Instruction::Sb { dest, src }),
				}
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
			TokenType::Inst(InstToken::Csr(csr_inst)) => {
				let (dest, src, target) = self.parse_rri()?;

				match csr_inst {
					CsrInstruction::Csrrw => Ok(Instruction::Csrrw { dest, src, target }),
					CsrInstruction::Csrrs => Ok(Instruction::Csrrs { dest, src, target }),
					CsrInstruction::Csrrc => Ok(Instruction::Csrrc { dest, src, target }),
				}
			},
			TokenType::Inst(InstToken::Csri(csri_inst)) => {
				let (dest, src, target) = self.parse_rii()?;

				match csri_inst {
					CsriInstruction::Csrrwi => Ok(Instruction::Csrrwi { dest, src, target }),
					CsriInstruction::Csrrsi => Ok(Instruction::Csrrsi { dest, src, target }),
					CsriInstruction::Csrrci => Ok(Instruction::Csrrci { dest, src, target }),
				}
			},
			TokenType::Inst(InstToken::Mdr(mdr_inst)) => {
				let (dest, src1, src2) = self.parse_rrr()?;

				match mdr_inst {
					MdrInstruction::Mul => Ok(Instruction::Mul { dest, src1, src2 }),
					MdrInstruction::Mulh => Ok(Instruction::Mulh { dest, src1, src2 }),
					MdrInstruction::Mulhu => Ok(Instruction::Mulhu { dest, src1, src2 }),
					MdrInstruction::Mulhsu => Ok(Instruction::Mulhsu { dest, src1, src2 }),
					MdrInstruction::Div => Ok(Instruction::Div { dest, src1, src2 }),
					MdrInstruction::Divu => Ok(Instruction::Divu { dest, src1, src2 }),
					MdrInstruction::Rem => Ok(Instruction::Rem { dest, src1, src2 }),
					MdrInstruction::Remu => Ok(Instruction::Remu { dest, src1, src2 }),
				}
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
				// Unwrap is safe as peek is Ok
				self.next().unwrap();

				let offset = self.parse_immediate()?;

				Some(AddrOffset { op: OffsetOperator::from(o), offset })
			},
			_ => None,
		};

		let close_token = self.next()?;
		if close_token.t != TokenType::SymRightBracket {
			return Err(ParseError::UnclosedDelimiter {
				src_file:       self.source_file.to_string(),
				delim_type:     "bracket".to_string(),
				found:          close_token.t.to_string(),
				close_location: Box::new(LocationInfo::from(close_token)),
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
					found:    next.t.to_string(),
					expected: "REGISTER".to_string(),
				})
			},
		}
	}

	/// Parse a single [`OrderingTarget`]
	fn parse_ordering_target(&mut self) -> Result<OrderingTarget, ParseError> {
		let ord_raw = self.next()?;
		match ord_raw.t {
			TokenType::Identifier(id) => {
				let flags = OrderingTarget::from(id);

				if flags.is_empty() {
					return Err(ParseError::InvalidOrderingSpecifier {
						src_file: self.source_file.to_string(),
						location: Box::new(LocationInfo::from(ord_raw)),
						spec:     id.to_string(),
					});
				}

				Ok(flags)
			},
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(ord_raw)),
					found:    ord_raw.t.to_string(),
					expected: "ORDERING_TARGET".to_string(),
				})
			},
		}
	}
}
