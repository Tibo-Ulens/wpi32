//! [`Parser`] functions to parse macro expressions

use super::ast::MacroInvocation;
use super::Parser;
use crate::error::ParseError;

impl<'s> Parser<'s> {
	pub(super) fn parse_macro_invocation<'r>(
		&'r mut self,
		id: &'s str,
	) -> Result<MacroInvocation<'s>, ParseError> {
		todo!()
	}
}
