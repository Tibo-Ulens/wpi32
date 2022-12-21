//! [`Lexer`] functions to process identifiers
//!
//! Handles the recognition of:
//!  - Instructions ([`InstToken`])
//!  - Registers ([`RegToken`])
//!  - Directives ([`DirToken`])
//!  - Section Names ([`TokenType::Section`])
//!  - Labels and LabelDefines ([`TokenType::Label`], [`TokenType::LocalLabel`],
//!    [`TokenType::LabelDefine`], [`TokenType::LocalLabelDefine`])

use super::token::{InstToken, RegToken};
use super::{
	BranchInstruction,
	CsrInstruction,
	CsriInstruction,
	Lexer,
	LoadInstruction,
	MdrInstruction,
	RriInstruction,
	RrrInstruction,
	StoreInstruction,
	Token,
	TokenType,
};
use crate::error::LexError;

impl<'s> Lexer<'s> {
	/// Attempt to match an identifier to an instruction, register, section
	/// name, or directive, or return a new label if a match is not found
	pub(super) fn match_identifier(&mut self, id: &'s str) -> Result<Token<'s>, LexError> {
		match &id.to_lowercase()[..] {
			"addi" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Addi)))),
			"andi" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Andi)))),
			"ori" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Ori)))),
			"xori" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Xori)))),
			"lsli" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Lsli)))),
			"lsri" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Lsri)))),
			"asri" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Asri)))),
			"slti" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Slti)))),
			"sltiu" => Ok(self.make_token(TokenType::Inst(InstToken::Rri(RriInstruction::Sltiu)))),
			"add" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Add)))),
			"sub" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Sub)))),
			"and" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::And)))),
			"or" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Or)))),
			"xor" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Xor)))),
			"lsl" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Lsl)))),
			"lsr" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Lsr)))),
			"asr" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Asr)))),
			"slt" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Slt)))),
			"sltu" => Ok(self.make_token(TokenType::Inst(InstToken::Rrr(RrrInstruction::Sltu)))),
			"lw" => Ok(self.make_token(TokenType::Inst(InstToken::Load(LoadInstruction::Lw)))),
			"lh" => Ok(self.make_token(TokenType::Inst(InstToken::Load(LoadInstruction::Lh)))),
			"lhu" => Ok(self.make_token(TokenType::Inst(InstToken::Load(LoadInstruction::Lhu)))),
			"lb" => Ok(self.make_token(TokenType::Inst(InstToken::Load(LoadInstruction::Lb)))),
			"lbu" => Ok(self.make_token(TokenType::Inst(InstToken::Load(LoadInstruction::Lbu)))),
			"sw" => Ok(self.make_token(TokenType::Inst(InstToken::Store(StoreInstruction::Sw)))),
			"sh" => Ok(self.make_token(TokenType::Inst(InstToken::Store(StoreInstruction::Sh)))),
			"sb" => Ok(self.make_token(TokenType::Inst(InstToken::Store(StoreInstruction::Sb)))),
			"lui" => Ok(self.make_token(TokenType::Inst(InstToken::Lui))),
			"auipc" => Ok(self.make_token(TokenType::Inst(InstToken::Auipc))),
			"beq" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Branch(BranchInstruction::Beq))))
			},
			"bne" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Branch(BranchInstruction::Bne))))
			},
			"blt" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Branch(BranchInstruction::Blt))))
			},
			"bltu" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Branch(BranchInstruction::Bltu))))
			},
			"bge" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Branch(BranchInstruction::Bge))))
			},
			"bgeu" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Branch(BranchInstruction::Bgeu))))
			},
			"jal" => Ok(self.make_token(TokenType::Inst(InstToken::Jal))),
			"jalr" => Ok(self.make_token(TokenType::Inst(InstToken::Jalr))),
			"ecall" => Ok(self.make_token(TokenType::Inst(InstToken::Ecall))),
			"ebreak" => Ok(self.make_token(TokenType::Inst(InstToken::Ebreak))),
			"fence" => Ok(self.make_token(TokenType::Inst(InstToken::Fence))),
			"fence.tso" => Ok(self.make_token(TokenType::Inst(InstToken::FenceTso))),
			"fence.i" => Ok(self.make_token(TokenType::Inst(InstToken::Fencei))),
			"csrrw" => Ok(self.make_token(TokenType::Inst(InstToken::Csr(CsrInstruction::Csrrw)))),
			"csrrs" => Ok(self.make_token(TokenType::Inst(InstToken::Csr(CsrInstruction::Csrrs)))),
			"csrrc" => Ok(self.make_token(TokenType::Inst(InstToken::Csr(CsrInstruction::Csrrc)))),
			"csrrwi" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Csri(CsriInstruction::Csrrwi))))
			},
			"csrrsi" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Csri(CsriInstruction::Csrrsi))))
			},
			"csrrci" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Csri(CsriInstruction::Csrrci))))
			},
			"mul" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Mul)))),
			"mulh" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Mulh)))),
			"mulhu" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Mulhu)))),
			"mulhsu" => {
				Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Mulhsu))))
			},
			"div" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Div)))),
			"divu" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Divu)))),
			"rem" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Rem)))),
			"remu" => Ok(self.make_token(TokenType::Inst(InstToken::Mdr(MdrInstruction::Remu)))),

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

			d if d.starts_with('#') => Ok(self.make_token(TokenType::Dir(d))),

			_ => Ok(self.make_token(TokenType::Identifier(id))),
		}
	}
}
