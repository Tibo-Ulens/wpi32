//! # Parser
//!
//! The parser is responsible for converting the stream of [`Token`]s produces
//! by the [`Lexer`](crate::lex::Lexer) into a structured abstract syntax tree
//! (see [`ast`]) as well as recognizing syntax errors
//!
//! ### usage
//! ```rust
//! use std::fs::File;
//! use std::io::Read;
//! use std::path::PathBuf;
//!
//! use asm::error::Error;
//! use asm::lex::{Lexer, Token};
//! use asm::parse::{Node, Parser};
//!
//! fn parser_example() -> Result<(), Error> {
//!     let src_file_name = "/foo/bar/baz.asm";
//!     let src_file_path = PathBuf::from(&src_file_name);
//!
//!     let mut file = File::open(src_file_path)?;
//!     let mut contents = String::new();
//!
//!     file.read_to_string(&mut contents)?;
//!
//!     let lexer = Lexer::new(&src_file_name, &contents);
//!     let tokens: Vec<Token> = lexer.into_iter().collect::<Result<Vec<Token>, Error>>()?;
//!
//!     let mut parser = Parser::new(&src_file_name, &tokens);
//!     let ast_root = parser.parse()?;
//!     println!("{}", Node::from(&ast_root));
//!
//!     Ok(())
//! }
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

pub use display::Node;

use self::ast::{Attribute, Comment, File, Item, LabeledBlock, Statement};
use crate::lex::{OpToken, Token, TokenType};

/// Main parser type
///
/// Wraps all internal state during parsing and provides a namespace for all
/// parser-related functions
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed as tokens containing
///    string literals will contain references instead of owned data
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
	pub fn parse(&'s mut self) -> Result<File<'s>, Error> {
		let mut attrs = vec![];
		let mut items = vec![];

		while let Ok(peek) = self.peek() {
			if peek.t == TokenType::SymInAttr || peek.t == TokenType::SymOutAttr {
				let attr = self.parse_attribute()?;
				attrs.push(attr);
			} else {
				let item = self.parse_item()?;
				items.push(item);
			}
		}

		Ok(File { attrs, items })
	}

	fn parse_attribute<'r>(&'r mut self) -> Result<Attribute<'s>, ParseError> { todo!() }

	/// Parse an [`Item`] consisting of:
	///  - Any amount of comments
	///  - An optional [`Statement`]
	///  - A newline
	///
	/// Consumes the final newline
	fn parse_item<'r>(&'r mut self) -> Result<Item<'s>, ParseError> {
		let mut comments = vec![];

		// Comments may appear before the statement
		while let TokenType::Comment(_) = self.peek()?.t {
			// Unwrap is safe as peek is [`Ok`]
			let comment_token = self.next().unwrap();
			comments.push(Comment::from(*comment_token));

			self.expect(TokenType::SymNewline)?;
		}

		let statement = self.tryparse_statement()?;

		// Comments may also appear after the statement
		if let TokenType::Comment(_) = self.peek()?.t {
			while let TokenType::Comment(_) = self.peek()?.t {
				// Unwrap is safe as peek is [`Ok`]
				let comment_token = self.next().unwrap();
				comments.push(Comment::from(*comment_token));

				self.expect(TokenType::SymNewline)?;
			}
		} else {
			// Comment or not, there should still be a newline
			self.expect(TokenType::SymNewline)?;
		}

		Ok(Item { comments, statement })
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

		Ok(LabeledBlock { label, items: lines })
	}
}
