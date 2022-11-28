#[rustfmt::skip]
pub(super) static TEST_SOURCE_CODE: &str = "\
; Test header comment
;

define_macro! movi {
	($r:reg, $( $i:ident ),+) => {
		addi $r, r0, $( $i )++
	}
}

#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))

#SECTION .text

_start {
	nested {
		movi!(r1, 5, 6, 7)
	}
}
";

#[rustfmt::skip]
pub(super) static TEST_TOKENS_STRING: &str = r###"[001:001]: COMMENT    "; Test header comment"         "```; Test header comment```\n"
[001:022]: SYMBOL     \n                              "; Test header comment```\n```"
[002:001]: COMMENT    ";"                             "```;```\n"
[002:002]: SYMBOL     \n                              ";```\n```"
[003:001]: SYMBOL     \n                              "```\n```"
[004:001]: IDENTIFIER define_macro                    "```define_macro```! movi {\n"
[004:013]: OPERATOR   !                               "define_macro```!``` movi {\n"
[004:015]: IDENTIFIER movi                            "define_macro! ```movi``` {\n"
[004:020]: SYMBOL     {{                              "define_macro! movi ```{```\n"
[004:021]: SYMBOL     \n                              "define_macro! movi {```\n```"
[005:002]: SYMBOL     (                               "```(```$r:reg, $( $i:ident ),+) => {\n"
[005:003]: OPERATOR   $                               "(```$```r:reg, $( $i:ident ),+) => {\n"
[005:004]: IDENTIFIER r                               "($```r```:reg, $( $i:ident ),+) => {\n"
[005:005]: OPERATOR   :                               "($r```:```reg, $( $i:ident ),+) => {\n"
[005:006]: IDENTIFIER reg                             "($r:```reg```, $( $i:ident ),+) => {\n"
[005:009]: SYMBOL     ,                               "($r:reg```,``` $( $i:ident ),+) => {\n"
[005:011]: OPERATOR   $                               "($r:reg, ```$```( $i:ident ),+) => {\n"
[005:012]: SYMBOL     (                               "($r:reg, $```(``` $i:ident ),+) => {\n"
[005:014]: OPERATOR   $                               "($r:reg, $( ```$```i:ident ),+) => {\n"
[005:015]: IDENTIFIER i                               "($r:reg, $( $```i```:ident ),+) => {\n"
[005:016]: OPERATOR   :                               "($r:reg, $( $i```:```ident ),+) => {\n"
[005:017]: IDENTIFIER ident                           "($r:reg, $( $i:```ident``` ),+) => {\n"
[005:023]: SYMBOL     )                               "($r:reg, $( $i:ident ```)```,+) => {\n"
[005:024]: SYMBOL     ,                               "($r:reg, $( $i:ident )```,```+) => {\n"
[005:025]: OPERATOR   +                               "($r:reg, $( $i:ident ),```+```) => {\n"
[005:026]: SYMBOL     )                               "($r:reg, $( $i:ident ),+```)``` => {\n"
[005:028]: SYMBOL     =>                              "($r:reg, $( $i:ident ),+) ```=>``` {\n"
[005:031]: SYMBOL     {{                              "($r:reg, $( $i:ident ),+) => ```{```\n"
[005:032]: SYMBOL     \n                              "($r:reg, $( $i:ident ),+) => {```\n```"
[006:003]: INSTRUCTION addi                           "```addi``` $r, r0, $( $i )++\n"
[006:008]: OPERATOR   $                               "addi ```$```r, r0, $( $i )++\n"
[006:009]: IDENTIFIER r                               "addi $```r```, r0, $( $i )++\n"
[006:010]: SYMBOL     ,                               "addi $r```,``` r0, $( $i )++\n"
[006:012]: REGISTER   r0                              "addi $r, ```r0```, $( $i )++\n"
[006:014]: SYMBOL     ,                               "addi $r, r0```,``` $( $i )++\n"
[006:016]: OPERATOR   $                               "addi $r, r0, ```$```( $i )++\n"
[006:017]: SYMBOL     (                               "addi $r, r0, $```(``` $i )++\n"
[006:019]: OPERATOR   $                               "addi $r, r0, $( ```$```i )++\n"
[006:020]: IDENTIFIER i                               "addi $r, r0, $( $```i``` )++\n"
[006:022]: SYMBOL     )                               "addi $r, r0, $( $i ```)```++\n"
[006:023]: OPERATOR   +                               "addi $r, r0, $( $i )```+```+\n"
[006:024]: OPERATOR   +                               "addi $r, r0, $( $i )+```+```\n"
[006:025]: SYMBOL     \n                              "addi $r, r0, $( $i )++```\n```"
[007:002]: SYMBOL     }}                              "```}```\n"
[007:003]: SYMBOL     \n                              "}```\n```"
[008:001]: SYMBOL     }}                              "```}```\n"
[008:002]: SYMBOL     \n                              "}```\n```"
[009:001]: SYMBOL     \n                              "```\n```"
[010:001]: DIRECTIVE  #CONST                          "```#CONST``` large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:008]: IDENTIFIER large_immediate                 "#CONST ```large_immediate``` 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:024]: NUM        0                               "#CONST large_immediate ```0``` ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:026]: OPERATOR   ?                               "#CONST large_immediate 0 ```?``` (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:028]: SYMBOL     (                               "#CONST large_immediate 0 ? ```(```1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:029]: NUM        1234                            "#CONST large_immediate 0 ? (```1_234``` + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:035]: OPERATOR   +                               "#CONST large_immediate 0 ? (1_234 ```+``` 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:037]: NUM        2                               "#CONST large_immediate 0 ? (1_234 + ```2``` * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:039]: OPERATOR   *                               "#CONST large_immediate 0 ? (1_234 + 2 ```*``` 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:041]: NUM        59                              "#CONST large_immediate 0 ? (1_234 + 2 * ```0o73```) << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:045]: SYMBOL     )                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73```)``` << (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:047]: OPERATOR   <<                              "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) ```<<``` (4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:050]: SYMBOL     (                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << ```(```4 ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:051]: NUM        4                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (```4``` ^ 0x5) : (0b1011 & (8 % 5))\n"
[010:053]: OPERATOR   ^                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ```^``` 0x5) : (0b1011 & (8 % 5))\n"
[010:055]: NUM        5                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ ```0x5```) : (0b1011 & (8 % 5))\n"
[010:058]: SYMBOL     )                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5```)``` : (0b1011 & (8 % 5))\n"
[010:060]: OPERATOR   :                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) ```:``` (0b1011 & (8 % 5))\n"
[010:062]: SYMBOL     (                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : ```(```0b1011 & (8 % 5))\n"
[010:063]: NUM        11                              "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (```0b1011``` & (8 % 5))\n"
[010:070]: OPERATOR   &                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 ```&``` (8 % 5))\n"
[010:072]: SYMBOL     (                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & ```(```8 % 5))\n"
[010:073]: NUM        8                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (```8``` % 5))\n"
[010:075]: OPERATOR   %                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 ```%``` 5))\n"
[010:077]: NUM        5                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % ```5```))\n"
[010:078]: SYMBOL     )                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5```)```)\n"
[010:079]: SYMBOL     )                               "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5)```)```\n"
[010:080]: SYMBOL     \n                              "#CONST large_immediate 0 ? (1_234 + 2 * 0o73) << (4 ^ 0x5) : (0b1011 & (8 % 5))```\n```"
[011:001]: SYMBOL     \n                              "```\n```"
[012:001]: DIRECTIVE  #SECTION                        "```#SECTION``` .text\n"
[012:010]: SECTION    .text                           "#SECTION ```.text```\n"
[012:015]: SYMBOL     \n                              "#SECTION .text```\n```"
[013:001]: SYMBOL     \n                              "```\n```"
[014:001]: IDENTIFIER _start                          "```_start``` {\n"
[014:008]: SYMBOL     {{                              "_start ```{```\n"
[014:009]: SYMBOL     \n                              "_start {```\n```"
[015:002]: IDENTIFIER nested                          "```nested``` {\n"
[015:009]: SYMBOL     {{                              "nested ```{```\n"
[015:010]: SYMBOL     \n                              "nested {```\n```"
[016:003]: IDENTIFIER movi                            "```movi```!(r1, 5, 6, 7)\n"
[016:007]: OPERATOR   !                               "movi```!```(r1, 5, 6, 7)\n"
[016:008]: SYMBOL     (                               "movi!```(```r1, 5, 6, 7)\n"
[016:009]: REGISTER   r1                              "movi!(```r1```, 5, 6, 7)\n"
[016:011]: SYMBOL     ,                               "movi!(r1```,``` 5, 6, 7)\n"
[016:013]: NUM        5                               "movi!(r1, ```5```, 6, 7)\n"
[016:014]: SYMBOL     ,                               "movi!(r1, 5```,``` 6, 7)\n"
[016:016]: NUM        6                               "movi!(r1, 5, ```6```, 7)\n"
[016:017]: SYMBOL     ,                               "movi!(r1, 5, 6```,``` 7)\n"
[016:019]: NUM        7                               "movi!(r1, 5, 6, ```7```)\n"
[016:020]: SYMBOL     )                               "movi!(r1, 5, 6, 7```)```\n"
[016:021]: SYMBOL     \n                              "movi!(r1, 5, 6, 7)```\n```"
[017:002]: SYMBOL     }}                              "```}```\n"
[017:003]: SYMBOL     \n                              "}```\n```"
[018:001]: SYMBOL     }}                              "```}```\n"
[018:002]: SYMBOL     \n                              "}```\n```""###;

#[rustfmt::skip]
pub(super) static TEST_AST_STRING: &str = r#"Root
  Preamble
    PreambleLine
      (Comment) "; Test header comment"
    PreambleLine
      (Comment) ";"
    Empty
    PreambleLine
      (PreambleStatement) MacroDefinition
        (Identifier) movi
        MacroRule
          Matcher
            TypedMatch
              (Id) r
              Reg
            (Raw) COMMA
            VariadicMatch
              TypedMatch
                (Id) i
                Ident
              (RepSep) (Token) COMMA
              OneOrMore
          Transcriber
            (Token) NEWLINE
            (Token) addi
            (Token) $
            (Token) r
            (Token) COMMA
            (Token) r0
            (Token) COMMA
            (Token) $
            (Token) (
            (Token) $
            (Token) i
            (Token) )
            (Token) +
            (Token) +
            (Token) NEWLINE
    Empty
    PreambleLine
      (PreambleStatement) (Directive) Const
        (Id) large_immediate
        (Value) 0 1234 2 59 * + 4 5 ^ << 11 8 5 % & : ?
    Empty
  Sections
    Section
      (Name) .text
      Lines
        Empty
        Line
          (Content) (Statement) LabeledBlock
            (Label) _start
            Empty
            Line
              (Content) (Statement) LabeledBlock
                (Label) nested
                Empty
                Line
                  (Content) (Statement) MacroInvocation
                    (Identifier) movi
                    (Arg) r1
                    (Arg) COMMA
                    (Arg) 5
                    (Arg) COMMA
                    (Arg) 6
                    (Arg) COMMA
                    (Arg) 7
"#;
