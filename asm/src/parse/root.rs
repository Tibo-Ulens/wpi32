//! AST root type definitions

use super::{Immediate, Instruction};

#[derive(Clone, Debug)]
pub(crate) struct Root<'s> {
	pub(crate) lines: Vec<Line<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Line<'s> {
	pub(crate) labl: Option<LabelId<'s>>,
	pub(crate) stmt: Option<Statement<'s>>,
	pub(crate) cmnt: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) enum Statement<'s> {
	Directive(Directive<'s>),
	Instruction(Instruction<'s>),
}

#[derive(Clone, Debug)]
pub(crate) enum LabelId<'l> {
	LabelDefine(Identifier<'l>),
	LocalLabelDefine(Identifier<'l>),
}

#[derive(Clone, Debug)]
pub(crate) struct Identifier<'i>(pub(crate) &'i str);

#[derive(Clone, Debug)]
pub(crate) enum Directive<'d> {
	Byte { data: Vec<Literal<'d>> },
	Half { data: Vec<Literal<'d>> },
	Word { data: Vec<Literal<'d>> },
	Repeat { amount: Literal<'d>, argument: Box<Directive<'d>> },
	Equ { id: Identifier<'d>, value: Literal<'d> },
}

#[derive(Clone, Debug)]
pub(crate) enum Literal<'t> {
	String(&'t str),
	Char(char),
	Number(isize),
	Immediate(Immediate<'t>),
}
