//! [`Parser`] functions to parse macro expressions

use super::ast::{
	MacroArgType,
	MacroDefinition,
	MacroInvocation,
	MacroMatch,
	MacroRule,
	MacroVarType,
};
use super::Parser;
use crate::error::{LocationInfo, ParseError};
use crate::lex::{OpToken, Token, TokenType};

impl<'s> Parser<'s> {
	/// Parse a [`MacroDefinition`] consisting of:
	///  - the `define_macro!` call
	///  - an [`Identifier`]
	///  - a [`MacroBody`]
	///
	/// Assumes the current token has [`TokenType`] [`TokenType::Identifier`]
	pub(super) fn parse_macro_definition<'r>(
		&'r mut self,
	) -> Result<MacroDefinition<'s>, ParseError> {
		// Consume the macro_define token
		// Unwrap is assumed to be safe
		assert_eq!(self.next().unwrap().t, TokenType::Identifier("define_macro"));

		// Consume the `!` specifying this as a macro call
		self.expect(TokenType::Op(OpToken::Exclamation))?;

		let id_token = self.next()?;
		let id = match id_token.t {
			TokenType::Identifier(id) => id,
			_ => {
				return Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(id_token)),
					found:    id_token.t.to_string(),
					expected: "IDENTIFIER".to_string(),
				});
			},
		};

		let body_open = self.paren_match_start()?;
		self.optional(TokenType::SymNewline);

		let mut rules = vec![];

		let first_rule = self.parse_macro_rule()?;
		rules.push(first_rule);

		while let Ok(peek) = self.peek() && peek.t == TokenType::SymComma {
			// Unwrap is safe as peek is [`Ok`]
			self.next().unwrap();
			self.optional(TokenType::SymNewline);

			let extra_rule = self.parse_macro_rule()?;
			rules.push(extra_rule);
		}

		self.optional(TokenType::SymNewline);

		self.paren_match_end(body_open)?;

		Ok(MacroDefinition { name: id, rules })
	}

	/// Parse a single [`MacroRule`] consisting of:
	///  - a list of [`MacroMatch`]es
	///  - the `=>` symbol
	///  - a list of transcribing [`Statement`]s
	fn parse_macro_rule<'r>(&'r mut self) -> Result<MacroRule<'s>, ParseError> {
		let mut matcher = vec![];
		let mut transcriber = vec![];

		let match_open = self.paren_match_start()?;

		while self.paren_match_end(match_open).is_err() {
			let r#match = self.parse_macro_match()?;
			matcher.push(r#match);
		}

		self.expect(TokenType::SymFatArrow)?;

		let transc_open = self.paren_match_start()?;

		while self.paren_match_end(transc_open).is_err() {
			transcriber.push(self.next()?.t);
		}

		Ok(MacroRule { matcher, transcriber })
	}

	/// Parse a single [`MacroMatch`] which is either a raw string of
	/// characters, some typed and named argument, or a variadic list of
	/// arguments
	fn parse_macro_match<'r>(&'r mut self) -> Result<MacroMatch<'s>, ParseError> {
		let t_or_dollar = self.next()?;
		if t_or_dollar.t == TokenType::Op(OpToken::Dollar) {
			let id_or_paren = self.next()?;
			match id_or_paren.t {
				TokenType::Identifier(id) => {
					self.expect(TokenType::Op(OpToken::Colon))?;
					let arg_type = self.parse_macro_arg_type()?;

					Ok(MacroMatch::Typed { id, arg_type })
				},
				TokenType::SymLeftParen => {
					let mut matches = vec![];
					matches.push(self.parse_macro_match()?);

					while self.paren_match_end(*id_or_paren).is_err() {
						matches.push(self.parse_macro_match()?);
					}

					let rep_sep = match self.peek()?.t {
						TokenType::Op(OpToken::Plus)
						| TokenType::Op(OpToken::Star)
						| TokenType::Op(OpToken::Question) => None,
						t => {
							// Unwrap is safe as peek is [`Ok`]
							self.next().unwrap();
							Some(t)
						},
					};

					let var_type = self.parse_macro_var_type()?;

					Ok(MacroMatch::Variadic { matches, rep_sep, var_type })
				},
				_ => {
					Err(ParseError::UnexpectedToken {
						src_file: self.source_file.to_string(),
						location: Box::new(LocationInfo::from(id_or_paren)),
						found:    id_or_paren.t.to_string(),
						expected: "IDENTIFIER or (".to_string(),
					})
				},
			}
		} else {
			Ok(MacroMatch::Raw(t_or_dollar.t))
		}
	}

	/// Parse a [type specifier](MacroArgType) for a macro argument
	fn parse_macro_arg_type(&mut self) -> Result<MacroArgType, ParseError> {
		let specifier = self.next()?;

		match specifier.t {
			TokenType::Identifier("inst") => Ok(MacroArgType::Inst),
			TokenType::Identifier("reg") => Ok(MacroArgType::Reg),
			TokenType::Identifier("dir") => Ok(MacroArgType::Dir),
			TokenType::Identifier("ident") => Ok(MacroArgType::Ident),
			TokenType::Identifier("imm") => Ok(MacroArgType::Imm),
			TokenType::Identifier("stmt") => Ok(MacroArgType::Stmt),
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(specifier)),
					found:    specifier.t.to_string(),
					expected: "IDENTIFIER".to_string(),
				})
			},
		}
	}

	/// Parse a [variadic specifier](MacroVarType) for a macro argument
	fn parse_macro_var_type(&mut self) -> Result<MacroVarType, ParseError> {
		let specifier = self.next()?;

		match specifier.t {
			TokenType::Op(OpToken::Question) => Ok(MacroVarType::Optional),
			TokenType::Op(OpToken::Plus) => Ok(MacroVarType::OneOrMore),
			TokenType::Op(OpToken::Star) => Ok(MacroVarType::Any),
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(specifier)),
					found:    specifier.t.to_string(),
					expected: "? or + or *".to_string(),
				})
			},
		}
	}

	/// Parse a [`MacroInvocation`] consisting of:
	///  - an [`Identifier`]
	///  - an exclamation mark
	///  - any pair of parentheses
	///  - any amount of arguments
	///
	/// Assumes the current token has [`TokenType`]
	/// [`TokenType::Op(OpToken::Exclamation`]
	///
	/// Takes the identifier as an argument
	pub(super) fn parse_macro_invocation<'r>(
		&'r mut self,
		id: &'s str,
	) -> Result<MacroInvocation<'s>, ParseError> {
		self.expect(TokenType::Op(OpToken::Exclamation))?;

		let open = self.paren_match_start()?;

		let mut args = vec![];
		while self.paren_match_end(open).is_err() {
			args.push(self.next()?.t);
		}

		Ok(MacroInvocation { name: id, args })
	}

	/// Attempt to detect and return any opening paren-style token
	/// (parentheses, brackets, and braces)
	fn paren_match_start<'r>(&'r mut self) -> Result<Token<'s>, ParseError> {
		let peek = self.peek()?;

		match peek.t {
			TokenType::SymLeftParen => {
				self.next().unwrap();
				Ok(*peek)
			},
			TokenType::SymLeftBracket => {
				self.next().unwrap();
				Ok(*peek)
			},
			TokenType::SymLeftBrace => {
				self.next().unwrap();
				Ok(*peek)
			},
			_ => {
				Err(ParseError::UnexpectedToken {
					src_file: self.source_file.to_string(),
					location: Box::new(LocationInfo::from(peek)),
					found:    peek.t.to_string(),
					expected: "( or [ or {".to_string(),
				})
			},
		}
	}

	/// Given an open paren-style token, attempt to detect the matching closing
	/// token
	fn paren_match_end<'r>(&'r mut self, open: Token<'s>) -> Result<(), ParseError> {
		match open.t {
			TokenType::SymLeftParen => {
				let peek = self.peek()?;
				match peek.t {
					TokenType::SymRightParen => {
						self.next().unwrap();
						Ok(())
					},
					_ => {
						Err(ParseError::UnclosedDelimiter {
							src_file:       self.source_file.to_string(),
							delim_type:     "parenthesis".to_string(),
							found:          peek.t.to_string(),
							close_location: Box::new(LocationInfo::from(peek)),
							open_location:  Box::new(LocationInfo::from(&open)),
						})
					},
				}
			},
			TokenType::SymLeftBracket => {
				let peek = self.peek()?;
				match peek.t {
					TokenType::SymRightBracket => {
						self.next().unwrap();
						Ok(())
					},
					_ => {
						Err(ParseError::UnclosedDelimiter {
							src_file:       self.source_file.to_string(),
							delim_type:     "bracket".to_string(),
							found:          peek.t.to_string(),
							close_location: Box::new(LocationInfo::from(peek)),
							open_location:  Box::new(LocationInfo::from(&open)),
						})
					},
				}
			},
			TokenType::SymLeftBrace => {
				let peek = self.peek()?;
				match peek.t {
					TokenType::SymRightBrace => {
						self.next().unwrap();
						Ok(())
					},
					_ => {
						Err(ParseError::UnclosedDelimiter {
							src_file:       self.source_file.to_string(),
							delim_type:     "brace".to_string(),
							found:          peek.t.to_string(),
							close_location: Box::new(LocationInfo::from(peek)),
							open_location:  Box::new(LocationInfo::from(&open)),
						})
					},
				}
			},
			_ => unreachable!(),
		}
	}
}
