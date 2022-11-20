//! Simple utility functions for the [`Lexer`](crate::lex::Lexer)

/// Checks whether a character is a valid identifier start character
///
/// Identifiers follow the regex
/// `[a-zA-Z$?_@#.][a-zA-Z$?_@#.0-9:]*`
pub(super) fn is_identifier_start(c: &char) -> bool {
	c.is_alphabetic() || *c == '$' || *c == '?' || *c == '_' || *c == '@' || *c == '.' || *c == '#'
}

/// Checks whether a character is a valid identifier character
///
/// Identifiers follow the regex
/// `[a-zA-Z!$?_~@.][a-zA-Z!$?_~@.0-9:]*`
pub(super) fn is_identifier(c: &char) -> bool {
	(is_identifier_start(c) || c.is_ascii_digit() || *c == ':') && *c != '.'
}

/// Checks whether a character is valid inside any binary, octal, decimal,
/// or hexadecimal number, including their radix identifiers (0b, 0o, 0x)
pub(super) fn is_digit_or_radix(c: &char) -> bool {
	c.is_ascii_hexdigit() || *c == 'x' || *c == 'X' || *c == 'o' || *c == 'O'
}
