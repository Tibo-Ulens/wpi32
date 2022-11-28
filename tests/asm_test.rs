use asm::error::Error;
use asm::lex::{Lexer, Token};
use asm::parse::{Node, Parser};

mod common;
use common::*;

#[test]
fn lexer_test() -> Result<(), Error> {
	let lexer = Lexer::new("test_file.asm", TEST_SOURCE_CODE);
	let tokens: Vec<Token> = lexer.into_iter().collect::<Result<Vec<Token>, Error>>()?;

	let repr = tokens.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\n");

	assert_eq!(repr, TEST_TOKENS_STRING);

	Ok(())
}

#[test]
fn parser_test() -> Result<(), Error> {
	let lexer = Lexer::new("test_file.asm", TEST_SOURCE_CODE);
	let tokens: Vec<Token> = lexer.into_iter().collect::<Result<Vec<Token>, Error>>()?;

	let mut parser = Parser::new("test_file.asm", &tokens);
	let ast_root = parser.parse()?;

	let repr = Node::from(&ast_root).to_string();

	assert_eq!(repr, TEST_AST_STRING);

	Ok(())
}
