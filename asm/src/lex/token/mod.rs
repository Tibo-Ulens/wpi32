use std::fmt::{Debug, Display};

mod directive;
mod instruction;
mod operator;
mod register;

pub(crate) use directive::DirToken;
pub(crate) use instruction::InstToken;
pub(crate) use operator::OpToken;
pub(crate) use register::RegToken;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum TokenType<'s> {
	Inst(InstToken),
	Reg(RegToken),
	Dir(DirToken),

	// Literals
	LitStr(String),
	LitChar(char),
	LitNum(isize),

	// Sections
	Section(&'s str),

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

	Op(OpToken),

	// Comments
	Comment(&'s str),
}

impl<'s> Debug for TokenType<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Inst(InstToken::Add) => write!(f, "{:<20} | {:<16}", "KEYWORD", "add"),
			Self::Inst(InstToken::Addi) => write!(f, "{:<20} | {:<16}", "KEYWORD", "addi"),
			Self::Inst(InstToken::Sub) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sub"),
			Self::Inst(InstToken::And) => write!(f, "{:<20} | {:<16}", "KEYWORD", "and"),
			Self::Inst(InstToken::Andi) => write!(f, "{:<20} | {:<16}", "KEYWORD", "andi"),
			Self::Inst(InstToken::Or) => write!(f, "{:<20} | {:<16}", "KEYWORD", "or"),
			Self::Inst(InstToken::Ori) => write!(f, "{:<20} | {:<16}", "KEYWORD", "ori"),
			Self::Inst(InstToken::Xor) => write!(f, "{:<20} | {:<16}", "KEYWORD", "xor"),
			Self::Inst(InstToken::Xori) => write!(f, "{:<20} | {:<16}", "KEYWORD", "xori"),
			Self::Inst(InstToken::Lsl) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsl"),
			Self::Inst(InstToken::Lsli) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsli"),
			Self::Inst(InstToken::Lsr) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsr"),
			Self::Inst(InstToken::Lsri) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lsri"),
			Self::Inst(InstToken::Asr) => write!(f, "{:<20} | {:<16}", "KEYWORD", "asr"),
			Self::Inst(InstToken::Asri) => write!(f, "{:<20} | {:<16}", "KEYWORD", "asri"),
			Self::Inst(InstToken::Slt) => write!(f, "{:<20} | {:<16}", "KEYWORD", "slt"),
			Self::Inst(InstToken::Slti) => write!(f, "{:<20} | {:<16}", "KEYWORD", "slti"),
			Self::Inst(InstToken::Sltu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sltu"),
			Self::Inst(InstToken::Sltiu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sltiu"),
			Self::Inst(InstToken::Lw) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lw"),
			Self::Inst(InstToken::Lh) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lh"),
			Self::Inst(InstToken::Lhu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lhu"),
			Self::Inst(InstToken::Lb) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lb"),
			Self::Inst(InstToken::Lbu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lbu"),
			Self::Inst(InstToken::Sw) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sw"),
			Self::Inst(InstToken::Sh) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sh"),
			Self::Inst(InstToken::Sb) => write!(f, "{:<20} | {:<16}", "KEYWORD", "sb"),
			Self::Inst(InstToken::Lui) => write!(f, "{:<20} | {:<16}", "KEYWORD", "lui"),
			Self::Inst(InstToken::Auipc) => write!(f, "{:<20} | {:<16}", "KEYWORD", "auipc"),
			Self::Inst(InstToken::Beq) => write!(f, "{:<20} | {:<16}", "KEYWORD", "beq"),
			Self::Inst(InstToken::Bne) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bne"),
			Self::Inst(InstToken::Blt) => write!(f, "{:<20} | {:<16}", "KEYWORD", "blt"),
			Self::Inst(InstToken::Bltu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bltu"),
			Self::Inst(InstToken::Bge) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bge"),
			Self::Inst(InstToken::Bgeu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "bgeu"),
			Self::Inst(InstToken::Jal) => write!(f, "{:<20} | {:<16}", "KEYWORD", "jal"),
			Self::Inst(InstToken::Jalr) => write!(f, "{:<20} | {:<16}", "KEYWORD", "jalr"),
			Self::Inst(InstToken::Ecall) => write!(f, "{:<20} | {:<16}", "KEYWORD", "ecall"),
			Self::Inst(InstToken::Ebreak) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "ebreak")
			},
			Self::Inst(InstToken::Fence) => write!(f, "{:<20} | {:<16}", "KEYWORD", "fence"),
			Self::Inst(InstToken::FenceTso) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "fence.tso")
			},
			Self::Inst(InstToken::Fencei) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "fence.i")
			},
			Self::Inst(InstToken::Csrrw) => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrw"),
			Self::Inst(InstToken::Csrrwi) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrwi")
			},
			Self::Inst(InstToken::Csrrs) => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrs"),
			Self::Inst(InstToken::Csrrsi) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrsi")
			},
			Self::Inst(InstToken::Csrrc) => write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrc"),
			Self::Inst(InstToken::Csrrci) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "csrrci")
			},
			Self::Inst(InstToken::Mul) => write!(f, "{:<20} | {:<16}", "KEYWORD", "mul"),
			Self::Inst(InstToken::Mulh) => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulh"),
			Self::Inst(InstToken::Mulhu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "mulhu"),
			Self::Inst(InstToken::Mulhsu) => {
				write!(f, "{:<20} | {:<16}", "KEYWORD", "mulhsu")
			},
			Self::Inst(InstToken::Div) => write!(f, "{:<20} | {:<16}", "KEYWORD", "div"),
			Self::Inst(InstToken::Divu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "divu"),
			Self::Inst(InstToken::Rem) => write!(f, "{:<20} | {:<16}", "KEYWORD", "rem"),
			Self::Inst(InstToken::Remu) => write!(f, "{:<20} | {:<16}", "KEYWORD", "remu"),

			Self::Reg(RegToken::R0) => write!(f, "{:<20} | {:<16}", "REGISTER", "r0"),
			Self::Reg(RegToken::R1) => write!(f, "{:<20} | {:<16}", "REGISTER", "r1"),
			Self::Reg(RegToken::R2) => write!(f, "{:<20} | {:<16}", "REGISTER", "r2"),
			Self::Reg(RegToken::R3) => write!(f, "{:<20} | {:<16}", "REGISTER", "r3"),
			Self::Reg(RegToken::R4) => write!(f, "{:<20} | {:<16}", "REGISTER", "r4"),
			Self::Reg(RegToken::R5) => write!(f, "{:<20} | {:<16}", "REGISTER", "r5"),
			Self::Reg(RegToken::R6) => write!(f, "{:<20} | {:<16}", "REGISTER", "r6"),
			Self::Reg(RegToken::R7) => write!(f, "{:<20} | {:<16}", "REGISTER", "r7"),
			Self::Reg(RegToken::R8) => write!(f, "{:<20} | {:<16}", "REGISTER", "r8"),
			Self::Reg(RegToken::R9) => write!(f, "{:<20} | {:<16}", "REGISTER", "r9"),
			Self::Reg(RegToken::R10) => write!(f, "{:<20} | {:<16}", "REGISTER", "r10"),
			Self::Reg(RegToken::R11) => write!(f, "{:<20} | {:<16}", "REGISTER", "r11"),
			Self::Reg(RegToken::R12) => write!(f, "{:<20} | {:<16}", "REGISTER", "r12"),
			Self::Reg(RegToken::R13) => write!(f, "{:<20} | {:<16}", "REGISTER", "r13"),
			Self::Reg(RegToken::R14) => write!(f, "{:<20} | {:<16}", "REGISTER", "r14"),
			Self::Reg(RegToken::R15) => write!(f, "{:<20} | {:<16}", "REGISTER", "r15"),
			Self::Reg(RegToken::R16) => write!(f, "{:<20} | {:<16}", "REGISTER", "r16"),
			Self::Reg(RegToken::R17) => write!(f, "{:<20} | {:<16}", "REGISTER", "r17"),
			Self::Reg(RegToken::R18) => write!(f, "{:<20} | {:<16}", "REGISTER", "r18"),
			Self::Reg(RegToken::R19) => write!(f, "{:<20} | {:<16}", "REGISTER", "r19"),
			Self::Reg(RegToken::R20) => write!(f, "{:<20} | {:<16}", "REGISTER", "r20"),
			Self::Reg(RegToken::R21) => write!(f, "{:<20} | {:<16}", "REGISTER", "r21"),
			Self::Reg(RegToken::R22) => write!(f, "{:<20} | {:<16}", "REGISTER", "r22"),
			Self::Reg(RegToken::R23) => write!(f, "{:<20} | {:<16}", "REGISTER", "r23"),
			Self::Reg(RegToken::R24) => write!(f, "{:<20} | {:<16}", "REGISTER", "r24"),
			Self::Reg(RegToken::R25) => write!(f, "{:<20} | {:<16}", "REGISTER", "r25"),
			Self::Reg(RegToken::R26) => write!(f, "{:<20} | {:<16}", "REGISTER", "r26"),
			Self::Reg(RegToken::R27) => write!(f, "{:<20} | {:<16}", "REGISTER", "r27"),
			Self::Reg(RegToken::R28) => write!(f, "{:<20} | {:<16}", "REGISTER", "r28"),
			Self::Reg(RegToken::R29) => write!(f, "{:<20} | {:<16}", "REGISTER", "r29"),
			Self::Reg(RegToken::R30) => write!(f, "{:<20} | {:<16}", "REGISTER", "r30"),
			Self::Reg(RegToken::R31) => write!(f, "{:<20} | {:<16}", "REGISTER", "r31"),

			Self::Dir(DirToken::Section) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$SECTION")
			},
			Self::Dir(DirToken::Bytes) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$BYTES"),
			Self::Dir(DirToken::Halves) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$HALVES")
			},
			Self::Dir(DirToken::Words) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$WORDS"),
			Self::Dir(DirToken::ResBytes) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$RES_BYTES")
			},
			Self::Dir(DirToken::ResHalves) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$RES_HALVES")
			},
			Self::Dir(DirToken::ResWords) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$RES_WORDS")
			},
			Self::Dir(DirToken::Repeat) => {
				write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$repeat")
			},
			Self::Dir(DirToken::Const) => write!(f, "{:<20} | {:<16}", "DIRECTIVE", "$CONST"),

			Self::Section(s) => write!(f, "{:<20} | {:<16}", "SECTION", s),

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

			Self::Op(OpToken::TernStart) => write!(f, "{:<20} | {:16}", "OPERATOR", "?"),
			Self::Op(OpToken::TernAlt) => write!(f, "{:<20} | {:16}", "OPERATOR", ":"),
			Self::Op(OpToken::LogicOr) => write!(f, "{:<20} | {:16}", "OPERATOR", "||"),
			Self::Op(OpToken::LogicXor) => write!(f, "{:<20} | {:16}", "OPERATOR", "^^"),
			Self::Op(OpToken::LogicAnd) => write!(f, "{:<20} | {:16}", "OPERATOR", "&&"),
			Self::Op(OpToken::BitOr) => write!(f, "{:<20} | {:<16}", "OPERATOR", "|"),
			Self::Op(OpToken::BitXor) => write!(f, "{:<20} | {:<16}", "OPERATOR", "^"),
			Self::Op(OpToken::BitAnd) => write!(f, "{:<20} | {:<16}", "OPERATOR", "&"),
			Self::Op(OpToken::Eq) => write!(f, "{:<20} | {:<16}", "OPERATOR", "=="),
			Self::Op(OpToken::Neq) => write!(f, "{:<20} | {:<16}", "OPERATOR", "!="),
			Self::Op(OpToken::Lt) => write!(f, "{:<20} | {:<16}", "OPERATOR", "<"),
			Self::Op(OpToken::Lte) => write!(f, "{:<20} | {:<16}", "OPERATOR", "<="),
			Self::Op(OpToken::Gt) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">"),
			Self::Op(OpToken::Gte) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">="),
			Self::Op(OpToken::Lsl) => write!(f, "{:<20} | {:<16}", "OPERATOR", "<<"),
			Self::Op(OpToken::Lsr) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">>"),
			Self::Op(OpToken::Asr) => write!(f, "{:<20} | {:<16}", "OPERATOR", ">>>"),
			Self::Op(OpToken::Plus) => write!(f, "{:<20} | {:<16}", "OPERATOR", "+"),
			Self::Op(OpToken::Minus) => write!(f, "{:<20} | {:<16}", "OPERATOR", "-"),
			Self::Op(OpToken::Mul) => write!(f, "{:<20} | {:<16}", "OPERATOR", "*"),
			Self::Op(OpToken::Div) => write!(f, "{:<20} | {:<16}", "OPERATOR", "/"),
			Self::Op(OpToken::Rem) => write!(f, "{:<20} | {:<16}", "OPERATOR", "%"),
			Self::Op(OpToken::LogicNot) => write!(f, "{:<20} | {:<16}", "OPERATOR", "!"),
			Self::Op(OpToken::BitNot) => write!(f, "{:<20} | {:<16}", "OPERATOR", "~"),

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

			Self::Section(s) => write!(f, "{}", s),

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
