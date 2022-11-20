//! AST root type definitions

mod immediate;
mod instruction;

pub(crate) use immediate::*;
pub(crate) use instruction::*;

#[derive(Clone, Debug)]
pub(crate) struct Root<'s> {
	pub(crate) preamble: Vec<PreambleLine<'s>>,
	pub(crate) sections: Vec<Section<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct PreambleLine<'s> {
	pub(crate) constdir: Option<ConstDirective<'s>>,
	pub(crate) comment:  Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) struct Section<'s> {
	pub(crate) name:  Identifier<'s>,
	pub(crate) lines: Vec<Line<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Line<'s> {
	pub(crate) content: Option<LineContent<'s>>,
	pub(crate) comment: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) enum LineContent<'s> {
	LabeledStatement { label: LabelId<'s>, stmt: Option<Statement<'s>> },
	Statement(Statement<'s>),
}

#[derive(Clone, Debug)]
pub(crate) enum Statement<'s> {
	Directive(Directive<'s>),
	Instruction(Instruction<'s>),
}

#[derive(Clone, Debug)]
pub(crate) enum LabelId<'l> {
	LabelDefine(&'l str),
	LocalLabelDefine(&'l str),
}

#[derive(Clone, Debug)]
pub(crate) struct Identifier<'i>(pub(crate) &'i str);

#[derive(Clone, Debug)]
pub(crate) struct ConstDirective<'d> {
	pub(crate) value: Literal<'d>,
}

#[derive(Clone, Debug)]
pub(crate) enum Directive<'d> {
	Bytes { data: Vec<Literal<'d>> },
	Halves { data: Vec<Literal<'d>> },
	Words { data: Vec<Literal<'d>> },

	ResBytes { data: Vec<Literal<'d>> },
	ResHalves { data: Vec<Literal<'d>> },
	ResWords { data: Vec<Literal<'d>> },

	Repeat { amount: Literal<'d>, argument: Box<Directive<'d>> },
}

#[derive(Clone, Debug)]
pub(crate) enum Literal<'t> {
	String(&'t str),
	Char(char),
	Immediate(Immediate<'t>),
}
