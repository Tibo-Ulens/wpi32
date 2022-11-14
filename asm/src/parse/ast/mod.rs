//! AST type definitions

mod immediate;
mod instruction;
mod register;

pub(crate) use immediate::*;
pub(crate) use instruction::*;
pub(crate) use register::*;

#[derive(Clone, Debug)]
pub(crate) struct Root<'s> {
	pub(crate) lines: Vec<Line<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Line<'s> {
	annotated_stmt: AnnotatedStatement<'s>,
}

#[derive(Clone, Debug)]
pub(crate) struct AnnotatedStatement<'s> {
	stmt: Statement<'s>,
	cmnt: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) enum Statement<'s> {
	LabelDefine { label: &'s str },
	LocalLabelDefine { label: &'s str },
	Directive(Directive<'s>),
	Instruction(Instruction<'s>),
}

#[derive(Clone, Debug)]
pub(crate) enum Directive<'d> {
	Byte { data: Vec<Literal<'d>> },
	Half { data: Vec<Literal<'d>> },
	Word { data: Vec<Literal<'d>> },
	Repeat { amount: usize, argument: Box<Directive<'d>> },
	Equ { name: &'d str, value: Literal<'d> },
}

#[derive(Clone, Debug)]
pub(crate) enum Literal<'t> {
	String(&'t str),
	Char(char),
	Number(isize),
	Immediate(Immediate<'t>),
}
