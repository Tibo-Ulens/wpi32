use std::iter::Peekable;
use std::str::Chars;

mod identifier;
mod literal;
mod token;
mod util;

use common::{Error, LexError};
pub(crate) use token::{OperatorToken, RegisterToken, Token, TokenType};

pub(crate) struct Lexer<'s> {
	pub(crate) source_file: String,
	source:                 &'s str,
	source_iter:            Peekable<Chars<'s>>,
	len:                    usize,

	start: usize,
	idx:   usize,

	line: usize,
	col:  usize,

	prev_nl: usize,
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Result<Token<'s>, Error>;

	fn next(&mut self) -> Option<Self::Item> { self.lex_token() }
}

impl<'s> Lexer<'s> {
	pub(crate) fn new(src_file: &str, source: &'s str) -> Self {
		Self {
			source_file: src_file.to_string(),
			source,
			source_iter: source.chars().peekable(),
			len: source.chars().count(),
			start: 0,
			idx: 0,
			line: 1,
			col: 1,
			prev_nl: 0,
		}
	}

	/// Peek at the next character
	fn peek(&mut self) -> Option<&char> { self.source_iter.peek() }

	/// Consume and return the next character
	fn next(&mut self) -> Option<char> {
		self.idx += 1;
		self.source_iter.next()
	}

	/// Get the current working line of source code
	fn get_curr_line(&self) -> &'s str {
		let next_nl =
			self.source[self.prev_nl..].find('\n').unwrap_or(self.source.len()) + self.prev_nl;

		&self.source[self.prev_nl..=next_nl]
	}

	/// Make a token given the lexers current state
	fn make_token(&self, t: TokenType<'s>) -> Token<'s> {
		Token {
			t,
			line: self.line,
			col: self.col,
			span: self.idx - self.start,
			source_line: self.get_curr_line(),
		}
	}

	/// Keep taking characters while a predicate holds true
	///
	/// Returns the slice of characters that satisfied the predicate, from the
	/// start of the current token up to the last character that satisfied the
	/// predicate
	fn take_while<F>(&mut self, pred: F) -> Result<&'s str, LexError>
	where
		F: for<'a> Fn(&'a char) -> bool,
	{
		// Return early if the immediately following character is None
		let mut peek = match self.peek() {
			Some(p) => *p,
			None => {
				return Err(LexError::UnexpectedEof {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					src_line: self.get_curr_line().to_string(),
				});
			},
		};

		// Columns start at 1
		let mut i = 1;
		while pred(&peek) {
			// Unwrap is safe as the previous iteration of the loop assures
			// there is a character
			self.next().unwrap();

			if self.idx >= self.len {
				return Err(LexError::UnexpectedEof {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + i,
					src_line: self.get_curr_line().to_string(),
				});
			}

			// Unwrap is safe as idx < len
			peek = *self.peek().unwrap();
			i += 1;
		}

		Ok(&self.source[self.start..self.idx])
	}

	/// Consume any available whitespace characters, updating the lexers state
	/// as it goes along
	///
	/// Returns [`None`] if no characters are left in the source iterator
	fn take_whitespace(&mut self) -> Option<()> {
		match self.peek()? {
			' ' | '\t' => {
				self.col += 1;

				self.next().unwrap();

				self.take_whitespace()
			},
			_ => Some(()),
		}
	}

	/// Lex a single token
	///
	/// Returns [`None`] if the iterator has ended, else returns a [`Token`] or an [`Error`]
	fn lex_token(&mut self) -> Option<Result<Token<'s>, Error>> {
		// Consume any leading whitespace
		self.take_whitespace()?;

		// take_whitespace updates self.idx, so self.start should be updated
		// accordingly to mark the start of a new token
		self.start = self.idx;

		let token = match self.next()? {
			'\n' => {
				// Token must be made before updating state so the it has
				// correct line and col values
				let token = self.make_token(TokenType::SymNewline);

				self.line += 1;
				// 0 instead of 1 because col gets incremented at the end of
				// the match
				self.col = 0;
				self.prev_nl = self.idx;

				Ok(token)
			},
			';' => {
				let comment = match self.take_while(|&c| c != '\n') {
					Ok(cmt) => cmt,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.make_token(TokenType::Comment(comment)))
			},
			',' => Ok(self.make_token(TokenType::SymComma)),
			'(' => Ok(self.make_token(TokenType::SymLeftParen)),
			')' => Ok(self.make_token(TokenType::SymRightParen)),
			'[' => Ok(self.make_token(TokenType::SymLeftBracket)),
			']' => Ok(self.make_token(TokenType::SymRightBracket)),
			'?' => Ok(self.make_token(TokenType::Op(OperatorToken::TernStart))),
			':' => Ok(self.make_token(TokenType::Op(OperatorToken::TernAlt))),
			'|' => {
				match self.peek()? {
					'|' => Ok(self.make_token(TokenType::Op(OperatorToken::LogicOr))),
					_ => Ok(self.make_token(TokenType::Op(OperatorToken::Or))),
				}
			},
			'^' => {
				match self.peek()? {
					'^' => Ok(self.make_token(TokenType::Op(OperatorToken::LogicXor))),
					_ => Ok(self.make_token(TokenType::Op(OperatorToken::Xor))),
				}
			},
			'&' => {
				match self.peek()? {
					'&' => Ok(self.make_token(TokenType::Op(OperatorToken::LogicAnd))),
					_ => Ok(self.make_token(TokenType::Op(OperatorToken::And))),
				}
			},
			'+' => Ok(self.make_token(TokenType::Op(OperatorToken::Plus))),
			'-' => Ok(self.make_token(TokenType::Op(OperatorToken::Minus))),
			'*' => Ok(self.make_token(TokenType::Op(OperatorToken::Mul))),
			'/' => Ok(self.make_token(TokenType::Op(OperatorToken::Div))),
			'%' => Ok(self.make_token(TokenType::Op(OperatorToken::Rem))),
			'=' => {
				match self.next()? {
					'=' => Ok(self.make_token(TokenType::Op(OperatorToken::Eq))),
					c => {
						Err(LexError::UnexpectedSymbol {
							src_file: self.source_file.to_string(),
							line:     self.line,
							col:      self.col,
							src_line: self.get_curr_line().to_string(),
							fnd:      c,
							ex:       '=',
						})
					},
				}
			},
			'!' => {
				match self.next()? {
					'=' => Ok(self.make_token(TokenType::Op(OperatorToken::Neq))),
					c => {
						Err(LexError::UnexpectedSymbol {
							src_file: self.source_file.to_string(),
							line:     self.line,
							col:      self.col,
							src_line: self.get_curr_line().to_string(),
							fnd:      c,
							ex:       '=',
						})
					},
				}
			},
			'<' => {
				match self.peek()? {
					'=' => {
						self.next()?;
						Ok(self.make_token(TokenType::Op(OperatorToken::Lte)))
					},
					'<' => {
						self.next()?;
						Ok(self.make_token(TokenType::Op(OperatorToken::Lsl)))
					},
					_ => Ok(self.make_token(TokenType::Op(OperatorToken::Lt))),
				}
			},
			'>' => {
				match self.peek()? {
					'=' => {
						self.next()?;
						Ok(self.make_token(TokenType::Op(OperatorToken::Gte)))
					},
					'>' => {
						self.next()?;

						match self.peek()? {
							'>' => {
								self.next()?;
								Ok(self.make_token(TokenType::Op(OperatorToken::Asr)))
							},
							_ => Ok(self.make_token(TokenType::Op(OperatorToken::Lsr))),
						}
					},
					_ => Ok(self.make_token(TokenType::Op(OperatorToken::Gt))),
				}
			},
			'\'' => {
				let raw = match self.try_take_char() {
					Ok(c) => c,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.make_token(TokenType::LitChar(raw)))
			},
			'"' => {
				let raw = match self.try_take_string() {
					Ok(s) => s,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.make_token(TokenType::LitStr(raw)))
			},
			n if n.is_ascii_digit() => {
				let num = match self.try_take_number(util::is_digit_or_radix) {
					Ok(n) => n,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.make_token(TokenType::LitNum(num)))
			},
			c if util::is_identifier_start(&c) => {
				let raw = match self.take_while(util::is_identifier) {
					Ok(id) => id,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.match_identifier(raw))
			},
			c => {
				Err(LexError::RawUnexpectedSymbol {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					src_line: self.get_curr_line().to_string(),
					fnd:      c,
				})
			},
		};

		// New column = previous column + length of the token
		self.col += self.idx - self.start;

		// Token is Result<Token, LexError>, convert it to
		// Result<Token, Error>
		let token = token.map_err(|e| e.into());

		Some(token)
	}
}
