//! Lexer functions handling the recognition of:
//!  - Keywords
//!  - Registers
//!  - Directives
//!  - Labels and Labeldefines

use super::token::{DirectiveToken, InstructionToken, RegisterToken};
use super::{Lexer, Token, TokenType};

impl<'s> Lexer<'s> {
	/// Attempt to match an identifier to a keyword, register, or directive,
	/// or return a new label if a match is not found
	pub(super) fn match_identifier(&mut self, id: &'s str) -> Token<'s> {
		match id {
			"add" => self.make_token(TokenType::Inst(InstructionToken::Add)),
			"addi" => self.make_token(TokenType::Inst(InstructionToken::Addi)),
			"sub" => self.make_token(TokenType::Inst(InstructionToken::Sub)),
			"and" => self.make_token(TokenType::Inst(InstructionToken::And)),
			"andi" => self.make_token(TokenType::Inst(InstructionToken::Andi)),
			"or" => self.make_token(TokenType::Inst(InstructionToken::Or)),
			"ori" => self.make_token(TokenType::Inst(InstructionToken::Ori)),
			"xor" => self.make_token(TokenType::Inst(InstructionToken::Xor)),
			"xori" => self.make_token(TokenType::Inst(InstructionToken::Xori)),
			"lsl" => self.make_token(TokenType::Inst(InstructionToken::Lsl)),
			"lsli" => self.make_token(TokenType::Inst(InstructionToken::Lsli)),
			"lsr" => self.make_token(TokenType::Inst(InstructionToken::Lsr)),
			"lsri" => self.make_token(TokenType::Inst(InstructionToken::Lsri)),
			"asr" => self.make_token(TokenType::Inst(InstructionToken::Asr)),
			"asri" => self.make_token(TokenType::Inst(InstructionToken::Asri)),
			"slt" => self.make_token(TokenType::Inst(InstructionToken::Slt)),
			"slti" => self.make_token(TokenType::Inst(InstructionToken::Slti)),
			"sltu" => self.make_token(TokenType::Inst(InstructionToken::Sltu)),
			"sltiu" => self.make_token(TokenType::Inst(InstructionToken::Sltiu)),
			"lw" => self.make_token(TokenType::Inst(InstructionToken::Lw)),
			"lh" => self.make_token(TokenType::Inst(InstructionToken::Lh)),
			"lhu" => self.make_token(TokenType::Inst(InstructionToken::Lhu)),
			"lb" => self.make_token(TokenType::Inst(InstructionToken::Lb)),
			"lbu" => self.make_token(TokenType::Inst(InstructionToken::Lbu)),
			"sw" => self.make_token(TokenType::Inst(InstructionToken::Sw)),
			"sh" => self.make_token(TokenType::Inst(InstructionToken::Sh)),
			"sb" => self.make_token(TokenType::Inst(InstructionToken::Sb)),
			"lui" => self.make_token(TokenType::Inst(InstructionToken::Lui)),
			"auipc" => self.make_token(TokenType::Inst(InstructionToken::Auipc)),
			"beq" => self.make_token(TokenType::Inst(InstructionToken::Beq)),
			"bne" => self.make_token(TokenType::Inst(InstructionToken::Bne)),
			"blt" => self.make_token(TokenType::Inst(InstructionToken::Blt)),
			"bltu" => self.make_token(TokenType::Inst(InstructionToken::Bltu)),
			"bge" => self.make_token(TokenType::Inst(InstructionToken::Bge)),
			"bgeu" => self.make_token(TokenType::Inst(InstructionToken::Bgeu)),
			"jal" => self.make_token(TokenType::Inst(InstructionToken::Jal)),
			"jalr" => self.make_token(TokenType::Inst(InstructionToken::Jalr)),
			"ecall" => self.make_token(TokenType::Inst(InstructionToken::Ecall)),
			"ebreak" => self.make_token(TokenType::Inst(InstructionToken::Ebreak)),
			"fence" => self.make_token(TokenType::Inst(InstructionToken::Fence)),
			"fence.tso" => self.make_token(TokenType::Inst(InstructionToken::FenceTso)),
			"fence.i" => self.make_token(TokenType::Inst(InstructionToken::Fencei)),
			"csrrw" => self.make_token(TokenType::Inst(InstructionToken::Csrrw)),
			"csrrwi" => self.make_token(TokenType::Inst(InstructionToken::Csrrwi)),
			"csrrs" => self.make_token(TokenType::Inst(InstructionToken::Csrrs)),
			"csrrsi" => self.make_token(TokenType::Inst(InstructionToken::Csrrsi)),
			"csrrc" => self.make_token(TokenType::Inst(InstructionToken::Csrrc)),
			"csrrci" => self.make_token(TokenType::Inst(InstructionToken::Csrrci)),
			"mul" => self.make_token(TokenType::Inst(InstructionToken::Mul)),
			"mulh" => self.make_token(TokenType::Inst(InstructionToken::Mulh)),
			"mulhu" => self.make_token(TokenType::Inst(InstructionToken::Mulhu)),
			"mulhsu" => self.make_token(TokenType::Inst(InstructionToken::Mulhsu)),
			"div" => self.make_token(TokenType::Inst(InstructionToken::Div)),
			"divu" => self.make_token(TokenType::Inst(InstructionToken::Divu)),
			"rem" => self.make_token(TokenType::Inst(InstructionToken::Rem)),
			"remu" => self.make_token(TokenType::Inst(InstructionToken::Remu)),

			"r0" => self.make_token(TokenType::Reg(RegisterToken::R0)),
			"r1" => self.make_token(TokenType::Reg(RegisterToken::R1)),
			"r2" => self.make_token(TokenType::Reg(RegisterToken::R2)),
			"r3" => self.make_token(TokenType::Reg(RegisterToken::R3)),
			"r4" => self.make_token(TokenType::Reg(RegisterToken::R4)),
			"r5" => self.make_token(TokenType::Reg(RegisterToken::R5)),
			"r6" => self.make_token(TokenType::Reg(RegisterToken::R6)),
			"r7" => self.make_token(TokenType::Reg(RegisterToken::R7)),
			"r8" => self.make_token(TokenType::Reg(RegisterToken::R8)),
			"r9" => self.make_token(TokenType::Reg(RegisterToken::R9)),
			"r10" => self.make_token(TokenType::Reg(RegisterToken::R10)),
			"r11" => self.make_token(TokenType::Reg(RegisterToken::R11)),
			"r12" => self.make_token(TokenType::Reg(RegisterToken::R12)),
			"r13" => self.make_token(TokenType::Reg(RegisterToken::R13)),
			"r14" => self.make_token(TokenType::Reg(RegisterToken::R14)),
			"r15" => self.make_token(TokenType::Reg(RegisterToken::R15)),
			"r16" => self.make_token(TokenType::Reg(RegisterToken::R16)),
			"r17" => self.make_token(TokenType::Reg(RegisterToken::R17)),
			"r18" => self.make_token(TokenType::Reg(RegisterToken::R18)),
			"r19" => self.make_token(TokenType::Reg(RegisterToken::R19)),
			"r20" => self.make_token(TokenType::Reg(RegisterToken::R20)),
			"r21" => self.make_token(TokenType::Reg(RegisterToken::R21)),
			"r22" => self.make_token(TokenType::Reg(RegisterToken::R22)),
			"r23" => self.make_token(TokenType::Reg(RegisterToken::R23)),
			"r24" => self.make_token(TokenType::Reg(RegisterToken::R24)),
			"r25" => self.make_token(TokenType::Reg(RegisterToken::R25)),
			"r26" => self.make_token(TokenType::Reg(RegisterToken::R26)),
			"r27" => self.make_token(TokenType::Reg(RegisterToken::R27)),
			"r28" => self.make_token(TokenType::Reg(RegisterToken::R28)),
			"r29" => self.make_token(TokenType::Reg(RegisterToken::R29)),
			"r30" => self.make_token(TokenType::Reg(RegisterToken::R30)),
			"r31" => self.make_token(TokenType::Reg(RegisterToken::R31)),

			"zero" => self.make_token(TokenType::Reg(RegisterToken::R0)),
			"ra" => self.make_token(TokenType::Reg(RegisterToken::R1)),
			"sp" => self.make_token(TokenType::Reg(RegisterToken::R2)),
			"gp" => self.make_token(TokenType::Reg(RegisterToken::R3)),
			"tp" => self.make_token(TokenType::Reg(RegisterToken::R4)),
			"fp" => self.make_token(TokenType::Reg(RegisterToken::R8)),
			"a0" => self.make_token(TokenType::Reg(RegisterToken::R10)),
			"a1" => self.make_token(TokenType::Reg(RegisterToken::R11)),
			"a2" => self.make_token(TokenType::Reg(RegisterToken::R12)),
			"a3" => self.make_token(TokenType::Reg(RegisterToken::R13)),
			"a4" => self.make_token(TokenType::Reg(RegisterToken::R14)),
			"a5" => self.make_token(TokenType::Reg(RegisterToken::R15)),
			"a6" => self.make_token(TokenType::Reg(RegisterToken::R16)),
			"a7" => self.make_token(TokenType::Reg(RegisterToken::R17)),
			"s0" => self.make_token(TokenType::Reg(RegisterToken::R8)),
			"s1" => self.make_token(TokenType::Reg(RegisterToken::R9)),
			"s2" => self.make_token(TokenType::Reg(RegisterToken::R18)),
			"s3" => self.make_token(TokenType::Reg(RegisterToken::R19)),
			"s4" => self.make_token(TokenType::Reg(RegisterToken::R20)),
			"s5" => self.make_token(TokenType::Reg(RegisterToken::R21)),
			"s6" => self.make_token(TokenType::Reg(RegisterToken::R22)),
			"s7" => self.make_token(TokenType::Reg(RegisterToken::R23)),
			"s8" => self.make_token(TokenType::Reg(RegisterToken::R24)),
			"s9" => self.make_token(TokenType::Reg(RegisterToken::R25)),
			"s10" => self.make_token(TokenType::Reg(RegisterToken::R26)),
			"s11" => self.make_token(TokenType::Reg(RegisterToken::R27)),
			"t0" => self.make_token(TokenType::Reg(RegisterToken::R5)),
			"t1" => self.make_token(TokenType::Reg(RegisterToken::R6)),
			"t2" => self.make_token(TokenType::Reg(RegisterToken::R7)),
			"t3" => self.make_token(TokenType::Reg(RegisterToken::R28)),
			"t4" => self.make_token(TokenType::Reg(RegisterToken::R29)),
			"t5" => self.make_token(TokenType::Reg(RegisterToken::R30)),
			"t6" => self.make_token(TokenType::Reg(RegisterToken::R31)),

			"$byte" => self.make_token(TokenType::Dir(DirectiveToken::Byte)),
			"$half" => self.make_token(TokenType::Dir(DirectiveToken::Half)),
			"$word" => self.make_token(TokenType::Dir(DirectiveToken::Word)),
			"$repeat" => self.make_token(TokenType::Dir(DirectiveToken::Repeat)),
			"$equ" => self.make_token(TokenType::Dir(DirectiveToken::Equ)),

			_ => {
				if id.starts_with('.') {
					if let Some(stripped) = id.strip_suffix(':') {
						self.make_token(TokenType::LocalLabelDefine(stripped))
					} else {
						self.make_token(TokenType::LocalLabel(id))
					}
				} else if let Some(stripped) = id.strip_suffix(':') {
					self.make_token(TokenType::LabelDefine(stripped))
				} else {
					self.make_token(TokenType::Label(id))
				}
			},
		}
	}
}
