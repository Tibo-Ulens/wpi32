//! Types and implementations to allow the AST to be printed using
//! [`ptree`](https://docs.rs/ptree/latest/ptree/)

use std::borrow::Cow;
use std::io::{Result, Write};

use ptree::{Style, TreeItem};

mod impls;

/// Converting to a type-erased AST [`Node`]
trait ToNode {
	/// Perform the conversion
	fn to_node(&self) -> Node;
}

/// Type-erased AST node
///
/// This is needed as [`ptree`](https://docs.rs/ptree/latest/ptree/) only lets
/// [`TreeItem`](https://docs.rs/ptree/latest/ptree/item/trait.TreeItem.html)s
/// have a single child type, however some AST nodes can have multiple
/// child types
///
/// Contains an optional `prefixes` field to allow at least some specificity as
/// to the type(s) of this node
#[derive(Clone)]
pub(crate) struct Node {
	pub(crate) prefixes: Vec<String>,
	pub(crate) repr:     String,
	pub(crate) children: Vec<Self>,
}

impl TreeItem for Node {
	type Child = Self;

	fn children(&self) -> Cow<[Self::Child]> { Cow::from(self.children.to_owned()) }

	fn write_self<W: Write>(&self, f: &mut W, style: &Style) -> Result<()> {
		if !(self.prefixes.is_empty()) {
			write!(
				f,
				"{} {}",
				style.paint(
					self.prefixes
						.iter()
						.map(|p| format!("({})", p))
						.collect::<Vec<String>>()
						.join(" ")
				),
				style.paint(self.repr.to_string())
			)
		} else {
			write!(f, "{}", style.paint(self.repr.to_string()))
		}
	}
}

impl Node {
	/// Consume a [`Node`] and return an identical one with a given prefix
	/// prepended to its prefix list
	fn add_prefix(self, prefix: &str) -> Self {
		let mut prefixes = self.prefixes;
		prefixes.insert(0, prefix.to_string());

		Node { prefixes, ..self }
	}
}
