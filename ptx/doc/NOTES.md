I'm convinced nobody actually uses parser generators in Rust:
* pomelo can't generate lexer (understandable, as it is a port of lemon and lemon can't do this either)
* pest can't do parse actions, you have to convert your parse tree to ast manually
* lalrpop can't do comments
  * and the day I wrote the line above it can
* antlr4rust is untried and requires java to build