//! [`Parser`] functions to parse [`Immediate`] expressions

use super::ast::Immediate;
use super::Parser;
use crate::error::{LocationInfo, ParseError};
use crate::lex::{OpToken, Token, TokenType};

/// Parser specifically to convert immediates into RPN
struct ImmediateParser<'i, 's> {
	/// Reference to the top level parser, used to get the source file name
	/// for errors
	parser:    &'i Parser<'s>,
	/// The token slice containing the immediate
	imm_slice: &'i [Token<'s>],
}

impl<'i, 's> ImmediateParser<'i, 's> {
	/// Creates a new parser for the given token slice
	///
	/// Assumes the token slice only contains valid immediate tokens
	fn new(imm_slice: &'i [Token<'s>], parser: &'i Parser<'s>) -> Self {
		Self { parser, imm_slice }
	}

	/// Assert that the parentheses in the slice of tokens are balanced
	fn check_parens_balanced(&self) -> Result<(), ParseError> {
		let mut paren_stack: Vec<Token> = vec![];

		for &token in self.imm_slice {
			if token.t == TokenType::SymLeftParen {
				paren_stack.push(token);
			} else if token.t == TokenType::SymRightParen {
				if paren_stack.is_empty() {
					return Err(ParseError::UnmatchedCloseParenthesis {
						src_file: self.parser.source_file.to_string(),
						location: Box::new(LocationInfo::from(&token)),
					});
				} else {
					// Unwrap is safe as stack is not empty
					let popped = paren_stack.pop().unwrap();

					if popped.t == token.t {
						return Err(ParseError::UnmatchedCloseParenthesis {
							src_file: self.parser.source_file.to_string(),
							location: Box::new(LocationInfo::from(&token)),
						});
					}
				}
			}
		}

		if !(paren_stack.is_empty()) {
			// Unwrap is safe as stack is not empty
			let popped = paren_stack.pop().unwrap();

			if popped.t == TokenType::SymRightParen {
				return Err(ParseError::UnmatchedCloseParenthesis {
					src_file: self.parser.source_file.to_string(),
					location: Box::new(LocationInfo::from(&popped)),
				});
			}

			// If the paren_stack is not empty then the slice must have at
			// least 1 element, so unwrap is safe
			return Err(ParseError::UnclosedParenthesis {
				src_file:       self.parser.source_file.to_string(),
				close_location: Box::new(LocationInfo::from(self.imm_slice.last().unwrap())),
				open_location:  Box::new(LocationInfo::from(&popped)),
			});
		}

		Ok(())
	}

	/// Parse the slice of tokens into an immediate in reverse polish notation
	///
	/// TODO: this is quite ugly and can probably be cleaned up a bit
	///
	/// Assumes the immediate slice contains balanced parentheses
	fn parse(&mut self) -> Result<Vec<Token<'s>>, ParseError> {
		self.check_parens_balanced()?;

		let mut rpn_stack: Vec<Token> = vec![];
		let mut op_stack: Vec<OpToken> = vec![];
		let mut op_stack_: Vec<Token> = vec![];
		let mut prev_was_operator = true;

		for &token in self.imm_slice {
			match &token.t {
				TokenType::LitNum(_) | TokenType::Identifier(_) => {
					prev_was_operator = false;
					rpn_stack.push(token);
				},
				TokenType::SymLeftParen => {
					prev_was_operator = true;
					op_stack.push(OpToken::LeftParen);
					op_stack_.push(token);
				},
				TokenType::SymRightParen => {
					'paren_loop: while let Some(op_peek) = op_stack.last() {
						// Unwraps are safe as op_stack is not empty
						if op_peek == &OpToken::LeftParen {
							op_stack.pop().unwrap();
							op_stack_.pop().unwrap();

							break 'paren_loop;
						} else {
							op_stack.pop().unwrap();
							let top = op_stack_.pop().unwrap();

							rpn_stack.push(top);
						}
					}

					prev_was_operator = false;
				},
				TokenType::Op(mut operator) if operator.is_al_operator() => {
					if prev_was_operator && operator == OpToken::Minus {
						operator = OpToken::UnaryMinus;
					}

					'op_loop: while let Some(op_peek) = op_stack.last() {
						if op_peek == &OpToken::LeftParen {
							break 'op_loop;
						}

						let curr_prec = operator.get_precedence();
						let prev_prec = op_peek.get_precedence();

						if curr_prec > prev_prec
							|| (curr_prec == prev_prec && operator.is_right_associative())
						{
							break 'op_loop;
						} else if curr_prec < prev_prec
							|| (curr_prec == prev_prec && !(operator.is_right_associative()))
						{
							// Unwraps are safe as op_stack is not empty
							op_stack.pop().unwrap();
							let top_ = op_stack_.pop().unwrap();

							rpn_stack.push(top_);
						}
					}

					prev_was_operator = true;
					op_stack.push(operator);
					op_stack_.push(token);
				},
				_ => todo!(),
			}
		}

		while !(op_stack.is_empty()) {
			// Unwraps are safe as op_stack is not empty
			op_stack.pop().unwrap();
			let token = op_stack_.pop().unwrap();

			rpn_stack.push(token);
		}

		Ok(rpn_stack)
	}
}

impl<'s> Parser<'s> {
	/// Parse an immediate expression into a list of tokens encoding the same
	/// immediate but in
	/// [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
	pub(super) fn parse_immediate<'r>(&'r mut self) -> Result<Immediate<'s>, ParseError> {
		let start = self.idx;
		while let Ok(peek) = self.peek() {
			match &peek.t {
				TokenType::LitNum(_)
				| TokenType::Identifier(_)
				| TokenType::SymLeftParen
				| TokenType::SymRightParen
				| TokenType::Op(_) => {
					// Unwrap is safe as peek is Ok
					self.next().unwrap();
				},
				_ => break,
			}
		}
		let end = self.idx;

		let imm_slice = &self.stream[start..end];
		let mut imm_parser = ImmediateParser::new(imm_slice, self);
		let rpn_tokens = imm_parser.parse()?;

		Ok(Immediate { rpn_tokens })
	}
}
