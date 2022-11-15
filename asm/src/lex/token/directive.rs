#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DirectiveToken {
	Byte,
	Half,
	Word,
	Repeat,
	Equ,
}
