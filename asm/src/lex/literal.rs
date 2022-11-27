//! [`Lexer`] functions to process literals
//!
//! handles the recognition of:
//!  - characters (for [`LitChar`](crate::lex::TokenType::LitChar))
//!  - string (for [`LitStr`](crate::lex::TokenType::LitStr))
//!  - numbers (for [`LitNum`](crate::lex::TokenType::LitNum))

use super::Lexer;
use crate::error::LexError;

impl<'s> Lexer<'s> {
	/// Convert a string with a 2 character escape code into its corresponding character
	fn unescape_string_to_char(&self, string: &str) -> Result<char, LexError> {
		match string {
			"\\n" => Ok('\n'),
			"\\r" => Ok('\r'),
			"\\t" => Ok('\t'),
			"\\\\" => Ok('\\'),
			"\\0" => Ok('\0'),
			"\\'" => Ok('\''),
			_ => {
				Err(LexError::InvalidEscape {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + 1,
					span:     2,
					src_line: self.get_curr_line().to_string(),
				})
			},
		}
	}

	/// Try to read a single character while handling escape sequences
	///
	/// Supported escape sequences:
	///  - `\n` - line feed
	///  - `\r` - carriage return
	///  - `\t` - htab
	///  - `\\` - backslash
	///  - `\0` - null
	///  - `\'` - single quote
	pub(super) fn try_take_char(&mut self) -> Result<char, LexError> {
		// Return early if the immediately following character is None
		let chr = match self.next() {
			Some(c) => c,
			None => {
				return Err(LexError::UnexpectedEof {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + 1,
					src_line: self.get_curr_line().to_string(),
				});
			},
		};

		if chr == '\\' {
			let escaped = match self.next() {
				Some(c) => c,
				None => {
					return Err(LexError::UnexpectedEof {
						src_file: self.source_file.to_string(),
						line:     self.line,
						col:      self.col + 2,
						src_line: self.get_curr_line().to_string(),
					});
				},
			};

			let close = match self.next() {
				Some(c) => c,
				None => {
					return Err(LexError::UnexpectedEof {
						src_file: self.source_file.to_string(),
						line:     self.line,
						col:      self.col + 3,
						src_line: self.get_curr_line().to_string(),
					});
				},
			};

			if close != '\'' {
				return Err(LexError::UnexpectedSymbol {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + 3,
					src_line: self.get_curr_line().to_string(),
					found:    close,
					expected: '\'',
				});
			}

			let mut unescaped_str = String::from(chr);
			unescaped_str.push(escaped);

			return self.unescape_string_to_char(&unescaped_str);
		}

		let close = match self.next() {
			Some(c) => c,
			None => {
				return Err(LexError::UnexpectedEof {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + 2,
					src_line: self.get_curr_line().to_string(),
				});
			},
		};

		if close != '\'' {
			return Err(LexError::UnexpectedSymbol {
				src_file: self.source_file.to_string(),
				line:     self.line,
				col:      self.col + 2,
				src_line: self.get_curr_line().to_string(),
				found:    close,
				expected: '\'',
			});
		}

		Ok(chr)
	}

	/// Read a string until a non-escaped " is encountered while also handling
	/// any escape sequences
	///
	/// Supported escape sequences:
	///  - `\n` - line feed
	///  - `\r` - carriage return
	///  - `\t` - htab
	///  - `\\` - backslash
	///  - `\0` - null
	///  - `\'` - single quote
	pub(super) fn try_take_string(&mut self) -> Result<&'s str, LexError> {
		// Return early if the immediately following character is None
		let mut peek = match self.peek() {
			Some(c) => *c,
			None => {
				return Err(LexError::UnexpectedEof {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + 1,
					src_line: self.get_curr_line().to_string(),
				});
			},
		};

		let mut i = 0;
		let mut prev = ' ';
		// Keep looping until a `"` without a preceding `\` is found
		while !(peek == '"' && prev != '\\') {
			// Unwrap is safe as the previous iteration of the loop assures
			// there is a character
			self.next().unwrap();

			if self.idx >= self.len {
				return Err(LexError::UnexpectedEof {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col + i + 2,
					src_line: self.get_curr_line().to_string(),
				});
			}

			prev = peek;
			// Unwrap is safe as idx < len
			peek = *self.peek().unwrap();
			i += 1;
		}

		// Take the closing quote
		//
		// Unwrap is safe as the last iteration of the loop assures the next
		// character is `"`
		self.next().unwrap();

		// + and - 1 to ignore the quotes
		let string_literal = &self.source[self.start + 1..self.idx - 1];

		Ok(string_literal)
	}

	/// Attempt to make a number starting from the lexers current position
	/// in the source
	///
	/// Can make decimal, hex, octal, or binary numbers depending on the
	/// supplied predicate function
	pub(super) fn try_take_number<F>(&mut self, pred: F) -> Result<isize, LexError>
	where
		F: Fn(char) -> bool,
	{
		let raw = match self.take_while(pred) {
			Ok(n) => n,
			Err(e) => return Err(e),
		};

		let num = if raw.starts_with("0x") {
			isize::from_str_radix(raw.trim_start_matches("0x"), 16).map_err(|_| {
				LexError::InvalidNumber {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					span:     raw.len(),
					src_line: self.get_curr_line().to_string(),
				}
			})
		} else if raw.starts_with("0o") {
			isize::from_str_radix(raw.trim_start_matches("0o"), 8).map_err(|_| {
				LexError::InvalidNumber {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					span:     raw.len(),
					src_line: self.get_curr_line().to_string(),
				}
			})
		} else if raw.starts_with("0b") {
			isize::from_str_radix(raw.trim_start_matches("0b"), 2).map_err(|_| {
				LexError::InvalidNumber {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					span:     raw.len(),
					src_line: self.get_curr_line().to_string(),
				}
			})
		} else {
			raw.parse::<isize>().map_err(|_| {
				LexError::InvalidNumber {
					src_file: self.source_file.to_string(),
					line:     self.line,
					col:      self.col,
					span:     raw.len(),
					src_line: self.get_curr_line().to_string(),
				}
			})
		};

		num
	}
}
