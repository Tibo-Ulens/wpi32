//! AST root type definitions

use super::{Immediate, Instruction};

#[derive(Clone, Debug)]
pub(crate) struct Root<'s> {
	pub(crate) lines: Vec<Line<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Line<'s> {
	pub(crate) stmt: Statement<'s>,
	pub(crate) cmnt: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) enum Statement<'s> {
	LabelDefine(&'s str),
	LocalLabelDefine(&'s str),
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
