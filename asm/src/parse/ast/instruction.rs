//! AST instruction type definitions

use bitflags::bitflags;

use super::Immediate;
use crate::lex::RegToken;

/// An assembly instruction
///
/// Most instructions contain some form of [register](RegToken) specifier
/// and/or an [`Immediate`] </br>
/// The load and store instructions contain [`Address`] calculations </br>
/// Fence instructions use [`OrderingTarget`]s
///
/// *EBNF not given as it is too chonky, look at the docs folder for grammar*
#[derive(Clone, Debug)]
pub(crate) enum Instruction<'s> {
	// Integer Register Immediate
	Addi { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Slti { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Sltiu { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Andi { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Ori { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Xori { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Lsli { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Lsri { dest: RegToken, src: RegToken, imm: Immediate<'s> },
	Asri { dest: RegToken, src: RegToken, imm: Immediate<'s> },

	// Integer Register Register
	Add { dest: RegToken, src1: RegToken, src2: RegToken },
	Slt { dest: RegToken, src1: RegToken, src2: RegToken },
	Sltu { dest: RegToken, src1: RegToken, src2: RegToken },
	And { dest: RegToken, src1: RegToken, src2: RegToken },
	Or { dest: RegToken, src1: RegToken, src2: RegToken },
	Xor { dest: RegToken, src1: RegToken, src2: RegToken },
	Lsl { dest: RegToken, src1: RegToken, src2: RegToken },
	Lsr { dest: RegToken, src1: RegToken, src2: RegToken },
	Asr { dest: RegToken, src1: RegToken, src2: RegToken },
	Sub { dest: RegToken, src1: RegToken, src2: RegToken },

	// Upper Immediate
	Lui { dest: RegToken, imm: Immediate<'s> },
	Auipc { dest: RegToken, imm: Immediate<'s> },

	// Jump and link
	Jal { dest: RegToken, offset: Immediate<'s> },
	// Jump and link register
	Jalr { dest: RegToken, base: RegToken, offset: Immediate<'s> },

	// Conditional Branch
	Beq { src1: RegToken, src2: RegToken, offset: Immediate<'s> },
	Bne { src1: RegToken, src2: RegToken, offset: Immediate<'s> },
	Blt { src1: RegToken, src2: RegToken, offset: Immediate<'s> },
	Bltu { src1: RegToken, src2: RegToken, offset: Immediate<'s> },
	Bge { src1: RegToken, src2: RegToken, offset: Immediate<'s> },
	Bgeu { src1: RegToken, src2: RegToken, offset: Immediate<'s> },

	// Load
	Lb { dest: RegToken, addr: Address<'s> },
	Lbu { dest: RegToken, addr: Address<'s> },
	Lh { dest: RegToken, addr: Address<'s> },
	Lhu { dest: RegToken, addr: Address<'s> },
	Lw { dest: RegToken, addr: Address<'s> },
	Lwu { dest: RegToken, addr: Address<'s> },
	// Store
	Sb { dest: RegToken, addr: Address<'s> },
	Sh { dest: RegToken, addr: Address<'s> },
	Sw { dest: RegToken, addr: Address<'s> },

	// Memory Ordering
	Fence { pred: OrderingTarget, succ: OrderingTarget },
	FenceTso { pred: OrderingTarget, succ: OrderingTarget },

	// System Interaction
	ECall,
	EBreak,

	// Instruction Fetch Fencing
	FenceI,

	// CSR RegisterToken
	CsrRw { dest: RegToken, src: RegToken, target: Immediate<'s> },
	CsrRs { dest: RegToken, src: RegToken, target: Immediate<'s> },
	CsrRc { dest: RegToken, src: RegToken, target: Immediate<'s> },
	// CSR Immediate
	CsrRwi { dest: RegToken, src: Immediate<'s>, target: Immediate<'s> },
	CsrRsi { dest: RegToken, src: Immediate<'s>, target: Immediate<'s> },
	CsrRci { dest: RegToken, src: Immediate<'s>, target: Immediate<'s> },

	// Multiply
	Mul { dest: RegToken, src1: RegToken, src2: RegToken },
	MulH { dest: RegToken, src1: RegToken, src2: RegToken },
	MulHU { dest: RegToken, src1: RegToken, src2: RegToken },
	MulHSU { dest: RegToken, src1: RegToken, src2: RegToken },

	// Divide
	Div { dest: RegToken, src1: RegToken, src2: RegToken },
	DivU { dest: RegToken, src1: RegToken, src2: RegToken },

	// Remainder
	Rem { dest: RegToken, src1: RegToken, src2: RegToken },
	RemU { dest: RegToken, src1: RegToken, src2: RegToken },
}

/// An address calculation for use in load/store instructions
///
/// Contains a base [register](RegToken) and an optional [offset](AddrOffset)
///
/// ```ebnf
/// address_calculation = "[", register, [ address_offset ] "]";
/// ```
#[derive(Clone, Debug)]
pub(crate) struct Address<'s> {
	pub(crate) base:   RegToken,
	pub(crate) offset: Option<AddrOffset<'s>>,
}

/// An offset for a specific [`Address`]
///
/// Contains an [operator](OffsetOperator) and some (offset)[Immediate}]
///
/// ```ebnf
/// address_offset = "+" | "-", immediate;
/// ```
#[derive(Clone, Debug)]
pub(crate) struct AddrOffset<'s> {
	pub(crate) op:     OffsetOperator,
	pub(crate) offset: Immediate<'s>,
}

/// The operator used in [`Address`]es with an offset
///
/// Can be either "+" or "-"
///
/// See [`AddrOffset`] for grammar
#[derive(Clone, Debug)]
pub(crate) enum OffsetOperator {
	Plus,
	Minus,
}

bitflags! {
	/// A target operation for a fence-type instruction
	///
	/// These targets specify what kinds of operations the fence instruction
	/// should synchronise against
	///
	/// Can be one of
	///  - `I`: input instruction
	///  - `O`: output instruction
	///  - `R`: read instruction
	///  - `w`: write instruction
	///
	/// ```ebnf
	/// ordering_operation = [ "i" ], [ "o" ], [ "r" ], [ "w" ];
	/// ```
	pub(crate) struct OrderingTarget: u8 {
		const I = 0b0000_0001;
		const O = 0b0000_0010;
		const R = 0b0000_0100;
		const W = 0b0000_1000;
	}
}
