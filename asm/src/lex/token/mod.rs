//! # Lexer Tokens
//!
//! Tokens or lexemes are the datatype returned by the [`Lexer`](super::Lexer)
//!
//! Tokens represent a fundamental 'chunk' of information in the language with
//! a specific name and an (optional) value

use std::fmt::{Debug, Display};

mod directive;
mod instruction;
mod operator;
mod register;

pub use directive::DirToken;
pub use instruction::InstToken;
pub use operator::OpToken;
pub use register::RegToken;

/// All possible types of token
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed to store any potential
///    string references in identifiers
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TokenType<'s> {
	/// An instruction (see also [`InstToken`])
	Inst(InstToken),
	/// A Register (see also [`RegToken`])
	Reg(RegToken),
	/// A directive (see also [`DirToken`])
	Dir(DirToken),

	/// **UNESCAPED** string literal, basckslash escaping should be performed
	/// when splitting into bytes
	LitStr(&'s str),
	LitChar(char),
	LitNum(isize),

	/// A section identifier
	Section(&'s str),

	/// A label reference
	Label(&'s str),
	/// A label definition
	LabelDefine(&'s str),
	/// A local label reference
	LocalLabel(&'s str),
	/// A local label definition
	LocalLabelDefine(&'s str),

	SymComma,
	SymNewline,
	SymLeftParen,
	SymRightParen,
	SymLeftBracket,
	SymRightBracket,

	/// An operand (see also [`OpToken`])
	Op(OpToken),

	Comment(&'s str),
}

impl<'s> Debug for TokenType<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let t = 10;
		let v = 16;

		match self {
			Self::Inst(InstToken::Add) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "add"),
			Self::Inst(InstToken::Addi) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "addi"),
			Self::Inst(InstToken::Sub) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "sub"),
			Self::Inst(InstToken::And) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "and"),
			Self::Inst(InstToken::Andi) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "andi"),
			Self::Inst(InstToken::Or) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "or"),
			Self::Inst(InstToken::Ori) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "ori"),
			Self::Inst(InstToken::Xor) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "xor"),
			Self::Inst(InstToken::Xori) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "xori"),
			Self::Inst(InstToken::Lsl) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lsl"),
			Self::Inst(InstToken::Lsli) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lsli"),
			Self::Inst(InstToken::Lsr) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lsr"),
			Self::Inst(InstToken::Lsri) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lsri"),
			Self::Inst(InstToken::Asr) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "asr"),
			Self::Inst(InstToken::Asri) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "asri"),
			Self::Inst(InstToken::Slt) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "slt"),
			Self::Inst(InstToken::Slti) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "slti"),
			Self::Inst(InstToken::Sltu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "sltu"),
			Self::Inst(InstToken::Sltiu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "sltiu"),
			Self::Inst(InstToken::Lw) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lw"),
			Self::Inst(InstToken::Lh) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lh"),
			Self::Inst(InstToken::Lhu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lhu"),
			Self::Inst(InstToken::Lb) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lb"),
			Self::Inst(InstToken::Lbu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lbu"),
			Self::Inst(InstToken::Sw) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "sw"),
			Self::Inst(InstToken::Sh) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "sh"),
			Self::Inst(InstToken::Sb) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "sb"),
			Self::Inst(InstToken::Lui) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "lui"),
			Self::Inst(InstToken::Auipc) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "auipc"),
			Self::Inst(InstToken::Beq) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "beq"),
			Self::Inst(InstToken::Bne) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "bne"),
			Self::Inst(InstToken::Blt) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "blt"),
			Self::Inst(InstToken::Bltu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "bltu"),
			Self::Inst(InstToken::Bge) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "bge"),
			Self::Inst(InstToken::Bgeu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "bgeu"),
			Self::Inst(InstToken::Jal) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "jal"),
			Self::Inst(InstToken::Jalr) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "jalr"),
			Self::Inst(InstToken::Ecall) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "ecall"),
			Self::Inst(InstToken::Ebreak) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "ebreak")
			},
			Self::Inst(InstToken::Fence) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "fence"),
			Self::Inst(InstToken::FenceTso) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "fence.tso")
			},
			Self::Inst(InstToken::Fencei) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "fence.i")
			},
			Self::Inst(InstToken::Csrrw) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "csrrw"),
			Self::Inst(InstToken::Csrrwi) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "csrrwi")
			},
			Self::Inst(InstToken::Csrrs) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "csrrs"),
			Self::Inst(InstToken::Csrrsi) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "csrrsi")
			},
			Self::Inst(InstToken::Csrrc) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "csrrc"),
			Self::Inst(InstToken::Csrrci) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "csrrci")
			},
			Self::Inst(InstToken::Mul) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "mul"),
			Self::Inst(InstToken::Mulh) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "mulh"),
			Self::Inst(InstToken::Mulhu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "mulhu"),
			Self::Inst(InstToken::Mulhsu) => {
				write!(f, "{:<t$} {:<v$}", "KEYWORD", "mulhsu")
			},
			Self::Inst(InstToken::Div) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "div"),
			Self::Inst(InstToken::Divu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "divu"),
			Self::Inst(InstToken::Rem) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "rem"),
			Self::Inst(InstToken::Remu) => write!(f, "{:<t$} {:<v$}", "KEYWORD", "remu"),

			Self::Reg(RegToken::R0) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r0"),
			Self::Reg(RegToken::R1) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r1"),
			Self::Reg(RegToken::R2) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r2"),
			Self::Reg(RegToken::R3) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r3"),
			Self::Reg(RegToken::R4) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r4"),
			Self::Reg(RegToken::R5) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r5"),
			Self::Reg(RegToken::R6) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r6"),
			Self::Reg(RegToken::R7) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r7"),
			Self::Reg(RegToken::R8) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r8"),
			Self::Reg(RegToken::R9) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r9"),
			Self::Reg(RegToken::R10) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r10"),
			Self::Reg(RegToken::R11) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r11"),
			Self::Reg(RegToken::R12) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r12"),
			Self::Reg(RegToken::R13) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r13"),
			Self::Reg(RegToken::R14) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r14"),
			Self::Reg(RegToken::R15) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r15"),
			Self::Reg(RegToken::R16) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r16"),
			Self::Reg(RegToken::R17) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r17"),
			Self::Reg(RegToken::R18) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r18"),
			Self::Reg(RegToken::R19) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r19"),
			Self::Reg(RegToken::R20) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r20"),
			Self::Reg(RegToken::R21) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r21"),
			Self::Reg(RegToken::R22) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r22"),
			Self::Reg(RegToken::R23) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r23"),
			Self::Reg(RegToken::R24) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r24"),
			Self::Reg(RegToken::R25) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r25"),
			Self::Reg(RegToken::R26) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r26"),
			Self::Reg(RegToken::R27) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r27"),
			Self::Reg(RegToken::R28) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r28"),
			Self::Reg(RegToken::R29) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r29"),
			Self::Reg(RegToken::R30) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r30"),
			Self::Reg(RegToken::R31) => write!(f, "{:<t$} {:<v$}", "REGISTER", "r31"),

			Self::Dir(DirToken::Section) => {
				write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$SECTION")
			},
			Self::Dir(DirToken::Bytes) => write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$BYTES"),
			Self::Dir(DirToken::Halves) => {
				write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$HALVES")
			},
			Self::Dir(DirToken::Words) => write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$WORDS"),
			Self::Dir(DirToken::ResBytes) => {
				write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$RES_BYTES")
			},
			Self::Dir(DirToken::ResHalves) => {
				write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$RES_HALVES")
			},
			Self::Dir(DirToken::ResWords) => {
				write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$RES_WORDS")
			},
			Self::Dir(DirToken::Repeat) => {
				write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$repeat")
			},
			Self::Dir(DirToken::Const) => write!(f, "{:<t$} {:<v$}", "DIRECTIVE", "$CONST"),

			Self::Section(s) => write!(f, "{:<t$} {:<v$}", "SECTION", s),

			Self::LitStr(s) => write!(f, "{:<t$} {:<v$}", "STRING", format!("{:?}", s)),
			Self::LitChar(c) => write!(f, "{:<t$} {:<v$}", "CHAR", format!("{:?}", c)),
			Self::LitNum(n) => write!(f, "{:<t$} {:<v$}", "NUM", n),

			Self::Label(l) => write!(f, "{:<t$} {:<v$}", "LABEL", l),
			Self::LabelDefine(l) => write!(f, "{:<t$} {:<v$}", "LABEL_DEFINE", l),
			Self::LocalLabel(ll) => write!(f, "{:<t$} {:<v$}", "LOCAL_LABEL", ll),
			Self::LocalLabelDefine(ll) => write!(f, "{:<t$} {:<v$}", "LOCAL_LABEL_DEFINE", ll),

			Self::SymComma => write!(f, "{:<t$} {:<v$}", "SYMBOL", ","),
			Self::SymNewline => write!(f, "{:<t$} {:<v$}", "SYMBOL", "\\n"),
			Self::SymLeftParen => write!(f, "{:<t$} {:<v$}", "SYMBOL", "("),
			Self::SymRightParen => write!(f, "{:<t$} {:<v$}", "SYMBOL", ")"),
			Self::SymLeftBracket => write!(f, "{:<t$} {:<v$}", "SYMBOL", "["),
			Self::SymRightBracket => write!(f, "{:<t$} {:<v$}", "SYMBOL", "]"),

			Self::Op(OpToken::TernStart) => write!(f, "{:<t$} {:v$}", "OPERATOR", "?"),
			Self::Op(OpToken::TernAlt) => write!(f, "{:<t$} {:v$}", "OPERATOR", ":"),
			Self::Op(OpToken::LogicOr) => write!(f, "{:<t$} {:v$}", "OPERATOR", "||"),
			Self::Op(OpToken::LogicXor) => write!(f, "{:<t$} {:v$}", "OPERATOR", "^^"),
			Self::Op(OpToken::LogicAnd) => write!(f, "{:<t$} {:v$}", "OPERATOR", "&&"),
			Self::Op(OpToken::BitOr) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "|"),
			Self::Op(OpToken::BitXor) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "^"),
			Self::Op(OpToken::BitAnd) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "&"),
			Self::Op(OpToken::Eq) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "=="),
			Self::Op(OpToken::Neq) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "!="),
			Self::Op(OpToken::Lt) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "<"),
			Self::Op(OpToken::Lte) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "<="),
			Self::Op(OpToken::Gt) => write!(f, "{:<t$} {:<v$}", "OPERATOR", ">"),
			Self::Op(OpToken::Gte) => write!(f, "{:<t$} {:<v$}", "OPERATOR", ">="),
			Self::Op(OpToken::Lsl) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "<<"),
			Self::Op(OpToken::Lsr) => write!(f, "{:<t$} {:<v$}", "OPERATOR", ">>"),
			Self::Op(OpToken::Asr) => write!(f, "{:<t$} {:<v$}", "OPERATOR", ">>>"),
			Self::Op(OpToken::Plus) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "+"),
			Self::Op(OpToken::Minus) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "-"),
			Self::Op(OpToken::Mul) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "*"),
			Self::Op(OpToken::Div) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "/"),
			Self::Op(OpToken::Rem) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "%"),
			Self::Op(OpToken::LogicNot) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "!"),
			Self::Op(OpToken::BitNot) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "~"),
			Self::Op(OpToken::UnaryMinus) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "-"),
			Self::Op(OpToken::LeftParen) => write!(f, "{:<t$} {:<v$}", "OPERATOR", "("),

			Self::Comment(cmt) => write!(f, "{:<t$} {:<v$}", "COMMENT", format!("{:?}", cmt)),
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

/// A single lexical token
///
/// Contains a token type and (optional) value, as well as information about
/// its position in the source code
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed to keep a reference to
///    the source line for this token and to store any potential references in its [`TokenType`]
#[derive(Clone, Copy, Debug)]
pub struct Token<'s> {
	/// The type of this token
	pub t:           TokenType<'s>,
	/// The line number of this token
	pub line:        usize,
	/// The column number of this token
	pub col:         usize,
	/// The length (in characters) of this token
	pub span:        usize,
	/// The line of source code containing this token
	pub source_line: &'s str,
}

impl<'s> Display for Token<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let start = self.col - 1; // Columns start at 1
		let left_arm = &self.source_line[..start].trim_start();
		let center = &self.source_line[start..start + self.span];
		let right_arm = &self.source_line[start + self.span..];

		let t = format!("{:?}", self.t);
		let annotated_src = format!("{}```{}```{}", left_arm, center, right_arm);

		write!(f, "[{:0>3}:{:0>3}]: {:<42} {:?}", self.line, self.col, t, annotated_src)
	}
}
