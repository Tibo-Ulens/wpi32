//! # Rewriter
//!
//! The rewriter is responsible for type checking and rewriting macro
//! invocations in the [`ast`]

use std::cell::RefCell;
use std::collections::HashMap;

use crate::error::MacroError;
use crate::parse::ast::{
	Line,
	MacroDefinition,
	MacroInvocation,
	MacroRule,
	PreambleStatement,
	Root,
	Section,
	Statement,
};

/// Main rewriter struct
///
/// Wraps all internal state needed for rewriting macros and provides a
/// namespace for all rewriting functions
///
/// ### Lifetimes
///  - `'s`: The lifetime of the reference to the source code string, needed as tokens containing
///    string literals will contain references instead of owned data
pub struct Rewriter<'s> {
	/// The name of the file being parsed (used for error messages)
	source_file: &'s str,
	/// All macro definitions found in the source code
	definitions: RefCell<HashMap<&'s str, Vec<MacroRule<'s>>>>,
}

impl<'s> Rewriter<'s> {
	/// Create a new rewriter given a source file name and an AST
	pub fn new<'r>(source_file: &'s str, ast: &'r mut Root<'s>) -> Self {
		Self { source_file, definitions: RefCell::new(HashMap::new()) }
	}

	/// Rewrite all macros found in the AST
	pub fn rewrite(&mut self, ast: &mut Root<'s>) -> Result<(), MacroError> {
		self.find_macro_definitions(ast);

		for section in ast.sections.iter_mut() {
			self.rewrite_section(section)?;
		}

		Ok(())
	}

	/// Scan through the AST and store all macro definitions in an easier to
	/// use internal representation
	fn find_macro_definitions(&self, ast: &mut Root<'s>) {
		let mut defs: Vec<MacroDefinition> = ast
			.preamble
			.iter()
			.filter_map(|pl| pl.statement.to_owned())
			.filter_map(|stmt| {
				if let PreambleStatement::MacroDefinition(mdef) = stmt { Some(mdef) } else { None }
			})
			.collect();

		let section_lines: Vec<&Line> = ast.sections.iter().map(|s| &s.lines).flatten().collect();

		let section_defs: Vec<MacroDefinition> =
			section_lines
				.iter()
				.filter_map(|l| l.statement.to_owned())
				.filter_map(|stmt| {
					if let Statement::MacroDefinition(mdef) = stmt { Some(mdef) } else { None }
				})
				.collect();

		defs.extend(section_defs);

		for def in defs {
			self.definitions.borrow_mut().insert(def.id, def.rules);
		}
	}

	fn rewrite_section(&mut self, section: &mut Section<'s>) -> Result<(), MacroError> {
		let mut rewritten_lines = vec![];

		for line in section.lines.iter_mut() {
			if let Some(Statement::MacroInvocation(invoc)) = &line.statement {
				let rewritten_macro = self.rewrite_macro_invocation(invoc)?;
			} else {
				rewritten_lines.push(line.to_owned());
			}
		}

		section.lines = rewritten_lines;

		Ok(())
	}

	fn rewrite_macro_invocation<'i, 'r: 'i>(
		&'r self,
		invocation: &'i MacroInvocation,
	) -> Result<Vec<Line<'s>>, MacroError> {
		let macro_definition = self.definitions.borrow().get(invocation.id);

		Ok(vec![])
	}
}
