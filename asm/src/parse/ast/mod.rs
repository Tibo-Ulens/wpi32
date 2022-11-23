//! # AST types
//!
//! The AST (Abstract Syntax Tree) is a datastructure that represents all the
//! information known about a given piece of source code. <br>
//! It provides an easier way of interacting with, modifying, and reasoning
//! about source code as compared to a stream of [`Token`](crate::lex::Token)s
//! or a raw string

// #![allow(missing_docs)]

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
	/// The preamble of the file (see [`PreambleLine`] for more info)
	pub preamble: Vec<PreambleLine<'s>>,
	/// All the sections of the file (see [`Section`] for more info)
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
	/// The optional [`ConstDirective`] in this line
	pub constdir: Option<ConstDirective<'s>>,
	/// The optional comment in this line
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
	/// The name of the section
	pub name:  &'s str,
	/// All the line of code within this section (see [`Line`] for more info)
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
	/// The optional content in this line
	pub content: Option<LineContent<'s>>,
	/// The optional comment in this line
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
	/// An optional [`Statement`] prefixed with a [`Label`](LabelId)
	LabeledStatement {
		/// The [`Label`](LabelId) in this line
		label: LabelId<'s>,
		/// The optional [`Statement`] in this line
		stmt:  Option<Statement<'s>>,
	},
	/// A [`Statement`] without any [`Label`](LabelId)
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
	/// A directive
	DataDirective(DataDirective<'s>),
	/// An instruction
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
	/// A global label definition
	LabelDefine(&'l str),
	/// A local label definition
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
	/// The [`Label`](LabelId) defining the name of this constant
	pub id:    LabelId<'d>,
	/// The value of this constant
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
	/// Encodes data as bytes
	Bytes(Vec<Literal<'d>>),
	/// Encodes data as halves
	Halves(Vec<Literal<'d>>),
	/// Encodes data as words
	Words(Vec<Literal<'d>>),

	/// Reserve a given amount of bytes
	ResBytes(Vec<Literal<'d>>),
	/// Reserve a given amount of halves
	ResHalves(Vec<Literal<'d>>),
	/// Reserve a given amount of words
	ResWords(Vec<Literal<'d>>),

	/// Repeat a given argument any amount of times
	Repeat {
		/// The amount of times to repeat the argument
		amount:   Literal<'d>,
		/// The data to repeat
		argument: Box<RepeatedData<'d>>,
	},
}

/// The data that can be repeated using a [repeat](DataDirective::Repeat)
/// directive
#[derive(Clone, Debug)]
pub enum RepeatedData<'r> {
	/// A directive that gets repeated
	Directive(DataDirective<'r>),
	/// An instruction that gets repeated
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
	/// A string literal
	String(&'t str),
	/// A character literal
	Char(char),
	/// An immediate (a number, label, or arithmetic expression)
	Immediate(Immediate<'t>),
}
