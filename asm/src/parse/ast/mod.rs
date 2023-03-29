//! # AST types
//!
//! The AST (Abstract Syntax Tree) is a datastructure that represents all the
//! information known about a given piece of source code. <br>
//! It provides an easier way of interacting with, modifying, and reasoning
//! about source code as compared to a stream of [`Token`](crate::lex::Token)s
//! or a raw string

// #![allow(missing_docs)]

mod instruction;
mod r#macro;

use std::fmt::{Display, Formatter, Result};

pub use instruction::*;
pub use r#macro::*;

use crate::lex::{Token, TokenType};

/// A single file of code, the root of the AST
///
/// Contains a set of [`Attribute`]s and a list of [`Item`]s
///
/// ```ebnf
/// file = { attribute }, { item };
/// ```
#[derive(Clone, Debug)]
pub struct File<'s> {
	/// The attributes of this file
	pub attrs: Vec<Attribute<'s>>,
	/// All the items of the file (see [`Item`] for more info)
	pub items: Vec<Item<'s>>,
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
/// outer_attribute = '#', '[', Token, { Token }, ']', newline;
/// inner_attribute = '#', '!', '[', Token, { Token }, ']', newline;
/// ```
#[derive(Clone, Debug)]
pub enum Attribute<'s> {
	Outer { name: Identifier<'s>, args: Vec<Token<'s>> },
	Inner { name: Identifier<'s>, args: Vec<Token<'s>> },
}

/// Any 'thing' in the source code, can have optional comments or just be
/// completely empty
///
/// ```ebnf
/// item =
///     (
///         { comment }
///         | ( [ statement ], { comment } )
///     ),
///     newline;
/// ```
#[derive(Clone, Debug)]
pub struct Item<'s> {
	pub comment:   Option<Comment<'s>>,
	pub statement: Option<Statement<'s>>,
}

/// Any statement in the source code, a 'thing' that does some action
///
/// ```ebnf
/// statement =
///     macro_definition
///     | macro_invocation
///     | labeled_block
///     | directive
///     | instruction
/// ;
/// ```
#[derive(Clone, Debug)]
pub enum Statement<'s> {
	/// A macro definition
	MacroDefinition(MacroDefinition<'s>),
	/// A macro invocation
	MacroInvocation(MacroInvocation<'s>),

	/// An extern label declaration
	Extern(Extern<'s>),
	/// An import directive to include other files
	Import(Import<'s>),

	/// A constant declaration \
	/// eg. `const foo = 123`
	ConstDefinition(ConstDefinition<'s>),

	/// A section block \
	/// eg. `section .text {}`
	SectionBlock(SectionBlock<'s>),

	/// A named lexical block \
	/// eg. `foo {}`
	LabeledBlock(BlockLabel<'s>),
	/// A non-scoped label \
	/// eg. `foo: addi r0, r0, 0`
	Label(Label<'s>),

	/// An instruction
	Instruction {
		/// The attributes of this instruction
		attrs: Vec<Attribute<'s>>,
		/// The instruction itself
		inst:  Instruction<'s>,
	},
}

/// An extern directive to declare a list of labels as external
///
/// ```ebnf
/// extern = "extern", identifier, { ",", identifier }
/// ```
#[derive(Clone, Debug)]
pub struct Extern<'s> {
	pub attrs:   Vec<Attribute<'s>>,
	/// The symbols to declare as external
	pub symbols: Vec<Identifier<'s>>,
}

/// An import directive to include other files
///
/// ```ebnf
/// import = "import", identifier, { ",", identifier }
/// ```
#[derive(Clone, Debug)]
pub struct Import<'s> {
	pub attrs: Vec<Attribute<'s>>,
	/// The files to import
	pub files: Vec<Identifier<'s>>,
}

/// An assemble-time constant definition
///
/// ```ebnf
/// const_definition = visibility, "const", identifier, "=", immediate
/// ```
#[derive(Clone, Debug)]
pub struct ConstDefinition<'s> {
	pub attrs:      Vec<Attribute<'s>>,
	pub visibility: Visibility,
	/// The name of the constant
	pub identifier: Identifier<'s>,
	/// The value of the constant
	pub value:      Immediate<'s>,
}

/// A block wrapping a specific section of the code
///
/// ```ebnf
/// section_block = "section", identifier, "{", { item }, "}"
/// ```
#[derive(Clone, Debug)]
pub struct SectionBlock<'s> {
	pub attrs:      Vec<Attribute<'s>>,
	pub block_name: Identifier<'s>,
	pub items:      Vec<Item<'s>>,
}

/// A lexical block identified by a label
///
/// ```ebnf
/// labeled_block = visibility, identifier, "{", { item }, "}";
/// ```
#[derive(Clone, Debug)]
pub struct BlockLabel<'s> {
	pub attrs:      Vec<Attribute<'s>>,
	pub visibility: Visibility,
	/// The label naming this block
	pub label:      Identifier<'s>,
	/// The content of this block
	pub items:      Vec<Option<Item<'s>>>,
}

/// A non-scoping label
///
/// ```ebnf
/// label = visibility, identifier, ":", item
/// ```
#[derive(Clone, Debug)]
pub struct Label<'s> {
	pub attrs:      Vec<Attribute<'s>>,
	pub visibility: Visibility,
	/// The name of the label
	pub label:      Identifier<'s>,
	/// The item this label is pointing to
	pub item:       Box<Item<'s>>,
}

/// Whether or not a specific item should be made public in the final
/// executable
///
/// ```ebnf
/// visibility = [ "public" ]
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Visibility {
	Public,
	Private,
}

/// An identifier used to name something
#[derive(Clone, Copy, Debug)]
pub struct Identifier<'s> {
	/// The actual value of the identifier
	pub id:          &'s str,
	/// The line number of this identifier
	pub line:        usize,
	/// The column number of this identifier
	pub col:         usize,
	/// The length (in characters) of this identifier
	pub span:        usize,
	/// The line of source code containing this identifier
	pub source_line: &'s str,
}

impl<'s> From<Token<'s>> for Identifier<'s> {
	fn from(value: Token<'s>) -> Self {
		let id = match value.t {
			TokenType::Identifier(id) => id,
			_ => unreachable!(),
		};

		Self {
			id,
			line: value.line,
			col: value.col,
			span: value.span,
			source_line: value.source_line,
		}
	}
}

/// An immediate value
///
/// This can range from a single number to a complex expression referencing
/// labels and constants
///
/// *EBNF not given as it is too chonky, look at the docs folder for grammar*
#[derive(Clone, Debug)]
pub struct Immediate<'s> {
	/// The tokens making up this immediate, parsed into
	/// [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
	pub rpn_tokens: Vec<Token<'s>>,
}

impl<'s> Display for Immediate<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let repr =
			self.rpn_tokens.iter().map(|t| t.t.to_string()).collect::<Vec<String>>().join(" ");

		write!(f, "{}", repr)
	}
}

/// A Comment
#[derive(Clone, Copy, Debug)]
pub struct Comment<'s> {
	/// The comment itself
	pub comment:     &'s str,
	/// The line number of this comment
	pub line:        usize,
	/// The column number of this comment
	pub col:         usize,
	/// The length (in characters) of this comment
	pub span:        usize,
	/// The line of source code containing this comment
	pub source_line: &'s str,
}

impl<'s> From<Token<'s>> for Comment<'s> {
	fn from(value: Token<'s>) -> Self {
		let comment = match value.t {
			TokenType::Comment(c) => c,
			_ => unreachable!(),
		};

		Self {
			comment,
			line: value.line,
			col: value.col,
			span: value.span,
			source_line: value.source_line,
		}
	}
}
