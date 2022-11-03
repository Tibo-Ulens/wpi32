use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TokenType<'s> {
	// Keywords
	// RV32I
	KwAdd,
	KwAddi,
	KwSub,
	KwAnd,
	KwAndi,
	KwOr,
	KwOri,
	KwXor,
	KwXori,
	KwLsl,
	KwLsli,
	KwLsr,
	KwLsri,
	KwAsr,
	KwAsri,
	KwSlt,
	KwSlti,
	KwSltu,
	KwSltiu,
	KwLw,
	KwLh,
	KwLhu,
	KwLb,
	KwLbu,
	KwSw,
	KwSh,
	KwSb,
	KwLui,
	KwAuipc,
	KwBeq,
	KwBne,
	KwBlt,
	KwBltu,
	KwBge,
	KwBgeu,
	KwJal,
	KwJalr,
	KwEcall,
	KwEbreak,
	KwFence,

	// Zifencei
	KwFencei,

	// Zicsr
	KwCsrrw,
	KwCsrrwi,
	KwCsrrs,
	KwCsrrsi,
	KwCsrrc,
	KwCsrrci,

	// M
	KwMul,
	KwMulh,
	KwMulhu,
	KwMulhsu,
	KwDiv,
	KwDivu,
	KwRem,
	KwRemu,

	// Registers
	RegR0,
	RegR1,
	RegR2,
	RegR3,
	RegR4,
	RegR5,
	RegR6,
	RegR7,
	RegR8,
	RegR9,
	RegR10,
	RegR11,
	RegR12,
	RegR13,
	RegR14,
	RegR15,
	RegR16,
	RegR17,
	RegR18,
	RegR19,
	RegR20,
	RegR21,
	RegR22,
	RegR23,
	RegR24,
	RegR25,
	RegR26,
	RegR27,
	RegR28,
	RegR29,
	RegR30,
	RegR31,

	// Directives
	DirByte,
	DirHalf,
	DirWord,
	DirRepeat,
	DirEqu,

	// Literals
	LitStr(String),
	LitChar(char),
	LitNum(u32),

	// Labels
	Label(&'s str),
	LabelDefine(&'s str),
	LocalLabel(&'s str),
	LocalLabelDefine(&'s str),

	// Symbols
	SymComma,
	SymLeftParen,
	SymRightParen,
	SymLeftBracket,
	SymRightBracket,

	// Operators
	OperatorOr,
	OperatorXor,
	OperatorAnd,
	OperatorEq,
	OperatorNeq,
	OperatorLt,
	OperatorLte,
	OperatorGt,
	OperatorGte,
	OperatorLsl,
	OperatorLsr,
	OperatorAsr,
	OperatorPlus,
	OperatorMinus,
	OperatorMul,
	OperatorDiv,
	OperatorRem,

	// Comments
	Comment(&'s str),
}

impl<'s> Display for TokenType<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::KwAdd => write!(f, "{:<20} | {:<16}", "KEYWORD", "add"),
			Self::KwAddi => write!(f, "{:<20} | {:<16}", "KEYWORD", "addi"),
			Self::KwSub => write!(f, "{:<20} | {:<16}", "KEYWORD", "sub"),
			Self::KwAnd => write!(f, "{:<20} | {:<16}", "KEYWORD", "and"),
			Self::KwAndi => write!(f, "{:<20} | {:<16}", "KEYWORD", "andi"),
			Self::KwOr => write!(f, "{:<20} | {:<16}", "KEYWORD", "or"),
			Self::KwOri => write!(f, "{:<20} | {:<16}", "KEYWORD", "ori"),
			Self::KwXor => write!(f, "{:<20} | {:<16}", "KEYWORD", "xor"),
			Self::KwXori => write!(f, "{:<20} | {:<16}", "KEYWORD", "xori"),
			Self::KwLsl => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsl"),
			Self::KwLsli => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsli"),
			Self::KwLsr => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsr"),
			Self::KwLsri => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsri"),
			Self::KwAsr => write!(f, "{:<20} | {:<16}", "KEYWORD", "asr"),
			Self::KwAsri => write!(f, "{:<20} | {:<16}", "KEYWORD", "asri"),
			Self::KwSlt => write!(f, "{:<20} | {:<16}", "KEYWORD", "slt"),
			Self::KwSlti => write!(f, "{:<20} | {:<16}", "KEYWORD", "slti"),
			Self::KwSltu => write!(f, "{:<20} | {:<16}", "KEYWORD", "sltu"),
			Self::KwSltiu => write!(f, "{:<20} | {:<16}", "KEYWORD", "sltiu"),
			Self::KwLw => write!(f, "{:<20} | {:<16}", "KEYWORD", "lw"),
			Self::KwLh => write!(f, "{:<20} | {:<16}", "KEYWORD", "lh"),
			Self::KwLhu => write!(f, "{:<20} | {:<16}", "KEYWORD", "lhu"),
			Self::KwLb => write!(f, "{:<20} | {:<16}", "KEYWORD", "lb"),
			Self::KwLbu => write!(f, "{:<20} | {:<16}", "KEYWORD", "lbu"),
			Self::KwSw => write!(f, "{:<20} | {:<16}", "KEYWORD", "sw"),
			Self::KwSh => write!(f, "{:<20} | {:<16}", "KEYWORD", "sh"),
			Self::KwSb => write!(f, "{:<20} | {:<16}", "KEYWORD", "sb"),
			Self::KwLui => write!(f, "{:<20} | {:<16}", "KEYWORD", "lui"),
			Self::KwAuipc => write!(f, "{:<20} | {:<16}", "KEYWORD", "auipc"),
			Self::KwBeq => write!(f, "{:<20} | {:<16}", "KEYWORD", "beq"),
			Self::KwBne => write!(f, "{:<20} | {:<16}", "KEYWORD", "bne"),
			Self::KwBlt => write!(f, "{:<20} | {:<16}", "KEYWORD", "blt"),
			Self::KwBltu => write!(f, "{:<20} | {:<16}", "KEYWORD", "bltu"),
			Self::KwBge => write!(f, "{:<20} | {:<16}", "KEYWORD", "bge"),
			Self::KwBgeu => write!(f, "{:<20} | {:<16}", "KEYWORD", "bgeu"),
			Self::KwJal => write!(f, "{:<20} | {:<16}", "KEYWORD", "jal"),
			Self::KwJalr => write!(f, "{:<20} | {:<16}", "KEYWORD", "jalr"),
			Self::KwEcall => write!(f, "{:<20} | {:<16}", "KEYWORD", "ecall"),
			Self::KwEbreak => write!(f, "{:<20} | {:<16}", "KEYWORD", "ebreak"),
			Self::KwFence => write!(f, "{:<20} | {:<16}", "KEYWORD", "fence"),
			Self::KwFencei => write!(f, "{:<20} | {:<16}", "KEYWORD", "fence.i"),
			Self::KwCsrrw => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrw"),
			Self::KwCsrrwi => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrwi"),
			Self::KwCsrrs => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrs"),
			Self::KwCsrrsi => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrsi"),
			Self::KwCsrrc => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrc"),
			Self::KwCsrrci => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrci"),
			Self::KwMul => write!(f, "{:<20} | {:<16}", "KEYWORD", "mul"),
			Self::KwMulh => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulh"),
			Self::KwMulhu => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulhu"),
			Self::KwMulhsu => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulhsu"),
			Self::KwDiv => write!(f, "{:<20} | {:<16}", "KEYWORD", "div"),
			Self::KwDivu => write!(f, "{:<20} | {:<16}", "KEYWORD", "divu"),
			Self::KwRem => write!(f, "{:<20} | {:<16}", "KEYWORD", "rem"),
			Self::KwRemu => write!(f, "{:<20} | {:<16}", "KEYWORD", "remu"),

			Self::RegR0 => write!(f, "{:<20} | {:<16}", "REGISTER", "r0"),
			Self::RegR1 => write!(f, "{:<20} | {:<16}", "REGISTER", "r1"),
			Self::RegR2 => write!(f, "{:<20} | {:<16}", "REGISTER", "r2"),
			Self::RegR3 => write!(f, "{:<20} | {:<16}", "REGISTER", "r3"),
			Self::RegR4 => write!(f, "{:<20} | {:<16}", "REGISTER", "r4"),
			Self::RegR5 => write!(f, "{:<20} | {:<16}", "REGISTER", "r5"),
			Self::RegR6 => write!(f, "{:<20} | {:<16}", "REGISTER", "r6"),
			Self::RegR7 => write!(f, "{:<20} | {:<16}", "REGISTER", "r7"),
			Self::RegR8 => write!(f, "{:<20} | {:<16}", "REGISTER", "r8"),
			Self::RegR9 => write!(f, "{:<20} | {:<16}", "REGISTER", "r9"),
			Self::RegR10 => write!(f, "{:<20} | {:<16}", "REGISTER", "r10"),
			Self::RegR11 => write!(f, "{:<20} | {:<16}", "REGISTER", "r11"),
			Self::RegR12 => write!(f, "{:<20} | {:<16}", "REGISTER", "r12"),
			Self::RegR13 => write!(f, "{:<20} | {:<16}", "REGISTER", "r13"),
			Self::RegR14 => write!(f, "{:<20} | {:<16}", "REGISTER", "r14"),
			Self::RegR15 => write!(f, "{:<20} | {:<16}", "REGISTER", "r15"),
			Self::RegR16 => write!(f, "{:<20} | {:<16}", "REGISTER", "r16"),
			Self::RegR17 => write!(f, "{:<20} | {:<16}", "REGISTER", "r17"),
			Self::RegR18 => write!(f, "{:<20} | {:<16}", "REGISTER", "r18"),
			Self::RegR19 => write!(f, "{:<20} | {:<16}", "REGISTER", "r19"),
			Self::RegR20 => write!(f, "{:<20} | {:<16}", "REGISTER", "r20"),
			Self::RegR21 => write!(f, "{:<20} | {:<16}", "REGISTER", "r21"),
			Self::RegR22 => write!(f, "{:<20} | {:<16}", "REGISTER", "r22"),
			Self::RegR23 => write!(f, "{:<20} | {:<16}", "REGISTER", "r23"),
			Self::RegR24 => write!(f, "{:<20} | {:<16}", "REGISTER", "r24"),
			Self::RegR25 => write!(f, "{:<20} | {:<16}", "REGISTER", "r25"),
			Self::RegR26 => write!(f, "{:<20} | {:<16}", "REGISTER", "r26"),
			Self::RegR27 => write!(f, "{:<20} | {:<16}", "REGISTER", "r27"),
			Self::RegR28 => write!(f, "{:<20} | {:<16}", "REGISTER", "r28"),
			Self::RegR29 => write!(f, "{:<20} | {:<16}", "REGISTER", "r29"),
			Self::RegR30 => write!(f, "{:<20} | {:<16}", "REGISTER", "r30"),
			Self::RegR31 => write!(f, "{:<20} | {:<16}", "REGISTER", "r31"),

			Self::DirByte => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$byte"),
			Self::DirHalf => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$half"),
			Self::DirWord => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$word"),
			Self::DirRepeat => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$repeat"),
			Self::DirEqu => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$equ"),

			Self::LitStr(s) => write!(f, "{:<20} | {:<16}", "STRING", format!("{:?}", s)),
			Self::LitChar(c) => write!(f, "{:<20} | {:<16}", "CHAR", format!("{:?}", c)),
			Self::LitNum(n) => write!(f, "{:<20} | {:<16}", "NUM", n),

			Self::Label(l) => write!(f, "{:<20} | {:<16}", "LABEL", l),
			Self::LabelDefine(l) => write!(f, "{:<20} | {:<16}", "LABEL_DEFINE", l),
			Self::LocalLabel(ll) => write!(f, "{:<20} | {:<16}", "LOCAL_LABEL", ll),
			Self::LocalLabelDefine(ll) => write!(f, "{:<20} | {:<16}", "LOCAL_LABEL_DEFINE", ll),

			Self::SymComma => write!(f, "{:<20} | {:<16}", "SYMBOL", ","),
			Self::SymLeftParen => write!(f, "{:<20} | {:<16}", "SYMBOL", "("),
			Self::SymRightParen => write!(f, "{:<20} | {:<16}", "SYMBOL", ")"),
			Self::SymLeftBracket => write!(f, "{:<20} | {:<16}", "SYMBOL", "["),
			Self::SymRightBracket => write!(f, "{:<20} | {:<16}", "SYMBOL", "]"),

			Self::OperatorOr => write!(f, "{:<20} | {:<16}", "OPERATOR", "|"),
			Self::OperatorXor => write!(f, "{:<20} | {:<16}", "OPERATOR", "^"),
			Self::OperatorAnd => write!(f, "{:<20} | {:<16}", "OPERATOR", "&"),
			Self::OperatorEq => write!(f, "{:<20} | {:<16}", "OPERATOR", "=="),
			Self::OperatorNeq => write!(f, "{:<20} | {:<16}", "OPERATOR", "!="),
			Self::OperatorLt => write!(f, "{:<20} | {:<16}", "OPERATOR", "<"),
			Self::OperatorLte => write!(f, "{:<20} | {:<16}", "OPERATOR", "<="),
			Self::OperatorGt => write!(f, "{:<20} | {:<16}", "OPERATOR", ">"),
			Self::OperatorGte => write!(f, "{:<20} | {:<16}", "OPERATOR", ">="),
			Self::OperatorLsl => write!(f, "{:<20} | {:<16}", "OPERATOR", "<<"),
			Self::OperatorLsr => write!(f, "{:<20} | {:<16}", "OPERATOR", ">>"),
			Self::OperatorAsr => write!(f, "{:<20} | {:<16}", "OPERATOR", ">>>"),
			Self::OperatorPlus => write!(f, "{:<20} | {:<16}", "OPERATOR", "+"),
			Self::OperatorMinus => write!(f, "{:<20} | {:<16}", "OPERATOR", "-"),
			Self::OperatorMul => write!(f, "{:<20} | {:<16}", "OPERATOR", "*"),
			Self::OperatorDiv => write!(f, "{:<20} | {:<16}", "OPERATOR", "/"),
			Self::OperatorRem => write!(f, "{:<20} | {:<16}", "OPERATOR", "%"),

			Self::Comment(cmt) => write!(f, "{:<20} | {:<16}", "COMMENT", format!("{:?}", cmt)),
		}
	}
}

#[derive(Clone, Debug)]
pub(crate) struct Token<'s> {
	pub(crate) t:           TokenType<'s>,
	pub(crate) line:        usize,
	pub(crate) col:         usize,
	pub(crate) span:        usize,
	pub(crate) source_line: &'s str,
}

impl<'s> Display for Token<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let start = self.col - 1; // Columns start at 1
		let left_arm = &self.source_line[..start].trim_start();
		let center = &self.source_line[start..start + self.span];
		let right_arm = &self.source_line[start + self.span..];

		let t = self.t.to_string();

		write!(
			f,
			"[{:0>3}:{:0>3}]: {:<32} | {}```{}```{}",
			self.line, self.col, t, left_arm, center, right_arm
		)
	}
}
