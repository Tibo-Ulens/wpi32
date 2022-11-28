use ansi_term::Colour::{Blue, Red, White};

use super::LocationInfo;

/// Format error message and file information into a header
pub(crate) fn make_info_header(header: &str, src_file: &str, loc_info: &LocationInfo) -> String {
	// Header line
	// `error: {msg}`
	let mut repr = format!("{} {}\n", Red.bold().paint("error:"), White.bold().paint(header));

	// File arrow
	// `--> {src_file}`
	repr.push_str(&format!(
		"  {} {}:{}:{}\n",
		Blue.bold().paint("-->"),
		src_file,
		loc_info.line,
		loc_info.col
	));

	repr
}

/// Format error location information into a pretty block
///
/// ```txt
///       |
/// {col} | {source_line}
///       |
/// ```
pub(super) fn make_info_body(header: Option<&str>, loc_info: &LocationInfo) -> String {
	let mut repr =
		if let Some(h) = header { format!("{}\n", White.bold().paint(h)) } else { String::new() };

	let line_len = format!("{}", loc_info.line).len();

	repr.push_str(&format!("{}{}\n", " ".repeat(line_len + 1), Blue.bold().paint("|")));
	repr.push_str(&format!(
		"{} {} {}\n",
		Blue.bold().paint(format!("{}", loc_info.line)),
		Blue.bold().paint("|"),
		loc_info.src_line.trim_end(),
	));
	repr.push_str(&format!("{}{} ", " ".repeat(line_len + 1), Blue.bold().paint("|")));

	// Span indicator
	// `    ^^^^^^^^^`

	// Columns start at 1
	for _ in 1..loc_info.col {
		repr.push(' ');
	}
	for _ in 0..loc_info.span {
		repr.push_str(&format!("{}", Red.bold().paint("^")));
	}

	repr.push('\n');

	repr
}
