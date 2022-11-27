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
use std::mem::discriminant;

use crate::error::{Error, LocationInfo, ParseError};

pub mod ast;
mod directive;
mod display;
mod immediate;
mod instruction;
mod r#macro;

pub(crate) use display::Node;

use self::ast::{
	ConstDirective,
	LabeledBlock,
	Line,
	Literal,
	PreambleLine,
	PreambleStatement,
	Root,
	Section,
	Statement,
};
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
	/// Will only consume the next token if its type matches the argument
	///
	/// TODO: make this accept a custom error return type for better errors
	/// (might need a closure)
	fn expect<'r>(&'r mut self, t: TokenType<'s>) -> Result<Token<'s>, ParseError> {
		let peek = self.peek()?;

		if discriminant(&peek.t) == discriminant(&t) {
			// Unwrap is safe as peek is [`Ok`]
			Ok(*self.next().unwrap())
		} else {
			let repr = peek.t.to_string();
			let srcf = self.source_file.to_string();
			let prev = self.prev();

			Err(ParseError::UnexpectedToken {
				src_file: srcf,
				location: Box::new(LocationInfo::from(prev)),
				found:    repr,
				expected: t.to_string(),
			})
		}
	}

	/// Returns [`Some`] if the next token matches a given type, else [`None`]
	///
	/// Will only consume the next token if its type matches the argument
	fn optional<'r>(&'r mut self, t: TokenType<'s>) -> Option<Token<'s>> {
		let peek = self.peek().ok()?;

		if discriminant(&peek.t) == discriminant(&t) {
			// Unwrap is safe as peek is [`Ok`]
			Some(*self.next().unwrap())
		} else {
			None
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
		let statement = self.tryparse_preamble_statement()?;

		let comment = if let TokenType::Comment(c) = self.peek()?.t {
			// Unwrap is safe as peek is Ok
			self.next().unwrap();
			Some(c)
		} else {
			None
		};

		let nl = self.peek()?;
		if nl.t != TokenType::SymNewline {
			// Unwrap is safe as peek is [`Ok`]
			self.next().unwrap();
			return Err(ParseError::UnexpectedToken {
				src_file: self.source_file.to_string(),
				location: Box::new(LocationInfo::from(nl)),
				found:    nl.t.to_string(),
				expected: "CONST DIRECTIVE or NEWLINE".to_string(),
			});
		}

		// TODO: throw a better error in case anything that isn't a constdir or
		// comment is found
		self.expect(TokenType::SymNewline)?;

		Ok(PreambleLine { statement, comment })
	}

	/// Try to parse a [`PreambleStatement`]
	///
	/// Returns [`None`] if the current [`Token`] cannot start a statement
	fn tryparse_preamble_statement<'r>(
		&'r mut self,
	) -> Result<Option<PreambleStatement<'s>>, ParseError> {
		let peek = self.peek()?;
		match &peek.t {
			TokenType::Identifier("define_macro") => {
				Ok(Some(PreambleStatement::MacroDefinition(self.parse_macro_definition()?)))
			},
			TokenType::Dir(DirToken::Regular(RegularDirective::Const)) => {
				Ok(Some(PreambleStatement::ConstDirective(self.parse_const_directive()?)))
			},
			TokenType::SymNewline => Ok(None),
			TokenType::Comment(_) => Ok(None),
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
					found:    peek.t.to_string(),
					expected: "CONST DIRECTIVE or MACRO DEFINITION or COMMENT or NEWLINE"
						.to_string(),
				})
			},
		}
	}

	/// Parse a `#CONST` directive consisting of:
	///  - The [`#CONST`](DirToken::Const) keyword
	///  - An [`Identifier`] name
	///  - A [`Literal`] value
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::Dir(DirToken::Const)`]
	fn parse_const_directive<'r>(&'r mut self) -> Result<ConstDirective<'s>, ParseError> {
		// Consume the #CONST token
		// Unwrap is assumed to be safe
		assert_matches!(
			self.next().unwrap().t,
			TokenType::Dir(DirToken::Regular(RegularDirective::Const))
		);

		let id_token = self.next()?;

		let id = match &id_token.t {
			TokenType::Identifier(id) => id,
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(id_token)),
					found:    id_token.t.to_string(),
					expected: "IDENTIFIER".to_string(),
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
					found:    peek.t.to_string(),
					expected: "STRING or CHAR or IMMEDIATE".to_string(),
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
		// Unwrap is assumed to be safe
		assert_eq!(self.next().unwrap().t, TokenType::Dir(DirToken::Section));

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
					found:    peek.t.to_string(),
					expected: ".TEXT or .DATA or .BSS".to_string(),
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
	///  - An optional [`Statement`]
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
			TokenType::Identifier("define_macro") => {
				Ok(Some(Statement::MacroDefinition(self.parse_macro_definition()?)))
			},
			TokenType::Identifier(id) => {
				self.next().unwrap();

				let peek = self.peek()?;
				match peek.t {
					TokenType::Op(OpToken::Exclamation) => {
						Ok(Some(Statement::MacroInvocation(self.parse_macro_invocation(id)?)))
					},
					TokenType::SymLeftBrace => {
						Ok(Some(Statement::LabeledBlock(self.parse_labeled_block(id)?)))
					},
					_ => {
						Err(ParseError::UnexpectedToken {
							src_file: self.source_file.to_string(),
							location: Box::new(LocationInfo::from(peek)),
							found:    peek.t.to_string(),
							expected: "! or {".to_string(),
						})
					},
				}
			},
			TokenType::Dir(_) => Ok(Some(Statement::Directive(self.parse_directive()?))),
			TokenType::Inst(_) => Ok(Some(Statement::Instruction(self.parse_instruction()?))),
			TokenType::SymNewline => Ok(None),
			TokenType::Comment(_) => Ok(None),
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
					found:    peek.t.to_string(),
					expected: "DIRECTIVE or INSTRUCTION or COMMENT or NEWLINE".to_string(),
				})
			},
		}
	}

	/// Parse a labeled block of code consisting of:
	///  - a [`label`](Identifier)
	///  - a block of [`Line`]s enclosed in {}
	///
	/// Assumes the current [`Token`] has [`TokenType`]
	/// [`TokenType::SymLeftBrace`]
	///
	/// Takes the label as an argument
	fn parse_labeled_block<'r>(
		&'r mut self,
		label: &'s str,
	) -> Result<LabeledBlock<'s>, ParseError> {
		let open = self.expect(TokenType::SymLeftBrace)?;

		let mut lines = vec![];
		while let Ok(peek) = self.peek() && peek.t != TokenType::SymRightBrace {
			let line = self.parse_line()?;
			lines.push(line);
		}

		let close = self.next()?;
		if close.t != TokenType::SymRightBrace {
			return Err(ParseError::UnclosedDelimiter {
				src_file:       self.source_file.to_string(),
				delim_type:     "brace".to_string(),
				found:          close.t.to_string(),
				close_location: Box::new(LocationInfo::from(close)),
				open_location:  Box::new(LocationInfo::from(&open)),
			});
		}

		Ok(LabeledBlock { label, lines })
	}
}
