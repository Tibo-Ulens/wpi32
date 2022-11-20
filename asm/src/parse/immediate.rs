//! [`Parser`] functions to parse [`Immediate`] expressions

use common::ParseError;

use super::ast::Immediate;
use super::Parser;

impl<'s> Parser<'s> {
	/// Recursively parse an immediate expresion
	pub(super) fn parse_immediate<'r>(&'r mut self) -> Result<Immediate<'s>, ParseError> { todo!() }
}
