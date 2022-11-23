//! AST immediate type definitions

use std::fmt::{Display, Formatter, Result};

use crate::lex::Token;

/// An immediate value
///
/// This can range from a single number to a complex expression referencing
/// labels and constants
///
/// *EBNF not given as it is too chonky, look at the docs folder for grammar*
#[derive(Clone, Debug)]
pub struct Immediate<'s> {
	/// The tokens making up this immediate, parsed into
	/// [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
	pub rpn_tokens: Vec<Token<'s>>,
}

impl<'s> Display for Immediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let repr =
			self.rpn_tokens.iter().map(|t| t.t.to_string()).collect::<Vec<String>>().join(" ");

		write!(f, "{}", repr)
	}
}
