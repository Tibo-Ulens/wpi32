//! AST root type definitions

use super::{Immediate, Instruction};

#[derive(Clone, Debug)]
pub(crate) struct Root<'s> {
	pub(crate) lines: Vec<Section<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) enum Section<'s> {
	Text { header: TextHeader, lines: Vec<TextLine<'s>> },
	Data { header: DataHeader, lines: Vec<DataLine<'s>> },
	Bss { header: BssHeader, lines: Vec<BssLine<'s>> },
}

#[derive(Clone, Debug)]
pub(crate) struct TextHeader;
#[derive(Clone, Debug)]
pub(crate) struct DataHeader;
#[derive(Clone, Debug)]
pub(crate) struct BssHeader;

#[derive(Clone, Debug)]
pub(crate) struct TextLine<'s> {
	pub(crate) labl: Option<LabelId<'s>>,
	pub(crate) stmt: Option<TextStatement<'s>>,
	pub(crate) cmnt: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) struct DataLine<'s> {
	pub(crate) labl: Option<LabelId<'s>>,
	pub(crate) dirv: Option<DataDirective<'s>>,
	pub(crate) cmnt: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) struct BssLine<'s> {
	pub(crate) labl: Option<LabelId<'s>>,
	pub(crate) dirv: Option<BssDirective<'s>>,
	pub(crate) cmnt: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub(crate) enum TextStatement<'s> {
	TextDirective(TextDirective<'s>),
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
pub(crate) enum TextDirective<'d> {
	Repeat { amount: Literal<'d>, argument: Box<TextDirective<'d>> },
	Define { id: Identifier<'d>, value: Literal<'d> },
}

#[derive(Clone, Debug)]
pub(crate) enum DataDirective<'d> {
	Bytes { data: Vec<Literal<'d>> },
	Halves { data: Vec<Literal<'d>> },
	Words { data: Vec<Literal<'d>> },
	Repeat { amount: Literal<'d>, argument: Box<DataDirective<'d>> },
	Define { id: Identifier<'d>, value: Literal<'d> },
}

#[derive(Clone, Debug)]
pub(crate) enum BssDirective<'d> {
	ResBytes { data: Vec<Literal<'d>> },
	ResHalves { data: Vec<Literal<'d>> },
	ResWords { data: Vec<Literal<'d>> },
	Repeat { amount: Literal<'d>, argument: Box<BssDirective<'d>> },
	Define { id: Identifier<'d>, value: Literal<'d> },
}

#[derive(Clone, Debug)]
pub(crate) enum Literal<'t> {
	String(&'t str),
	Char(char),
	Number(isize),
	Immediate(Immediate<'t>),
}
