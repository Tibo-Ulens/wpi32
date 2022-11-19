use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InstToken {
	Add,
	Addi,
	Sub,
	And,
	Andi,
	Or,
	Ori,
	Xor,
	Xori,
	Lsl,
	Lsli,
	Lsr,
	Lsri,
	Asr,
	Asri,
	Slt,
	Slti,
	Sltu,
	Sltiu,
	Lw,
	Lh,
	Lhu,
	Lb,
	Lbu,
	Sw,
	Sh,
	Sb,
	Lui,
	Auipc,
	Beq,
	Bne,
	Blt,
	Bltu,
	Bge,
	Bgeu,
	Jal,
	Jalr,
	Ecall,
	Ebreak,
	Fence,
	FenceTso,
	Fencei,
	Csrrw,
	Csrrwi,
	Csrrs,
	Csrrsi,
	Csrrc,
	Csrrci,
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
			Self::Add => write!(f, "add"),
			Self::Addi => write!(f, "addi"),
			Self::Sub => write!(f, "sub"),
			Self::And => write!(f, "and"),
			Self::Andi => write!(f, "andi"),
			Self::Or => write!(f, "or"),
			Self::Ori => write!(f, "ori"),
			Self::Xor => write!(f, "xor"),
			Self::Xori => write!(f, "xori"),
			Self::Lsl => write!(f, "lsl"),
			Self::Lsli => write!(f, "lsli"),
			Self::Lsr => write!(f, "lsr"),
			Self::Lsri => write!(f, "lsri"),
			Self::Asr => write!(f, "asr"),
			Self::Asri => write!(f, "asri"),
			Self::Slt => write!(f, "slt"),
			Self::Slti => write!(f, "slti"),
			Self::Sltu => write!(f, "sltu"),
			Self::Sltiu => write!(f, "sltiu"),
			Self::Lw => write!(f, "lw"),
			Self::Lh => write!(f, "lh"),
			Self::Lhu => write!(f, "lhu"),
			Self::Lb => write!(f, "lb"),
			Self::Lbu => write!(f, "lbu"),
			Self::Sw => write!(f, "sw"),
			Self::Sh => write!(f, "sh"),
			Self::Sb => write!(f, "sb"),
			Self::Lui => write!(f, "lui"),
			Self::Auipc => write!(f, "auipc"),
			Self::Beq => write!(f, "beq"),
			Self::Bne => write!(f, "bne"),
			Self::Blt => write!(f, "blt"),
			Self::Bltu => write!(f, "bltu"),
			Self::Bge => write!(f, "bge"),
			Self::Bgeu => write!(f, "bgeu"),
			Self::Jal => write!(f, "jal"),
			Self::Jalr => write!(f, "jalr"),
			Self::Ecall => write!(f, "ecall"),
			Self::Ebreak => write!(f, "ebreak"),
			Self::Fence => write!(f, "fence"),
			Self::FenceTso => write!(f, "fence.tso"),
			Self::Fencei => write!(f, "fence.i"),
			Self::Csrrw => write!(f, "csrrw"),
			Self::Csrrwi => write!(f, "csrrwi"),
			Self::Csrrs => write!(f, "csrrs"),
			Self::Csrrsi => write!(f, "csrrsi"),
			Self::Csrrc => write!(f, "csrrc"),
			Self::Csrrci => write!(f, "csrrci"),
			Self::Mul => write!(f, "mul"),
			Self::Mulh => write!(f, "mulh"),
			Self::Mulhu => write!(f, "mulhu"),
			Self::Mulhsu => write!(f, "mulhsu"),
			Self::Div => write!(f, "div"),
			Self::Divu => write!(f, "divu"),
			Self::Rem => write!(f, "rem"),
			Self::Remu => write!(f, "remu"),
		}
	}
}
