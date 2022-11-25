//! Simple utility functions for the [`Lexer`](super::Lexer)

/// Checks whether a character is valid inside any binary, octal, decimal,
/// or hexadecimal number, including their radix identifiers (0b, 0o, 0x)
pub(super) fn is_digit_or_radix(c: char) -> bool {
	c.is_ascii_hexdigit() || c == 'x' || c == 'X' || c == 'o' || c == 'O'
}
