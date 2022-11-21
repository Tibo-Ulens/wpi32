//! [`Parser`] functions to parse [`Instruction`]s

use common::ParseError;

use super::ast::Instruction;
use super::Parser;

impl<'s> Parser<'s> {
	/// Parse any valid [`Instruction`]
	///
	/// Assumes the current [`Token`] has [`TokenType`] [`TokenType::Inst`]
	pub(super) fn parse_instruction<'r>(&'r mut self) -> Result<Instruction<'s>, ParseError> {
		todo!()
	}
}
