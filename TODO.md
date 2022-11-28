# TODO

## Assembler

### Parser
 - [ ] More detailed parser errors (maybe see if lexer errors can be improved
       as well but i can't rlly think of anything)

### Macros
 - [ ] Macro definitions and invocations get parsed properly, but the
       invocations need to perform type checking and term rewriting (variadic
	   pattern matching will probably be painful) so they can actually modify
	   the AST
   - [ ] Update docs for assembler to explain macro rewriting

### Normalization
 - [ ] Immediate expressions should get evaluated parsing and macro rewriting
       has completed so they can be replaced by a single numeric literal
   - [ ] Update docs for assembler to explain constant folding

### Code Generation
 - [ ] Once the AST has been fully normalized it should be converted to an ELF
       object file so it can be linked into an executable

# Done
