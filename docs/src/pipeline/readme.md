# Pipeline

The pipeline for going from vildlang to vildcode, is quite linear, and not much unlike how many other languages do it. Though currently, not much of the pipeline is implemented. The plan is to have a system like this:

```d2
direction: down

Source code: { shape: rectangle }
Lexer: {}
Parser: {}
AST: { shape: rectangle }
Lowering: {}
Vildcode: { shape: rectangle }

Source code -> Lexer: tokens
Lexer -> Parser: token stream
Parser -> AST: builds
AST -> Lowering: consumes
Lowering -> Vildcode: emits VildHIR
```

Beyond this point, Nemunas takes the VildHIR and makes it into VildMIR, which then gets further compiled, until it is executable code.
