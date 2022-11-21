//! # AST types
//!
//! The AST (Abstract Syntax Tree) is a datastructure that represents all the
//! information known about a given piece of source code. <br>
//! It provides an easier way of interacting with, modifying, and reasoning
//! about source code as compared to a stream of [`Token`](crate::lex::Token)s
//! or a raw string

#![allow(missing_docs)]

mod immediate;
mod instruction;

pub use immediate::*;
pub use instruction::*;

/// The root of the AST
///
/// Contains a [`preamble`](PreambleLine) and a list of [`Section`]s
///
/// ```ebnf
/// root = [ preamble ], { section };
/// ```
#[derive(Clone, Debug)]
pub struct Root<'s> {
	pub preamble: Vec<PreambleLine<'s>>,
	pub sections: Vec<Section<'s>>,
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
pub struct PreambleLine<'s> {
	pub constdir: Option<ConstDirective<'s>>,
	pub comment:  Option<&'s str>,
}

/// A single assembler
///
/// Sections are used to indicate the function of a certain piece of code <br>
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
pub struct Section<'s> {
	pub name:  &'s str,
	pub lines: Vec<Line<'s>>,
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
pub struct Line<'s> {
	pub content: Option<LineContent<'s>>,
	pub comment: Option<&'s str>,
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
pub enum LineContent<'s> {
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
pub enum Statement<'s> {
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
pub enum LabelId<'l> {
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
pub struct ConstDirective<'d> {
	pub id:    LabelId<'d>,
	pub value: Literal<'d>,
}

/// A directive that creates or otherwise manipulates data
///
/// Data directives can:
///  - define initialised data as bytes, halves, or words
///  - reserve a given number bytes, halves, or words
///  - repeat any other [`DataDirective`] or [`Instruction`] a given number of times
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
pub enum DataDirective<'d> {
	Bytes { data: Vec<Literal<'d>> },
	Halves { data: Vec<Literal<'d>> },
	Words { data: Vec<Literal<'d>> },

	ResBytes { data: Vec<Literal<'d>> },
	ResHalves { data: Vec<Literal<'d>> },
	ResWords { data: Vec<Literal<'d>> },

	Repeat { amount: Literal<'d>, argument: Box<RepeatedData<'d>> },
}

#[derive(Clone, Debug)]
pub enum RepeatedData<'r> {
	Directive(DataDirective<'r>),
	Instruction(Instruction<'r>),
}

/// A literal value
///
/// Can be a string, character, or an [`Immediate`]
///
/// ```ebnf
/// literal = string | char | immediate;
/// ```
#[derive(Clone, Debug)]
pub enum Literal<'t> {
	String(&'t str),
	Char(char),
	Immediate(Immediate<'t>),
}
