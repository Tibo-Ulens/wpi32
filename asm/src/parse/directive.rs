//! [`Parser`] functions to parse [`Directive`] expressions

use super::ast::{Directive, Literal};
use super::Parser;
use crate::error::ParseError;
use crate::lex::{DirToken, RegularDirective, TokenType};

impl<'s> Parser<'s> {
	/// Parse any of the following [`Directive`]s:
	///  - [`#BYTES`](RegularDirective::Bytes)
	///  - [`#HALVES`](RegularDirective::Halves)
	///  - [`#WORDS`](RegularDirective::Words)
	///  - [`#RES_BYTES`](RegularDirective::ResBytes)
	///  - [`#RES_HALVES`](RegularDirective::ResHalves)
	///  - [`#RES_WORDS`](RegularDirective::ResWords)
	///  - [`#CONST`](RegularDirective::Const)
	///
	/// Assumes the current [`Token`](crate::lex::Token) has [`TokenType`]
	/// [`TokenType::Dir`]
	pub(super) fn parse_directive<'r>(&'r mut self) -> Result<Directive<'s>, ParseError> {
		// Unwrap is assumed to be safe
		let directive_token = self.peek().unwrap();

		match &directive_token.t {
			TokenType::Dir(DirToken::Regular(RegularDirective::Const)) => {
				self.parse_const_directive().map(Directive::Const)
			},
			TokenType::Dir(DirToken::Regular(dir)) => {
				// unwrap is safe as peek must've existed
				self.next().unwrap();
				let data = self.parse_literal_list()?;

				match dir {
					RegularDirective::Bytes => Ok(Directive::Bytes(data)),
					RegularDirective::Halves => Ok(Directive::Halves(data)),
					RegularDirective::Words => Ok(Directive::Words(data)),
					RegularDirective::ResBytes => Ok(Directive::ResBytes(data)),
					RegularDirective::ResHalves => Ok(Directive::ResHalves(data)),
					RegularDirective::ResWords => Ok(Directive::ResWords(data)),
					_ => unreachable!(),
				}
			},
			_ => unreachable!(),
		}
	}

	/// Parse a value list for any of the following [`DataDirective`]s:
	///  - [`#BYTES`](DirToken::Bytes)
	///  - [`#HALVES`](DirToken::Halves)
	///  - [`#WORDS`](DirToken::Words)
	///  - [`#RES_BYTES`](DirToken::ResBytes)
	///  - [`#RES_HALVES`](DirToken::ResHalves)
	///  - [`#RES_WORDS`](DirToken::ResWords)
	fn parse_literal_list<'r>(&'r mut self) -> Result<Vec<Literal<'s>>, ParseError> {
		let mut data = vec![];

		let value = self.parse_literal()?;
		data.push(value);

		// Keep looking for literals as long as a comma is found
		while let Ok(peek) = self.peek() && peek.t == TokenType::SymComma {
			// Unwrap is safe as peek is Ok
			self.next().unwrap();

			let value = self.parse_literal()?;
			data.push(value);
		}

		Ok(data)
	}
}
