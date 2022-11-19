use super::{Node, ToNode};
use crate::lex::RegisterToken;
use crate::parse::{
	Address,
	Directive,
	Identifier,
	Immediate,
	Instruction,
	Line,
	Literal,
	OffsetOperator,
	OrderingTarget,
	Root,
	Statement, LabelId,
};

impl<'s> ToNode for Root<'s> {
	fn to_node(&self) -> Node {
		Node {
			prefixes: vec![],
			repr:     "Root".to_string(),
			children: self.lines.iter().map(|l| l.to_node()).collect(),
		}
	}
}

impl<'s> ToNode for Line<'s> {
	fn to_node(&self) -> Node {
		let mut children = vec![];

		if let Some(labl) = &self.labl {
			children.push(labl.to_node());
		}

		if let Some(stmt) = &self.stmt {
			children.push(stmt.to_node());
		}

		if let Some(cmnt) = self.cmnt {
			children.push(Node {
				prefixes: vec!["Comment".to_string()],
				repr:     format!("{:?}", cmnt),
				children: vec![],
			});
		}

		Node { prefixes: vec![], repr: "Line".to_string(), children }
	}
}

impl<'s> ToNode for LabelId<'s> {
	fn to_node(&self) -> Node {
		match self {
			Self::LabelDefine(id) => {
				Node {
					prefixes: vec![],
					repr: "Label".to_string(),
					children: vec![id.to_node()],
				}
			},
			Self::LocalLabelDefine(id) => {
				Node {
					prefixes: vec![],
					repr: "LocalLabel".to_string(),
					children: vec![id.to_node()],
				}
			},
		}
	}
}

impl<'s> ToNode for Statement<'s> {
	fn to_node(&self) -> Node {
		match self {
			Self::Directive(dir) => dir.to_node().add_prefix("Statement"),
			Self::Instruction(inst) => inst.to_node().add_prefix("Statement"),
		}
	}
}

impl<'s> ToNode for Identifier<'s> {
	fn to_node(&self) -> Node {
		Node {
			prefixes: vec!["Identifier".to_string()],
			repr:     self.0.to_string(),
			children: vec![],
		}
	}
}

impl<'s> ToNode for Directive<'s> {
	fn to_node(&self) -> Node {
		match self {
			Self::Byte { data } => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Byte".to_string(),
					children: data.iter().map(|d| d.to_node()).collect(),
				}
			},
			Self::Half { data } => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Half".to_string(),
					children: data.iter().map(|d| d.to_node()).collect(),
				}
			},
			Self::Word { data } => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Word".to_string(),
					children: data.iter().map(|d| d.to_node()).collect(),
				}
			},
			Self::Repeat { amount, argument } => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Repeat".to_string(),
					children: vec![
						amount.to_node().add_prefix("Amount"),
						argument.to_node().add_prefix("Argument"),
					],
				}
			},
			Self::Equ { id, value } => {
				Node {
					prefixes: vec!["Directive".to_string()],
					repr:     "Equ".to_string(),
					children: vec![
						id.to_node().add_prefix("Id"),
						value.to_node().add_prefix("Value"),
					],
				}
			},
		}
	}
}

impl<'s> ToNode for Literal<'s> {
	fn to_node(&self) -> Node {
		match self {
			Self::String(s) => {
				Node {
					prefixes: vec!["Literal".to_string(), "String".to_string()],
					repr:     format!("{:?}", s),
					children: vec![],
				}
			},
			Self::Char(c) => {
				Node {
					prefixes: vec!["Literal".to_string(), "Char".to_string()],
					repr:     format!("{:?}", c),
					children: vec![],
				}
			},
			Self::Number(n) => {
				Node {
					prefixes: vec!["Literal".to_string(), "Number".to_string()],
					repr:     n.to_string(),
					children: vec![],
				}
			},
			Self::Immediate(imm) => imm.to_node(),
		}
	}
}

/// The big fella
impl<'s> ToNode for Instruction<'s> {
	fn to_node(&self) -> Node {
		match self {
			Self::Addi { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Addi".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Slti { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Slti".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Sltiu { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sltiu".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Andi { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Andi".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Ori { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Ori".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Xori { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Xori".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Lsli { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsli".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Lsri { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsri".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Asri { dest, src, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Asri".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Add { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Add".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Slt { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Slt".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Sltu { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sltu".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::And { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "And".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Or { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Or".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Xor { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Xor".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Lsl { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsl".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Lsr { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lsr".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Asr { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Asr".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Sub { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sub".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Lui { dest, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lui".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Auipc { dest, imm } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Auipc".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						imm.to_node().add_prefix("Imm"),
					],
				}
			},
			Self::Jal { dest, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Jal".to_string(),
					children: vec![
						dest.to_node().add_prefix("dest"),
						offset.to_node().add_prefix("offset"),
					],
				}
			},
			Self::Jalr { dest, base, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Jalr".to_string(),
					children: vec![
						dest.to_node().add_prefix("dest"),
						base.to_node().add_prefix("base"),
						offset.to_node().add_prefix("offset"),
					],
				}
			},
			Self::Beq { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Beq".to_string(),
					children: vec![
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
						offset.to_node().add_prefix("Offset"),
					],
				}
			},
			Self::Bne { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bne".to_string(),
					children: vec![
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
						offset.to_node().add_prefix("Offset"),
					],
				}
			},
			Self::Blt { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Blt".to_string(),
					children: vec![
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
						offset.to_node().add_prefix("Offset"),
					],
				}
			},
			Self::Bltu { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bltu".to_string(),
					children: vec![
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
						offset.to_node().add_prefix("Offset"),
					],
				}
			},
			Self::Bge { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bge".to_string(),
					children: vec![
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
						offset.to_node().add_prefix("Offset"),
					],
				}
			},
			Self::Bgeu { src1, src2, offset } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Bgeu".to_string(),
					children: vec![
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
						offset.to_node().add_prefix("Offset"),
					],
				}
			},
			Self::Lb { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lb".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Lbu { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lbu".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Lh { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lh".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Lhu { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lhu".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Lw { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lw".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Lwu { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Lwu".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Sb { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sb".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Sh { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sh".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Sw { dest, addr } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Sw".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						addr.to_node().add_prefix("Addr"),
					],
				}
			},
			Self::Fence { pred, succ } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Fence".to_string(),
					children: vec![
						pred.to_node().add_prefix("Pred"),
						succ.to_node().add_prefix("Succ"),
					],
				}
			},
			Self::FenceTso { pred, succ } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "FenceTso".to_string(),
					children: vec![
						pred.to_node().add_prefix("Pred"),
						succ.to_node().add_prefix("Succ"),
					],
				}
			},
			Self::ECall => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "ECall".to_string(),
					children: vec![],
				}
			},
			Self::EBreak => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "EBreak".to_string(),
					children: vec![],
				}
			},
			Self::FenceI => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "FenceI".to_string(),
					children: vec![],
				}
			},
			Self::CsrRw { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRw".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						target.to_node().add_prefix("Target"),
					],
				}
			},
			Self::CsrRs { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRs".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						target.to_node().add_prefix("Target"),
					],
				}
			},
			Self::CsrRc { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRc".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						target.to_node().add_prefix("Target"),
					],
				}
			},
			Self::CsrRwi { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRwi".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						target.to_node().add_prefix("Target"),
					],
				}
			},
			Self::CsrRsi { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRsi".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						target.to_node().add_prefix("Target"),
					],
				}
			},
			Self::CsrRci { dest, src, target } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "CsrRci".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src.to_node().add_prefix("Src"),
						target.to_node().add_prefix("Target"),
					],
				}
			},
			Self::Mul { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Mul".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::MulH { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "MulH".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::MulHU { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "MulHU".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::MulHSU { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "MulHSU".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Div { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Div".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::DivU { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "DivU".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::Rem { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "Rem".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
			Self::RemU { dest, src1, src2 } => {
				Node {
					prefixes: vec!["Instruction".to_string()],
					repr:     "RemU".to_string(),
					children: vec![
						dest.to_node().add_prefix("Dest"),
						src1.to_node().add_prefix("Src1"),
						src2.to_node().add_prefix("Src2"),
					],
				}
			},
		}
	}
}

impl ToNode for RegisterToken {
	fn to_node(&self) -> Node {
		Node {
			prefixes: vec!["Register".to_string()],
			repr:     self.to_string(),
			children: vec![],
		}
	}
}

impl<'s> ToNode for Immediate<'s> {
	fn to_node(&self) -> Node {
		Node { prefixes: vec![], repr: self.to_string(), children: vec![] }
	}
}

impl<'s> ToNode for Address<'s> {
	fn to_node(&self) -> Node {
		let children = if let Some(offset) = &self.offset {
			vec![
				self.base.to_node().add_prefix("Base"),
				offset.op.to_node().add_prefix("Op"),
				offset.imm.to_node().add_prefix("Imm"),
			]
		} else {
			vec![self.base.to_node().add_prefix("Base")]
		};

		Node { prefixes: vec![], repr: "Address".to_string(), children }
	}
}

impl ToNode for OffsetOperator {
	fn to_node(&self) -> Node {
		let repr = match self {
			Self::Plus => "+",
			Self::Minus => "-",
		};

		Node { prefixes: vec![], repr: repr.to_string(), children: vec![] }
	}
}

impl ToNode for OrderingTarget {
	fn to_node(&self) -> Node {
		let mut repr = String::new();

		if self.contains(Self::I) {
			repr.push('I');
		}
		if self.contains(Self::O) {
			repr.push('O');
		}
		if self.contains(Self::R) {
			repr.push('R');
		}
		if self.contains(Self::W) {
			repr.push('W');
		}

		Node { prefixes: vec![], repr, children: vec![] }
	}
}
