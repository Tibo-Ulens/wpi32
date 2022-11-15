use common::{Error, ParseError};

mod immediate;
mod instruction;
mod root;

pub(crate) use immediate::*;
pub(crate) use instruction::*;
pub(crate) use root::*;

use crate::lex::{Token, TokenType};

pub(crate) struct Parser<'l, 's: 'l> {
	stream: &'l [Token<'s>],

	source_file: String,
	len:         usize,
	idx:         usize,
}

impl<'l, 's> Parser<'l, 's> {
	pub(crate) fn new(src_file: &str, stream: &'l [Token<'s>]) -> Self {
		Self { stream, source_file: src_file.to_owned(), len: stream.len(), idx: 0 }
	}

	/// Return the next token in the stream
	fn next<'r>(&'r mut self) -> Result<&'l Token<'s>, ParseError> {
		if self.idx < self.len {
			self.idx += 1;
			Ok(&self.stream[self.idx - 1])
		} else {
			let srcf = self.source_file.to_owned();
			let prev = self.prev();

			Err(ParseError::UnexpectedEof {
				src_file: srcf,
				line:     prev.line,
				col:      prev.col,
				src_line: prev.source_line.to_owned(),
			})
		}
	}

	/// Peek at the next token in the stream
	fn peek(&self) -> Result<&'l Token<'s>, ParseError> {
		if self.idx < self.len - 1 {
			Ok(&self.stream[self.idx + 1])
		} else {
			let srcf = self.source_file.to_owned();
			let prev = self.prev();

			Err(ParseError::UnexpectedEof {
				src_file: srcf,
				line:     prev.line,
				col:      prev.col,
				src_line: prev.source_line.to_owned(),
			})
		}
	}

	/// Return the previous token in the stream
	fn prev(&self) -> &Token { &self.stream[self.idx - 1] }

	/// Returns [`Ok`] if the next token matches the given
	/// [`TokenType`](crate::lex::TokenType), else returns [`Err`]
	fn expect(&mut self, t: TokenType) -> Result<(), ParseError> {
		let next_type = self.next()?.t.to_owned();

		if next_type == t {
			Ok(())
		} else {
			let repr = next_type.to_string();
			let srcf = self.source_file.to_owned();
			let prev = self.prev();

			Err(ParseError::UnexpectedToken {
				src_file: srcf,
				line:     prev.line,
				col:      prev.col,
				span:     prev.span,
				src_line: prev.source_line.to_owned(),
				fnd:      repr,
				ex:       t.to_string(),
			})
		}
	}

	pub(crate) fn parse(&'l mut self) -> Result<Root<'s>, Error> {
		let mut lines = vec![];

		while self.peek().is_ok() {
			let stmt = self.tryparse_statement()?;
			let cmnt = self.tryparse_comment()?;

			self.expect(TokenType::SymNewline)?;

			lines.push(Line { stmt, cmnt });
		}

		Ok(Root { lines })
	}

	fn tryparse_statement<'r>(&'r mut self) -> Result<Statement<'s>, ParseError> {
		let peek = self.peek()?;
		match &peek.t {
			TokenType::LabelDefine(ld) => {
				self.next().unwrap();
				Ok(Statement::LabelDefine(ld))
			},
			TokenType::LocalLabelDefine(lld) => {
				self.next().unwrap();
				Ok(Statement::LocalLabelDefine(lld))
			},
			TokenType::Dir(_) => Ok(Statement::Directive(self.tryparse_directive()?)),
			TokenType::Inst(_) => Ok(Statement::Instruction(self.tryparse_instruction()?)),
			_ => todo!(),
		}
	}

	fn tryparse_comment<'r>(&'r mut self) -> Result<Option<&'s str>, ParseError> {
		let peek = self.peek()?;
		if let TokenType::Comment(c) = peek.t { Ok(Some(c)) } else { Ok(None) }
	}

	fn tryparse_directive<'r>(&'r mut self) -> Result<Directive<'s>, ParseError> { todo!() }

	fn tryparse_instruction<'r>(&'r mut self) -> Result<Instruction<'s>, ParseError> { todo!() }
}
