use std::iter::Peekable;

pub(crate) mod ast;

use crate::lex::{Lexer, Token};

pub(crate) struct Parser<'s> {
	lexer: Peekable<Lexer<'s>>,
}

impl<'s> Parser<'s> {
	pub(crate) fn new(lexer: Lexer<'s>) -> Self { Self { lexer: lexer.into_iter().peekable() } }

	/// Peek at the next lexeme
	#[inline(always)]
	fn peek(&mut self) -> Option<&Token> { self.lexer.peek() }

	/// Consume and return the next lexeme
	#[inline(always)]
	fn next(&mut self) -> Option<Token> { self.lexer.next() }
}
