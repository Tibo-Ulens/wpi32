//! # AST types
//!
//! The AST (Abstract Syntax Tree) is a datastructure that represents all the
//! information known about a given piece of source code. </br>
//! It provides an easier way of interacting with, modifying, and reasoning
//! about source code as compared to a stream of [`Token`](crate::lex::Token)s
//! or a raw string

mod immediate;
mod instruction;

pub(crate) use immediate::*;
pub(crate) use instruction::*;

/// The root of the AST
///
/// Contains a [`preamble`](PreambleLine) and a list of [`Section`]s
///
/// ```ebnf
/// root = [ preamble ], { section };
/// ```
#[derive(Clone, Debug)]
pub(crate) struct Root<'s> {
	pub(crate) preamble: Vec<PreambleLine<'s>>,
	pub(crate) sections: Vec<Section<'s>>,
}

/// A single line in the preamble of some source code
///
/// The preamble will not be emitted to any named sections, and so cannot
/// contain any actual data. However, it can be used to define constants using
/// [`ConstDirective`]s or to write header comments.
///
/// ```ebnf
/// preamble_line =
///     { whitespace },
///     [ const_directive ],
///     [ comment ],
///     newline;
/// ```
#[derive(Clone, Debug)]
pub(crate) struct PreambleLine<'s> {
	pub(crate) constdir: Option<ConstDirective<'s>>,
	pub(crate) comment:  Option<&'s str>,
}

/// A single assembler
///
/// Sections are used to indicate the function of a certain piece of code </br>
/// The assembler currently recognizes three section types:
///  - `.TEXT` sections: executable instructions
///  - `.DATA` sections: initialized read/write data
///  - `.BSS` sections: uninitialized read/write data
///
/// Each section has a name and a list of [`Line`]s
///
/// ```ebnf
/// section = section_header, { line };
/// section_header = "#SECTION", section_name, newline;
/// section_name = ".TEXT" | ".DATA" | ".BSS";
/// ```
#[derive(Clone, Debug)]
pub(crate) struct Section<'s> {
	pub(crate) name:  &'s str,
	pub(crate) lines: Vec<Line<'s>>,
}

/// A single line of code in a [`Section`]
///
/// Lines can be empty, or contain some [`content`](LineContent) or a comment,
/// or both, optionally preceded by some whitespace
///
/// ```ebnf
/// line =
///     { whitespace },
///     [ line_content ],
///     [ comment ],
///     newline;
/// ```
#[derive(Clone, Debug)]
pub(crate) struct Line<'s> {
	pub(crate) content: Option<LineContent<'s>>,
	pub(crate) comment: Option<&'s str>,
}

/// The potential content of a [`Line`]
///
/// Can be either a [`Label`](LabelId) followed by an optional [`Statement`],
/// or just a [`Statement`]
///
/// ```ebnf
/// line_content = labeled_statement | statement;
/// labeled_statement = label_id, [ const_directive | statement ];
/// label_id = label_define | local_label_define;
/// ```
#[derive(Clone, Debug)]
pub(crate) enum LineContent<'s> {
	LabeledStatement { label: LabelId<'s>, stmt: Option<Statement<'s>> },
	Statement(Statement<'s>),
}

/// A single assembly statement
///
/// Can be either a [`DataDirective`] or an [`Instruction`]
///
/// ```ebnf
/// statement = data_directive | instruction;
/// ```
#[derive(Clone, Debug)]
pub(crate) enum Statement<'s> {
	DataDirective(DataDirective<'s>),
	Instruction(Instruction<'s>),
}

/// A label prefixing a [`Statement`]
///
/// Can be either an absolute label, or a local label
///
/// ```ebnf
/// label_id = label_define | local_label_define;
/// label_define = label, ":";
/// local_label_define = local_label, ":";
/// local_label = ".", label;
/// label = identifier_initial, { identifier_subsequent };
/// ```
#[derive(Clone, Debug)]
pub(crate) enum LabelId<'l> {
	LabelDefine(&'l str),
	LocalLabelDefine(&'l str),
}

/// A directive to declare assemble-time constants
///
/// Sets the prefixed [`LabelId`] equal to its [`value`](Literal)
///
/// ```ebnf
/// const_directive = "#CONST", literal;
/// ```
#[derive(Clone, Debug)]
pub(crate) struct ConstDirective<'d> {
	pub(crate) id:    LabelId<'d>,
	pub(crate) value: Literal<'d>,
}

/// A directive that creates or otherwise manipulates data
///
/// Data directives can:
///  - define initialised data as bytes, halves, or words
///  - reserve a given number bytes, halves, or words
///  - repeat any other [`DataDirective`] a given number of times
///
/// ```ebnf
/// data_directive =
///     bytes_directive,
///     halves_directive
///     words_directive,
///     res_bytes_directive,
///     res_halves_directive,
///     res_words_directive,
///     repeat_directive;
/// ```

#[derive(Clone, Debug)]
pub(crate) enum DataDirective<'d> {
	Bytes { data: Vec<Literal<'d>> },
	Halves { data: Vec<Literal<'d>> },
	Words { data: Vec<Literal<'d>> },

	ResBytes { data: Vec<Literal<'d>> },
	ResHalves { data: Vec<Literal<'d>> },
	ResWords { data: Vec<Literal<'d>> },

	Repeat { amount: Literal<'d>, argument: Box<DataDirective<'d>> },
}

/// A literal value
///
/// Can be a string, character, or an [`Immediate`]
///
/// ```ebnf
/// literal = string | char | immediate;
/// ```
#[derive(Clone, Debug)]
pub(crate) enum Literal<'t> {
	String(&'t str),
	Char(char),
	Immediate(Immediate<'t>),
}
