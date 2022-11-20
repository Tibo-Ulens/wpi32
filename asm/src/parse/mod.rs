use std::assert_matches::assert_matches;

use common::{Error, ParseError};

pub(crate) mod ast;
mod display;
mod immediate;

use self::ast::{
	ConstDirective,
	Directive,
	Identifier,
	Instruction,
	LabelId,
	Line,
	LineContent,
	Literal,
	PreambleLine,
	Root,
	Section,
	Statement,
};
use crate::lex::{DirToken, OpToken, Token, TokenType};

pub(crate) struct Parser<'s> {
	stream: &'s [Token<'s>],

	source_file: String,
	len:         usize,
	idx:         usize,
}

impl<'s> Parser<'s> {
	pub(crate) fn new(src_file: &str, stream: &'s [Token<'s>]) -> Self {
		Self { stream, source_file: src_file.to_string(), len: stream.len(), idx: 0 }
	}

	/// Return the next token in the stream
	///
	/// Returns [`ParseError::UnexpectedEof`] if the next token is [`None`]
	fn next<'r>(&'r mut self) -> Result<&'s Token<'s>, ParseError> {
		if self.idx < self.len {
			self.idx += 1;
			Ok(&self.stream[self.idx - 1])
		} else {
			let srcf = self.source_file.to_string();
			let prev = self.prev();

			Err(ParseError::UnexpectedEof {
				src_file: srcf,
				line:     prev.line,
				col:      prev.col,
				src_line: prev.source_line.to_string(),
			})
		}
	}

	/// Peek at the next token in the stream
	///
	/// Returns [`ParseError::UnexpectedEof`] if the next token is [`None`]
	fn peek(&self) -> Result<&'s Token<'s>, ParseError> {
		if self.idx < self.len - 1 {
			Ok(&self.stream[self.idx])
		} else {
			let srcf = self.source_file.to_string();
			let prev = self.prev();

			Err(ParseError::UnexpectedEof {
				src_file: srcf,
				line:     prev.line,
				col:      prev.col,
				src_line: prev.source_line.to_string(),
			})
		}
	}

	/// Return the previous token in the stream
	fn prev(&self) -> &Token { &self.stream[self.idx - 1] }

	/// Returns [`Ok`] if the next token matches the given
	/// [`TokenType`](crate::lex::TokenType), else returns [`Err`]
	fn expect(&mut self, t: TokenType) -> Result<(), ParseError> {
		let next_type = &self.next()?.t;

		if next_type == &t {
			Ok(())
		} else {
			let repr = next_type.to_string();
			let srcf = self.source_file.to_string();
			let prev = self.prev();

			Err(ParseError::UnexpectedToken {
				src_file: srcf,
				line:     prev.line,
				col:      prev.col,
				span:     prev.span,
				src_line: prev.source_line.to_string(),
				fnd:      repr,
				ex:       t.to_string(),
			})
		}
	}

	/// Parse the token stream into an AST [`Root`]
	///
	/// Assumes the token stream ends on a newline
	pub(crate) fn parse(&'s mut self) -> Result<Root<'s>, Error> {
		let mut preamble = vec![];
		let mut sections = vec![];

		// As long as there is no section header we're in the preamble
		while self.peek()?.t != TokenType::Dir(DirToken::Section) {
			let preambleline = self.parse_preambleline()?;
			preamble.push(preambleline);
		}

		// Unwrap is safe as && short circuits
		while self.peek().is_ok() && self.peek().unwrap().t == TokenType::Dir(DirToken::Section) {
			let section = self.parse_section()?;

			sections.push(section);
		}

		Ok(Root { preamble, sections })
	}

	/// Parse a preamble line consisting of:
	///  - An optional [`#CONST`](DirToken::Const) directive
	///  - An optional Comment
	///  - A newline
	///
	/// Consumes the trailing newline
	fn parse_preambleline<'r>(&'r mut self) -> Result<PreambleLine<'s>, ParseError> {
		let constdir = if let TokenType::Dir(DirToken::Const) = self.peek()?.t {
			Some(self.parse_constdir()?)
		} else {
			None
		};

		let comment = if let TokenType::Comment(c) = self.peek()?.t {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(c)
		} else {
			None
		};

		self.expect(TokenType::SymNewline)?;

		Ok(PreambleLine { constdir, comment })
	}

	/// Parse a `#CONST` directive consisting of:
	///  - The [`#CONST`](DirToken::Const) keyword
	///  - A [`Literal`] value
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::Dir(DirToken::Const)`]
	fn parse_constdir<'r>(&'r mut self) -> Result<ConstDirective<'s>, ParseError> {
		// Consume the `#CONST` directive token
		// Unwrap is safe as [`self.peek()`] is [`Some`]
		assert_eq!(self.peek().unwrap().t, TokenType::Dir(DirToken::Const));
		self.next().unwrap();

		let value = self.parse_literal()?;

		Ok(ConstDirective { value })
	}

	/// Parse a [`Literal`] consisting of either:
	///  - A [string literal](Literal::String)
	///  - A [char literal](Literal::Char)
	///  - An [immediate](Literal::Immediate)
	fn parse_literal<'r>(&'r mut self) -> Result<Literal<'s>, ParseError> {
		let peek = self.peek()?;

		// Unwraps are safe as [`self.peek()`] is [`Some`]
		let lit = match &peek.t {
			TokenType::LitStr(s) => {
				self.next().unwrap();
				Literal::String(s)
			},
			TokenType::LitChar(c) => {
				self.next().unwrap();
				Literal::Char(*c)
			},
			TokenType::Op(OpToken::Plus | OpToken::Minus | OpToken::BitNot | OpToken::LogicNot) => {
				Literal::Immediate(self.parse_immediate()?)
			},
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					line:     peek.line,
					col:      peek.col,
					span:     peek.span,
					src_line: peek.source_line.to_string(),
					fnd:      peek.t.to_string(),
					ex:       "STRING or CHAR or IMMEDIATE".to_string(),
				});
			},
		};

		Ok(lit)
	}

	/// Parse a section consisting of:
	///  - A section header ([#SECTION directive](DirToken::Section) + name)
	///  - Any amount of [`Line`]s
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::Dir(DirToken::Section)`]
	fn parse_section<'r>(&'r mut self) -> Result<Section<'s>, ParseError> {
		// Consume the `#SECTION` directive token
		// Unwrap is safe as [`self.peek()`] is [`Some`]
		assert_eq!(self.peek().unwrap().t, TokenType::Dir(DirToken::Section));
		self.next().unwrap();

		let peek = self.peek()?;
		let name = match peek.t {
			TokenType::Section(s) => Identifier(s),
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					line:     peek.line,
					col:      peek.col,
					span:     peek.span,
					src_line: peek.source_line.to_string(),
					fnd:      peek.t.to_string(),
					ex:       ".TEXT or .DATA or .BSS".to_string(),
				});
			},
		};

		// Section headers must end with a newline
		self.expect(TokenType::SymNewline)?;

		let mut lines = vec![];

		// As long as there are tokens remaining and they aren't section
		// directives we stay in the same section
		while let Ok(peek) = self.peek() && peek.t != TokenType::Dir(DirToken::Section) {
			let line = self.parse_line()?;
			lines.push(line);
		}

		Ok(Section { name, lines })
	}

	/// Parse a (section) [`Line`] consisting of:
	///  - Optionally either a labeled statement or an unlabeled statement (see [`LineContent`])
	///  - An optional comment
	///  - A newline
	///
	/// Consumes the final newline
	fn parse_line<'r>(&'r mut self) -> Result<Line<'s>, ParseError> {
		let peek = self.peek()?;
		let content = match peek.t {
			TokenType::SymNewline => None,
			TokenType::LabelDefine(_) | TokenType::LocalLabelDefine(_) => {
				Some(self.parse_labeled_statement()?)
			},
			_ => self.tryparse_statement()?.map(LineContent::Statement),
		};

		let comment = if let TokenType::Comment(c) = self.peek()?.t {
			// Unwrap is safe as [`self.peek()`] is [`Some`]
			self.next().unwrap();
			Some(c)
		} else {
			None
		};

		self.expect(TokenType::SymNewline)?;

		Ok(Line { content, comment })
	}

	/// Parse a [`LabeledStatement`](LineContent::LabeledStatement) consisting
	/// of:
	///  - A (local) [label definition](LabelId)
	///  - An optional [`Statement`]
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::LabelDefine`] or [`TokenType::LocalLabelDefine`]
	fn parse_labeled_statement<'r>(&'r mut self) -> Result<LineContent<'s>, ParseError> {
		// Consume the label definition
		// Unwrap is safe as [`self.peek()`] is [`Some`]
		let label_token = self.next().unwrap();
		assert_matches!(label_token.t, TokenType::LabelDefine(_) | TokenType::LocalLabelDefine(_));

		let label = match &label_token.t {
			TokenType::LabelDefine(ld) => LabelId::LabelDefine(ld),
			TokenType::LocalLabelDefine(ld) => LabelId::LocalLabelDefine(ld),
			_ => unreachable!(),
		};

		let stmt = self.tryparse_statement()?;

		Ok(LineContent::LabeledStatement { label, stmt })
	}

	/// Try to parse a [`Statement`]
	///
	/// Returns [`None`] if the current [`Token`] cannot start a statement
	fn tryparse_statement<'r>(&'r mut self) -> Result<Option<Statement<'s>>, ParseError> {
		let peek = self.peek()?;
		match &peek.t {
			TokenType::Dir(_) => Ok(Some(Statement::Directive(self.parse_directive()?))),
			TokenType::Inst(_) => Ok(Some(Statement::Instruction(self.parse_instruction()?))),
			TokenType::SymNewline => Ok(None),
			TokenType::Comment(_) => Ok(None),
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					line:     peek.line,
					col:      peek.col,
					span:     peek.span,
					src_line: peek.source_line.to_string(),
					fnd:      peek.t.to_string(),
					ex:       "DIRECTIVE or INSTRUCTION or COMMENT or NEWLINE".to_string(),
				})
			},
		}
	}

	/// Parse any of the following [`Directive`]s:
	///  - [`#BYTES`](DirToken::Bytes)
	///  - [`#HALVES`](DirToken::Halves)
	///  - [`#WORDS`](DirToken::Words)
	///  - [`#RES_BYTES`](DirToken::ResBytes)
	///  - [`#RES_HALVES`](DirToken::ResHalves)
	///  - [`#RES_WORDS`](DirToken::ResWords)
	///  - [`#REPEAT`](DirToken::Repeat)
	///
	/// Assumes the current [`Token`] has [`TokenType`] [`TokenType::Dir`]
	fn parse_directive<'r>(&'r mut self) -> Result<Directive<'s>, ParseError> { todo!() }

	/// Parse any valid [`Instruction`]
	///
	/// Assumes the current [`Token`] has [`TokenType`] [`TokenType::Inst`]
	fn parse_instruction<'r>(&'r mut self) -> Result<Instruction<'s>, ParseError> { todo!() }
}
