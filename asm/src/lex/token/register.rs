//! Register Tokens

use std::fmt::{Display, Formatter, Result};

/// A tokentype to identify registers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum RegToken {
	R0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
	R16,
	R17,
	R18,
	R19,
	R20,
	R21,
	R22,
	R23,
	R24,
	R25,
	R26,
	R27,
	R28,
	R29,
	R30,
	R31,
}

impl Display for RegToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::R0 => write!(f, "r0"),
			Self::R1 => write!(f, "r1"),
			Self::R2 => write!(f, "r2"),
			Self::R3 => write!(f, "r3"),
			Self::R4 => write!(f, "r4"),
			Self::R5 => write!(f, "r5"),
			Self::R6 => write!(f, "r6"),
			Self::R7 => write!(f, "r7"),
			Self::R8 => write!(f, "r8"),
			Self::R9 => write!(f, "r9"),
			Self::R10 => write!(f, "r10"),
			Self::R11 => write!(f, "r11"),
			Self::R12 => write!(f, "r12"),
			Self::R13 => write!(f, "r13"),
			Self::R14 => write!(f, "r14"),
			Self::R15 => write!(f, "r15"),
			Self::R16 => write!(f, "r16"),
			Self::R17 => write!(f, "r17"),
			Self::R18 => write!(f, "r18"),
			Self::R19 => write!(f, "r19"),
			Self::R20 => write!(f, "r20"),
			Self::R21 => write!(f, "r21"),
			Self::R22 => write!(f, "r22"),
			Self::R23 => write!(f, "r23"),
			Self::R24 => write!(f, "r24"),
			Self::R25 => write!(f, "r25"),
			Self::R26 => write!(f, "r26"),
			Self::R27 => write!(f, "r27"),
			Self::R28 => write!(f, "r28"),
			Self::R29 => write!(f, "r29"),
			Self::R30 => write!(f, "r30"),
			Self::R31 => write!(f, "r31"),
		}
	}
}
