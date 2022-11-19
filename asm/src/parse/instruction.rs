//! AST instruction type definitions

use bitflags::bitflags;

use super::Immediate;
use crate::lex::RegisterToken;

#[derive(Clone, Debug)]
pub(crate) enum Instruction<'s> {
	// Integer RegisterToken Immediate
	Addi { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Slti { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Sltiu { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Andi { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Ori { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Xori { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Lsli { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Lsri { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },
	Asri { dest: RegisterToken, src: RegisterToken, imm: Immediate<'s> },

	// Integer RegisterToken RegisterToken
	Add { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Slt { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Sltu { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	And { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Or { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Xor { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Lsl { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Lsr { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Asr { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	Sub { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },

	// Upper Immediate
	Lui { dest: RegisterToken, imm: Immediate<'s> },
	Auipc { dest: RegisterToken, imm: Immediate<'s> },

	// Jump and link
	Jal { dest: RegisterToken, offset: Immediate<'s> },
	// Jump and link register
	Jalr { dest: RegisterToken, base: RegisterToken, offset: Immediate<'s> },

	// Conditional Branch
	Beq { src1: RegisterToken, src2: RegisterToken, offset: Immediate<'s> },
	Bne { src1: RegisterToken, src2: RegisterToken, offset: Immediate<'s> },
	Blt { src1: RegisterToken, src2: RegisterToken, offset: Immediate<'s> },
	Bltu { src1: RegisterToken, src2: RegisterToken, offset: Immediate<'s> },
	Bge { src1: RegisterToken, src2: RegisterToken, offset: Immediate<'s> },
	Bgeu { src1: RegisterToken, src2: RegisterToken, offset: Immediate<'s> },

	// Load
	Lb { dest: RegisterToken, addr: Address<'s> },
	Lbu { dest: RegisterToken, addr: Address<'s> },
	Lh { dest: RegisterToken, addr: Address<'s> },
	Lhu { dest: RegisterToken, addr: Address<'s> },
	Lw { dest: RegisterToken, addr: Address<'s> },
	Lwu { dest: RegisterToken, addr: Address<'s> },
	// Store
	Sb { dest: RegisterToken, addr: Address<'s> },
	Sh { dest: RegisterToken, addr: Address<'s> },
	Sw { dest: RegisterToken, addr: Address<'s> },

	// Memory Ordering
	Fence { pred: OrderingTarget, succ: OrderingTarget },
	FenceTso { pred: OrderingTarget, succ: OrderingTarget },

	// System Interaction
	ECall,
	EBreak,

	// Instruction Fetch Fencing
	FenceI,

	// CSR RegisterToken
	CsrRw { dest: RegisterToken, src: RegisterToken, target: Immediate<'s> },
	CsrRs { dest: RegisterToken, src: RegisterToken, target: Immediate<'s> },
	CsrRc { dest: RegisterToken, src: RegisterToken, target: Immediate<'s> },
	// CSR Immediate
	CsrRwi { dest: RegisterToken, src: Immediate<'s>, target: Immediate<'s> },
	CsrRsi { dest: RegisterToken, src: Immediate<'s>, target: Immediate<'s> },
	CsrRci { dest: RegisterToken, src: Immediate<'s>, target: Immediate<'s> },

	// Multiply
	Mul { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	MulH { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	MulHU { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	MulHSU { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },

	// Divide
	Div { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	DivU { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },

	// Remainder
	Rem { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
	RemU { dest: RegisterToken, src1: RegisterToken, src2: RegisterToken },
}

#[derive(Clone, Debug)]
pub(crate) struct Address<'s> {
	pub(crate) base:   RegisterToken,
	pub(crate) offset: Option<AddrOffset<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AddrOffset<'s> {
	pub(crate) op:  OffsetOperator,
	pub(crate) imm: Immediate<'s>,
}

#[derive(Clone, Debug)]
pub(crate) enum OffsetOperator {
	Plus,
	Minus,
}

bitflags! {
	pub(crate) struct OrderingTarget: u8 {
		const I = 0b0000_0001;
		const O = 0b0000_0010;
		const R = 0b0000_0100;
		const W = 0b0000_1000;
	}
}
