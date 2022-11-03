use crate::lex::Token;

pub(crate) struct Root<'s> {
	pub(crate) stmts: Vec<Statement<'s>>,
}

pub(crate) enum Statement<'s> {
	LabelDefine { label: &'s str },
	LocalLabelDefine { label: &'s str },
	Directive(DirectiveStmt<'s>),
	Instruction(InstructionStmt),
}

pub(crate) enum DirectiveStmt<'d> {
	ByteDirective { data: Vec<Literal<'d>> },
	HalfDirective { data: Vec<Literal<'d>> },
	WordDirective { data: Vec<Literal<'d>> },
	RepeatDirective { amount: usize, argument: Box<DirectiveStmt<'d>> },
	EquDirective { name: &'d str, value: Literal<'d> },
}

pub(crate) enum Literal<'t> {
	String(&'t str),
	Char(char),
	Number(i32),
	Immediate(Vec<Token<'t>>),
}

pub(crate) enum InstructionStmt {}
