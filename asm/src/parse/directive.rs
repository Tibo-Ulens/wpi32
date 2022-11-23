//! [`Parser`] functions to parse [`DataDirective`] expressions

use super::ast::{DataDirective, Literal, RepeatedData};
use super::Parser;
use crate::error::{LocationInfo, ParseError};
use crate::lex::{DirToken, TokenType};

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
			TokenType::Dir(DirToken::Bytes) => {
				let data = self.parse_literal_list()?;
				Ok(DataDirective::Bytes(data))
			},
			TokenType::Dir(DirToken::Halves) => {
				let data = self.parse_literal_list()?;
				Ok(DataDirective::Halves(data))
			},
			TokenType::Dir(DirToken::Words) => {
				let data = self.parse_literal_list()?;
				Ok(DataDirective::Words(data))
			},
			TokenType::Dir(DirToken::ResBytes) => {
				let data = self.parse_literal_list()?;
				Ok(DataDirective::ResBytes(data))
			},
			TokenType::Dir(DirToken::ResHalves) => {
				let data = self.parse_literal_list()?;
				Ok(DataDirective::ResHalves(data))
			},
			TokenType::Dir(DirToken::ResWords) => {
				let data = self.parse_literal_list()?;
				Ok(DataDirective::ResWords(data))
			},
			TokenType::Dir(DirToken::Repeat) => self.parse_repeat_directive(),
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

	/// Parse [`#REPEAT`](DirToken::Repeat) [`DataDirective`]s
	fn parse_repeat_directive<'r>(&'r mut self) -> Result<DataDirective<'s>, ParseError> {
		let amount = self.parse_literal()?;

		match amount {
			Literal::Char(_) => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(self.prev())),
					fnd:      self.prev().t.to_string(),
					ex:       "IMMEDIATE".to_string(),
				});
			},
			Literal::String(_) => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(self.prev())),
					fnd:      self.prev().t.to_string(),
					ex:       "IMMEDIATE".to_string(),
				});
			},
			_ => (),
		}

		let peek = self.peek()?;
		let argument = match &peek.t {
			TokenType::Dir(_) => RepeatedData::Directive(self.parse_datadirective()?),
			TokenType::Inst(_) => RepeatedData::Instruction(self.parse_instruction()?),
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
					fnd:      peek.t.to_string(),
					ex:       "DIRECTIVE or INSTRUCTION".to_string(),
				});
			},
		};

		Ok(DataDirective::Repeat { amount, argument: Box::new(argument) })
	}
}
