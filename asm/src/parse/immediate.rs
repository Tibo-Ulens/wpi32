//! AST immediate type definitions

#[derive(Clone, Debug)]
pub(crate) struct Immediate<'s> {
	lhs: XorImmediate<'s>,
	rhs: Option<XorImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct XorImmediate<'s> {
	lhs: AndImmediate<'s>,
	rhs: Option<AndImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AndImmediate<'s> {
	lhs: EqImmediate<'s>,
	rhs: Option<EqImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct EqImmediate<'s> {
	lhs: OrdImmediate<'s>,
	rhs: Option<OrdImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct OrdImmediate<'s> {
	lhs: ShiftImmediate<'s>,
	rhs: Option<ShiftImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct ShiftImmediate<'s> {
	lhs: AddSubImmediate<'s>,
	rhs: Option<AddSubImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct AddSubImmediate<'s> {
	lhs: MulDivRemImmediate<'s>,
	rhs: Option<MulDivRemImmediate<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) struct MulDivRemImmediate<'s> {
	lhs: Operand<'s>,
	rhs: Option<Operand<'s>>,
}

#[derive(Clone, Debug)]
pub(crate) enum Operand<'s> {
	Label(&'s str),
	LocalLabel(&'s str),
	Number(isize),
	Immediate(Box<Immediate<'s>>),
}
