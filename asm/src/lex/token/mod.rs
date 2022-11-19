use std::fmt::{Debug, Display};

mod directive;
mod instruction;
mod operator;
mod register;

pub(crate) use directive::DirectiveToken;
pub(crate) use instruction::InstructionToken;
pub(crate) use operator::OperatorToken;
pub(crate) use register::RegisterToken;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum TokenType<'s> {
	Inst(InstructionToken),
	Reg(RegisterToken),
	Dir(DirectiveToken),

	// Literals
	LitStr(String),
	LitChar(char),
	LitNum(isize),

	// Labels
	Label(&'s str),
	LabelDefine(&'s str),
	LocalLabel(&'s str),
	LocalLabelDefine(&'s str),

	// Symbols
	SymComma,
	SymNewline,
	SymLeftParen,
	SymRightParen,
	SymLeftBracket,
	SymRightBracket,

	Op(OperatorToken),

	// Comments
	Comment(&'s str),
}

impl<'s> Debug for TokenType<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Inst(InstructionToken::Add) => write!(f, "{:<20} | {:<16}", "KEYWORD", "add"),
			Self::Inst(InstructionToken::Addi) => write!(f, "{:<20} | {:<16}", "KEYWORD", "addi"),
			Self::Inst(InstructionToken::Sub) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sub"),
			Self::Inst(InstructionToken::And) => write!(f, "{:<20} | {:<16}", "KEYWORD", "and"),
			Self::Inst(InstructionToken::Andi) => write!(f, "{:<20} | {:<16}", "KEYWORD", "andi"),
			Self::Inst(InstructionToken::Or) => write!(f, "{:<20} | {:<16}", "KEYWORD", "or"),
			Self::Inst(InstructionToken::Ori) => write!(f, "{:<20} | {:<16}", "KEYWORD", "ori"),
			Self::Inst(InstructionToken::Xor) => write!(f, "{:<20} | {:<16}", "KEYWORD", "xor"),
			Self::Inst(InstructionToken::Xori) => write!(f, "{:<20} | {:<16}", "KEYWORD", "xori"),
			Self::Inst(InstructionToken::Lsl) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsl"),
			Self::Inst(InstructionToken::Lsli) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsli"),
			Self::Inst(InstructionToken::Lsr) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsr"),
			Self::Inst(InstructionToken::Lsri) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsri"),
			Self::Inst(InstructionToken::Asr) => write!(f, "{:<20} | {:<16}", "KEYWORD", "asr"),
			Self::Inst(InstructionToken::Asri) => write!(f, "{:<20} | {:<16}", "KEYWORD", "asri"),
			Self::Inst(InstructionToken::Slt) => write!(f, "{:<20} | {:<16}", "KEYWORD", "slt"),
			Self::Inst(InstructionToken::Slti) => write!(f, "{:<20} | {:<16}", "KEYWORD", "slti"),
			Self::Inst(InstructionToken::Sltu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sltu"),
			Self::Inst(InstructionToken::Sltiu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sltiu"),
			Self::Inst(InstructionToken::Lw) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lw"),
			Self::Inst(InstructionToken::Lh) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lh"),
			Self::Inst(InstructionToken::Lhu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lhu"),
			Self::Inst(InstructionToken::Lb) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lb"),
			Self::Inst(InstructionToken::Lbu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lbu"),
			Self::Inst(InstructionToken::Sw) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sw"),
			Self::Inst(InstructionToken::Sh) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sh"),
			Self::Inst(InstructionToken::Sb) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sb"),
			Self::Inst(InstructionToken::Lui) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lui"),
			Self::Inst(InstructionToken::Auipc) => write!(f, "{:<20} | {:<16}", "KEYWORD", "auipc"),
			Self::Inst(InstructionToken::Beq) => write!(f, "{:<20} | {:<16}", "KEYWORD", "beq"),
			Self::Inst(InstructionToken::Bne) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bne"),
			Self::Inst(InstructionToken::Blt) => write!(f, "{:<20} | {:<16}", "KEYWORD", "blt"),
			Self::Inst(InstructionToken::Bltu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bltu"),
			Self::Inst(InstructionToken::Bge) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bge"),
			Self::Inst(InstructionToken::Bgeu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bgeu"),
			Self::Inst(InstructionToken::Jal) => write!(f, "{:<20} | {:<16}", "KEYWORD", "jal"),
			Self::Inst(InstructionToken::Jalr) => write!(f, "{:<20} | {:<16}", "KEYWORD", "jalr"),
			Self::Inst(InstructionToken::Ecall) => write!(f, "{:<20} | {:<16}", "KEYWORD", "ecall"),
			Self::Inst(InstructionToken::Ebreak) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "ebreak")
			},
			Self::Inst(InstructionToken::Fence) => write!(f, "{:<20} | {:<16}", "KEYWORD", "fence"),
			Self::Inst(InstructionToken::FenceTso) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "fence.tso")
			},
			Self::Inst(InstructionToken::Fencei) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "fence.i")
			},
			Self::Inst(InstructionToken::Csrrw) => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrw"),
			Self::Inst(InstructionToken::Csrrwi) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrwi")
			},
			Self::Inst(InstructionToken::Csrrs) => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrs"),
			Self::Inst(InstructionToken::Csrrsi) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrsi")
			},
			Self::Inst(InstructionToken::Csrrc) => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrc"),
			Self::Inst(InstructionToken::Csrrci) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrci")
			},
			Self::Inst(InstructionToken::Mul) => write!(f, "{:<20} | {:<16}", "KEYWORD", "mul"),
			Self::Inst(InstructionToken::Mulh) => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulh"),
			Self::Inst(InstructionToken::Mulhu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulhu"),
			Self::Inst(InstructionToken::Mulhsu) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "mulhsu")
			},
			Self::Inst(InstructionToken::Div) => write!(f, "{:<20} | {:<16}", "KEYWORD", "div"),
			Self::Inst(InstructionToken::Divu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "divu"),
			Self::Inst(InstructionToken::Rem) => write!(f, "{:<20} | {:<16}", "KEYWORD", "rem"),
			Self::Inst(InstructionToken::Remu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "remu"),

			Self::Reg(RegisterToken::R0) => write!(f, "{:<20} | {:<16}", "REGISTER", "r0"),
			Self::Reg(RegisterToken::R1) => write!(f, "{:<20} | {:<16}", "REGISTER", "r1"),
			Self::Reg(RegisterToken::R2) => write!(f, "{:<20} | {:<16}", "REGISTER", "r2"),
			Self::Reg(RegisterToken::R3) => write!(f, "{:<20} | {:<16}", "REGISTER", "r3"),
			Self::Reg(RegisterToken::R4) => write!(f, "{:<20} | {:<16}", "REGISTER", "r4"),
			Self::Reg(RegisterToken::R5) => write!(f, "{:<20} | {:<16}", "REGISTER", "r5"),
			Self::Reg(RegisterToken::R6) => write!(f, "{:<20} | {:<16}", "REGISTER", "r6"),
			Self::Reg(RegisterToken::R7) => write!(f, "{:<20} | {:<16}", "REGISTER", "r7"),
			Self::Reg(RegisterToken::R8) => write!(f, "{:<20} | {:<16}", "REGISTER", "r8"),
			Self::Reg(RegisterToken::R9) => write!(f, "{:<20} | {:<16}", "REGISTER", "r9"),
			Self::Reg(RegisterToken::R10) => write!(f, "{:<20} | {:<16}", "REGISTER", "r10"),
			Self::Reg(RegisterToken::R11) => write!(f, "{:<20} | {:<16}", "REGISTER", "r11"),
			Self::Reg(RegisterToken::R12) => write!(f, "{:<20} | {:<16}", "REGISTER", "r12"),
			Self::Reg(RegisterToken::R13) => write!(f, "{:<20} | {:<16}", "REGISTER", "r13"),
			Self::Reg(RegisterToken::R14) => write!(f, "{:<20} | {:<16}", "REGISTER", "r14"),
			Self::Reg(RegisterToken::R15) => write!(f, "{:<20} | {:<16}", "REGISTER", "r15"),
			Self::Reg(RegisterToken::R16) => write!(f, "{:<20} | {:<16}", "REGISTER", "r16"),
			Self::Reg(RegisterToken::R17) => write!(f, "{:<20} | {:<16}", "REGISTER", "r17"),
			Self::Reg(RegisterToken::R18) => write!(f, "{:<20} | {:<16}", "REGISTER", "r18"),
			Self::Reg(RegisterToken::R19) => write!(f, "{:<20} | {:<16}", "REGISTER", "r19"),
			Self::Reg(RegisterToken::R20) => write!(f, "{:<20} | {:<16}", "REGISTER", "r20"),
			Self::Reg(RegisterToken::R21) => write!(f, "{:<20} | {:<16}", "REGISTER", "r21"),
			Self::Reg(RegisterToken::R22) => write!(f, "{:<20} | {:<16}", "REGISTER", "r22"),
			Self::Reg(RegisterToken::R23) => write!(f, "{:<20} | {:<16}", "REGISTER", "r23"),
			Self::Reg(RegisterToken::R24) => write!(f, "{:<20} | {:<16}", "REGISTER", "r24"),
			Self::Reg(RegisterToken::R25) => write!(f, "{:<20} | {:<16}", "REGISTER", "r25"),
			Self::Reg(RegisterToken::R26) => write!(f, "{:<20} | {:<16}", "REGISTER", "r26"),
			Self::Reg(RegisterToken::R27) => write!(f, "{:<20} | {:<16}", "REGISTER", "r27"),
			Self::Reg(RegisterToken::R28) => write!(f, "{:<20} | {:<16}", "REGISTER", "r28"),
			Self::Reg(RegisterToken::R29) => write!(f, "{:<20} | {:<16}", "REGISTER", "r29"),
			Self::Reg(RegisterToken::R30) => write!(f, "{:<20} | {:<16}", "REGISTER", "r30"),
			Self::Reg(RegisterToken::R31) => write!(f, "{:<20} | {:<16}", "REGISTER", "r31"),

			Self::Dir(DirectiveToken::Byte) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$byte"),
			Self::Dir(DirectiveToken::Half) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$half"),
			Self::Dir(DirectiveToken::Word) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$word"),
			Self::Dir(DirectiveToken::Repeat) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$repeat")
			},
			Self::Dir(DirectiveToken::Equ) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$equ"),

			Self::LitStr(s) => write!(f, "{:<20} | {:<16}", "STRING", format!("{:?}", s)),
			Self::LitChar(c) => write!(f, "{:<20} | {:<16}", "CHAR", format!("{:?}", c)),
			Self::LitNum(n) => write!(f, "{:<20} | {:<16}", "NUM", n),

			Self::Label(l) => write!(f, "{:<20} | {:<16}", "LABEL", l),
			Self::LabelDefine(l) => write!(f, "{:<20} | {:<16}", "LABEL_DEFINE", l),
			Self::LocalLabel(ll) => write!(f, "{:<20} | {:<16}", "LOCAL_LABEL", ll),
			Self::LocalLabelDefine(ll) => write!(f, "{:<20} | {:<16}", "LOCAL_LABEL_DEFINE", ll),

			Self::SymComma => write!(f, "{:<20} | {:<16}", "SYMBOL", ","),
			Self::SymNewline => write!(f, "{:<20} | {:<16}", "SYMBOL", "\\n"),
			Self::SymLeftParen => write!(f, "{:<20} | {:<16}", "SYMBOL", "("),
			Self::SymRightParen => write!(f, "{:<20} | {:<16}", "SYMBOL", ")"),
			Self::SymLeftBracket => write!(f, "{:<20} | {:<16}", "SYMBOL", "["),
			Self::SymRightBracket => write!(f, "{:<20} | {:<16}", "SYMBOL", "]"),

			Self::Op(OperatorToken::TernStart) => write!(f, "{:<20} | {:16}", "OPERATOR", "?"),
			Self::Op(OperatorToken::TernAlt) => write!(f, "{:<20} | {:16}", "OPERATOR", ":"),
			Self::Op(OperatorToken::LogicOr) => write!(f, "{:<20} | {:16}", "OPERATOR", "||"),
			Self::Op(OperatorToken::LogicXor) => write!(f, "{:<20} | {:16}", "OPERATOR", "^^"),
			Self::Op(OperatorToken::LogicAnd) => write!(f, "{:<20} | {:16}", "OPERATOR", "&&"),
			Self::Op(OperatorToken::Or) => write!(f, "{:<20} | {:<16}", "OPERATOR", "|"),
			Self::Op(OperatorToken::Xor) => write!(f, "{:<20} | {:<16}", "OPERATOR", "^"),
			Self::Op(OperatorToken::And) => write!(f, "{:<20} | {:<16}", "OPERATOR", "&"),
			Self::Op(OperatorToken::Eq) => write!(f, "{:<20} | {:<16}", "OPERATOR", "=="),
			Self::Op(OperatorToken::Neq) => write!(f, "{:<20} | {:<16}", "OPERATOR", "!="),
			Self::Op(OperatorToken::Lt) => write!(f, "{:<20} | {:<16}", "OPERATOR", "<"),
			Self::Op(OperatorToken::Lte) => write!(f, "{:<20} | {:<16}", "OPERATOR", "<="),
			Self::Op(OperatorToken::Gt) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">"),
			Self::Op(OperatorToken::Gte) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">="),
			Self::Op(OperatorToken::Lsl) => write!(f, "{:<20} | {:<16}", "OPERATOR", "<<"),
			Self::Op(OperatorToken::Lsr) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">>"),
			Self::Op(OperatorToken::Asr) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">>>"),
			Self::Op(OperatorToken::Plus) => write!(f, "{:<20} | {:<16}", "OPERATOR", "+"),
			Self::Op(OperatorToken::Minus) => write!(f, "{:<20} | {:<16}", "OPERATOR", "-"),
			Self::Op(OperatorToken::Mul) => write!(f, "{:<20} | {:<16}", "OPERATOR", "*"),
			Self::Op(OperatorToken::Div) => write!(f, "{:<20} | {:<16}", "OPERATOR", "/"),
			Self::Op(OperatorToken::Rem) => write!(f, "{:<20} | {:<16}", "OPERATOR", "%"),

			Self::Comment(cmt) => write!(f, "{:<20} | {:<16}", "COMMENT", format!("{:?}", cmt)),
		}
	}
}

impl<'s> Display for TokenType<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Inst(inst) => write!(f, "{}", inst),

			Self::Reg(r) => write!(f, "{}", r),

			Self::Dir(d) => write!(f, "{}", d),

			Self::LitStr(s) => write!(f, "{:?}", s),
			Self::LitChar(c) => write!(f, "{:?}", c),
			Self::LitNum(n) => write!(f, "{}", n),

			Self::Label(l) => write!(f, "{:?}", l),
			Self::LabelDefine(l) => write!(f, "{:?}", l),
			Self::LocalLabel(ll) => write!(f, "{:?}", ll),
			Self::LocalLabelDefine(ll) => write!(f, "{:?}", ll),

			Self::SymComma => write!(f, "COMMA"),
			Self::SymNewline => write!(f, "NEWLINE"),
			Self::SymLeftParen => write!(f, "("),
			Self::SymRightParen => write!(f, ")"),
			Self::SymLeftBracket => write!(f, "["),
			Self::SymRightBracket => write!(f, "]"),

			Self::Op(o) => write!(f, "{}", o),

			Self::Comment(cmt) => write!(f, "COMMENT ({:?})", cmt),
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

		let t = format!("{:?}", self.t);
		let annotated_src = format!("{}```{}```{}", left_arm, center, right_arm);

		write!(f, "[{:0>3}:{:0>3}]: {:<32} | {:?}", self.line, self.col, t, annotated_src)
	}
}
