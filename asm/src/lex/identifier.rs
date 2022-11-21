//! [`Lexer`] functions to process identifiers
//!
//! Handles the recognition of:
//!  - Instructions ([`InstToken`])
//!  - Registers ([`RegToken`])
//!  - Directives ([`DirToken`])
//!  - Section Names ([`TokenType::Section`])
//!  - Labels and LabelDefines ([`TokenType::Label`], [`TokenType::LocalLabel`],
//!    [`TokenType::LabelDefine`], [`TokenType::LocalLabelDefine`])

use super::token::{DirToken, InstToken, RegToken};
use super::{Lexer, Token, TokenType};
use crate::error::LexError;

impl<'s> Lexer<'s> {
	/// Attempt to match an identifier to an instruction, register, section
	/// name, or directive, or return a new label if a match is not found
	pub(super) fn match_identifier(&mut self, id: &'s str) -> Result<Token<'s>, LexError> {
		match id {
			"add" => Ok(self.make_token(TokenType::Inst(InstToken::Add))),
			"addi" => Ok(self.make_token(TokenType::Inst(InstToken::Addi))),
			"sub" => Ok(self.make_token(TokenType::Inst(InstToken::Sub))),
			"and" => Ok(self.make_token(TokenType::Inst(InstToken::And))),
			"andi" => Ok(self.make_token(TokenType::Inst(InstToken::Andi))),
			"or" => Ok(self.make_token(TokenType::Inst(InstToken::Or))),
			"ori" => Ok(self.make_token(TokenType::Inst(InstToken::Ori))),
			"xor" => Ok(self.make_token(TokenType::Inst(InstToken::Xor))),
			"xori" => Ok(self.make_token(TokenType::Inst(InstToken::Xori))),
			"lsl" => Ok(self.make_token(TokenType::Inst(InstToken::Lsl))),
			"lsli" => Ok(self.make_token(TokenType::Inst(InstToken::Lsli))),
			"lsr" => Ok(self.make_token(TokenType::Inst(InstToken::Lsr))),
			"lsri" => Ok(self.make_token(TokenType::Inst(InstToken::Lsri))),
			"asr" => Ok(self.make_token(TokenType::Inst(InstToken::Asr))),
			"asri" => Ok(self.make_token(TokenType::Inst(InstToken::Asri))),
			"slt" => Ok(self.make_token(TokenType::Inst(InstToken::Slt))),
			"slti" => Ok(self.make_token(TokenType::Inst(InstToken::Slti))),
			"sltu" => Ok(self.make_token(TokenType::Inst(InstToken::Sltu))),
			"sltiu" => Ok(self.make_token(TokenType::Inst(InstToken::Sltiu))),
			"lw" => Ok(self.make_token(TokenType::Inst(InstToken::Lw))),
			"lh" => Ok(self.make_token(TokenType::Inst(InstToken::Lh))),
			"lhu" => Ok(self.make_token(TokenType::Inst(InstToken::Lhu))),
			"lb" => Ok(self.make_token(TokenType::Inst(InstToken::Lb))),
			"lbu" => Ok(self.make_token(TokenType::Inst(InstToken::Lbu))),
			"sw" => Ok(self.make_token(TokenType::Inst(InstToken::Sw))),
			"sh" => Ok(self.make_token(TokenType::Inst(InstToken::Sh))),
			"sb" => Ok(self.make_token(TokenType::Inst(InstToken::Sb))),
			"lui" => Ok(self.make_token(TokenType::Inst(InstToken::Lui))),
			"auipc" => Ok(self.make_token(TokenType::Inst(InstToken::Auipc))),
			"beq" => Ok(self.make_token(TokenType::Inst(InstToken::Beq))),
			"bne" => Ok(self.make_token(TokenType::Inst(InstToken::Bne))),
			"blt" => Ok(self.make_token(TokenType::Inst(InstToken::Blt))),
			"bltu" => Ok(self.make_token(TokenType::Inst(InstToken::Bltu))),
			"bge" => Ok(self.make_token(TokenType::Inst(InstToken::Bge))),
			"bgeu" => Ok(self.make_token(TokenType::Inst(InstToken::Bgeu))),
			"jal" => Ok(self.make_token(TokenType::Inst(InstToken::Jal))),
			"jalr" => Ok(self.make_token(TokenType::Inst(InstToken::Jalr))),
			"ecall" => Ok(self.make_token(TokenType::Inst(InstToken::Ecall))),
			"ebreak" => Ok(self.make_token(TokenType::Inst(InstToken::Ebreak))),
			"fence" => Ok(self.make_token(TokenType::Inst(InstToken::Fence))),
			"fence.tso" => Ok(self.make_token(TokenType::Inst(InstToken::FenceTso))),
			"fence.i" => Ok(self.make_token(TokenType::Inst(InstToken::Fencei))),
			"csrrw" => Ok(self.make_token(TokenType::Inst(InstToken::Csrrw))),
			"csrrwi" => Ok(self.make_token(TokenType::Inst(InstToken::Csrrwi))),
			"csrrs" => Ok(self.make_token(TokenType::Inst(InstToken::Csrrs))),
			"csrrsi" => Ok(self.make_token(TokenType::Inst(InstToken::Csrrsi))),
			"csrrc" => Ok(self.make_token(TokenType::Inst(InstToken::Csrrc))),
			"csrrci" => Ok(self.make_token(TokenType::Inst(InstToken::Csrrci))),
			"mul" => Ok(self.make_token(TokenType::Inst(InstToken::Mul))),
			"mulh" => Ok(self.make_token(TokenType::Inst(InstToken::Mulh))),
			"mulhu" => Ok(self.make_token(TokenType::Inst(InstToken::Mulhu))),
			"mulhsu" => Ok(self.make_token(TokenType::Inst(InstToken::Mulhsu))),
			"div" => Ok(self.make_token(TokenType::Inst(InstToken::Div))),
			"divu" => Ok(self.make_token(TokenType::Inst(InstToken::Divu))),
			"rem" => Ok(self.make_token(TokenType::Inst(InstToken::Rem))),
			"remu" => Ok(self.make_token(TokenType::Inst(InstToken::Remu))),

			"r0" => Ok(self.make_token(TokenType::Reg(RegToken::R0))),
			"r1" => Ok(self.make_token(TokenType::Reg(RegToken::R1))),
			"r2" => Ok(self.make_token(TokenType::Reg(RegToken::R2))),
			"r3" => Ok(self.make_token(TokenType::Reg(RegToken::R3))),
			"r4" => Ok(self.make_token(TokenType::Reg(RegToken::R4))),
			"r5" => Ok(self.make_token(TokenType::Reg(RegToken::R5))),
			"r6" => Ok(self.make_token(TokenType::Reg(RegToken::R6))),
			"r7" => Ok(self.make_token(TokenType::Reg(RegToken::R7))),
			"r8" => Ok(self.make_token(TokenType::Reg(RegToken::R8))),
			"r9" => Ok(self.make_token(TokenType::Reg(RegToken::R9))),
			"r10" => Ok(self.make_token(TokenType::Reg(RegToken::R10))),
			"r11" => Ok(self.make_token(TokenType::Reg(RegToken::R11))),
			"r12" => Ok(self.make_token(TokenType::Reg(RegToken::R12))),
			"r13" => Ok(self.make_token(TokenType::Reg(RegToken::R13))),
			"r14" => Ok(self.make_token(TokenType::Reg(RegToken::R14))),
			"r15" => Ok(self.make_token(TokenType::Reg(RegToken::R15))),
			"r16" => Ok(self.make_token(TokenType::Reg(RegToken::R16))),
			"r17" => Ok(self.make_token(TokenType::Reg(RegToken::R17))),
			"r18" => Ok(self.make_token(TokenType::Reg(RegToken::R18))),
			"r19" => Ok(self.make_token(TokenType::Reg(RegToken::R19))),
			"r20" => Ok(self.make_token(TokenType::Reg(RegToken::R20))),
			"r21" => Ok(self.make_token(TokenType::Reg(RegToken::R21))),
			"r22" => Ok(self.make_token(TokenType::Reg(RegToken::R22))),
			"r23" => Ok(self.make_token(TokenType::Reg(RegToken::R23))),
			"r24" => Ok(self.make_token(TokenType::Reg(RegToken::R24))),
			"r25" => Ok(self.make_token(TokenType::Reg(RegToken::R25))),
			"r26" => Ok(self.make_token(TokenType::Reg(RegToken::R26))),
			"r27" => Ok(self.make_token(TokenType::Reg(RegToken::R27))),
			"r28" => Ok(self.make_token(TokenType::Reg(RegToken::R28))),
			"r29" => Ok(self.make_token(TokenType::Reg(RegToken::R29))),
			"r30" => Ok(self.make_token(TokenType::Reg(RegToken::R30))),
			"r31" => Ok(self.make_token(TokenType::Reg(RegToken::R31))),

			"zero" => Ok(self.make_token(TokenType::Reg(RegToken::R0))),
			"ra" => Ok(self.make_token(TokenType::Reg(RegToken::R1))),
			"sp" => Ok(self.make_token(TokenType::Reg(RegToken::R2))),
			"gp" => Ok(self.make_token(TokenType::Reg(RegToken::R3))),
			"tp" => Ok(self.make_token(TokenType::Reg(RegToken::R4))),
			"fp" => Ok(self.make_token(TokenType::Reg(RegToken::R8))),
			"a0" => Ok(self.make_token(TokenType::Reg(RegToken::R10))),
			"a1" => Ok(self.make_token(TokenType::Reg(RegToken::R11))),
			"a2" => Ok(self.make_token(TokenType::Reg(RegToken::R12))),
			"a3" => Ok(self.make_token(TokenType::Reg(RegToken::R13))),
			"a4" => Ok(self.make_token(TokenType::Reg(RegToken::R14))),
			"a5" => Ok(self.make_token(TokenType::Reg(RegToken::R15))),
			"a6" => Ok(self.make_token(TokenType::Reg(RegToken::R16))),
			"a7" => Ok(self.make_token(TokenType::Reg(RegToken::R17))),
			"s0" => Ok(self.make_token(TokenType::Reg(RegToken::R8))),
			"s1" => Ok(self.make_token(TokenType::Reg(RegToken::R9))),
			"s2" => Ok(self.make_token(TokenType::Reg(RegToken::R18))),
			"s3" => Ok(self.make_token(TokenType::Reg(RegToken::R19))),
			"s4" => Ok(self.make_token(TokenType::Reg(RegToken::R20))),
			"s5" => Ok(self.make_token(TokenType::Reg(RegToken::R21))),
			"s6" => Ok(self.make_token(TokenType::Reg(RegToken::R22))),
			"s7" => Ok(self.make_token(TokenType::Reg(RegToken::R23))),
			"s8" => Ok(self.make_token(TokenType::Reg(RegToken::R24))),
			"s9" => Ok(self.make_token(TokenType::Reg(RegToken::R25))),
			"s10" => Ok(self.make_token(TokenType::Reg(RegToken::R26))),
			"s11" => Ok(self.make_token(TokenType::Reg(RegToken::R27))),
			"t0" => Ok(self.make_token(TokenType::Reg(RegToken::R5))),
			"t1" => Ok(self.make_token(TokenType::Reg(RegToken::R6))),
			"t2" => Ok(self.make_token(TokenType::Reg(RegToken::R7))),
			"t3" => Ok(self.make_token(TokenType::Reg(RegToken::R28))),
			"t4" => Ok(self.make_token(TokenType::Reg(RegToken::R29))),
			"t5" => Ok(self.make_token(TokenType::Reg(RegToken::R30))),
			"t6" => Ok(self.make_token(TokenType::Reg(RegToken::R31))),

			d if d.starts_with('#') => {
				match d {
					"#SECTION" => Ok(self.make_token(TokenType::Dir(DirToken::Section))),
					"#BYTES" => Ok(self.make_token(TokenType::Dir(DirToken::Bytes))),
					"#HALVES" => Ok(self.make_token(TokenType::Dir(DirToken::Halves))),
					"#WORDS" => Ok(self.make_token(TokenType::Dir(DirToken::Words))),
					"#RES_BYTES" => Ok(self.make_token(TokenType::Dir(DirToken::ResBytes))),
					"#RES_HALVES" => Ok(self.make_token(TokenType::Dir(DirToken::ResHalves))),
					"#RES_WORDS" => Ok(self.make_token(TokenType::Dir(DirToken::ResWords))),
					"#REPEAT" => Ok(self.make_token(TokenType::Dir(DirToken::Repeat))),
					"#CONST" => Ok(self.make_token(TokenType::Dir(DirToken::Const))),
					_ => {
						Err(LexError::InvalidDirective {
							src_file: self.source_file.clone(),
							line:     self.line,
							col:      self.col,
							span:     d.len(),
							src_line: self.get_curr_line().to_string(),
							dir:      d.to_string(),
						})
					},
				}
			},

			".TEXT" => Ok(self.make_token(TokenType::Section(id))),
			".DATA" => Ok(self.make_token(TokenType::Section(id))),
			".BSS" => Ok(self.make_token(TokenType::Section(id))),

			_ => {
				if id.starts_with('.') {
					if let Some(stripped) = id.strip_suffix(':') {
						Ok(self.make_token(TokenType::LocalLabelDefine(stripped)))
					} else {
						Ok(self.make_token(TokenType::LocalLabel(id)))
					}
				} else if let Some(stripped) = id.strip_suffix(':') {
					Ok(self.make_token(TokenType::LabelDefine(stripped)))
				} else {
					Ok(self.make_token(TokenType::Label(id)))
				}
			},
		}
	}
}
