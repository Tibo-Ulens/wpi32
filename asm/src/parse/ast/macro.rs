use super::{Line, Statement};
use crate::lex::TokenType;
/// A macro definition
///
/// ```ebnf
/// macro_definiton =
///     "define_macro", "!",
///     identifier,
///     macro_body;
/// macro_body =
///     ( "(", { macro_rules }, ")" )
///     | ( "[", { macro_rules }, "]" )
///     | ( "{", { macro_rules }, "}" );
/// macro_rules = macro_rule, { comma, macro_rule };
/// ```
#[derive(Clone, Debug)]
pub struct MacroDefinition<'s> {
	/// The name of the macro getting defined
	id:    &'s str,
	/// The rules making up this macro
	rules: Vec<MacroRule<'s>>,
}

/// One of the rules making up a specific macro
///
/// Contains a list of [`MacroMatch`]es to specify an argument pattern to
/// detect and a transcriber containing one or more [`Line`]s which will get
/// substituted into the AST when the macro gets called
///
/// ```ebnf
/// macro_rule = macro_matcher, "=>", macro_transcriber;
/// macro_matcher = "(", { macro_match } ")";
/// macro_transcriber =
///     line
///     | ( "(", { line }, ")" )
///     | ( "[", { line }, "]" )
///     | ( "{", { line }, "}" );
/// ```
#[derive(Clone, Debug)]
pub struct MacroRule<'s> {
	/// The pattern matching this rule
	matcher:     Vec<MacroMatch<'s>>,
	/// The body of the rule
	transcriber: Vec<Line<'s>>,
}

/// A single pattern to get matched against in macros
///
/// Can be a raw [`TokenType`] literal, a single [typed](MacroArgType)
/// identifier, or a variadic matcher
///
/// ```ebnf
/// macro_match =
///     ?tokentype?
///     | ( "$", identifier, ":", macro_arg_type )
///     | ( "$", "(", ( macro_match, { macro_match } ), ")", macro_var_type );
/// macro_arg_type = "inst", "reg", "dir", "ident", "imm", "stmt";
/// macro_var_type = "?" | "+" | "*";
/// ```
#[derive(Clone, Debug)]
pub enum MacroMatch<'s> {
	/// A literal string of characters
	Raw(TokenType<'s>),
	/// A single argument
	Typed { id: &'s str, arg_type: MacroArgType },
	/// A variadic (list of) arguments
	Variadic { matches: Vec<MacroMatch<'s>>, var_type: MacroVarType },
}

/// The possible type specifiers that can be used in a [`MacroMatch`]
#[derive(Clone, Debug)]
pub enum MacroArgType {
	/// Any instruction keyword
	Inst,
	/// Any register keyword
	Reg,
	/// Any directive keyword
	Dir,
	/// Any identifier
	Ident,
	/// Any immediate
	Imm,
	/// Any statement
	Stmt,
}

/// The possible types of 'variadicity' that a variadic [`MacroMatch`] can
/// detect
#[derive(Clone, Debug)]
pub enum MacroVarType {
	/// 0 or 1 matches
	Optional,
	/// 1 or more matches
	OneOrMore,
	/// 0 or more matches
	Any,
}

/// A macro invocation
///
/// ```ebnf
/// macro_invocation =
///     identifier, "!",
///     | ( "(", { statement }, ")" )
///     | ( "[", { statement }, "]" )
///     | ( "{", { statement }, "}" );
/// ```
#[derive(Clone, Debug)]
pub struct MacroInvocation<'s> {
	/// The name of the macro getting called
	id:   &'s str,
	/// The arguments passed to the macro
	args: Vec<Statement<'s>>,
}
