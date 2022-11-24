//! [`Parser`] functions to parse [`DataDirective`] expressions

use super::ast::{DataDirective, Literal};
use super::Parser;
use crate::error::ParseError;
use crate::lex::{DirToken, RegularDirective, TokenType};

impl<'s> Parser<'s> {
	/// Parse any of the following [`DataDirective`]s:
	///  - [`#BYTES`](DirToken::Bytes)
	///  - [`#HALVES`](DirToken::Halves)
	///  - [`#WORDS`](DirToken::Words)
	///  - [`#RES_BYTES`](DirToken::ResBytes)
	///  - [`#RES_HALVES`](DirToken::ResHalves)
	///  - [`#RES_WORDS`](DirToken::ResWords)
	///  - [`#REPEAT`](DirToken::Repeat)
	///
	/// Assumes the current [`Token`](crate::lex::Token) has [`TokenType`]
	/// [`TokenType::Dir`]
	pub(super) fn parse_datadirective<'r>(&'r mut self) -> Result<DataDirective<'s>, ParseError> {
		// Unwrap is assumed to be safe
		let directive_token = self.next().unwrap();

		match &directive_token.t {
			TokenType::Dir(DirToken::Regular(dir)) => {
				let data = self.parse_literal_list()?;

				match dir {
					RegularDirective::Bytes => Ok(DataDirective::Bytes(data)),
					RegularDirective::Halves => Ok(DataDirective::Halves(data)),
					RegularDirective::Words => Ok(DataDirective::Words(data)),
					RegularDirective::ResBytes => Ok(DataDirective::ResBytes(data)),
					RegularDirective::ResHalves => Ok(DataDirective::ResHalves(data)),
					RegularDirective::ResWords => Ok(DataDirective::ResWords(data)),
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
