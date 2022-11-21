use ansi_term::Colour::{Blue, Red, White};

/// Format error information into a pretty block
pub(crate) fn make_info_block(
	header: &str,
	src_file: &str,
	line: usize,
	col: usize,
	span: usize,
	src: &str,
) -> String {
	let line_len = format!("{}", line).len();

	// Header line
	// `error: {msg}`
	let mut repr = format!("{} {}\n", Red.bold().paint("error:"), White.bold().paint(header));

	// File arrow
	// `--> {src_file}`
	repr.push_str(&format!("  {} {}:{}:{}\n", Blue.bold().paint("-->"), src_file, line, col));

	// Info block
	// ```
	//       |
	// {col} | {source_line}
	//       |
	// ```
	repr.push_str(&format!("{}{}\n", " ".repeat(line_len + 1), Blue.bold().paint("|")));
	repr.push_str(&format!(
		"{} {} {}\n",
		Blue.bold().paint(format!("{}", line)),
		Blue.bold().paint("|"),
		src.trim_end(),
	));
	repr.push_str(&format!("{}{} ", " ".repeat(line_len + 1), Blue.bold().paint("|")));

	// Span indicator
	// `    ^^^^^^^^^`

	// Columns start at 1
	for _ in 1..col {
		repr.push(' ');
	}
	for _ in 0..span {
		repr.push_str(&format!("{}", Red.bold().paint("^")));
	}

	repr.push('\n');

	repr
}
