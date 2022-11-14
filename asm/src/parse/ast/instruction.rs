//! AST instruction type definitions

use tinyvec::ArrayVec;

use super::{Immediate, Register};

#[derive(Clone, Debug)]
pub(crate) enum Instruction<'s> {
	// Integer Register Immediate
	Addi { dest: Register, src: Register, imm: Immediate<'s> },
	Slti { dest: Register, src: Register, imm: Immediate<'s> },
	Sltiu { dest: Register, src: Register, imm: Immediate<'s> },
	Andi { dest: Register, src: Register, imm: Immediate<'s> },
	Ori { dest: Register, src: Register, imm: Immediate<'s> },
	Xori { dest: Register, src: Register, imm: Immediate<'s> },
	Lsli { dest: Register, src: Register, imm: Immediate<'s> },
	Lsri { dest: Register, src: Register, imm: Immediate<'s> },
	Asri { dest: Register, src: Register, imm: Immediate<'s> },

	// Integer Register Register
	Add { dest: Register, src1: Register, src2: Register },
	Slt { dest: Register, src1: Register, src2: Register },
	Sltu { dest: Register, src1: Register, src2: Register },
	And { dest: Register, src1: Register, src2: Register },
	Or { dest: Register, src1: Register, src2: Register },
	Xor { dest: Register, src1: Register, src2: Register },
	Lsl { dest: Register, src1: Register, src2: Register },
	Lsr { dest: Register, src1: Register, src2: Register },
	Asr { dest: Register, src1: Register, src2: Register },
	Sub { dest: Register, src1: Register, src2: Register },

	// Upper Immediate
	Lui { dest: Register, imm: Immediate<'s> },
	Auipc { dest: Register, imm: Immediate<'s> },

	// Jump
	Jmp { dest: Register, offset: Immediate<'s> },
	// Jump Register
	JmpReg { dest: Register, base: Register, offset: Immediate<'s> },

	// Conditional Branch
	Beq { src1: Register, src2: Register, offset: Immediate<'s> },
	Bne { src1: Register, src2: Register, offset: Immediate<'s> },
	Blt { src1: Register, src2: Register, offset: Immediate<'s> },
	Bltu { src1: Register, src2: Register, offset: Immediate<'s> },
	Bge { src1: Register, src2: Register, offset: Immediate<'s> },
	Bgeu { src1: Register, src2: Register, offset: Immediate<'s> },

	// Load
	Lb { dest: Register, addr: Address<'s> },
	Lbu { dest: Register, addr: Address<'s> },
	Lh { dest: Register, addr: Address<'s> },
	Lhu { dest: Register, addr: Address<'s> },
	Lw { dest: Register, addr: Address<'s> },
	Lwu { dest: Register, addr: Address<'s> },
	// Store
	Sb { dest: Register, addr: Address<'s> },
	Sh { dest: Register, addr: Address<'s> },
	Sw { dest: Register, addr: Address<'s> },

	// Memory Ordering
	Fence { pred: ArrayVec<[OrderingTarget; 4]>, succ: ArrayVec<[OrderingTarget; 4]> },
	FenceTso { pred: ArrayVec<[OrderingTarget; 4]>, succ: ArrayVec<[OrderingTarget; 4]> },

	// System Interaction
	ECall,
	EBreak,

	// Instruction Fetch Fencing
	FenceI,

	// CSR Register
	CsrRw { dest: Register, src: Register, target: Immediate<'s> },
	CsrRs { dest: Register, src: Register, target: Immediate<'s> },
	CsrRc { dest: Register, src: Register, target: Immediate<'s> },
	// CSR Immediate
	CsrRwi { dest: Register, src: Immediate<'s>, target: Immediate<'s> },
	CsrRsi { dest: Register, src: Immediate<'s>, target: Immediate<'s> },
	CsrRci { dest: Register, src: Immediate<'s>, target: Immediate<'s> },

	// Multiply
	Mul { dest: Register, src1: Register, src2: Register },
	MulH { dest: Register, src1: Register, src2: Register },
	MulHU { dest: Register, src1: Register, src2: Register },
	MulHSU { dest: Register, src1: Register, src2: Register },

	// Divide
	Div { dest: Register, src1: Register, src2: Register },
	DivU { dest: Register, src1: Register, src2: Register },

	// Remainder
	Rem { dest: Register, src1: Register, src2: Register },
	RemU { dest: Register, src1: Register, src2: Register },
}

#[derive(Clone, Debug)]
pub(crate) struct Address<'s> {
	base:   Register,
	offset: Option<AddrOffset<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AddrOffset<'s> {
	op:  OffsetOperator,
	imm: Immediate<'s>,
}

#[derive(Clone, Debug)]
pub(crate) enum OffsetOperator {
	Plus,
	Minus,
}

#[derive(Clone, Debug)]
pub(crate) enum OrderingTarget {
	I,
	O,
	R,
	W,
}

impl Default for OrderingTarget {
	fn default() -> Self { Self::I }
}
