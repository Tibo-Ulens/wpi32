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
mod r#macro;

pub use immediate::*;
pub use instruction::*;
pub use r#macro::*;

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

/// A directive to declare assemble-time constants
///
/// Sets the prefixed [`LabelId`] equal to its [`value`](Literal)
///
/// ```ebnf
/// const_directive = "#CONST", literal;
/// ```
#[derive(Clone, Debug)]
pub struct ConstDirective<'s> {
	/// The identifier defining the name of this constant
	pub id:    &'s str,
	/// The value of this constant
	pub value: Literal<'s>,
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
/// Lines can be empty, or contain some [`Statement`] or a comment,
/// or both, optionally preceded by some whitespace
///
/// ```ebnf
/// line =
///     { whitespace },
///     [ statement ],
///     [ comment ],
///     newline;
/// ```
#[derive(Clone, Debug)]
pub struct Line<'s> {
	/// The optional content in this line
	pub statement: Option<Statement<'s>>,
	/// The optional comment in this line
	pub comment:   Option<&'s str>,
}

/// A single assembly statement
///
/// Can be a [`MacroDefinition`], [`MacroInvocation`], [`LabeledBlock`],
/// [`Directive`], or an [`Instruction`]
///
/// ```ebnf
/// statement =
///     macro_definition
///     | macro_invocation
///     | labeled_block
///     | directive
///     | instruction;
/// ```
#[derive(Clone, Debug)]
pub enum Statement<'s> {
	/// A macro definition
	MacroDefinition(MacroDefinition<'s>),
	/// A macro invocation
	MacroInvocation(MacroInvocation<'s>),
	/// A labeled block of code
	LabeledBlock(LabeledBlock<'s>),
	/// A directive
	Directive(Directive<'s>),
	/// An instruction
	Instruction(Instruction<'s>),
}

/// A block of code enclosed by a label
///
/// ```ebnf
/// labeled_block = identifier, "{", { line }, "}";
/// ```
#[derive(Clone, Debug)]
pub struct LabeledBlock<'s> {
	/// The label naming this block
	label: &'s str,
	/// The content of this block
	lines: Vec<Line<'s>>,
}

/// A directive that creates or otherwise manipulates data
///
/// Directives can:
///  - define initialised data as bytes, halves, or words
///  - reserve a given number bytes, halves, or words
///  - declare an identifier as a constant
///
/// ```ebnf
/// directive =
///     bytes_directive
///     | halves_directive
///     | words_directive
///     | res_bytes_directive
///     | res_halves_directive
///     | res_words_directive
///     | const_directive;
/// ```
#[derive(Clone, Debug)]
pub enum Directive<'s> {
	/// Encodes data as bytes
	Bytes(Vec<Literal<'s>>),
	/// Encodes data as halves
	Halves(Vec<Literal<'s>>),
	/// Encodes data as words
	Words(Vec<Literal<'s>>),

	/// Reserve a given amount of bytes
	ResBytes(Vec<Literal<'s>>),
	/// Reserve a given amount of halves
	ResHalves(Vec<Literal<'s>>),
	/// Reserve a given amount of words
	ResWords(Vec<Literal<'s>>),

	/// Declare some identifier to be a constant
	Const(ConstDirective<'s>),
}

/// A literal value
///
/// Can be a string, character, or an [`Immediate`]
///
/// ```ebnf
/// literal = string | char | immediate;
/// ```
#[derive(Clone, Debug)]
pub enum Literal<'s> {
	/// A string literal
	String(&'s str),
	/// A character literal
	Char(char),
	/// An immediate (a number, label, or arithmetic expression)
	Immediate(Immediate<'s>),
}
