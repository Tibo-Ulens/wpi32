use std::iter::Peekable;

pub(crate) mod ast;

use common::Error;

use crate::lex::{Lexer, Token};

pub(crate) struct Parser<'l, 's: 'l> {
	lexer: Peekable<&'l mut Lexer<'s>>,
}

impl<'l, 's> Parser<'l, 's> {
	pub(crate) fn new(lexer: &'l mut Lexer<'s>) -> Self {
		Self { lexer: lexer.into_iter().peekable() }
	}

	/// Peek at the next lexeme
	fn peek(&mut self) -> Option<&Result<Token, Error>> { self.lexer.peek() }

	/// Consume and return the next lexeme
	fn next(&mut self) -> Option<Result<Token, Error>> { self.lexer.next() }
}
