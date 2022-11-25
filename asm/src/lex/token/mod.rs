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

pub use directive::*;
pub use instruction::*;
pub use operator::*;
pub use register::*;

/// All possible types of token
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed to store any potential
///    string references in identifiers
#[derive(Clone, Copy, PartialEq, Eq)]
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
	/// Escaped character litera
	LitChar(char),
	/// Numeric literal
	LitNum(isize),

	/// A section identifier
	Section(&'s str),

	/// An identifier
	Identifier(&'s str),

	/// `,`
	SymComma,
	/// `\n`
	SymNewline,
	/// `(`
	SymLeftParen,
	/// `)`
	SymRightParen,
	/// `[`
	SymLeftBracket,
	/// `]`
	SymRightBracket,
	/// `{`
	SymLeftBrace,
	/// `}`
	SymRightBrace,

	/// An operator (see also [`OpToken`])
	Op(OpToken),

	/// A comment
	Comment(&'s str),
}

impl<'s> Debug for TokenType<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let t = 10;
		let v = 16;

		match self {
			Self::Inst(inst) => write!(f, "{:<t$} {:<v$}", "INSTRUCTION", inst),

			Self::Reg(reg) => write!(f, "{:<t$} {:<v$}", "REGISTER", reg),

			Self::Dir(dir) => write!(f, "{:<t$} {:<v$}", "DIRECTIVE", dir),

			Self::Section(s) => write!(f, "{:<t$} {:<v$}", "SECTION", s),

			Self::LitStr(s) => write!(f, "{:<t$} {:<v$}", "STRING", format!("{:?}", s)),
			Self::LitChar(c) => write!(f, "{:<t$} {:<v$}", "CHAR", format!("{:?}", c)),
			Self::LitNum(n) => write!(f, "{:<t$} {:<v$}", "NUM", n),

			Self::Identifier(i) => write!(f, "{:<t$} {:<v$}", "IDENTIFIER", i),

			Self::SymComma => write!(f, "{:<t$} {:<v$}", "SYMBOL", ","),
			Self::SymNewline => write!(f, "{:<t$} {:<v$}", "SYMBOL", "\\n"),
			Self::SymLeftParen => write!(f, "{:<t$} {:<v$}", "SYMBOL", "("),
			Self::SymRightParen => write!(f, "{:<t$} {:<v$}", "SYMBOL", ")"),
			Self::SymLeftBracket => write!(f, "{:<t$} {:<v$}", "SYMBOL", "["),
			Self::SymRightBracket => write!(f, "{:<t$} {:<v$}", "SYMBOL", "]"),
			Self::SymLeftBrace => write!(f, "{:<t$} {:<v$}", "SYMBOL", "{{"),
			Self::SymRightBrace => write!(f, "{:<t$} {:<v$}", "SYMBOL", "}}"),

			Self::Op(op) => write!(f, "{:<t$} {:<v$}", "OPERATOR", op),

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

			Self::Identifier(i) => write!(f, "{}", i),

			Self::SymComma => write!(f, "COMMA"),
			Self::SymNewline => write!(f, "NEWLINE"),
			Self::SymLeftParen => write!(f, "("),
			Self::SymRightParen => write!(f, ")"),
			Self::SymLeftBracket => write!(f, "["),
			Self::SymRightBracket => write!(f, "]"),
			Self::SymLeftBrace => write!(f, "{{"),
			Self::SymRightBrace => write!(f, "}}"),

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
