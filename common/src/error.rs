#[derive(Debug, Error)]
pub enum Error {
	#[error("Wrong file type\nExpected a '{expected}' file, found a '{found}' file")]
	WrongFileType { found: String, expected: String },
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Lex(#[from] LexError),
}

fn make_info_block(line: usize, col: usize, span: usize, src: String) -> String {
	let mut repr = String::new();
	repr.push_str("    |\n");
	repr.push_str(&format!("{:<3} | {}\n", line, src));
	repr.push_str("    | ");

	// Columns start at 1
	for _ in 1..col {
		repr.push(' ');
	}
	for _ in 0..span {
		repr.push('^');
	}

	repr.push('\n');

	repr
}

#[derive(Debug, Error)]
pub enum LexError {
	#[error(
		"\n[{line}:{col}]: unexpected end-of-file\n{}",
		make_info_block(*.line, *.col, 1, .src_line.to_string())
	)]
	UnexpectedEof { line: usize, col: usize, src_line: String },
	#[error(
		"\n[{line}:{col}]: found unexpected symbol {fnd:?}, expected {ex:?}\n{}",
		make_info_block(*.line, *.col, 1, .src_line.to_string())
	)]
	UnexpectedSymbol {
		line:     usize,
		col:      usize,
		src_line: String,
		fnd:      char,
		ex:       char,
	},
	#[error(
		"\n[{line}:{col}]: found unexpected symbol {fnd:?}\n{}",
		make_info_block(*.line, *.col, 1, .src_line.to_string())
	)]
	RawUnexpectedSymbol { line: usize, col: usize, src_line: String, fnd: char },
	#[error(
		"\n[{line}:{col}]: invalid number\n{}",
		make_info_block(*.line, *.col, *.span, .src_line.to_string())
	)]
	InvalidNumber { line: usize, col: usize, span: usize, src_line: String },
}
