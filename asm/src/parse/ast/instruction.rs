//! AST instruction type definitions

use bitflags::bitflags;

use super::Immediate;
use crate::lex::{OpToken, RegToken};

/// An assembly instruction
///
/// Most instructions contain some form of [register](RegToken) specifier
/// and/or an [`Immediate`] <br>
/// The load and store instructions contain [`Address`] calculations <br>
/// Fence instructions use [`OrderingTarget`]s
///
/// *EBNF not given as it is too chonky, look at the docs folder for grammar*
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Instruction<'s> {
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
	// Store
	Sb { dest: Address<'s>, src: RegToken },
	Sh { dest: Address<'s>, src: RegToken },
	Sw { dest: Address<'s>, src: RegToken },

	// Memory Ordering
	Fence { pred: OrderingTarget, succ: OrderingTarget },
	FenceTso { pred: OrderingTarget, succ: OrderingTarget },

	// System Interaction
	Ecall,
	Ebreak,

	// Instruction Fetch Fencing
	Fencei,

	// CSR RegisterToken
	Csrrw { dest: RegToken, src: RegToken, target: Immediate<'s> },
	Csrrs { dest: RegToken, src: RegToken, target: Immediate<'s> },
	Csrrc { dest: RegToken, src: RegToken, target: Immediate<'s> },
	// CSR Immediate
	Csrrwi { dest: RegToken, src: Immediate<'s>, target: Immediate<'s> },
	Csrrsi { dest: RegToken, src: Immediate<'s>, target: Immediate<'s> },
	Csrrci { dest: RegToken, src: Immediate<'s>, target: Immediate<'s> },

	// Multiply
	Mul { dest: RegToken, src1: RegToken, src2: RegToken },
	Mulh { dest: RegToken, src1: RegToken, src2: RegToken },
	Mulhu { dest: RegToken, src1: RegToken, src2: RegToken },
	Mulhsu { dest: RegToken, src1: RegToken, src2: RegToken },

	// Divide
	Div { dest: RegToken, src1: RegToken, src2: RegToken },
	Divu { dest: RegToken, src1: RegToken, src2: RegToken },

	// Remainder
	Rem { dest: RegToken, src1: RegToken, src2: RegToken },
	Remu { dest: RegToken, src1: RegToken, src2: RegToken },
}

/// An address calculation for use in load/store instructions
///
/// Contains a base [register](RegToken) and an optional [offset](AddrOffset)
///
/// ```ebnf
/// address_calculation = "[", register, [ address_offset ] "]";
/// ```
#[derive(Clone, Debug)]
pub struct Address<'s> {
	/// The base register of the address
	pub base:   RegToken,
	/// The optional offset to add to the base register
	pub offset: Option<AddrOffset<'s>>,
}

/// An offset for a specific [`Address`]
///
/// Contains an [operator](OffsetOperator) and some (offset)[Immediate}]
///
/// ```ebnf
/// address_offset = "+" | "-", immediate;
/// ```
#[derive(Clone, Debug)]
pub struct AddrOffset<'s> {
	/// The operator of the offset (+ or -)
	pub op:     OffsetOperator,
	/// The actual value of the offset
	pub offset: Immediate<'s>,
}

/// The operator used in [`Address`]es with an offset
///
/// Can be either "+" or "-"
///
/// See [`AddrOffset`] for grammar
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum OffsetOperator {
	Plus,
	Minus,
}

impl From<&OpToken> for OffsetOperator {
	fn from(value: &OpToken) -> Self {
		match value {
			OpToken::Plus => Self::Plus,
			OpToken::Minus => Self::Minus,
			_ => unimplemented!(),
		}
	}
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
	pub struct OrderingTarget: u8 {
		/// An input instruction
		const I = 0b0000_0001;
		/// An output instruction
		const O = 0b0000_0010;
		/// A memory read instruction
		const R = 0b0000_0100;
		/// A memory write instruction
		const W = 0b0000_1000;
	}
}

impl From<&str> for OrderingTarget {
	fn from(value: &str) -> Self {
		let mut flags = Self::empty();

		if value.contains('I') {
			flags.set(Self::I, true);
		}
		if value.contains('O') {
			flags.set(Self::O, true);
		}
		if value.contains('R') {
			flags.set(Self::R, true);
		}
		if value.contains('W') {
			flags.set(Self::W, true);
		}

		flags
	}
}
