I'm convinced nobody actually uses parser generators in Rust:
* pomelo can't generate lexer (understandable, as it is a port of lemon and lemon can't do this either)
* pest can't do parse actions, you have to convert your parse tree to ast manually
* lalrpop can't do comments
  * and the day I wrote the line above it can
  * reports parsing errors as byte offsets
  * if you want to skip parsing one of the alternatives functional design gets quite awkward
* antlr4rust is untried and requires java to build
* no library supports island grammars

What to emit?
* SPIR-V
  * Better library support, easier to emit
  * Can by optimized by IGC
  * Can't do some things (not sure what exactly yet)
    * But we can work around things with inline VISA
* VISA
  * Quicker compilation