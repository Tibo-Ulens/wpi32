//! # Lexer
//!
//! The lexer is responsible for converting the input text file from a raw
//! string of characters into a stream of [`Token`]s that are easier to work
//! with, as well as handling any possible escape sequences, whitespace, and
//! (some) bracket matching, and recognizing lexicographical errors
//!
//! ### Usage
//! ```rust
//! let src_file_name = "/foo/bar/baz.asm";
//! let src_file_path = PathBuf::from(&src_file_name);
//!
//! let mut file = File::open(src_file_path)?;
//! let mut contents = String::new();
//! file.read_to_string(&mut contents)?;
//!
//! let lexer = Lexer::new(&src_file_path_name, &contents);
//! let tokens: Vec<Token> = lexer.into_iter().collect::<Result<Vec<Token>, Error>>()?;
//! ```

use std::iter::Peekable;
use std::str::Chars;

mod identifier;
mod literal;
mod token;

pub use token::*;

use crate::error::{Error, LexError};

/// Main Lexer type
///
/// Wraps all internal state during lexing and provides a namespace for all
/// lexer-related functions
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed as (most) tokens
///    containing string literals will contain references instead of owned data
pub struct Lexer<'s> {
	/// The name of the file being parsed (used for error messages)
	pub(crate) source_file: &'s str,
	/// The string of source code (used for error messages)
	source:                 &'s str,
	/// An iterator over the characters of the source code
	source_iter:            Peekable<Chars<'s>>,
	/// The length of the source code (in [`char`]s)
	len:                    usize,

	/// The start of the current token
	start: usize,
	/// The current index into the source file
	idx:   usize,

	/// The current line position of the lexer
	pub line: usize,
	/// The current column position of the lexer
	pub col:  usize,

	/// The index of the previous newline character (used for errors)
	prev_nl: usize,
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Result<Token<'s>, Error>;

	fn next(&mut self) -> Option<Self::Item> { self.lex_token() }
}

impl<'s> Lexer<'s> {
	/// Create a new lexer given a source file name and a string of source code
	pub fn new(source_file: &'s str, source: &'s str) -> Self {
		Self {
			source_file,
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

	/// Peek at the next [`char`]
	///
	/// Returns [`None`] if no characters are left
	fn peek(&mut self) -> Option<&char> { self.source_iter.peek() }

	/// Consume and return the next [`char`]
	///
	/// Returns [`None`] if no characters are left
	fn next(&mut self) -> Option<char> {
		self.idx += 1;
		self.source_iter.next()
	}

	/// Get a reference to the current working line of source code
	fn get_curr_line(&self) -> &'s str {
		let next_nl =
			self.source[self.prev_nl..].find('\n').unwrap_or(self.source.len()) + self.prev_nl;

		&self.source[self.prev_nl..=next_nl]
	}

	/// Make a [`Token`] given the [`Lexer`]s current state and a [`TokenType`]
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
	/// start of the current token up to, and including, the last character
	/// that satisfied the predicate
	fn take_while<F>(&mut self, pred: F) -> Result<&'s str, LexError>
	where
		F: Fn(char) -> bool,
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
		while pred(peek) {
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

	/// Consume any available whitespace characters, updating the [`Lexer`]s
	/// state as it goes along
	///
	/// Returns [`None`] if no characters are left
	///
	/// TODO: handle tabs not being 1 char wide
	fn take_whitespace(&mut self) -> Option<()> {
		match self.peek()? {
			' ' | '\t' => {
				self.col += 1;

				// Unwrap is safe as peek is some
				self.next().unwrap();

				self.take_whitespace()
			},
			_ => Some(()),
		}
	}

	/// Lex a single [`Token`]
	///
	/// Returns [`None`] if the iterator has ended <br>
	/// Returns [`Error`] if a lexical error was found
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
				let comment = match self.take_while(|c| c != '\n') {
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
			'{' => Ok(self.make_token(TokenType::SymLeftBrace)),
			'}' => Ok(self.make_token(TokenType::SymRightBrace)),
			'?' => Ok(self.make_token(TokenType::Op(OpToken::Question))),
			':' => Ok(self.make_token(TokenType::Op(OpToken::Colon))),
			'$' => Ok(self.make_token(TokenType::Op(OpToken::Dollar))),
			'|' => {
				match self.peek()? {
					'|' => {
						self.next().unwrap(); // Unwrap is safe as peek is some
						Ok(self.make_token(TokenType::Op(OpToken::LogicOr)))
					},
					_ => Ok(self.make_token(TokenType::Op(OpToken::BitOr))),
				}
			},
			'^' => {
				match self.peek()? {
					'^' => {
						self.next().unwrap(); // Unwrap is safe as peek is some
						Ok(self.make_token(TokenType::Op(OpToken::LogicXor)))
					},
					_ => Ok(self.make_token(TokenType::Op(OpToken::BitXor))),
				}
			},
			'&' => {
				match self.peek()? {
					'&' => {
						self.next().unwrap(); // Unwrap is safe as peek is some
						Ok(self.make_token(TokenType::Op(OpToken::LogicAnd)))
					},
					_ => Ok(self.make_token(TokenType::Op(OpToken::BitAnd))),
				}
			},
			'+' => Ok(self.make_token(TokenType::Op(OpToken::Plus))),
			'-' => Ok(self.make_token(TokenType::Op(OpToken::Minus))),
			'*' => Ok(self.make_token(TokenType::Op(OpToken::Star))),
			'/' => Ok(self.make_token(TokenType::Op(OpToken::Slash))),
			'%' => Ok(self.make_token(TokenType::Op(OpToken::Percent))),
			'=' => {
				match self.next()? {
					'=' => Ok(self.make_token(TokenType::Op(OpToken::Eq))),
					'>' => Ok(self.make_token(TokenType::SymFatArrow)),
					c => {
						Err(LexError::UnexpectedSymbol {
							src_file: self.source_file.to_string(),
							line:     self.line,
							col:      self.col,
							src_line: self.get_curr_line().to_string(),
							found:    c,
							expected: '=',
						})
					},
				}
			},
			'!' => {
				match self.peek()? {
					'=' => {
						self.next().unwrap(); // Unwrap is safe as peek is some
						Ok(self.make_token(TokenType::Op(OpToken::Neq)))
					},
					_ => Ok(self.make_token(TokenType::Op(OpToken::Exclamation))),
				}
			},
			'~' => Ok(self.make_token(TokenType::Op(OpToken::BitNot))),
			'<' => {
				match self.peek()? {
					'=' => {
						self.next()?;
						Ok(self.make_token(TokenType::Op(OpToken::Lte)))
					},
					'<' => {
						self.next()?;
						Ok(self.make_token(TokenType::Op(OpToken::Lsl)))
					},
					_ => Ok(self.make_token(TokenType::Op(OpToken::Lt))),
				}
			},
			'>' => {
				match self.peek()? {
					'=' => {
						self.next()?;
						Ok(self.make_token(TokenType::Op(OpToken::Gte)))
					},
					'>' => {
						self.next()?;

						match self.peek()? {
							'>' => {
								self.next()?;
								Ok(self.make_token(TokenType::Op(OpToken::Asr)))
							},
							_ => Ok(self.make_token(TokenType::Op(OpToken::Lsr))),
						}
					},
					_ => Ok(self.make_token(TokenType::Op(OpToken::Gt))),
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
				let num = match self.try_take_number() {
					Ok(n) => n,
					Err(e) => return Some(Err(e.into())),
				};

				Ok(self.make_token(TokenType::LitNum(num)))
			},
			c if unicode_ident::is_xid_start(c) || c == '#' || c == '_' || c == '.' => {
				let raw = match self.take_while(|c| unicode_ident::is_xid_continue(c) || c == '_') {
					Ok(id) => id,
					Err(e) => return Some(Err(e.into())),
				};

				self.match_identifier(raw)
			},
			c => {
				Err(LexError::RawUnexpectedSymbol {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					src_line: self.get_curr_line().to_string(),
					found:    c,
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
