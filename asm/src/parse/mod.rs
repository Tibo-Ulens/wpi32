//! # Parser
//!
//! The parser is responsible for converting the stream of [`Token`]s produces
//! by the [`Lexer`](crate::lex::Lexer) into a structured abstract syntax tree
//! (see [`ast`]) as well as recognizing syntax errors
//!
//! ### usage
//! ```rust
//! let src_file_name = "/foo/bar/baz.asm";
//! let src_file_path = PathBuf::from(&src_file_name);
//!
//! let mut file = File::open(src_file_path)?;
//! let mut contents = String::new();
//!
//! file.read_to_string(&mut contents)?;
//!
//! let lexer = Lexer::new(&src_file_name, &contents);
//! let tokens: Vec<Token> = lexer.into_iter().collect::<Result<Vec<Token>, Error>>()?;
//!
//! let mut parser = Parser::new(&src_file_name, &tokens);
//! let ast_root = parser.parse()?;
//! ```

use std::assert_matches::assert_matches;

use crate::error::{Error, LocationInfo, ParseError};

pub mod ast;
mod directive;
mod display;
mod immediate;
mod instruction;
mod r#macro;

pub(crate) use display::Node;

use self::ast::{ConstDirective, Line, Literal, PreambleLine, Root, Section, Statement};
use crate::lex::{DirToken, OpToken, RegularDirective, Token, TokenType};

/// Main parser type
///
/// Wraps all internal state during parsing and provides a namespace for all
/// parser-related functions
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed as (most) tokens
///    containing string literals will contain references instead of owned data
#[derive(Clone, Copy, Debug)]
pub struct Parser<'s> {
	/// The stream of lexemes
	stream: &'s [Token<'s>],

	/// The name of the file being parsed (used for error messages)
	source_file: &'s str,
	/// The length of the token stream
	len:         usize,
	/// The current index into the token stream
	idx:         usize,
}

impl<'s> Parser<'s> {
	/// Create a new parser given a source file name and a stream of [`Token`]s
	pub fn new(source_file: &'s str, stream: &'s [Token<'s>]) -> Self {
		Self { stream, source_file, len: stream.len(), idx: 0 }
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
				location: Box::new(LocationInfo::from(prev)),
			})
		}
	}

	/// Peek at the next token in the stream
	///
	/// Returns [`ParseError::UnexpectedEof`] if the next token is [`None`]
	fn peek(&self) -> Result<&'s Token<'s>, ParseError> {
		if self.idx < self.len {
			Ok(&self.stream[self.idx])
		} else {
			let srcf = self.source_file.to_string();
			let prev = self.prev();

			Err(ParseError::UnexpectedEof {
				src_file: srcf,
				location: Box::new(LocationInfo::from(prev)),
			})
		}
	}

	/// Return the previous token in the stream
	fn prev(&self) -> &Token { &self.stream[self.idx - 1] }

	/// Returns [`Ok`] if the next token matches the given
	/// [`TokenType`](crate::lex::TokenType), else returns [`Err`]
	///
	/// TODO: make this accept a custom error return type for better errors
	/// (might need a closure)
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
				location: Box::new(LocationInfo::from(prev)),
				fnd:      repr,
				ex:       t.to_string(),
			})
		}
	}

	/// Parse the token stream into an AST [`Root`]
	///
	/// Assumes the token stream ends on a newline
	pub fn parse(&'s mut self) -> Result<Root<'s>, Error> {
		let mut preamble = vec![];
		let mut sections = vec![];

		// As long as there is no section header we're in the preamble
		while let Ok(peek) = self.peek() && peek.t != TokenType::Dir(DirToken::Section) {
			let preambleline = self.parse_preambleline()?;
			preamble.push(preambleline);
		}

		while let Ok(peek) = self.peek() && peek.t == TokenType::Dir(DirToken::Section) {
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
		let peek = self.peek()?;
		let constdir = match &peek.t {
			TokenType::Dir(DirToken::Regular(RegularDirective::Const)) => {
				Some(self.parse_const_directive()?)
			},
			_ => None,
		};

		let comment = if let TokenType::Comment(c) = self.peek()?.t {
			// Unwrap is safe as peek is Ok
			self.next().unwrap();
			Some(c)
		} else {
			None
		};

		self.expect(TokenType::SymNewline)?;

		Ok(PreambleLine { constdir, comment })
	}

	/// Parse a `#CONST` directive consisting of:
	///  - A [`LabelId`] name
	///  - The [`#CONST`](DirToken::Const) keyword
	///  - A [`Literal`] value
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::Dir(DirToken::Const)`]
	fn parse_const_directive<'r>(&'r mut self) -> Result<ConstDirective<'s>, ParseError> {
		// Consume the label definition
		// Unwrap is safe as peek is Ok
		assert_matches!(self.peek().unwrap().t, TokenType::Identifier(_));
		self.next().unwrap();

		let id_token = self.next()?;

		let id = match &id_token.t {
			TokenType::Identifier(id) => id,
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(id_token)),
					fnd:      id_token.t.to_string(),
					ex:       "IDENTIFIER".to_string(),
				});
			},
		};

		let value = self.parse_literal()?;

		Ok(ConstDirective { id, value })
	}

	/// Parse a [`Literal`] consisting of either:
	///  - A [string literal](Literal::String)
	///  - A [char literal](Literal::Char)
	///  - An [immediate](Literal::Immediate)
	fn parse_literal<'r>(&'r mut self) -> Result<Literal<'s>, ParseError> {
		let peek = self.peek()?;

		// Unwrap is safe as peek is Ok
		let lit = match &peek.t {
			TokenType::LitStr(s) => {
				self.next().unwrap();
				Literal::String(s)
			},
			TokenType::LitChar(c) => {
				self.next().unwrap();
				Literal::Char(*c)
			},
			TokenType::Op(
				OpToken::Plus | OpToken::Minus | OpToken::BitNot | OpToken::Exclamation,
			)
			| TokenType::SymLeftParen
			| TokenType::LitNum(_)
			| TokenType::Identifier(_) => Literal::Immediate(self.parse_immediate()?),
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
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
		// Unwrap is safe as peek is Ok
		assert_eq!(self.peek().unwrap().t, TokenType::Dir(DirToken::Section));
		self.next().unwrap();

		let peek = self.peek()?;
		let name = match peek.t {
			TokenType::Section(s) => {
				// Unwrap is safe as peek is Ok
				self.next().unwrap();
				s
			},
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
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
		let statement = self.tryparse_statement()?;

		let comment = if let TokenType::Comment(c) = self.peek()?.t {
			// Unwrap is safe as peek is Ok
			self.next().unwrap();
			Some(c)
		} else {
			None
		};

		self.expect(TokenType::SymNewline)?;

		Ok(Line { statement, comment })
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
					location: Box::new(LocationInfo::from(peek)),
					fnd:      peek.t.to_string(),
					ex:       "DIRECTIVE or INSTRUCTION or COMMENT or NEWLINE".to_string(),
				})
			},
		}
	}
}
