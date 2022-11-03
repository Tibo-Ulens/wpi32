use std::iter::Peekable;
use std::str::Chars;

mod identifier;
mod literal;
mod token;

use common::{Error, LexError};
pub(crate) use token::{Token, TokenType};

pub(crate) struct Lexer<'s> {
	source:      &'s str,
	source_iter: Peekable<Chars<'s>>,
	len:         usize,

	start: usize,
	idx:   usize,

	line: usize,
	col:  usize,

	prev_nl: usize,
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Token<'s>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.lex_token() {
			None => None,
			Some(res) => {
				match res {
					Ok(t) => Some(t),
					Err(e) => {
						// If a lex error occurs, keep searching so it can
						// print all potential errors, but don't return any
						// more lexemes
						error!("{}", e);

						while let Some(res) = self.lex_token() {
							if let Err(e) = res {
								error!("{}", e);
							}
						}

						None
					},
				}
			},
		}
	}
}

impl<'s> Lexer<'s> {
	pub(crate) fn new(source: &'s str) -> Self {
		Self {
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

		&self.source[self.prev_nl..next_nl]
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

	/// Checks whether a character is a valid identifier start character
	///
	/// Identifiers follow the regex
	/// `[a-zA-Z!$&?^_~@.][a-zA-Z!$&?^_~@.0-9:]*`
	fn is_identifier_start(c: &char) -> bool {
		c.is_alphabetic()
			|| *c == '!' || *c == '$'
			|| *c == '&' || *c == '?'
			|| *c == '^' || *c == '_'
			|| *c == '~' || *c == '@'
			|| *c == '.'
	}

	/// Checks whether a character is a valid identifier character
	///
	/// Identifiers follow the regex
	/// `[a-zA-Z!$&?^_~@.][a-zA-Z!$&?^_~@.0-9:]*`
	fn is_identifier(c: &char) -> bool {
		Self::is_identifier_start(c) || c.is_ascii_digit() || *c == ':'
	}

	/// Checks whether a character is valid inside any binary, octal, decimal,
	/// or hexadecimal number, including their radix identifiers (0b, 0o, 0x)
	fn is_digit_or_radix(c: &char) -> bool {
		c.is_ascii_hexdigit() || *c == 'x' || *c == 'X' || *c == 'o' || *c == 'O'
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
			'\n' => {
				self.line += 1;
				self.col = 0;
				self.prev_nl = self.idx;

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
			',' => Ok(self.make_token(TokenType::SymComma)),
			'(' => Ok(self.make_token(TokenType::SymLeftParen)),
			')' => Ok(self.make_token(TokenType::SymRightParen)),
			'[' => Ok(self.make_token(TokenType::SymLeftBracket)),
			']' => Ok(self.make_token(TokenType::SymRightBracket)),
			'|' => Ok(self.make_token(TokenType::OperatorOr)),
			'^' => Ok(self.make_token(TokenType::OperatorXor)),
			'&' => Ok(self.make_token(TokenType::OperatorAnd)),
			'+' => Ok(self.make_token(TokenType::OperatorPlus)),
			'-' => Ok(self.make_token(TokenType::OperatorMinus)),
			'*' => Ok(self.make_token(TokenType::OperatorMul)),
			'/' => Ok(self.make_token(TokenType::OperatorDiv)),
			'%' => Ok(self.make_token(TokenType::OperatorRem)),
			'=' => {
				match self.next()? {
					'=' => Ok(self.make_token(TokenType::OperatorEq)),
					c => {
						Err(LexError::UnexpectedSymbol {
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
					'=' => Ok(self.make_token(TokenType::OperatorNeq)),
					c => {
						Err(LexError::UnexpectedSymbol {
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
						Ok(self.make_token(TokenType::OperatorLte))
					},
					'<' => {
						self.next()?;
						Ok(self.make_token(TokenType::OperatorLsl))
					},
					_ => Ok(self.make_token(TokenType::OperatorLt)),
				}
			},
			'>' => {
				match self.peek()? {
					'=' => {
						self.next()?;
						Ok(self.make_token(TokenType::OperatorGte))
					},
					'>' => {
						self.next()?;

						match self.peek()? {
							'>' => {
								self.next()?;
								Ok(self.make_token(TokenType::OperatorAsr))
							},
							_ => Ok(self.make_token(TokenType::OperatorLsr)),
						}
					},
					_ => Ok(self.make_token(TokenType::OperatorGt)),
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
				let num = match self.try_take_number(Self::is_digit_or_radix) {
					Ok(n) => n,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.make_token(TokenType::LitNum(num)))
			},
			c if Self::is_identifier_start(&c) => {
				let raw = match self.take_while(Self::is_identifier) {
					Ok(id) => id,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.match_identifier(raw))
			},
			c => {
				Err(LexError::RawUnexpectedSymbol {
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
