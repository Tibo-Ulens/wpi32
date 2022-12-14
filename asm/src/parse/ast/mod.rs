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

use crate::lex::Token;

/// A single file of code, the root of the AST
///
/// Contains a set of [`Attribute`]s and a list of [`Item`]s
///
/// ```ebnf
/// file = { item };
/// ```
#[derive(Clone, Debug)]
pub struct File<'s> {
	/// The attributes of this file
	pub attrs: Vec<Attribute<'s>>,
	/// All the items of the file (see [`Item`] for more info)
	pub items: Vec<Option<Item<'s>>>,
}

/// An attribute specifying some potential modification of the item it
/// references
///
/// Attributes can be either outer attributes (attributes that 'wrap' some
/// item) or inner attributes (attributes that are contained within the item
/// they're modifying)
///
/// An attribute will have a name specifying what attribute should be added,
/// and an optional list of arguments to said attribute
///
/// ```ebnf
/// attribute = outer_attribute | inner_attribute;
/// outer_attribute = '#', '[', Token, { Token }, ']'
/// inner_attribute = '#', '!', '[', Token, { Token }, ']'
/// ```
#[derive(Clone, Debug)]
pub enum Attribute<'s> {
	Outer { name: Identifier<'s>, args: Vec<Token<'s>> },
	Inner { name: Identifier<'s>, args: Vec<Token<'s>> },
}

/// A 'thing' that can appear within the source code
///
/// ```ebnf
/// item = (
///     ( macro_definition, [ comment ] )
///     | ( macro_invocation, [ comment ] )
///     | ( labeled_block, [ comment ] )
///     | ( directive, [ comment ] )
///     | ( instruction, [ comment ] )
///     | comment
/// ), newline;
/// ```
#[derive(Clone, Debug)]
pub enum Item<'s> {
	/// A comment
	Comment(&'s str),
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

/// A block identified by a label
///
/// ```ebnf
/// labeled_block = identifier, "{", { item }, "}";
/// ```
#[derive(Clone, Debug)]
pub struct LabeledBlock<'s> {
	/// The label naming this block
	pub label: Identifier<'s>,
	/// The content of this block
	pub items: Vec<Option<Item<'s>>>,
}

/// A directive specifying how some data should be handled, interpreted, or
/// modified
///
/// Directives can:
///  - define initialised data as bytes, halves, or words
///  - reserve a given number bytes, halves, or words
///  - declare an identifier as a constant
///  - specify what symbols to make available globally
///  - potentially more in the future
///
/// ```ebnf
/// directive = '#', Token, { Token };
/// ```
#[derive(Clone, Debug)]
pub struct Directive<'s> {
	pub name:      Identifier<'s>,
	pub arguments: Vec<Token<'s>>,
}

/// An identifier used to name something
#[derive(Clone, Debug)]
pub struct Identifier<'s> {
	/// The actual value of the identifier
	pub id:          &'s str,
	/// The line number of this token
	pub line:        usize,
	/// The column number of this token
	pub col:         usize,
	/// The length (in characters) of this token
	pub span:        usize,
	/// The line of source code containing this token
	pub source_line: &'s str,
}
