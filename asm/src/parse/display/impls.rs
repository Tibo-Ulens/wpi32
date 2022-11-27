//! Implementations of `From<T> for Node` for all AST types

use super::Node;
use crate::lex::RegToken;
use crate::parse::ast::{
	Address,
	ConstDirective,
	Directive,
	Immediate,
	Instruction,
	LabeledBlock,
	Line,
	Literal,
	MacroArgType,
	MacroDefinition,
	MacroInvocation,
	MacroMatch,
	MacroRule,
	MacroVarType,
	OffsetOperator,
	OrderingTarget,
	PreambleLine,
	PreambleStatement,
	Root,
	Section,
	Statement,
};

impl<'s> From<&Root<'s>> for Node {
	fn from(value: &Root) -> Self {
		let preamble_lines = Node {
			prefixes: vec![],
			repr:     "Preamble".to_string(),
			children: value.preamble.iter().map(|l| l.into()).collect(),
		};
		let sections = Node {
			prefixes: vec![],
			repr:     "Sections".to_string(),
			children: value.sections.iter().map(|s| s.into()).collect(),
		};

		Node {
			prefixes: vec![],
			repr:     "Root".to_string(),
			children: vec![preamble_lines, sections],
		}
	}
}

impl<'s> From<&PreambleLine<'s>> for Node {
	fn from(value: &PreambleLine) -> Self {
		let mut children = vec![];

		if let Some(stmt) = &value.statement {
			children.push(Node::from(stmt).add_prefix("PreambleStatement"));
		}

		if let Some(comment) = value.comment {
			children.push(Node {
				prefixes: vec!["Comment".to_string()],
				repr:     format!("{:?}", comment),
				children: vec![],
			});
		}

		if children.is_empty() {
			Node { prefixes: vec![], repr: "Empty".to_string(), children }
		} else {
			Node { prefixes: vec![], repr: "PreambleLine".to_string(), children }
		}
	}
}

impl<'s> From<&PreambleStatement<'s>> for Node {
	fn from(value: &PreambleStatement<'s>) -> Self {
		match value {
			PreambleStatement::ConstDirective(const_dir) => const_dir.into(),
			PreambleStatement::MacroDefinition(m_def) => m_def.into(),
		}
	}
}

impl<'s> From<&ConstDirective<'s>> for Node {
	fn from(value: &ConstDirective) -> Self {
		let mut children = vec![Node {
			prefixes: vec!["Id".to_string()],
			repr:     value.id.to_string(),
			children: vec![],
		}];

		children.push(Node::from(&value.value).add_prefix("Value"));

		Node { prefixes: vec!["Directive".to_string()], repr: "Const".to_string(), children }
	}
}

impl<'s> From<&Section<'s>> for Node {
	fn from(value: &Section) -> Self {
		let lines = Node {
			prefixes: vec![],
			repr:     "Lines".to_string(),
			children: value.lines.iter().map(|l| l.into()).collect(),
		};

		let mut children = vec![Node {
			prefixes: vec!["Name".to_string()],
			repr:     value.name.to_string(),
			children: vec![],
		}];
		children.push(lines);

		Node { prefixes: vec![], repr: "Section".to_string(), children }
	}
}

impl<'s> From<&Line<'s>> for Node {
	fn from(value: &Line) -> Self {
		let mut children = vec![];

		if let Some(stmt) = &value.statement {
			children.push(Node::from(stmt).add_prefix("Content"));
		}

		if let Some(comment) = value.comment {
			children.push(Node {
				prefixes: vec!["Comment".to_string()],
				repr:     format!("{:?}", comment),
				children: vec![],
			});
		}

		if children.is_empty() {
			Node { prefixes: vec![], repr: "Empty".to_string(), children }
		} else {
			Node { prefixes: vec![], repr: "Line".to_string(), children }
		}
	}
}

impl<'s> From<&Statement<'s>> for Node {
	fn from(value: &Statement) -> Self {
		match value {
			Statement::MacroDefinition(m_def) => Node::from(m_def).add_prefix("Statement"),
			Statement::MacroInvocation(m_invoc) => Node::from(m_invoc).add_prefix("Statement"),
			Statement::LabeledBlock(l_block) => Node::from(l_block).add_prefix("Statement"),
			Statement::Directive(dir) => Node::from(dir).add_prefix("Statement"),
			Statement::Instruction(inst) => Node::from(inst).add_prefix("Statement"),
		}
	}
}

impl<'s> From<&MacroDefinition<'s>> for Node {
	fn from(value: &MacroDefinition<'s>) -> Self {
		let mut children = vec![Node {
			prefixes: vec!["Identifier".to_string()],
			repr:     value.id.to_string(),
			children: vec![],
		}];

		children.extend(value.rules.iter().map(|r| r.into()));

		Node { prefixes: vec![], repr: "MacroDefinition".to_string(), children }
	}
}

impl<'s> From<&MacroRule<'s>> for Node {
	fn from(value: &MacroRule<'s>) -> Self {
		let children = vec![
			Node {
				prefixes: vec![],
				repr:     "Matcher".to_string(),
				children: value.matcher.iter().map(|m| m.into()).collect(),
			},
			Node {
				prefixes: vec![],
				repr:     "Transcriber".to_string(),
				children: value
					.transcriber
					.iter()
					.map(|t| {
						Node {
							prefixes: vec!["Token".to_string()],
							repr:     t.to_string(),
							children: vec![],
						}
					})
					.collect(),
			},
		];

		Node { prefixes: vec![], repr: "MacroRule".to_string(), children }
	}
}

impl<'s> From<&MacroMatch<'s>> for Node {
	fn from(value: &MacroMatch<'s>) -> Self {
		match value {
			MacroMatch::Raw(t) => {
				Node {
					prefixes: vec!["Raw".to_string()],
					repr:     t.to_string(),
					children: vec![],
				}
			},
			MacroMatch::Typed { id, arg_type } => {
				Node {
					prefixes: vec![],
					repr:     "TypedMatch".to_string(),
					children: vec![
						Node {
							prefixes: vec!["Id".to_string()],
							repr:     id.to_string(),
							children: vec![],
						},
						arg_type.into(),
					],
				}
			},
			MacroMatch::Variadic { matches, rep_sep, var_type } => {
				let mut children = vec![];
				children.extend(matches.iter().map(|m| m.into()));
				if let Some(rp) = rep_sep {
					children.push(Node {
						prefixes: vec!["RepSep".to_string(), "Token".to_string()],
						repr:     rp.to_string(),
						children: vec![],
					});
				}
				children.push(var_type.into());

				Node { prefixes: vec![], repr: "VariadicMatch".to_string(), children }
			},
		}
	}
}

impl From<&MacroArgType> for Node {
	fn from(value: &MacroArgType) -> Self {
		let repr = match value {
			MacroArgType::Inst => "Inst".to_string(),
			MacroArgType::Reg => "Reg".to_string(),
			MacroArgType::Dir => "Dir".to_string(),
			MacroArgType::Ident => "Ident".to_string(),
			MacroArgType::Imm => "Imm".to_string(),
			MacroArgType::Stmt => "Stmt".to_string(),
		};

		Node { prefixes: vec![], repr, children: vec![] }
	}
}

impl From<&MacroVarType> for Node {
	fn from(value: &MacroVarType) -> Self {
		let repr = match value {
			MacroVarType::Optional => "Optional".to_string(),
			MacroVarType::OneOrMore => "OneOrMore".to_string(),
			MacroVarType::Any => "Any".to_string(),
		};

		Node { prefixes: vec![], repr, children: vec![] }
	}
}

impl<'s> From<&MacroInvocation<'s>> for Node {
	fn from(value: &MacroInvocation<'s>) -> Self {
		let mut children = vec![Node {
			prefixes: vec!["Identifier".to_string()],
			repr:     value.id.to_string(),
			children: vec![],
		}];

		children.extend(
			value
				.args
				.iter()
				.map(|tt| tt.to_string())
				.map(|s| Node { prefixes: vec!["Arg".to_string()], repr: s, children: vec![] }),
		);

		Node { prefixes: vec![], repr: "MacroInvocation".to_string(), children }
	}
}

impl<'s> From<&LabeledBlock<'s>> for Node {
	fn from(value: &LabeledBlock<'s>) -> Self {
		let mut children = vec![Node {
			prefixes: vec!["Label".to_string()],
			repr:     value.label.to_string(),
			children: vec![],
		}];

		children.extend(value.lines.iter().map(|l| l.into()));

		Node { prefixes: vec![], repr: "LabeledBlock".to_string(), children }
	}
}

impl<'s> From<&Directive<'s>> for Node {
	fn from(value: &Directive) -> Self {
		match value {
			Directive::Bytes(data) => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Bytes".to_string(),
					children: data.iter().map(|d| d.into()).collect(),
				}
			},
			Directive::Halves(data) => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Halves".to_string(),
					children: data.iter().map(|d| d.into()).collect(),
				}
			},
			Directive::Words(data) => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Words".to_string(),
					children: data.iter().map(|d| d.into()).collect(),
				}
			},
			Directive::ResBytes(data) => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "ResBytes".to_string(),
					children: data.iter().map(|d| d.into()).collect(),
				}
			},
			Directive::ResHalves(data) => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "ResHalves".to_string(),
					children: data.iter().map(|d| d.into()).collect(),
				}
			},
			Directive::ResWords(data) => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "ResWords".to_string(),
					children: data.iter().map(|d| d.into()).collect(),
				}
			},
			Directive::Const(const_dir) => Node::from(const_dir),
		}
	}
}

impl<'s> From<&Literal<'s>> for Node {
	fn from(value: &Literal) -> Self {
		match value {
			Literal::String(s) => {
				Node {
					prefixes: vec!["Literal".to_string(), "String".to_string()],
					repr:     format!("{:?}", s),
					children: vec![],
				}
			},
			Literal::Char(c) => {
				Node {
					prefixes: vec!["Literal".to_string(), "Char".to_string()],
					repr:     format!("{:?}", c),
					children: vec![],
				}
			},
			Literal::Immediate(imm) => imm.into(),
		}
	}
}

/// The big fella
impl<'s> From<&Instruction<'s>> for Node {
	fn from(value: &Instruction) -> Self {
		match value {
			Instruction::Addi { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Addi".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Slti { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Slti".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Sltiu { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sltiu".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Andi { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Andi".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Ori { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Ori".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Xori { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Xori".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Lsli { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsli".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Lsri { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsri".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Asri { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Asri".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Add { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Add".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Slt { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Slt".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Sltu { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sltu".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::And { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "And".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Or { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Or".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Xor { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Xor".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Lsl { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsl".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Lsr { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsr".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Asr { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Asr".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Sub { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sub".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Lui { dest, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lui".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Auipc { dest, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Auipc".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(imm).add_prefix("Imm"),
					],
				}
			},
			Instruction::Jal { dest, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Jal".to_string(),
					children: vec![
						Node::from(dest).add_prefix("dest"),
						Node::from(offset).add_prefix("offset"),
					],
				}
			},
			Instruction::Jalr { dest, base, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Jalr".to_string(),
					children: vec![
						Node::from(dest).add_prefix("dest"),
						Node::from(base).add_prefix("base"),
						Node::from(offset).add_prefix("offset"),
					],
				}
			},
			Instruction::Beq { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Beq".to_string(),
					children: vec![
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
						Node::from(offset).add_prefix("Offset"),
					],
				}
			},
			Instruction::Bne { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bne".to_string(),
					children: vec![
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
						Node::from(offset).add_prefix("Offset"),
					],
				}
			},
			Instruction::Blt { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Blt".to_string(),
					children: vec![
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
						Node::from(offset).add_prefix("Offset"),
					],
				}
			},
			Instruction::Bltu { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bltu".to_string(),
					children: vec![
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
						Node::from(offset).add_prefix("Offset"),
					],
				}
			},
			Instruction::Bge { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bge".to_string(),
					children: vec![
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
						Node::from(offset).add_prefix("Offset"),
					],
				}
			},
			Instruction::Bgeu { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bgeu".to_string(),
					children: vec![
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
						Node::from(offset).add_prefix("Offset"),
					],
				}
			},
			Instruction::Lb { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lb".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(addr).add_prefix("Addr"),
					],
				}
			},
			Instruction::Lbu { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lbu".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(addr).add_prefix("Addr"),
					],
				}
			},
			Instruction::Lh { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lh".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(addr).add_prefix("Addr"),
					],
				}
			},
			Instruction::Lhu { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lhu".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(addr).add_prefix("Addr"),
					],
				}
			},
			Instruction::Lw { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lw".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(addr).add_prefix("Addr"),
					],
				}
			},
			Instruction::Sb { dest, src } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sb".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
					],
				}
			},
			Instruction::Sh { dest, src } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sh".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
					],
				}
			},
			Instruction::Sw { dest, src } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sw".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
					],
				}
			},
			Instruction::Fence { pred, succ } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Fence".to_string(),
					children: vec![
						Node::from(pred).add_prefix("Pred"),
						Node::from(succ).add_prefix("Succ"),
					],
				}
			},
			Instruction::FenceTso { pred, succ } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "FenceTso".to_string(),
					children: vec![
						Node::from(pred).add_prefix("Pred"),
						Node::from(succ).add_prefix("Succ"),
					],
				}
			},
			Instruction::Ecall => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "ECall".to_string(),
					children: vec![],
				}
			},
			Instruction::Ebreak => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "EBreak".to_string(),
					children: vec![],
				}
			},
			Instruction::Fencei => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "FenceI".to_string(),
					children: vec![],
				}
			},
			Instruction::Csrrw { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRw".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(target).add_prefix("Target"),
					],
				}
			},
			Instruction::Csrrs { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRs".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(target).add_prefix("Target"),
					],
				}
			},
			Instruction::Csrrc { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRc".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(target).add_prefix("Target"),
					],
				}
			},
			Instruction::Csrrwi { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRwi".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(target).add_prefix("Target"),
					],
				}
			},
			Instruction::Csrrsi { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRsi".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(target).add_prefix("Target"),
					],
				}
			},
			Instruction::Csrrci { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRci".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src).add_prefix("Src"),
						Node::from(target).add_prefix("Target"),
					],
				}
			},
			Instruction::Mul { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Mul".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Mulh { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "MulH".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Mulhu { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "MulHU".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Mulhsu { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "MulHSU".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Div { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Div".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Divu { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "DivU".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Rem { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Rem".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
			Instruction::Remu { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "RemU".to_string(),
					children: vec![
						Node::from(dest).add_prefix("Dest"),
						Node::from(src1).add_prefix("Src1"),
						Node::from(src2).add_prefix("Src2"),
					],
				}
			},
		}
	}
}

impl From<&RegToken> for Node {
	fn from(value: &RegToken) -> Self {
		Node {
			prefixes: vec!["Register".to_string()],
			repr:     value.to_string(),
			children: vec![],
		}
	}
}

impl<'s> From<&Immediate<'s>> for Node {
	fn from(value: &Immediate) -> Self {
		Node { prefixes: vec![], repr: value.to_string(), children: vec![] }
	}
}

impl<'s> From<&Address<'s>> for Node {
	fn from(value: &Address) -> Self {
		let children = if let Some(offset) = &value.offset {
			vec![
				Node::from(&value.base).add_prefix("Base"),
				Node::from(&offset.op).add_prefix("Op"),
				Node::from(&offset.offset).add_prefix("Offset"),
			]
		} else {
			vec![Node::from(&value.base).add_prefix("Base")]
		};

		Node { prefixes: vec![], repr: "Address".to_string(), children }
	}
}

impl From<&OffsetOperator> for Node {
	fn from(value: &OffsetOperator) -> Self {
		let repr = match value {
			OffsetOperator::Plus => "+",
			OffsetOperator::Minus => "-",
		};

		Node { prefixes: vec![], repr: repr.to_string(), children: vec![] }
	}
}

impl From<&OrderingTarget> for Node {
	fn from(value: &OrderingTarget) -> Self {
		let mut repr = String::new();

		if value.contains(OrderingTarget::I) {
			repr.push('I');
		}
		if value.contains(OrderingTarget::O) {
			repr.push('O');
		}
		if value.contains(OrderingTarget::R) {
			repr.push('R');
		}
		if value.contains(OrderingTarget::W) {
			repr.push('W');
		}

		Node { prefixes: vec![], repr, children: vec![] }
	}
}
