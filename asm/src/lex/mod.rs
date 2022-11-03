use std::iter::Peekable;
use std::str::Chars;

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
	#[inline(always)]
	fn peek(&mut self) -> Option<&char> { self.source_iter.peek() }

	/// Consume and return the next character
	#[inline(always)]
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

	/// Attempt to match an identifier to a keyword, directive, or identifier,
	/// or return a new label if a match is not found
	fn match_identifier(&mut self, id: &'s str) -> Token<'s> {
		match id {
			"add" => self.make_token(TokenType::KwAdd),
			"addi" => self.make_token(TokenType::KwAddi),
			"sub" => self.make_token(TokenType::KwSub),
			"and" => self.make_token(TokenType::KwAnd),
			"andi" => self.make_token(TokenType::KwAndi),
			"or" => self.make_token(TokenType::KwOr),
			"ori" => self.make_token(TokenType::KwOri),
			"xor" => self.make_token(TokenType::KwXor),
			"xori" => self.make_token(TokenType::KwXori),
			"lsl" => self.make_token(TokenType::KwLsl),
			"lsli" => self.make_token(TokenType::KwLsli),
			"lsr" => self.make_token(TokenType::KwLsr),
			"lsri" => self.make_token(TokenType::KwLsri),
			"asr" => self.make_token(TokenType::KwAsr),
			"asri" => self.make_token(TokenType::KwAsri),
			"slt" => self.make_token(TokenType::KwSlt),
			"slti" => self.make_token(TokenType::KwSlti),
			"sltu" => self.make_token(TokenType::KwSltu),
			"sltiu" => self.make_token(TokenType::KwSltiu),
			"lw" => self.make_token(TokenType::KwLw),
			"lh" => self.make_token(TokenType::KwLh),
			"lhu" => self.make_token(TokenType::KwLhu),
			"lb" => self.make_token(TokenType::KwLb),
			"lbu" => self.make_token(TokenType::KwLbu),
			"sw" => self.make_token(TokenType::KwSw),
			"sh" => self.make_token(TokenType::KwSh),
			"sb" => self.make_token(TokenType::KwSb),
			"lui" => self.make_token(TokenType::KwLui),
			"auipc" => self.make_token(TokenType::KwAuipc),
			"beq" => self.make_token(TokenType::KwBeq),
			"bne" => self.make_token(TokenType::KwBne),
			"blt" => self.make_token(TokenType::KwBlt),
			"bltu" => self.make_token(TokenType::KwBltu),
			"bge" => self.make_token(TokenType::KwBge),
			"bgeu" => self.make_token(TokenType::KwBgeu),
			"jal" => self.make_token(TokenType::KwJal),
			"jalr" => self.make_token(TokenType::KwJalr),
			"ecall" => self.make_token(TokenType::KwEcall),
			"ebreak" => self.make_token(TokenType::KwEbreak),
			"fence" => self.make_token(TokenType::KwFence),
			"fence.i" => self.make_token(TokenType::KwFencei),
			"csrrw" => self.make_token(TokenType::KwCsrrw),
			"csrrwi" => self.make_token(TokenType::KwCsrrwi),
			"csrrs" => self.make_token(TokenType::KwCsrrs),
			"csrrsi" => self.make_token(TokenType::KwCsrrsi),
			"csrrc" => self.make_token(TokenType::KwCsrrc),
			"csrrci" => self.make_token(TokenType::KwCsrrci),
			"mul" => self.make_token(TokenType::KwMul),
			"mulh" => self.make_token(TokenType::KwMulh),
			"mulhu" => self.make_token(TokenType::KwMulhu),
			"mulhsu" => self.make_token(TokenType::KwMulhsu),
			"div" => self.make_token(TokenType::KwDiv),
			"divu" => self.make_token(TokenType::KwDivu),
			"rem" => self.make_token(TokenType::KwRem),
			"remu" => self.make_token(TokenType::KwRemu),

			"r0" => self.make_token(TokenType::RegR0),
			"r1" => self.make_token(TokenType::RegR1),
			"r2" => self.make_token(TokenType::RegR2),
			"r3" => self.make_token(TokenType::RegR3),
			"r4" => self.make_token(TokenType::RegR4),
			"r5" => self.make_token(TokenType::RegR5),
			"r6" => self.make_token(TokenType::RegR6),
			"r7" => self.make_token(TokenType::RegR7),
			"r8" => self.make_token(TokenType::RegR8),
			"r9" => self.make_token(TokenType::RegR9),
			"r10" => self.make_token(TokenType::RegR10),
			"r11" => self.make_token(TokenType::RegR11),
			"r12" => self.make_token(TokenType::RegR12),
			"r13" => self.make_token(TokenType::RegR13),
			"r14" => self.make_token(TokenType::RegR14),
			"r15" => self.make_token(TokenType::RegR15),
			"r16" => self.make_token(TokenType::RegR16),
			"r17" => self.make_token(TokenType::RegR17),
			"r18" => self.make_token(TokenType::RegR18),
			"r19" => self.make_token(TokenType::RegR19),
			"r20" => self.make_token(TokenType::RegR20),
			"r21" => self.make_token(TokenType::RegR21),
			"r22" => self.make_token(TokenType::RegR22),
			"r23" => self.make_token(TokenType::RegR23),
			"r24" => self.make_token(TokenType::RegR24),
			"r25" => self.make_token(TokenType::RegR25),
			"r26" => self.make_token(TokenType::RegR26),
			"r27" => self.make_token(TokenType::RegR27),
			"r28" => self.make_token(TokenType::RegR28),
			"r29" => self.make_token(TokenType::RegR29),
			"r30" => self.make_token(TokenType::RegR30),
			"r31" => self.make_token(TokenType::RegR31),
			"zero" => self.make_token(TokenType::RegR0),

			"ra" => self.make_token(TokenType::RegR1),
			"sp" => self.make_token(TokenType::RegR2),
			"gp" => self.make_token(TokenType::RegR3),
			"tp" => self.make_token(TokenType::RegR4),
			"fp" => self.make_token(TokenType::RegR8),
			"a0" => self.make_token(TokenType::RegR10),
			"a1" => self.make_token(TokenType::RegR11),
			"a2" => self.make_token(TokenType::RegR12),
			"a3" => self.make_token(TokenType::RegR13),
			"a4" => self.make_token(TokenType::RegR14),
			"a5" => self.make_token(TokenType::RegR15),
			"a6" => self.make_token(TokenType::RegR16),
			"a7" => self.make_token(TokenType::RegR17),
			"s0" => self.make_token(TokenType::RegR8),
			"s1" => self.make_token(TokenType::RegR9),
			"s2" => self.make_token(TokenType::RegR18),
			"s3" => self.make_token(TokenType::RegR19),
			"s4" => self.make_token(TokenType::RegR20),
			"s5" => self.make_token(TokenType::RegR21),
			"s6" => self.make_token(TokenType::RegR22),
			"s7" => self.make_token(TokenType::RegR23),
			"s8" => self.make_token(TokenType::RegR24),
			"s9" => self.make_token(TokenType::RegR25),
			"s10" => self.make_token(TokenType::RegR26),
			"s11" => self.make_token(TokenType::RegR27),
			"t0" => self.make_token(TokenType::RegR5),
			"t1" => self.make_token(TokenType::RegR6),
			"t2" => self.make_token(TokenType::RegR7),
			"t3" => self.make_token(TokenType::RegR28),
			"t4" => self.make_token(TokenType::RegR29),
			"t5" => self.make_token(TokenType::RegR30),
			"t6" => self.make_token(TokenType::RegR31),

			".byte" => self.make_token(TokenType::DirByte),
			".half" => self.make_token(TokenType::DirHalf),
			".word" => self.make_token(TokenType::DirWord),
			".repeat" => self.make_token(TokenType::DirRepeat),
			".equ" => self.make_token(TokenType::DirEqu),

			_ => {
				if id.starts_with('.') {
					if let Some(stripped) = id.strip_suffix(':') {
						self.make_token(TokenType::LocalLabelDefine(stripped))
					} else {
						self.make_token(TokenType::LocalLabel(id))
					}
				} else if let Some(stripped) = id.strip_suffix(':') {
					self.make_token(TokenType::LabelDefine(stripped))
				} else {
					self.make_token(TokenType::Label(id))
				}
			},
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

	#[inline(always)]
	fn is_identifier_start(c: &char) -> bool {
		c.is_alphabetic()
			|| *c == '!' || *c == '$'
			|| *c == '&' || *c == '?'
			|| *c == '^' || *c == '_'
			|| *c == '~' || *c == '@'
			|| *c == '.'
	}

	#[inline(always)]
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
