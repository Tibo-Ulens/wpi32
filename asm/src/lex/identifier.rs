//! Lexer functions handling the recognition of:
//!  - Keywords
//!  - Registers
//!  - Directives
//!  - Labels and Labeldefines

use super::{Lexer, Token, TokenType};

impl<'s> Lexer<'s> {
	/// Attempt to match an identifier to a keyword, register, or directive,
	/// or return a new label if a match is not found
	pub(super) fn match_identifier(&mut self, id: &'s str) -> Token<'s> {
		match id {
			"add" => self.make_token(TokenType::KwAdd),
			"addi" => self.make_token(TokenType::KwAddi),
			"sub" => self.make_token(TokenType::KwSub),
			"and" => self.make_token(TokenType::KwAnd),
			"andi" => self.make_token(TokenType::KwAndi),
			"or" => self.make_token(TokenType::KwOr),
			"ori" => self.make_token(TokenType::KwOri),
			"xor" => self.make_token(TokenType::KwXor),
			"xori" => self.make_token(TokenType::KwXori),
			"lsl" => self.make_token(TokenType::KwLsl),
			"lsli" => self.make_token(TokenType::KwLsli),
			"lsr" => self.make_token(TokenType::KwLsr),
			"lsri" => self.make_token(TokenType::KwLsri),
			"asr" => self.make_token(TokenType::KwAsr),
			"asri" => self.make_token(TokenType::KwAsri),
			"slt" => self.make_token(TokenType::KwSlt),
			"slti" => self.make_token(TokenType::KwSlti),
			"sltu" => self.make_token(TokenType::KwSltu),
			"sltiu" => self.make_token(TokenType::KwSltiu),
			"lw" => self.make_token(TokenType::KwLw),
			"lh" => self.make_token(TokenType::KwLh),
			"lhu" => self.make_token(TokenType::KwLhu),
			"lb" => self.make_token(TokenType::KwLb),
			"lbu" => self.make_token(TokenType::KwLbu),
			"sw" => self.make_token(TokenType::KwSw),
			"sh" => self.make_token(TokenType::KwSh),
			"sb" => self.make_token(TokenType::KwSb),
			"lui" => self.make_token(TokenType::KwLui),
			"auipc" => self.make_token(TokenType::KwAuipc),
			"beq" => self.make_token(TokenType::KwBeq),
			"bne" => self.make_token(TokenType::KwBne),
			"blt" => self.make_token(TokenType::KwBlt),
			"bltu" => self.make_token(TokenType::KwBltu),
			"bge" => self.make_token(TokenType::KwBge),
			"bgeu" => self.make_token(TokenType::KwBgeu),
			"jal" => self.make_token(TokenType::KwJal),
			"jalr" => self.make_token(TokenType::KwJalr),
			"ecall" => self.make_token(TokenType::KwEcall),
			"ebreak" => self.make_token(TokenType::KwEbreak),
			"fence" => self.make_token(TokenType::KwFence),
			"fence.i" => self.make_token(TokenType::KwFencei),
			"csrrw" => self.make_token(TokenType::KwCsrrw),
			"csrrwi" => self.make_token(TokenType::KwCsrrwi),
			"csrrs" => self.make_token(TokenType::KwCsrrs),
			"csrrsi" => self.make_token(TokenType::KwCsrrsi),
			"csrrc" => self.make_token(TokenType::KwCsrrc),
			"csrrci" => self.make_token(TokenType::KwCsrrci),
			"mul" => self.make_token(TokenType::KwMul),
			"mulh" => self.make_token(TokenType::KwMulh),
			"mulhu" => self.make_token(TokenType::KwMulhu),
			"mulhsu" => self.make_token(TokenType::KwMulhsu),
			"div" => self.make_token(TokenType::KwDiv),
			"divu" => self.make_token(TokenType::KwDivu),
			"rem" => self.make_token(TokenType::KwRem),
			"remu" => self.make_token(TokenType::KwRemu),

			"r0" => self.make_token(TokenType::RegR0),
			"r1" => self.make_token(TokenType::RegR1),
			"r2" => self.make_token(TokenType::RegR2),
			"r3" => self.make_token(TokenType::RegR3),
			"r4" => self.make_token(TokenType::RegR4),
			"r5" => self.make_token(TokenType::RegR5),
			"r6" => self.make_token(TokenType::RegR6),
			"r7" => self.make_token(TokenType::RegR7),
			"r8" => self.make_token(TokenType::RegR8),
			"r9" => self.make_token(TokenType::RegR9),
			"r10" => self.make_token(TokenType::RegR10),
			"r11" => self.make_token(TokenType::RegR11),
			"r12" => self.make_token(TokenType::RegR12),
			"r13" => self.make_token(TokenType::RegR13),
			"r14" => self.make_token(TokenType::RegR14),
			"r15" => self.make_token(TokenType::RegR15),
			"r16" => self.make_token(TokenType::RegR16),
			"r17" => self.make_token(TokenType::RegR17),
			"r18" => self.make_token(TokenType::RegR18),
			"r19" => self.make_token(TokenType::RegR19),
			"r20" => self.make_token(TokenType::RegR20),
			"r21" => self.make_token(TokenType::RegR21),
			"r22" => self.make_token(TokenType::RegR22),
			"r23" => self.make_token(TokenType::RegR23),
			"r24" => self.make_token(TokenType::RegR24),
			"r25" => self.make_token(TokenType::RegR25),
			"r26" => self.make_token(TokenType::RegR26),
			"r27" => self.make_token(TokenType::RegR27),
			"r28" => self.make_token(TokenType::RegR28),
			"r29" => self.make_token(TokenType::RegR29),
			"r30" => self.make_token(TokenType::RegR30),
			"r31" => self.make_token(TokenType::RegR31),
			"zero" => self.make_token(TokenType::RegR0),

			"ra" => self.make_token(TokenType::RegR1),
			"sp" => self.make_token(TokenType::RegR2),
			"gp" => self.make_token(TokenType::RegR3),
			"tp" => self.make_token(TokenType::RegR4),
			"fp" => self.make_token(TokenType::RegR8),
			"a0" => self.make_token(TokenType::RegR10),
			"a1" => self.make_token(TokenType::RegR11),
			"a2" => self.make_token(TokenType::RegR12),
			"a3" => self.make_token(TokenType::RegR13),
			"a4" => self.make_token(TokenType::RegR14),
			"a5" => self.make_token(TokenType::RegR15),
			"a6" => self.make_token(TokenType::RegR16),
			"a7" => self.make_token(TokenType::RegR17),
			"s0" => self.make_token(TokenType::RegR8),
			"s1" => self.make_token(TokenType::RegR9),
			"s2" => self.make_token(TokenType::RegR18),
			"s3" => self.make_token(TokenType::RegR19),
			"s4" => self.make_token(TokenType::RegR20),
			"s5" => self.make_token(TokenType::RegR21),
			"s6" => self.make_token(TokenType::RegR22),
			"s7" => self.make_token(TokenType::RegR23),
			"s8" => self.make_token(TokenType::RegR24),
			"s9" => self.make_token(TokenType::RegR25),
			"s10" => self.make_token(TokenType::RegR26),
			"s11" => self.make_token(TokenType::RegR27),
			"t0" => self.make_token(TokenType::RegR5),
			"t1" => self.make_token(TokenType::RegR6),
			"t2" => self.make_token(TokenType::RegR7),
			"t3" => self.make_token(TokenType::RegR28),
			"t4" => self.make_token(TokenType::RegR29),
			"t5" => self.make_token(TokenType::RegR30),
			"t6" => self.make_token(TokenType::RegR31),

			"$byte" => self.make_token(TokenType::DirByte),
			"$half" => self.make_token(TokenType::DirHalf),
			"$word" => self.make_token(TokenType::DirWord),
			"$repeat" => self.make_token(TokenType::DirRepeat),
			"$equ" => self.make_token(TokenType::DirEqu),

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
