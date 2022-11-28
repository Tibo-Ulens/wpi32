//! Types and implementations to allow the AST to be printed as a structured
//! tree

use std::fmt::{Display, Formatter, Write};

mod impls;

/// Type-erased AST node
///
/// This is needed to generalise AST nodes so we can pretty-print them more
/// easily
///
/// Contains an optional `prefixes` field to allow at least some specificity as
/// to the type(s) of this node
#[derive(Clone)]
pub struct Node {
	/// Optional prefixes providing extra type information
	pub prefixes: Vec<String>,
	/// The representation of this node
	pub repr:     String,
	/// The children of this node
	pub children: Vec<Self>,
}

impl Display for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { print_item(f, self, 1) }
}

impl Node {
	/// Consume a [`Node`] and return an identical one with a given prefix
	/// prepended to its prefix list
	fn add_prefix(self, prefix: &str) -> Self {
		let mut prefixes = self.prefixes;
		prefixes.insert(0, prefix.to_string());

		Node { prefixes, ..self }
	}

	/// Print this node as a string
	fn write_self<W: Write>(&self, f: &mut W) -> std::fmt::Result {
		if !(self.prefixes.is_empty()) {
			writeln!(
				f,
				"{} {}",
				self.prefixes.iter().map(|p| format!("({})", p)).collect::<Vec<String>>().join(" "),
				self.repr,
			)
		} else {
			writeln!(f, "{}", self.repr)
		}
	}
}

/// Recursively write a [`Node`] and its children
fn print_item(f: &mut Formatter<'_>, t: &Node, level: usize) -> std::fmt::Result {
	t.write_self(f)?;

	let children = &t.children;
	if let Some((last, children)) = children.split_last() {
		for c in children {
			write!(f, "{}", "  ".repeat(level))?;
			print_item(f, c, level + 1)?;
		}

		write!(f, "{}", "  ".repeat(level))?;
		print_item(f, last, level + 1)?;
	}

	Ok(())
}
