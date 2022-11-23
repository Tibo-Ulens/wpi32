//! Instruction Tokens

#![allow(missing_docs)]

use std::fmt::{Display, Formatter, Result};

/// A tokentype to identify instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstToken {
	Rri(RriInstruction),
	Rrr(RrrInstruction),
	Branch(BranchInstruction),
	Load(LoadInstruction),
	Store(StoreInstruction),
	Mdr(MdrInstruction),
	Csr(CsrInstruction),
	Csri(CsriInstruction),
	Lui,
	Auipc,
	Jal,
	Jalr,
	Ecall,
	Ebreak,
	Fence,
	FenceTso,
	Fencei,
}

/// Instructions taking 2 registers and an immediate as arguments
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RriInstruction {
	Addi,
	Andi,
	Ori,
	Xori,
	Lsli,
	Lsri,
	Asri,
	Slti,
	Sltiu,
}

/// Instructions taking 3 registers as arguments
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RrrInstruction {
	Add,
	Sub,
	And,
	Or,
	Xor,
	Lsl,
	Lsr,
	Asr,
	Slt,
	Sltu,
}

/// Branch instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchInstruction {
	Beq,
	Bne,
	Blt,
	Bltu,
	Bge,
	Bgeu,
}

/// Memory load instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadInstruction {
	Lw,
	Lh,
	Lhu,
	Lb,
	Lbu,
}

/// Memory store instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreInstruction {
	Sw,
	Sh,
	Sb,
}

/// CSR instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsrInstruction {
	Csrrw,
	Csrrs,
	Csrrc,
}

/// CSR immediate instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsriInstruction {
	Csrrwi,
	Csrrsi,
	Csrrci,
}

/// Multiply, divide, or remainder instructions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MdrInstruction {
	Mul,
	Mulh,
	Mulhu,
	Mulhsu,
	Div,
	Divu,
	Rem,
	Remu,
}

impl Display for InstToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Rri(RriInstruction::Addi) => write!(f, "addi"),
			Self::Rri(RriInstruction::Andi) => write!(f, "andi"),
			Self::Rri(RriInstruction::Ori) => write!(f, "ori"),
			Self::Rri(RriInstruction::Xori) => write!(f, "xori"),
			Self::Rri(RriInstruction::Lsli) => write!(f, "lsli"),
			Self::Rri(RriInstruction::Lsri) => write!(f, "lsri"),
			Self::Rri(RriInstruction::Asri) => write!(f, "asri"),
			Self::Rri(RriInstruction::Slti) => write!(f, "slti"),
			Self::Rri(RriInstruction::Sltiu) => write!(f, "sltiu"),
			Self::Rrr(RrrInstruction::Add) => write!(f, "add"),
			Self::Rrr(RrrInstruction::Sub) => write!(f, "sub"),
			Self::Rrr(RrrInstruction::And) => write!(f, "and"),
			Self::Rrr(RrrInstruction::Or) => write!(f, "or"),
			Self::Rrr(RrrInstruction::Xor) => write!(f, "xor"),
			Self::Rrr(RrrInstruction::Lsl) => write!(f, "lsl"),
			Self::Rrr(RrrInstruction::Lsr) => write!(f, "lsr"),
			Self::Rrr(RrrInstruction::Asr) => write!(f, "asr"),
			Self::Rrr(RrrInstruction::Slt) => write!(f, "slt"),
			Self::Rrr(RrrInstruction::Sltu) => write!(f, "sltu"),
			Self::Load(LoadInstruction::Lw) => write!(f, "lw"),
			Self::Load(LoadInstruction::Lh) => write!(f, "lh"),
			Self::Load(LoadInstruction::Lhu) => write!(f, "lhu"),
			Self::Load(LoadInstruction::Lb) => write!(f, "lb"),
			Self::Load(LoadInstruction::Lbu) => write!(f, "lbu"),
			Self::Store(StoreInstruction::Sw) => write!(f, "sw"),
			Self::Store(StoreInstruction::Sh) => write!(f, "sh"),
			Self::Store(StoreInstruction::Sb) => write!(f, "sb"),
			Self::Lui => write!(f, "lui"),
			Self::Auipc => write!(f, "auipc"),
			Self::Branch(BranchInstruction::Beq) => write!(f, "beq"),
			Self::Branch(BranchInstruction::Bne) => write!(f, "bne"),
			Self::Branch(BranchInstruction::Blt) => write!(f, "blt"),
			Self::Branch(BranchInstruction::Bltu) => write!(f, "bltu"),
			Self::Branch(BranchInstruction::Bge) => write!(f, "bge"),
			Self::Branch(BranchInstruction::Bgeu) => write!(f, "bgeu"),
			Self::Jal => write!(f, "jal"),
			Self::Jalr => write!(f, "jalr"),
			Self::Ecall => write!(f, "ecall"),
			Self::Ebreak => write!(f, "ebreak"),
			Self::Fence => write!(f, "fence"),
			Self::FenceTso => write!(f, "fence.tso"),
			Self::Fencei => write!(f, "fence.i"),
			Self::Csr(CsrInstruction::Csrrw) => write!(f, "csrrw"),
			Self::Csr(CsrInstruction::Csrrs) => write!(f, "csrrs"),
			Self::Csr(CsrInstruction::Csrrc) => write!(f, "csrrc"),
			Self::Csri(CsriInstruction::Csrrwi) => write!(f, "csrrwi"),
			Self::Csri(CsriInstruction::Csrrsi) => write!(f, "csrrsi"),
			Self::Csri(CsriInstruction::Csrrci) => write!(f, "csrrci"),
			Self::Mdr(MdrInstruction::Mul) => write!(f, "mul"),
			Self::Mdr(MdrInstruction::Mulh) => write!(f, "mulh"),
			Self::Mdr(MdrInstruction::Mulhu) => write!(f, "mulhu"),
			Self::Mdr(MdrInstruction::Mulhsu) => write!(f, "mulhsu"),
			Self::Mdr(MdrInstruction::Div) => write!(f, "div"),
			Self::Mdr(MdrInstruction::Divu) => write!(f, "divu"),
			Self::Mdr(MdrInstruction::Rem) => write!(f, "rem"),
			Self::Mdr(MdrInstruction::Remu) => write!(f, "remu"),
		}
	}
}
